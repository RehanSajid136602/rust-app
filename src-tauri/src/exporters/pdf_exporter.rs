//! PDF export for invoices and quotations.

use crate::models::{Invoice, Quotation, CompanySettings, PaymentStatus};
use genpdf::{self, Alignment, Element as _};
use genpdf::{elements, fonts, style};
use genpdf::PageDecorator;
use genpdf::render;
use genpdf::error;
use genpdf::Mm;
use genpdf::Position;
use genpdf::Scale;
use genpdf::Rotation;
use rust_decimal::Decimal;
use image23::io::Reader as ImageReader23;
use image23::imageops::FilterType;
use image23::DynamicImage;

const HEADER_BYTES: &[u8] = include_bytes!("../../../public/header.png");
const WHATSAPP_BYTES: &[u8] = include_bytes!("../../../public/whatsapp.png");
const EMAIL_BYTES: &[u8] = include_bytes!("../../../public/email.png");
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
    // Show integer without decimals if whole number, otherwise 2 decimal places
    if frac == 0 {
        format!("{}{}", sign, with_commas)
    } else {
        format!("{}{}.{:02}", sign, with_commas, frac)
    }
}

/// Format date from YYYY-MM-DD to DD-MM-YYYY
fn fmt_date(date_str: &str) -> String {
    if date_str.is_empty() {
        return String::new();
    }
    // If already in DD-MM-YYYY format, return as-is
    if date_str.contains('-') && date_str.len() == 10 {
        let parts: Vec<&str> = date_str.split('-').collect();
        if parts.len() == 3 && parts[0].len() == 2 {
            // Already DD-MM-YYYY format
            return date_str.to_string();
        }
        // Convert from YYYY-MM-DD to DD-MM-YYYY
        if parts.len() == 3 && parts[0].len() == 4 {
            return format!("{}-{}-{}", parts[2], parts[1], parts[0]);
        }
    }
    date_str.to_string()
}

/// Convert image data to RGB PNG bytes (genpdf doesn't support alpha)
fn img_to_rgb_png(data: &[u8]) -> Option<Vec<u8>> {
    let img = image::load_from_memory(data).ok()?;
    let rgb = img.to_rgb8();
    let mut buf = std::io::Cursor::new(Vec::new());
    rgb.write_to(&mut buf, image::ImageFormat::Png).ok()?;
    Some(buf.into_inner())
}

/// Resize image to fit max width while maintaining aspect ratio, return RGB PNG bytes
fn resize_to_fit(data: &[u8], max_width: u32) -> Option<Vec<u8>> {
    let img = image::load_from_memory(data).ok()?;
    let (w, h) = (img.width(), img.height());
    if w <= max_width {
        return img_to_rgb_png(data);
    }
    let ratio = max_width as f64 / w as f64;
    let new_h = (h as f64 * ratio) as u32;
    let resized = img.resize(max_width, new_h, image::imageops::FilterType::Lanczos3);
    let rgb = resized.to_rgb8();
    let mut buf = std::io::Cursor::new(Vec::new());
    rgb.write_to(&mut buf, image::ImageFormat::Png).ok()?;
    Some(buf.into_inner())
}

/// Load and resize an icon from embedded bytes to the given pixel dimensions
fn decode_icon(data: &[u8], size: u32) -> Option<DynamicImage> {
    let img = ImageReader23::new(std::io::Cursor::new(data))
        .with_guessed_format().ok()?
        .decode().ok()?;
    Some(img.resize_exact(size, size, FilterType::Lanczos3))
}

/// Strip unicode box-drawing chars, non-printable, and control chars from text
fn sanitize_text(s: &str) -> String {
    s.chars().filter(|c| {
        let u = *c as u32;
        // Keep printable ASCII + common Latin supplement + spaces + common punctuation
        // Strip: control chars (0-31, 127), box drawing, geometric shapes, misc symbols, dingbats
        !(u <= 31 || u == 127
            || (0x2500..=0x27BF).contains(&u)  // box drawing, block elements, geometric shapes, misc symbols, dingbats
            || (0x1F300..=0x1F9FF).contains(&u)  // emoji/symbols
            || (0xFE00..=0xFE0F).contains(&u)    // variation selectors
        )
    }).collect()
}

/// Custom page decorator that pins a styled footer at the bottom of each page.
struct FooterDecorator {
    inner: genpdf::SimplePageDecorator,
    address: String,
    phone1: String,
    phone2: String,
    email: String,
    wa_icon: DynamicImage,
    em_icon: DynamicImage,
}

