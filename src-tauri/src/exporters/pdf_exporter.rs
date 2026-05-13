//! PDF export for invoices and quotations.

use crate::models::{Invoice, Quotation, CompanySettings, PaymentStatus};
use genpdf::{self, Alignment, Element as _};
use genpdf::{elements, fonts, style};
use rust_decimal::Decimal;
use image::GenericImageView;

const HEADER_IMG: &str = "../public/header.png";
const FOOTER_IMG: &str = "../public/footer.png";
const FONT_DIRS: &[&str] = &[
    "/usr/share/fonts/liberation",
    "/usr/share/fonts/truetype/liberation",
    "/usr/share/fonts/TTF",
    "/usr/share/fonts/truetype",
    "/usr/share/fonts/truetype/dejavu",
];

fn load_font() -> fonts::FontFamily<fonts::FontData> {
    let font_dir = FONT_DIRS
        .iter()
        .find(|path| std::path::Path::new(path).exists())
        .unwrap_or(&"/usr/share/fonts");

    fonts::from_files(font_dir, "LiberationSans", None)
        .or_else(|_| fonts::from_files(font_dir, "DejaVuSans", None))
        .or_else(|_| {
            for dir in FONT_DIRS {
                if let Ok(f) = fonts::from_files(dir, "LiberationSans", None) {
                    return Ok(f);
                }
            }
            Err(genpdf::error::Error::new("No font found", genpdf::error::ErrorKind::InvalidFont))
        })
        .expect("Could not find any suitable font. Install liberation-fonts or dejavu-fonts.")
}

fn fmt_amount(n: Decimal) -> String {
    let n_f64: f64 = n.try_into().unwrap_or(0.0);
    let whole = n_f64.trunc() as i64;
    let frac = (n_f64.fract().abs() * 100.0).round() as u32;
    let with_commas = whole
        .abs()
        .to_string()
        .chars()
        .rev()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(",")
        .chars()
        .rev()
        .collect::<String>();
    let sign = if whole < 0 { "-" } else { "" };
    format!("{}{}.{:02}", sign, with_commas, frac)
}

fn resize_image(path: &str, max_width: u32) -> Option<Vec<u8>> {
    let img = image::open(path).ok()?;
    let (w, h) = img.dimensions();
    if w <= max_width {
        return std::fs::read(path).ok();
    }
    let ratio = max_width as f64 / w as f64;
    let new_h = (h as f64 * ratio) as u32;
    let resized = img.resize(max_width, new_h, image::imageops::FilterType::Lanczos3);
    let mut buf = std::io::Cursor::new(Vec::new());
    resized.write_to(&mut buf, image::ImageFormat::Png).ok()?;
    Some(buf.into_inner())
}

fn render_header(doc: &mut genpdf::Document) {
    if let Some(data) = resize_image(HEADER_IMG, 1800) {
        if let Ok(img) = elements::Image::from_reader(std::io::Cursor::new(data)) {
            doc.push(img);
        }
    }
    doc.push(elements::Break::new(0.4));
}

fn render_footer(doc: &mut genpdf::Document) {
    doc.push(elements::Break::new(0.5));
    if let Some(data) = resize_image(FOOTER_IMG, 900) {
        if let Ok(img) = elements::Image::from_reader(std::io::Cursor::new(data)) {
            doc.push(img);
        }
    }
}

fn render_document_info(
    doc: &mut genpdf::Document,
    doc_type: &str,
    doc_number: &str,
    date_val: &str,
    extra_label: &str,
    extra_val: &str,
    ref_number: &str,
    client_name: &str,
    client_address: &str,
    settings: &CompanySettings,
) {
    let small = style::Style::new().with_font_size(9);
    let small_bold = style::Style::new().with_font_size(9).bold();

    // Document header line
    let mut header_parts = vec![format!("{} {}", doc_type, doc_number)];
    if !date_val.is_empty() {
        header_parts.push(format!("Date: {}", date_val));
    }
    if !extra_val.is_empty() {
        header_parts.push(format!("{}: {}", extra_label, extra_val));
    }
    if !ref_number.is_empty() {
        header_parts.push(format!("Ref: {}", ref_number));
    }
    doc.push(
        elements::Paragraph::new(header_parts.join("  |  "))
            .styled(small_bold),
    );
    doc.push(elements::Break::new(0.3));

    // Client info
    if !client_name.is_empty() {
        doc.push(elements::Paragraph::new("To,").styled(small));
        doc.push(
            elements::Paragraph::new(client_name)
                .styled(style::Effect::Bold)
                .styled(small),
        );
        if !client_address.is_empty() {
            doc.push(elements::Paragraph::new(client_address).styled(small));
        }
        doc.push(elements::Break::new(0.2));
    }

    if !settings.salutation.is_empty() {
        doc.push(elements::Paragraph::new(&settings.salutation).styled(small));
    }
    if !settings.body_text.is_empty() {
        doc.push(elements::Break::new(0.1));
        doc.push(
            elements::Paragraph::new(&settings.body_text)
                .styled(style::Style::new().italic().with_font_size(8)),
        );
    }
    doc.push(elements::Break::new(0.5));
}

