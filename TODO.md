# Zahra Invoice Desktop - Detailed TODO

**Last Updated:** May 12, 2026
**Current Phase:** 4 (Frontend UI) ✅

---

## Legend

- [ ] Task not started
- [~] Task in progress
- [x] Task complete
- [!] Blocked

---

## PHASE 1: Foundation

### 1.1 Project Setup ✅

- [x] Create project directory
- [x] Initialize Tauri v2 project
- [x] Install npm dependencies
- [x] Update Cargo.toml with dependencies
- [x] Fix crate name in main.rs
- [x] Verify `cargo check` passes
- [x] Create ARCHITECTURE.md
- [x] Create MIGRATION.md
- [x] Create TODO.md (this file)

**Verification:** `cargo check` and `npm run build` pass

---

### 1.2 Database Layer [~]

#### 1.2.1 Connection Module
- [ ] Create `src-tauri/src/database/mod.rs`
- [ ] Implement `get_connection()` function
- [ ] Configure WAL mode
- [ ] Configure foreign keys
- [ ] Configure busy timeout
- [ ] Add connection pooling (Arc<Mutex>)

#### 1.2.2 Migrations
- [ ] Create `src-tauri/migrations/` directory
- [ ] Create `001_initial.sql` from Python schema
- [ ] Implement migration runner in `database/migrations.rs`
- [ ] Create migrations table if not exists
- [ ] Test migration execution

#### 1.2.3 Database Initialization
- [ ] Create `init_database()` function
- [ ] Run migrations on startup
- [ ] Seed default company settings
- [ ] Test with existing `invoice_app.db`

**Verification:** Can query existing data from Python DB

**Blockers:** None
**Estimated:** 16 hours

---

### 1.3 Domain Models [~]

#### 1.3.1 Core Models
- [ ] Create `src-tauri/src/models/mod.rs`
- [ ] Create `Product` struct with serde
- [ ] Create `Client` struct with serde
- [ ] Create `Invoice` struct with serde
- [ ] Create `InvoiceItem` struct with serde
- [ ] Create `Quotation` struct with serde
- [ ] Create `QuotationItem` struct with serde
- [ ] Create `CompanySettings` struct with serde
- [ ] Create `LedgerEntry` struct with serde

#### 1.3.2 Helper Types
- [ ] Create `Totals` struct (subtotal, tax, discount, grand_total)
- [ ] Create `PaymentStatus` enum (unpaid, partial, paid)
- [ ] Create `CreateInvoiceRequest` DTO
- [ ] Create `UpdateInvoiceRequest` DTO

#### 1.3.3 Serialization Tests
- [ ] Test JSON serialization
- [ ] Test deserialization
- [ ] Verify field names match frontend expectations

**Verification:** `cargo test models` passes

**Blockers:** None
**Estimated:** 8 hours

---

### 1.4 Repositories [~]

#### 1.4.1 Product Repository
- [ ] Create `src-tauri/src/repositories/mod.rs`
- [ ] Create `repositories/product_repo.rs`
- [ ] Implement `get_all()`
- [ ] Implement `get_by_id()`
- [ ] Implement `search()` with fuzzy matching
- [ ] Implement `create()`
- [ ] Implement `update()`
- [ ] Implement `delete()`
- [ ] Implement `upsert()` (for import)

#### 1.4.2 Client Repository
- [ ] Create `repositories/client_repo.rs`
- [ ] Implement `get_all()`
- [ ] Implement `get_by_id()`
- [ ] Implement `create()`
- [ ] Implement `update()`
- [ ] Implement `delete()`
- [ ] Implement `update_balance()`
- [ ] Implement `get_with_ledger()`

#### 1.4.3 Invoice Repository
- [ ] Create `repositories/invoice_repo.rs`
- [ ] Implement `get_all()`
- [ ] Implement `get_by_id()` with items
- [ ] Implement `create()` with items
- [ ] Implement `update()` with items
- [ ] Implement `delete()`
- [ ] Implement `get_next_invoice_number()`
- [ ] Implement `get_by_client()`

#### 1.4.4 Quotation Repository
- [ ] Create `repositories/quotation_repo.rs`
- [ ] Implement same methods as InvoiceRepository
- [ ] Implement `get_next_quotation_number()`

#### 1.4.5 Settings Repository
- [ ] Create `repositories/settings_repo.rs`
- [ ] Implement `get()` (singleton)
- [ ] Implement `update()`
- [ ] Implement `get_next_invoice_number()`
- [ ] Implement `increment_invoice_number()`

