export interface CompanySettings {
  company_name: string
  address: string
  phone: string
  email: string
  gst_number: string
  pan_number: string
  tagline?: string
  ntn_number?: string
  office_address?: string
  phone1?: string
  phone2?: string
  logo_path?: string
  invoice_prefix?: string
  next_invoice_number?: number
  quotation_prefix?: string
  next_quotation_number?: number
  salutation?: string
  body_text?: string
  banner_color?: string
  footer_color?: string
}

export interface UpdateSettingsRequest {
  company_name?: string
  address?: string
  phone?: string
  email?: string
  gst_number?: string
  pan_number?: string
}