impl FooterDecorator {
    fn new(address: String, phone1: String, phone2: String, email: String) -> Self {
        let wa = decode_icon(WHATSAPP_BYTES, 64).expect("Failed to decode WhatsApp icon");
        let em = decode_icon(EMAIL_BYTES, 64).expect("Failed to decode email icon");
        Self { inner: genpdf::SimplePageDecorator::new(), address, phone1, phone2, email, wa_icon: wa, em_icon: em }
    }

    fn set_margins(&mut self, margins: impl Into<genpdf::Margins>) {
        self.inner.set_margins(margins);
    }
}

impl PageDecorator for FooterDecorator {
    fn decorate_page<'a>(
        &mut self,
        context: &genpdf::Context,
        mut area: render::Area<'a>,
        style: style::Style,
    ) -> Result<render::Area<'a>, error::Error> {
        area = self.inner.decorate_page(context, area, style)?;

        let footer_h = Mm::from(15.0);
        let content_h = area.size().height;
        if content_h <= footer_h + Mm::from(5.0) {
            return Ok(area);
        }

        // ---- ROW 1: Red address bar ----
        {
            let areas = area.split_horizontally(&[1]);
            let mut row1 = areas.into_iter().next().unwrap();
            row1.add_offset(Position::new(Mm::from(0.0), content_h - footer_h));
            row1.set_height(Mm::from(6.5));

            let addr_clean = sanitize_text(&self.address);
            if !addr_clean.is_empty() {
                let red_style = style::Style::new()
                    .bold()
                    .with_font_size(9)
                    .with_color(style::Color::Rgb(0xE8, 0x37, 0x2A));
                let mut p = elements::Paragraph::new(addr_clean)
                    .aligned(Alignment::Center)
                    .styled(red_style)
                    .padded(2);
                p.render(context, row1, style)?;
            }
        }

        // ---- ROW 2: Contact icons + text ----
        let phone1 = sanitize_text(&self.phone1);
        let phone2 = sanitize_text(&self.phone2);
        let email = sanitize_text(&self.email);

        let mut entries: Vec<(&DynamicImage, &str)> = Vec::new();
        if !phone1.is_empty() { entries.push((&self.wa_icon, &phone1)); }
        if !phone2.is_empty() { entries.push((&self.wa_icon, &phone2)); }
        if !email.is_empty() { entries.push((&self.em_icon, &email)); }

        if !entries.is_empty() {
            let n = entries.len();
            let item_w: usize = 22;
            let gap_w: usize = 1;
            let total_content = item_w * n + gap_w * n.saturating_sub(1);
            let margin_w = (100usize.saturating_sub(total_content)) / 2;

            let mut weights: Vec<usize> = vec![margin_w];
            for i in 0..n {
                weights.push(item_w);
                if i < n - 1 {
                    weights.push(gap_w);
                }
            }
            weights.push(margin_w);

            let areas2 = area.split_horizontally(&[1]);
            let mut row2 = areas2.into_iter().next().unwrap();
            row2.add_offset(Position::new(Mm::from(0.0), content_h - footer_h + Mm::from(7.0)));
            row2.set_height(Mm::from(7.5));
            let cols = row2.split_horizontally(&weights);

            let txt_style = style::Style::new().with_font_size(8);
            // Item columns are at odd indices: 1, 3, 5, ... (0 is left margin, 2 is gap, etc.)
            for (i, (icon, text)) in entries.iter().enumerate() {
                let col_idx = 1 + i * 2;
                cols[col_idx].add_image(icon, Position::new(Mm::from(0.5), Mm::from(0.5)),
                    Scale::new(1.0, 1.0), Rotation::default(), Some(300.0));
                let _ = cols[col_idx].print_str(&context.font_cache,
                    Position::new(Mm::from(6.5), Mm::from(2.0)), txt_style, text);
            }
        }

        // Shrink content area so content doesn't overflow into footer
        area.set_height(content_h - footer_h - Mm::from(2.0));
        Ok(area)
    }
}