#### 1.4.6 Ledger Repository
- [ ] Create `repositories/ledger_repo.rs`
- [ ] Implement `get_by_client()`
- [ ] Implement `add_entry()`
- [ ] Implement `delete_by_invoice()`
- [ ] Implement `calculate_balance()`

**Verification:** All repos have unit tests

**Blockers:** Models must be complete
**Estimated:** 24 hours

---

## PHASE 2: Core Features

### 2.1 Services [~]

#### 2.1.1 Invoice Service
- [ ] Create `src-tauri/src/services/mod.rs`
- [ ] Create `services/invoice_service.rs`
- [ ] Implement `calculate_totals()`
- [ ] Implement `create()`
- [ ] Implement `update()`
- [ ] Implement `validate()`

#### 2.1.2 Quotation Service
- [ ] Create `services/quotation_service.rs`
- [ ] Implement same methods as InvoiceService

#### 2.1.3 Payment Service
- [ ] Create `services/payment_service.rs`
- [ ] Implement `record_payment()`
- [ ] Implement `update_ledger()`
- [ ] Implement `calculate_remaining_debt()`

#### 2.1.4 Search Service
- [ ] Create `services/search_service.rs`
- [ ] Implement `search_products()` with strsim
- [ ] Configure fuzzy threshold
- [ ] Add caching

#### 2.1.5 Settings Service
- [ ] Create `services/settings_service.rs`
- [ ] Implement `get_settings()`
- [ ] Implement `update_settings()`

**Verification:** Service tests pass

**Blockers:** Repositories must be complete
**Estimated:** 20 hours

---

### 2.2 Tauri Commands [~]

#### 2.2.1 Command Module
- [ ] Create `src-tauri/src/commands/mod.rs`
- [ ] Register all commands in `lib.rs`

#### 2.2.2 Invoice Commands
- [ ] Create `commands/invoice.rs`
- [ ] Implement `get_invoices()`
- [ ] Implement `get_invoice(id)`
- [ ] Implement `create_invoice(data)`
- [ ] Implement `update_invoice(id, data)`
- [ ] Implement `delete_invoice(id)`

#### 2.2.3 Quotation Commands
- [ ] Create `commands/quotation.rs`
- [ ] Implement same commands as invoices

#### 2.2.4 Client Commands
- [ ] Create `commands/client.rs`
- [ ] Implement `get_clients()`
- [ ] Implement `get_client(id)`
- [ ] Implement `create_client(data)`
- [ ] Implement `update_client(id, data)`
- [ ] Implement `delete_client(id)`
- [ ] Implement `get_client_ledger(id)`
- [ ] Implement `record_payment(data)`

#### 2.2.5 Product Commands
- [ ] Create `commands/product.rs`
- [ ] Implement `get_products()`
- [ ] Implement `search_products(query)`
- [ ] Implement `create_product(data)`
- [ ] Implement `update_product(id, data)`
- [ ] Implement `delete_product(id)`

#### 2.2.6 Settings Commands
- [ ] Create `commands/settings.rs`
- [ ] Implement `get_settings()`
- [ ] Implement `update_settings(data)`

**Verification:** Frontend can invoke all commands

**Blockers:** Services must be complete
**Estimated:** 12 hours

---

### 2.3 Frontend Setup [~]

#### 2.3.1 Dependencies
- [ ] Install Pinia: `npm install pinia`
- [ ] Install Vue Router: `npm install vue-router`
- [ ] Install Headless UI: `npm install @headlessui/vue`
- [ ] Install Heroicons: `npm install @heroicons/vue`

