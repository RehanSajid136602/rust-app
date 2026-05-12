# Zahra Invoice Desktop - Migration Architecture

## Project Overview

**Zahra Invoice Desktop** is a modern Tauri v2 + Rust + Vue 3 desktop application for generating and managing invoices and quotations for Zahra Enterprises.

This is a **migration** of an existing Python PyQt6 application to a modern, performant, and maintainable technology stack.

---

## Technology Stack

### Backend
- **Rust** (Edition 2024) - Systems programming language for safety and performance
- **Tauri v2** - Desktop application framework
- **rusqlite v0.39** - SQLite database wrapper
- **rust_decimal** - Precise currency calculations
- **tokio** - Async runtime
- **strsim** - Fuzzy string matching for product search
- **rust_xlsxwriter** - Excel export
- **calamine** - Excel import

### Frontend
- **Vue 3** - Progressive JavaScript framework
- **TypeScript** - Type-safe JavaScript
- **Tailwind CSS v4** - Utility-first CSS framework
- **Vite** - Build tool and dev server
- **Pinia** - State management
- **Headless UI** - Unstyled, accessible components

### Database
- **SQLite** - Embedded relational database
- **WAL Mode** - Write-Ahead Logging for concurrency
- **Migrations** - Schema versioning system

---

## Architecture

### Clean Architecture with Repository Pattern

```
┌─────────────────────────────────────────────────────────────────┐
│                      PRESENTATION LAYER                          │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │                    Vue 3 Frontend                        │    │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐   │    │
│  │  │ Pages    │ │Components│ │Composables│ │  Stores  │   │    │
│  │  │ (Routes) │ │  (UI)    │ │ (Logic)   │ │ (Pinia)  │   │    │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘   │    │
│  └─────────────────────────────────────────────────────────┘    │
│                              │                                   │
│         @tauri-apps/plugin-* (TypeScript API)                   │
└──────────────────────────────┼───────────────────────────────────┘
                               │ IPC (Serializable types)
┌──────────────────────────────┼───────────────────────────────────┐
│                       DOMAIN LAYER                               │
│  ┌───────────────────────────▼───────────────────────────────┐  │
│  │                    Tauri Commands                          │  │
│  │  • Input validation                                        │  │
│  │  • Error translation                                       │  │
│  │  • State management hooks                                  │  │
│  └───────────────────────────┬───────────────────────────────┘  │
│                              │                                  │
│  ┌───────────────────────────▼───────────────────────────────┐  │
│  │                   Services (Business Logic)                │  │
│  │  • InvoiceService  • QuotationService  • ClientService    │  │
│  │  • PaymentService  • SettingsService   • SearchService    │  │
│  └───────────────────────────┬───────────────────────────────┘  │
│                              │                                  │
│  ┌───────────────────────────▼───────────────────────────────┐  │
│  │                  Repositories (Data Access)                │  │
│  │  • InvoiceRepository  • ClientRepository  • ProductRepo   │  │
│  │  • QuotationRepository  • SettingsRepository  • LedgerRepo│  │
│  └───────────────────────────┬───────────────────────────────┘  │
│                              │                                  │
│  ┌───────────────────────────▼───────────────────────────────┐  │
│  │                     Domain Models                          │  │
│  │  • Invoice  • InvoiceItem  • Client  • Product  • Ledger  │  │
│  │  • Quotation  • QuotationItem  • CompanySettings          │  │
│  └────────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────────┘
                               │
                    rusqlite + migrations
                               │
┌──────────────────────────────▼───────────────────────────────────┐
│                      SQLite Database                              │
│  • WAL mode  • Foreign keys  • Indexed queries  • Transactions  │
└──────────────────────────────────────────────────────────────────┘
```

---

## Project Structure

```
zahra-invoice-desktop/
├── README.md                 # Project overview
├── ARCHITECTURE.md           # This file
├── MIGRATION.md              # Migration progress tracking
├── TODO.md                   # Detailed task tracking
├── package.json              # Frontend dependencies
├── tsconfig.json             # TypeScript config
├── tailwind.config.ts        # Tailwind v4 config
├── vite.config.ts            # Vite config
├── frontend/
│   ├── index.html
│   ├── src/
│   │   ├── main.ts           # App entry
│   │   ├── App.vue           # Root component
│   │   ├── components/       # Reusable UI components
│   │   ├── pages/            # Page components
│   │   ├── composables/      # Reusable logic
│   │   ├── stores/           # Pinia stores
│   │   ├── types/            # TypeScript types
│   │   ├── services/         # API wrappers
│   │   └── styles/           # Global styles
├── src-tauri/
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── capabilities/         # Tauri v2 security
│   ├── migrations/           # SQL migrations
│   └── src/
│       ├── main.rs           # Rust entry
│       ├── lib.rs            # Tauri setup
│       ├── commands/         # Tauri commands
│       ├── services/         # Business logic
│       ├── repositories/     # Data access
│       ├── models/           # Domain models
│       ├── database/         # DB layer
│       ├── exporters/        # PDF/Excel export
│       ├── importers/        # Catalog import
│       ├── errors/           # Error types
│       └── utils/            # Utilities
└── original-python/          # BACKUP - Original app
```