fn render_items_with_totals(doc: &mut genpdf::Document, items: &[crate::models::InvoiceItem], invoice: &Invoice) {
    if items.is_empty() {
        return;
    }

    let bold = style::Effect::Bold;
    let small = style::Style::new().with_font_size(9);
    let net_style = style::Style::new().with_font_size(10).bold();

    // Column weights: S.No=6, Item=38, Qty=12, Price=16, Total=20
    let mut table = elements::TableLayout::new(vec![6, 38, 12, 16, 20]);
    table.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));

    // Header row
    table.row()
        .element(elements::Paragraph::new("S.No").aligned(Alignment::Center).styled(bold).padded(1))
        .element(elements::Paragraph::new("Item Description").aligned(Alignment::Center).styled(bold).padded(1))
        .element(elements::Paragraph::new("Qty").aligned(Alignment::Center).styled(bold).padded(1))
        .element(elements::Paragraph::new("Price").aligned(Alignment::Center).styled(bold).padded(1))
        .element(elements::Paragraph::new("Total").aligned(Alignment::Center).styled(bold).padded(1))
        .push().expect("header row");

    // Body rows
    for item in items {
        table.row()
            .element(elements::Paragraph::new(format!("{}", item.sno)).aligned(Alignment::Center).padded(1))
            .element(elements::Paragraph::new(&item.item_name).aligned(Alignment::Center).padded(1))
            .element(elements::Paragraph::new(fmt_amount(item.quantity)).aligned(Alignment::Center).padded(1))
            .element(elements::Paragraph::new(fmt_amount(item.price_per_unit)).aligned(Alignment::Center).padded(1))
            .element(elements::Paragraph::new(fmt_amount(item.total_price)).aligned(Alignment::Center).styled(bold).padded(1))
            .push().expect("item row");
    }

    // ---- Totals section inside the table ----

    // Subtotal
    table.row()
        .element(elements::Paragraph::new("").padded(0))
        .element(elements::Paragraph::new("").padded(0))
        .element(elements::Paragraph::new("").padded(0))
        .element(elements::Paragraph::new("Subtotal").aligned(Alignment::Center).styled(small).padded(1))
        .element(elements::Paragraph::new(fmt_amount(invoice.subtotal)).aligned(Alignment::Center).styled(small).padded(1))
        .push().expect("subtotal row");

    // Discount
    if invoice.discount_total != Decimal::ZERO {
        table.row()
            .element(elements::Paragraph::new("").padded(0))
            .element(elements::Paragraph::new("").padded(0))
            .element(elements::Paragraph::new("").padded(0))
            .element(elements::Paragraph::new("Discount").aligned(Alignment::Center).styled(small).padded(1))
            .element(elements::Paragraph::new(fmt_amount(invoice.discount_total)).aligned(Alignment::Center).styled(small).padded(1))
            .push().expect("discount row");
    }

    // Tax
    if invoice.tax_total != Decimal::ZERO {
        table.row()
            .element(elements::Paragraph::new("").padded(0))
            .element(elements::Paragraph::new("").padded(0))
            .element(elements::Paragraph::new("").padded(0))
            .element(elements::Paragraph::new("Tax").aligned(Alignment::Center).styled(small).padded(1))
            .element(elements::Paragraph::new(fmt_amount(invoice.tax_total)).aligned(Alignment::Center).styled(small).padded(1))
            .push().expect("tax row");
    }

    // Adjustment
    if invoice.adjustment_amount != Decimal::ZERO {
        let lbl = if invoice.adjustment_label.is_empty() { "Adjustment" } else { &invoice.adjustment_label };
        table.row()
            .element(elements::Paragraph::new("").padded(0))
            .element(elements::Paragraph::new("").padded(0))
            .element(elements::Paragraph::new("").padded(0))
            .element(elements::Paragraph::new(lbl).aligned(Alignment::Center).styled(small).padded(1))
            .element(elements::Paragraph::new(fmt_amount(invoice.adjustment_amount)).aligned(Alignment::Center).styled(small).padded(1))
            .push().expect("adjustment row");
    }

    // Net Amount
    table.row()
        .element(elements::Paragraph::new("").padded(0))
        .element(elements::Paragraph::new("").padded(0))
        .element(elements::Paragraph::new("").padded(0))
        .element(elements::Paragraph::new("Net Amount").aligned(Alignment::Center).styled(net_style).padded(1))
        .element(elements::Paragraph::new(fmt_amount(invoice.total)).aligned(Alignment::Center).styled(net_style).padded(1))
        .push().expect("net amount row");

    doc.push(table);
    doc.push(elements::Break::new(0.4));
}