#### 2.3.2 Configuration
- [ ] Configure Tailwind v4 in `tailwind.config.ts`
- [ ] Add custom colors (navy #1a2540, orange #e05a2b)
- [ ] Configure Vue Router
- [ ] Configure Pinia stores

#### 2.3.3 Base Components
- [ ] Create `components/ui/Button.vue`
- [ ] Create `components/ui/Input.vue`
- [ ] Create `components/ui/Select.vue`
- [ ] Create `components/ui/Modal.vue`
- [ ] Create `components/ui/Toast.vue`
- [ ] Create `components/ui/Card.vue`

#### 2.3.4 Form Components
- [ ] Create `components/forms/MoneyInput.vue`
- [ ] Create `components/forms/DateInput.vue`
- [ ] Create `components/forms/Autocomplete.vue`

#### 2.3.5 Table Components
- [ ] Create `components/tables/DataTable.vue`
- [ ] Create `components/tables/TableCell.vue`
- [ ] Create `components/tables/TableHeader.vue`

#### 2.3.6 Layout
- [ ] Create `layouts/MainLayout.vue`
- [ ] Create `components/layout/Sidebar.vue`
- [ ] Create `components/layout/Header.vue`

**Verification:** Components render correctly

**Blockers:** None (can start in parallel)
**Estimated:** 20 hours

---

### 2.4 Pages [~]

#### 2.4.1 Invoice Pages
- [ ] Create `pages/InvoiceForm.vue`
- [ ] Create `pages/InvoiceList.vue`
- [ ] Implement item table with dynamic rows
- [ ] Implement totals calculation
- [ ] Implement client selection

#### 2.4.2 Quotation Pages
- [ ] Create `pages/QuotationForm.vue`
- [ ] Create `pages/QuotationList.vue`
- [ ] Implement convert to invoice

#### 2.4.3 Client Pages
- [ ] Create `pages/ClientsPage.vue`
- [ ] Create `components/Clients/LedgerModal.vue`
- [ ] Create `components/Clients/PaymentModal.vue`

#### 2.4.4 Settings Page
- [ ] Create `pages/SettingsPage.vue`
- [ ] Implement company settings form

**Verification:** All pages render and function

**Blockers:** Commands must be complete
**Estimated:** 40 hours

---

## PHASE 3: Advanced Features

### 3.1 Export/Import [~]

#### 3.1.1 Excel Export
- [ ] Create `src-tauri/src/exporters/mod.rs`
- [ ] Create `exporters/excel_exporter.rs`
- [ ] Implement `generate_invoice_excel()`
- [ ] Add Tauri command `export_invoice_excel(id, path)`

#### 3.1.2 PDF Bridge
- [ ] Copy `exporter.py` to `src-tauri/pdf_bridge.py`
- [ ] Create `exporters/pdf_exporter.rs`
- [ ] Implement subprocess call
- [ ] Add Tauri command `export_invoice_pdf(id, path)`

#### 3.1.3 Catalog Import
- [ ] Create `src-tauri/src/importers/mod.rs`
- [ ] Create `importers/catalog_importer.rs`
- [ ] Implement Excel import (calamine)
- [ ] Implement PDF import (Python bridge)
- [ ] Add Tauri command `import_catalog(path)`

**Verification:** Export/import works correctly

**Blockers:** Core features must work
**Estimated:** 16 hours

---

### 3.2 Fuzzy Search [~]

#### 3.2.1 Backend
- [ ] Integrate strsim in search service
- [ ] Configure Levenshtein distance
- [ ] Add result caching
- [ ] Add debouncing on command side

#### 3.2.2 Frontend
- [ ] Implement Autocomplete component
- [ ] Add debouncing (200ms)
- [ ] Add keyboard navigation
- [ ] Add loading state

**Verification:** Search is fast (< 30ms)

**Blockers:** None
**Estimated:** 8 hours

---

## PHASE 4: Polish

### 4.1 Styling [~]

- [ ] Match QSS color palette exactly
- [ ] Implement hover states
- [ ] Implement focus states
- [ ] Add transitions/animations
- [ ] Implement card shadows
- [ ] Match table styling
- [ ] Match button styling

**Verification:** Visual match with original app

**Blockers:** Pages must be complete
**Estimated:** 20 hours

---

### 4.2 Testing [~]

#### 4.2.1 Rust Tests
- [ ] Write repository tests
- [ ] Write service tests
- [ ] Write command tests
- [ ] Write model serialization tests

#### 4.2.2 Frontend Tests
- [ ] Write component tests
- [ ] Write composable tests
- [ ] Write store tests

**Verification:** Test coverage > 70%

**Blockers:** Features must be complete
**Estimated:** 24 hours

---

### 4.3 Build & Distribution [~]

- [ ] Configure Linux bundles (AppImage, .deb)
- [ ] Configure Windows bundles (MSI, NSIS)
- [ ] Test builds on both platforms
- [ ] Write release notes
- [ ] Create changelog

**Verification:** Builds run without errors

**Blockers:** All features complete
**Estimated:** 8 hours

---

## Summary

| Phase | Tasks Complete | Tasks Total | Progress |
|-------|---------------|-------------|----------|
| 1. Foundation | 9 | 57 | 16% |
| 2. Core Features | 0 | 60+ | 0% |
| 3. Advanced | 0 | 20+ | 0% |
| 4. Polish | 0 | 30+ | 0% |
| **TOTAL** | **9** | **167+** | **5%** |

---

## Immediate Next Steps

1. [ ] Create database connection module
2. [ ] Create domain models
3. [ ] Create repositories

**Target Completion:** May 26, 2026