---

## Key Design Decisions

### 1. Repository Pattern for Data Access
**Why:** Separates SQL queries from business logic, makes testing easier, allows swapping database implementations.

**Implementation:**
```rust
// repositories/invoice_repo.rs
pub struct InvoiceRepository {
    conn: Arc<Mutex<Connection>>,
}

impl InvoiceRepository {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }
    
    pub fn get_all(&self) -> Result<Vec<Invoice>> {
        // SQL query here
    }
}
```

### 2. Service Layer for Business Logic
**Why:** Encapsulates business rules, reusable across commands, testable without Tauri.

**Implementation:**
```rust
// services/invoice_service.rs
pub struct InvoiceService {
    repo: InvoiceRepository,
}

impl InvoiceService {
    pub fn calculate_totals(&self, items: &[InvoiceItem]) -> Totals {
        // Business logic here
    }
}
```

### 3. rusqlite over tauri-plugin-sql
**Why:** 
- No dependency conflicts (tauri-plugin-sql uses sqlx which conflicts with rusqlite)
- More ergonomic API for Rust code
- Full control over connection management
- Better for complex transactions

### 4. Python Bridge for PDF Generation (Phase 1)
**Why:**
- ReportLab produces pixel-perfect invoices
- `printpdf` crate has steep learning curve
- Preserves existing PDF quality
- Can be rewritten in Rust later (Phase 2)

### 5. rust_decimal for Currency
**Why:**
- Floating point arithmetic is unsafe for money
- `rust_decimal` provides exact decimal arithmetic
- Serde integration for JSON serialization

### 6. Composition API over Options API
**Why:**
- Better TypeScript integration
- More reusable logic (composables)
- Closer mental model to Rust functions
- Vue 3 best practice for new projects

---

## Security Model

### Tauri v2 Capabilities

Tauri v2 uses a capability-based security model. All dangerous operations are denied by default.

```json
// src-tauri/capabilities/default.json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "main-capability",
  "description": "Main window capabilities",
  "windows": ["main"],
  "permissions": [
    "core:path:default",
    "core:event:default",
    "core:window:default",
    "dialog:default",
    "dialog:allow-open",
    "dialog:allow-save",
    "fs:default",
    "fs:allow-read",
    "fs:allow-write",
    "shell:default"
  ]
}
```

### Input Validation

All user input is validated on the Rust backend:
- Client names: non-empty, max 200 chars
- Email: regex validation
- Phone: format validation
- Numbers: range validation
- Currency: 2 decimal places

---

## Error Handling

### Rust Error Types

```rust
// errors/mod.rs
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Invoice not found: {0}")]
    InvoiceNotFound(i32),
    
    #[error("PDF generation failed: {0}")]
    PdfGeneration(String),
}
```

### Frontend Error Handling

```typescript
// services/api.ts
export async function invoke<T>(command: string, args?: any): Promise<T> {
  try {
    return await tauri.invoke(command, args)
  } catch (error) {
    // Translate to user-friendly message
    throw new UserFriendlyError(error)
  }
}
```

---

## Testing Strategy

### Rust Tests

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_calculate_invoice_totals() {
        let items = vec![
            InvoiceItem { quantity: 2, price: 100, discount: 10, tax: 18 },
        ];
        let totals = calculate_totals(&items);
        assert_eq!(totals.subtotal, 200);
        assert_eq!(totals.grand_total, 208);
    }
}
```

### Frontend Tests

```typescript
import { mount } from '@vue/test-utils'
import InvoiceForm from '@/pages/InvoiceForm.vue'

test('calculates totals correctly', async () => {
  const wrapper = mount(InvoiceForm)
  await wrapper.setData({ items: [{ quantity: 2, price: 100 }] })
  expect(wrapper.vm.subtotal).toBe(200)
})
```

---

## Migration Phases

| Phase | Duration | Focus |
|-------|----------|-------|
| 1. Foundation | Weeks 1-3 | Project setup, database, models |
| 2. Core Features | Weeks 4-7 | Invoice/Quotation CRUD, Vue frontend |
| 3. Advanced | Weeks 8-10 | Search, export/import, PDF bridge |
| 4. Polish | Weeks 11-12 | Styling, testing, optimization |

---

## Performance Goals

| Metric | Target |
|--------|--------|
| Binary size | < 15 MB |
| RAM usage | < 100 MB |
| Startup time | < 1 second |
| Fuzzy search | < 30ms |
| DB query (1000 rows) | < 50ms |

---

## Future Enhancements

1. **Native PDF generation** - Rewrite ReportLab logic in Rust using `printpdf`
2. **Multi-window support** - Separate invoice preview window
3. **Cloud sync** - Optional cloud backup
4. **Mobile app** - Tauri mobile support (iOS/Android)
5. **Dark mode** - Theme switching

---

## References

- [Tauri v2 Documentation](https://v2.tauri.app/)
- [Vue 3 Documentation](https://vuejs.org/)
- [rusqlite Documentation](https://docs.rs/rusqlite/)
- [Tailwind CSS v4](https://tailwindcss.com/)
- [Original Python App](../Invoice-Python/invoice-app/)