fn render_header(doc: &mut genpdf::Document) {
    let data = resize_to_fit(HEADER_BYTES, 1800).unwrap_or_else(|| HEADER_BYTES.to_vec());
    if let Ok(img) = elements::Image::from_reader(std::io::Cursor::new(data)) {
        doc.push(img);
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

    // Bordered invoice reference box: 2 columns, 2 rows
    let mut ref_box = elements::TableLayout::new(vec![50, 50]);
    ref_box.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));

    // Row 1: Document Type + Number | Date + Extra Date
    let left1 = if !extra_val.is_empty() {
        format!("{} # {}  |  {}: {}", doc_type, doc_number, extra_label, fmt_date(extra_val))
    } else {
        format!("{} # {}", doc_type, doc_number)
    };
    let right1 = if !date_val.is_empty() {
        format!("Date: {}", fmt_date(date_val))
    } else {
        String::new()
    };
    ref_box.row()
        .element(elements::Paragraph::new(left1).styled(small_bold).padded(3))
        .element(elements::Paragraph::new(right1).aligned(Alignment::Right).styled(small).padded(3))
        .push().expect("ref box row 1");

    // Row 2: Ref | To: Client
    let ref_text = if !ref_number.is_empty() {
        format!("Ref: {}", ref_number)
    } else {
        String::new()
    };
    let client = sanitize_text(client_name.trim());
    let addr = sanitize_text(client_address.trim());
    let to_text = if !addr.is_empty() {
        format!("To: {} ({})", client, addr)
    } else if !client.is_empty() {
        format!("To: {}", client)
    } else {
        String::new()
    };
    ref_box.row()
        .element(elements::Paragraph::new(ref_text).padded(3))
        .element(elements::Paragraph::new(to_text).aligned(Alignment::Right).styled(style::Effect::Bold).padded(3))
        .push().expect("ref box row 2");

    doc.push(ref_box);
    doc.push(elements::Break::new(0.4));

    if !settings.salutation.is_empty() {
        doc.push(elements::Paragraph::new(&settings.salutation).styled(small));
    }
    if !settings.body_text.is_empty() {
        doc.push(
            elements::Paragraph::new(&settings.body_text)
                .styled(style::Style::new().italic().with_font_size(8))
                .padded((0, 0, 6, 0)),
        );
    }
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
        let row_total = if item.total_price == Decimal::ZERO && (item.quantity != Decimal::ZERO || item.price_per_unit != Decimal::ZERO) {
            item.quantity * item.price_per_unit
        } else {
            item.total_price
        };
        table.row()
            .element(elements::Paragraph::new(format!("{}", item.sno)).aligned(Alignment::Center).padded(1))
            .element(elements::Paragraph::new(&item.item_name).aligned(Alignment::Center).padded(1))
            .element(elements::Paragraph::new(fmt_amount(item.quantity)).aligned(Alignment::Center).padded(1))
            .element(elements::Paragraph::new(fmt_amount(item.price_per_unit)).aligned(Alignment::Center).padded(1))
            .element(elements::Paragraph::new(fmt_amount(row_total)).aligned(Alignment::Center).styled(bold).padded(1))
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

    let mut decorator = FooterDecorator::new(
        settings.office_address.clone(),
        settings.phone1.clone(),
        settings.phone2.clone(),
        settings.email.clone(),
    );
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
        doc.push(elements::Break::new(0.3));
    }

    doc.push(elements::Break::new(0.3));
    doc.push(
        elements::Paragraph::new("Thank you for considering Zahra Enterprises as business partner.")
            .aligned(Alignment::Center)
            .styled(style::Style::new().with_font_size(8)),
    );
    doc.push(
        elements::Paragraph::new("And hope for a good business relationship in future.")
            .aligned(Alignment::Center)
            .styled(style::Style::new().with_font_size(8)),
    );

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

    let mut decorator = FooterDecorator::new(
        settings.office_address.clone(),
        settings.phone1.clone(),
        settings.phone2.clone(),
        settings.email.clone(),
    );
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
        doc.push(elements::Break::new(0.3));
    }

    doc.push(elements::Break::new(0.3));
    doc.push(
        elements::Paragraph::new("Thank you for considering Zahra Enterprises as business partner.")
            .aligned(Alignment::Center)
            .styled(style::Style::new().with_font_size(8)),
    );
    doc.push(
        elements::Paragraph::new("And hope for a good business relationship in future.")
            .aligned(Alignment::Center)
            .styled(style::Style::new().with_font_size(8)),
    );

    doc.render_to_file(output_path)
        .map_err(|e| format!("Failed to write PDF: {}", e))?;

    Ok(())
}
