//! CompanySettings model.

use serde::{Deserialize, Serialize};

/// Company settings (singleton - only one row in DB)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanySettings {
    pub id: i32,
    pub company_name: String,
    pub tagline: String,
    pub ntn_number: String,
    pub office_address: String,
    pub phone1: String,
    pub phone2: String,
    pub email: String,
    pub logo_path: String,
    pub invoice_prefix: String,
    pub next_invoice_number: i32,
    pub quotation_prefix: String,
    pub next_quotation_number: i32,
    pub salutation: String,
    pub body_text: String,
    pub banner_color: String,
    pub footer_color: String,
}

impl CompanySettings {
    /// Create default company settings
    pub fn default_settings() -> Self {
        Self {
            id: 1,
            company_name: "ZAHRA ENTERPRISES".to_string(),
            tagline: "Deals in lab Consumables, Reagents & Medical Equipments.".to_string(),
            ntn_number: "NTN NO. 2140708-8".to_string(),
            office_address: "Office # 2-3, Basement Asif Plaza, Fazal-e-Haq Road, Blue Area, Islamabad".to_string(),
            phone1: "0300-5259751".to_string(),
            phone2: "0345-8510130".to_string(),
            email: "zahraenterprises4@gmail.com".to_string(),
            logo_path: String::new(),
            invoice_prefix: "ZE".to_string(),
            next_invoice_number: 1,
            quotation_prefix: "QT #".to_string(),
            next_quotation_number: 1,
            salutation: "Respected Sir,".to_string(),
            body_text: "This is with reference to our quotation submitted; we are pleased to inform you that we have delivered following items.".to_string(),
            banner_color: "#1a2540".to_string(),
            footer_color: "#e05a2b".to_string(),
        }
    }
    
    /// Generate the next invoice number
    pub fn generate_invoice_number(&self, year: u32) -> String {
        format!("{}-{}-{:04}", self.invoice_prefix, year, self.next_invoice_number)
    }
    
    /// Generate the next quotation number
    pub fn generate_quotation_number(&self, year: u32) -> String {
        format!("{}-{}-{:04}", self.quotation_prefix, year, self.next_quotation_number)
    }
}

impl Default for CompanySettings {
    fn default() -> Self {
        Self::default_settings()
    }
}

/// Request DTO for updating company settings
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateSettingsRequest {
    pub company_name: String,
    pub tagline: String,
    pub ntn_number: String,
    pub office_address: String,
    pub phone1: String,
    pub phone2: String,
    pub email: String,
    pub logo_path: String,
    pub invoice_prefix: String,
    pub quotation_prefix: String,
    pub salutation: String,
    pub body_text: String,
    pub banner_color: String,
    pub footer_color: String,
}