// ---- Public exporters ----

pub fn export_invoice_pdf(
    invoice: &Invoice,
    settings: &CompanySettings,
    output_path: &str,
) -> Result<(), String> {
    let font = load_font();
    let mut doc = genpdf::Document::new(font);
    let doc_num = if invoice.invoice_number.is_empty() { &invoice.ref_number } else { &invoice.invoice_number };
    doc.set_title(&format!("Invoice {}", doc_num));
    doc.set_minimal_conformance();

    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    render_header(&mut doc);
    render_document_info(
        &mut doc, "Invoice", doc_num,
        &invoice.invoice_date, "Due Date", &invoice.due_date,
        &invoice.ref_number, &invoice.client_name, &invoice.client_address,
        settings,
    );
    render_items_with_totals(&mut doc, &invoice.items, invoice);

    if !invoice.notes.is_empty() {
        doc.push(
            elements::Paragraph::new(format!("Notes: {}", invoice.notes))
                .styled(style::Style::new().italic().with_font_size(8)),
        );
        doc.push(elements::Break::new(0.5));
    }

    render_footer(&mut doc);

    doc.render_to_file(output_path)
        .map_err(|e| format!("Failed to write PDF: {}", e))?;

    Ok(())
}

pub fn export_quotation_pdf(
    quotation: &Quotation,
    settings: &CompanySettings,
    output_path: &str,
) -> Result<(), String> {
    let font = load_font();
    let mut doc = genpdf::Document::new(font);
    doc.set_title(&format!("Quotation {}", quotation.quotation_number));
    doc.set_minimal_conformance();

    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    render_header(&mut doc);
    render_document_info(
        &mut doc, "Quotation", &quotation.quotation_number,
        &quotation.quotation_date, "Valid Until", &quotation.valid_until,
        &quotation.ref_number, &quotation.client_name, &quotation.client_address,
        settings,
    );

    let invoice_items: Vec<crate::models::InvoiceItem> = quotation.items.iter().map(|qi| {
        crate::models::InvoiceItem {
            id: None, invoice_id: None, sno: qi.sno,
            item_name: qi.item_name.clone(), quantity: qi.quantity,
            price_per_unit: qi.price_per_unit, discount_amount: qi.discount_amount,
            tax_amount: qi.tax_amount, total_price: qi.total_price,
        }
    }).collect();

    let temp_invoice = Invoice {
        id: None,
        invoice_number: quotation.quotation_number.clone(),
        ref_number: quotation.ref_number.clone(),
        client_id: quotation.client_id,
        client_name: quotation.client_name.clone(),
        client_address: quotation.client_address.clone(),
        invoice_date: quotation.quotation_date.clone(),
        due_date: quotation.valid_until.clone(),
        subtotal: quotation.subtotal,
        tax_total: quotation.tax_total,
        discount_total: quotation.discount_total,
        grand_total: quotation.grand_total,
        amount_paid: Decimal::ZERO,
        remaining_debt: Decimal::ZERO,
        adjustment_label: quotation.adjustment_label.clone(),
        adjustment_amount: quotation.adjustment_amount,
        total: quotation.total,
        notes: quotation.notes.clone(),
        status: PaymentStatus::Unpaid,
        created_at: None,
        items: invoice_items.clone(),
    };
    render_items_with_totals(&mut doc, &invoice_items, &temp_invoice);

    if !quotation.notes.is_empty() {
        doc.push(
            elements::Paragraph::new(format!("Notes: {}", quotation.notes))
                .styled(style::Style::new().italic().with_font_size(8)),
        );
        doc.push(elements::Break::new(0.5));
    }

    render_footer(&mut doc);

    doc.render_to_file(output_path)
        .map_err(|e| format!("Failed to write PDF: {}", e))?;

    Ok(())
}
