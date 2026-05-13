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
        // Image is already small enough, return original bytes
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
    if let Some(data) = resize_image(HEADER_IMG, 800) {
        if let Ok(img) = elements::Image::from_reader(std::io::Cursor::new(data)) {
            doc.push(img);
        }
    }
    doc.push(elements::Break::new(0.3));
}

fn render_footer(doc: &mut genpdf::Document) {
    doc.push(elements::Break::new(0.5));
    if let Some(data) = resize_image(FOOTER_IMG, 800) {
        if let Ok(img) = elements::Image::from_reader(std::io::Cursor::new(data)) {
            doc.push(img);
        }
    }
}

fn render_company_info(doc: &mut genpdf::Document, settings: &CompanySettings) {
    doc.push(
        elements::Paragraph::new(&settings.company_name)
            .aligned(Alignment::Center)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(13)),
    );
    if !settings.tagline.is_empty() {
        doc.push(
            elements::Paragraph::new(&settings.tagline)
                .aligned(Alignment::Center)
                .styled(style::Style::new().italic().with_font_size(7)),
        );
    }
    if !settings.office_address.is_empty() {
        doc.push(
            elements::Paragraph::new(format!(
                "{}, Ph: {}, {}",
                settings.office_address, settings.phone1, settings.email
            ))
            .aligned(Alignment::Center)
            .styled(style::Style::new().with_font_size(7)),
        );
    }
    if !settings.ntn_number.is_empty() {
        doc.push(
            elements::Paragraph::new(format!("NTN: {}", settings.ntn_number))
                .aligned(Alignment::Center)
                .styled(style::Effect::Bold)
                .styled(style::Style::new().with_font_size(9)),
        );
    }
    doc.push(elements::Break::new(0.4));
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

    // Document header: type, number, date
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

    // Client address block
    if !client_name.is_empty() {
        doc.push(
            elements::Paragraph::new(format!("To,"))
                .styled(small),
        );
        doc.push(
            elements::Paragraph::new(client_name)
                .styled(style::Effect::Bold)
                .styled(small),
        );
        if !client_address.is_empty() {
            doc.push(
                elements::Paragraph::new(client_address)
                    .styled(small),
            );
        }
        doc.push(elements::Break::new(0.2));
    }

    // Salutation
    if !settings.salutation.is_empty() {
        doc.push(
            elements::Paragraph::new(&settings.salutation)
                .styled(small),
        );
    }
    // Body text
    if !settings.body_text.is_empty() {
        doc.push(elements::Break::new(0.1));
        doc.push(
            elements::Paragraph::new(&settings.body_text)
                .styled(style::Style::new().italic().with_font_size(8)),
        );
    }
    doc.push(elements::Break::new(0.1));
}

fn render_items_table(doc: &mut genpdf::Document, items: &[crate::models::InvoiceItem]) {
    if items.is_empty() {
        return;
    }

    // Column weights: #=4, Item=40, Qty=12, Price=16, Total=20
    let mut table = elements::TableLayout::new(vec![4, 40, 12, 16, 20]);
    table.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));

    // Header row
    let header_style = style::Effect::Bold;
    table
        .row()
        .element(
            elements::Paragraph::new("#")
                .aligned(Alignment::Center)
                .styled(header_style)
                .padded(1),
        )
        .element(
            elements::Paragraph::new("Item Description")
                .styled(header_style)
                .padded(1),
        )
        .element(
            elements::Paragraph::new("Qty")
                .aligned(Alignment::Center)
                .styled(header_style)
                .padded(1),
        )
        .element(
            elements::Paragraph::new("Price")
                .aligned(Alignment::Right)
                .styled(header_style)
                .padded(1),
        )
        .element(
            elements::Paragraph::new("Total")
                .aligned(Alignment::Right)
                .styled(header_style)
                .padded(1),
        )
        .push()
        .expect("Invalid table header row");

    // Body rows
    for item in items {
        table
            .row()
            .element(
                elements::Paragraph::new(format!("{}", item.sno))
                    .aligned(Alignment::Center)
                    .padded(1),
            )
            .element(elements::Paragraph::new(&item.item_name).padded(1))
            .element(
                elements::Paragraph::new(fmt_amount(item.quantity))
                    .aligned(Alignment::Right)
                    .padded(1),
            )
            .element(
                elements::Paragraph::new(fmt_amount(item.price_per_unit))
                    .aligned(Alignment::Right)
                    .padded(1),
            )
            .element(
                elements::Paragraph::new(fmt_amount(item.total_price))
                    .aligned(Alignment::Right)
                    .styled(style::Effect::Bold)
                    .padded(1),
            )
            .push()
            .expect("Invalid table row");
    }

    doc.push(table);
    doc.push(elements::Break::new(0.4));
}

