use rust_xlsxwriter::*;
use rust_decimal::Decimal;
use crate::models::Invoice;

fn d_to_f64(d: Decimal) -> f64 {
    d.to_string().parse().unwrap_or(0.0)
}

macro_rules! xl {
    ($expr:expr) => {
        $expr.map_err(|e| e.to_string())?
    };
}

pub fn export_invoices_excel(invoices: &[Invoice], output_path: &str) -> Result<(), String> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    let headers = [
        "Invoice #", "Client", "Date", "Due Date", "Status",
        "Item", "Qty", "Price", "Discount", "Tax", "Item Total",
        "Subtotal", "Discount Total", "Tax Total", "Adjustment", "Net Amount",
        "Paid", "Remaining",
    ];

    let hdr = Format::new()
        .set_bold()
        .set_background_color(Color::RGB(0x1A2540))
        .set_font_color(Color::White)
        .set_border(FormatBorder::Thin)
        .set_text_wrap();

    let cell = Format::new().set_border(FormatBorder::Thin);
    let money = Format::new().set_border(FormatBorder::Thin).set_num_format("#,##0.00");
    let green = Format::new().set_border(FormatBorder::Thin).set_font_color(Color::Green).set_num_format("#,##0.00");
    let red = Format::new().set_border(FormatBorder::Thin).set_font_color(Color::Red).set_num_format("#,##0.00");

    for (col, h) in headers.iter().enumerate() {
        xl!(worksheet.write_with_format(0, col as u16, *h, &hdr));
    }

    let widths = [14u16, 22, 12, 12, 10, 28, 8, 10, 10, 10, 12, 12, 12, 12, 10, 12, 10, 10];
    for (i, w) in widths.iter().enumerate() {
        xl!(worksheet.set_column_width(i as u16, *w));
    }

    let mut row = 1u32;

    for inv in invoices {
        if inv.items.is_empty() {
            xl!(worksheet.write_with_format(row, 0, &inv.invoice_number, &cell));
            xl!(worksheet.write_with_format(row, 1, &inv.client_name, &cell));
            xl!(worksheet.write_with_format(row, 2, &inv.invoice_date, &cell));
            xl!(worksheet.write_with_format(row, 3, &inv.due_date, &cell));
            xl!(worksheet.write_with_format(row, 4, inv.status.as_str(), &cell));
            xl!(worksheet.write_with_format(row, 11, d_to_f64(inv.subtotal), &money));
            xl!(worksheet.write_with_format(row, 15, d_to_f64(inv.total), &money));
            xl!(worksheet.write_with_format(row, 16, d_to_f64(inv.amount_paid), &green));
            xl!(worksheet.write_with_format(row, 17, d_to_f64(inv.remaining_debt), &red));
            row += 1;
            continue;
        }

        for item in &inv.items {
            let rem = d_to_f64(inv.remaining_debt);
            let rem_fmt = if rem > 0.0 { &red } else { &green };

            xl!(worksheet.write_with_format(row, 0, &inv.invoice_number, &cell));
            xl!(worksheet.write_with_format(row, 1, &inv.client_name, &cell));
            xl!(worksheet.write_with_format(row, 2, &inv.invoice_date, &cell));
            xl!(worksheet.write_with_format(row, 3, &inv.due_date, &cell));
            xl!(worksheet.write_with_format(row, 4, inv.status.as_str(), &cell));
            xl!(worksheet.write_with_format(row, 5, &item.item_name, &cell));
            xl!(worksheet.write_with_format(row, 6, d_to_f64(item.quantity), &cell));
            xl!(worksheet.write_with_format(row, 7, d_to_f64(item.price_per_unit), &money));
            xl!(worksheet.write_with_format(row, 8, d_to_f64(item.discount_amount), &money));
            xl!(worksheet.write_with_format(row, 9, d_to_f64(item.tax_amount), &money));
            xl!(worksheet.write_with_format(row, 10, d_to_f64(item.total_price), &money));
            xl!(worksheet.write_with_format(row, 11, d_to_f64(inv.subtotal), &money));
            xl!(worksheet.write_with_format(row, 12, d_to_f64(inv.discount_total), &money));
            xl!(worksheet.write_with_format(row, 13, d_to_f64(inv.tax_total), &money));
            xl!(worksheet.write_with_format(row, 14, d_to_f64(inv.adjustment_amount), &money));
            xl!(worksheet.write_with_format(row, 15, d_to_f64(inv.total), &money));
            xl!(worksheet.write_with_format(row, 16, d_to_f64(inv.amount_paid), &green));
            xl!(worksheet.write_with_format(row, 17, rem, rem_fmt));

            row += 1;
        }
    }

    xl!(worksheet.set_freeze_panes(1, 0));
    xl!(workbook.save(output_path));
    Ok(())
}