fn render_totals(doc: &mut genpdf::Document, invoice: &Invoice) {
    let mut table = elements::TableLayout::new(vec![1, 1]);
    let right = Alignment::Right;
    let font_size = style::Style::new().with_font_size(9);
    let font_bold = style::Style::new().with_font_size(9).bold();

    // Subtotal
    table.row()
        .element(elements::Paragraph::new("Subtotal").aligned(right).styled(font_size).padded(1))
        .element(elements::Paragraph::new(fmt_amount(invoice.subtotal)).aligned(right).styled(font_size).padded(1))
        .push().expect("subtotal row");

    // Discount (if any)
    if invoice.discount_total != Decimal::ZERO {
        table.row()
            .element(elements::Paragraph::new("Less: Discount").aligned(right).styled(font_size).padded(1))
            .element(elements::Paragraph::new(fmt_amount(invoice.discount_total)).aligned(right).styled(font_size).padded(1))
            .push().expect("discount row");
    }

    // Tax (if any)
    if invoice.tax_total != Decimal::ZERO {
        table.row()
            .element(elements::Paragraph::new("Add: Tax").aligned(right).styled(font_size).padded(1))
            .element(elements::Paragraph::new(fmt_amount(invoice.tax_total)).aligned(right).styled(font_size).padded(1))
            .push().expect("tax row");
    }

    if invoice.subtotal != invoice.grand_total {
        table.row()
            .element(elements::Paragraph::new("Grand Total").aligned(right).styled(font_bold).padded(1))
            .element(elements::Paragraph::new(fmt_amount(invoice.grand_total)).aligned(right).styled(font_bold).padded(1))
            .push().expect("grand total row");
    }

    // Adjustment (if non-zero)
    if invoice.adjustment_amount != Decimal::ZERO {
        let label = if invoice.adjustment_label.is_empty() { "Adjustment" } else { &invoice.adjustment_label };
        table.row()
            .element(elements::Paragraph::new(label).aligned(right).styled(font_size).padded(1))
            .element(elements::Paragraph::new(fmt_amount(invoice.adjustment_amount)).aligned(right).styled(font_size).padded(1))
            .push().expect("adjustment row");
    }

    // Net Amount
    let net_style = style::Style::new().with_font_size(10).bold();
    table.row()
        .element(elements::Paragraph::new("Net Amount").aligned(right).styled(net_style).padded(1))
        .element(elements::Paragraph::new(fmt_amount(invoice.total)).aligned(right).styled(net_style).padded(1))
        .push().expect("net amount row");

    doc.push(table);
    doc.push(elements::Break::new(0.3));
}

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
    render_company_info(&mut doc, settings);
    render_document_info(
        &mut doc, "Invoice", doc_num,
        &invoice.invoice_date, "Due Date", &invoice.due_date,
        &invoice.ref_number, &invoice.client_name, &invoice.client_address,
        settings,
    );
    render_items_table(&mut doc, &invoice.items);
    render_totals(&mut doc, invoice);

    if !invoice.notes.is_empty() {
        doc.push(
            elements::Paragraph::new(format!("Notes: {}", invoice.notes))
                .styled(style::Style::new().italic().with_font_size(8)),
        );
        doc.push(elements::Break::new(0.15));
    }

    if invoice.amount_paid > Decimal::ZERO {
        let status_str = match invoice.status {
            PaymentStatus::Paid => "PAID",
            PaymentStatus::Partial => "PARTIALLY PAID",
            PaymentStatus::Unpaid => "UNPAID",
        };
        doc.push(
            elements::Paragraph::new(format!(
                "Paid: Rs. {}   Balance: Rs. {}   Status: {}",
                fmt_amount(invoice.amount_paid),
                fmt_amount(invoice.remaining_debt),
                status_str,
            ))
            .styled(style::Style::new().bold().with_font_size(8)),
        );
        doc.push(elements::Break::new(0.15));
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
    render_company_info(&mut doc, settings);
    render_document_info(
        &mut doc, "Quotation", &quotation.quotation_number,
        &quotation.quotation_date, "Valid Until", &quotation.valid_until,
        &quotation.ref_number, &quotation.client_name, &quotation.client_address,
        settings,
    );

    let invoice_items: Vec<crate::models::InvoiceItem> = quotation
        .items
        .iter()
        .map(|qi| crate::models::InvoiceItem {
            id: None,
            invoice_id: None,
            sno: qi.sno,
            item_name: qi.item_name.clone(),
            quantity: qi.quantity,
            price_per_unit: qi.price_per_unit,
            discount_amount: qi.discount_amount,
            tax_amount: qi.tax_amount,
            total_price: qi.total_price,
        })
        .collect();

    render_items_table(&mut doc, &invoice_items);

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
        items: invoice_items,
    };
    render_totals(&mut doc, &temp_invoice);

    if !quotation.notes.is_empty() {
        doc.push(
            elements::Paragraph::new(format!("Notes: {}", quotation.notes))
                .styled(style::Style::new().italic().with_font_size(8)),
        );
        doc.push(elements::Break::new(0.15));
    }

    doc.push(
        elements::Paragraph::new(format!("Status: {}", quotation.status))
            .styled(style::Style::new().bold().with_font_size(8)),
    );
    doc.push(elements::Break::new(0.15));

    render_footer(&mut doc);

    doc.render_to_file(output_path)
        .map_err(|e| format!("Failed to write PDF: {}", e))?;

    Ok(())
}
