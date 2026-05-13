# Zahra Invoice Desktop — Project Cost Estimate

---

## 1. What Has Already Been Built

### ✅ Complete Features (production-ready)

| Module | Status | Details |
|--------|--------|---------|
| **Desktop shell** | Complete | Tauri v2 + Vue 3 + TypeScript + Tailwind CSS v4 + SQLite |
| **Database layer** | Complete | 8 tables, 11 indexes, WAL mode, versioned migration system, 17 seed products |
| **Product CRUD** | Complete | Create/read/update/delete, search, Excel import, PDF import (calamine + pdf-extract) |
| **Client CRUD** | Complete | Full CRUD with balance display, `get_all_balances`, cascade delete to ledger |
| **Invoice CRUD** | Complete | Dynamic line items, autocomplete product search, totals calc, payment recording, status badges |
| **Quotation CRUD** | Complete | Full lifecycle (draft→sent→accepted→rejected), convert-to-invoice, status management |
| **Settings page** | Complete | Company name, address, phones, email, GST, PAN |
| **Product autocomplete** | Complete | 150ms debounce, keyboard nav, portal dropdown (escape clipping), focus management |
| **PDF export (Rust)** | Complete | Native genpdf: LiberationSans fonts, header/footer images, 5-column items table, totals section |
| **Browser print** | Complete | HTML/CSS print template, A4 portrait, Times New Roman scoped |
| **Reference number gen** | Complete | Year-based `ZE-{YEAR}-{NNN}` via tauri-plugin-store |
| **File operations** | Complete | Open file/folder cross-platform (xdg-open/explorer/open) |
| **Currency precision** | Complete | `rust_decimal` for all money math (no FP errors) |
| **Error handling** | Complete | `AppError` enum (Database, Validation, NotFound, etc.) + `thiserror` |
| **Data validation** | Complete | Email, phone (PK format), required fields, positive amounts, length checks |
| **Ledger backend** | Complete | Full debit/credit system, balance recalculation, cascade on invoice delete |
| **Rust unit tests** | Complete | 10+ tests across repos, importers, utilities |

### ⚠️ Partially Built

| Item | Status | Gap |
|------|--------|-----|
| **Excel Export** | Half-built | `rust_xlsxwriter` crate installed, no Tauri command or UI trigger |
| **Styling/branding** | Drifted | Tailwind uses blue (#3b82f6), original app uses navy (#1a2540) + orange (#e05a2b) |
| **Payment recording** | Functional but thin | Works via invoice edit form only; no dedicated payment history UI |
| **Print PDF pipeline** | Disconnected | UI uses `window.print()` (browser dialog), not the Rust `export_invoice_pdf` command |
| **Component library** | Empty shells | `ui/`, `forms/`, `tables/`, `layout/` directories exist but empty; all UI is inline |
| **Pinia stores** | Unused | `createPinia()` called but `src/stores/` is empty; all state is local `ref()` |

---

## 2. What Is Still Missing

| # | Feature | Severity | Notes |
|---|---------|----------|-------|
| 1 | **Excel Export** | High | Core expectation since import exists; users will ask for it |
| 2 | **Client Ledger UI** | High | Backend fully built, zero frontend — users can't see transaction history |
| 3 | **Dashboard charts** | Medium | Revenue trends, monthly summaries — competitor parity |
| 4 | **Rust PDF connected to UI** | Medium | `export_invoice_pdf` exists but never called from frontend |
| 5 | **Frontend type definitions** | Medium | `src/types/` empty, all types defined inline in `.vue` files |
| 6 | **Frontend service layer** | Medium | `src/services/` empty, raw `invoke()` calls scattered across pages |
| 7 | **Pinia state stores** | Medium | `src/stores/` empty, no shared state management |
| 8 | **Brand colors applied** | Low | Navy/orange scheme stored in `company_settings` but not reflected in CSS |
| 9 | **CI/CD pipeline** | Medium | No GitHub Actions, no automated cross-platform builds, no release automation |
| 10 | **Code signing** | Medium | No signing certs for Windows (.msi) or macOS (.dmg), required for distribution |
| 11 | **Frontend tests** | Medium | No vitest setup, zero component/unit tests on frontend |
| 12 | **Lint scripts** | Low | `package.json` has no `lint` or `typecheck` scripts |
| 13 | **HTML title** | Low | Still says "Tauri + Vue + Typescript App" |
| 14 | **Dark mode** | Nice-to-have | Tailwind v4 supports it natively |
| 15 | **Multi-window** | Nice-to-have | Tauri supports it, not implemented |
| 16 | **Cloud sync** | Nice-to-have | Mentioned in docs, zero code |
| 17 | **Fuzzy search** | Low | `strsim` crate installed but search uses basic SQL `LIKE` |

---

## 3. Time Estimate to Complete

### Production-Ready (must-do)

| Task | Hours | Breakdown |
|------|-------|-----------|
| Excel Export (Rust command + UI) | 10 | xlsxwriter config, column formatting, download trigger, UI button |
| Client Ledger UI page | 8 | New route, table with debit/credit/balance, date filter, client selector |
| Dashboard charts | 10 | Chart.js or ApexCharts, revenue over time, top clients, outstanding pie |
| Rust PDF connected to UI | 3 | Wire `export_invoice_pdf` to UI button, replace `window.print()` call |
| Frontend types extraction | 5 | Move inline interfaces to `src/types/`, add JSDoc |
| Frontend service layer | 8 | Create `src/services/` wrappers, replace raw `invoke()` calls |
| Pinia stores | 8 | Extract shared state (products cache, invoice draft, print data) |
| Brand color fix | 4 | Update Tailwind theme to navy/orange, match original app |
| CI/CD (GitHub Actions) | 8 | Cross-platform matrix build (Windows/macOS/Linux), auto-release on tag |
| Code signing setup | 6 | OV cert purchase, Tauri `signCommand` config, notarization for macOS |
| Frontend tests (vitest) | 14 | Setup, component tests for 6 pages, composable tests, coverage |
| Lint/typecheck scripts | 3 | eslint + prettier config, `vue-tsc --noEmit`, `package.json` scripts |
| HTML title + misc polish | 2 | Title, favicon, transition animations, hover states |
| **Subtotal (essential)** | **89h** | |

### Nice-to-Have (Phase 2)

| Task | Hours |
|------|-------|
| Dark mode | 5 |
| Multi-window support | 18 |
| Cloud sync (Supabase) | 50 |
| Fuzzy search (Levenshtein) | 6 |
| Email invoices directly | 12 |
| **Subtotal (optional)** | **91h** |

---

## 4. Cost to Build From Scratch

This app represents ~6,200 lines of code (2,700 Vue/TS + 3,500 Rust) with a Tauri desktop architecture, SQLite local DB, and 6 full CRUD pages. Realistic from-scratch estimate by a single mid-level developer:

| Item | Hours | PKR Rate (₨2,500/hr) | PKR Total | USD Rate ($20/hr intl.) | USD Total |
|------|-------|----------------------|-----------|------------------------|-----------|
| Project setup (Tauri + Vue + Tailwind + SQLite) | 12 | 2,500 | ₨30,000 | $20 | $240 |
| Database design + migration system | 16 | 2,500 | ₨40,000 | $20 | $320 |
| Models + repos + services (Rust) | 40 | 3,500 | ₨140,000 | $25 | $1,000 |
| 33 Tauri commands + IPC wiring | 24 | 3,500 | ₨84,000 | $25 | $600 |
| PDF export (genpdf) | 20 | 3,500 | ₨70,000 | $30 | $600 |
| Excel/PDF import | 16 | 3,500 | ₨56,000 | $30 | $480 |
| Dashboard page | 8 | 2,500 | ₨20,000 | $20 | $160 |
| Products page (CRUD + import) | 10 | 2,500 | ₨25,000 | $20 | $200 |
| Clients page (CRUD + balance) | 8 | 2,500 | ₨20,000 | $20 | $160 |
| Invoices page (CRUD, autocomplete, totals, payment) | 24 | 3,000 | ₨72,000 | $25 | $600 |
| Quotations page (CRUD, lifecycle, convert) | 20 | 3,000 | ₨60,000 | $25 | $500 |
| Settings page | 6 | 2,500 | ₨15,000 | $20 | $120 |
| Print template (HTML/CSS) | 8 | 2,500 | ₨20,000 | $20 | $160 |
| Autocomplete component | 8 | 3,000 | ₨24,000 | $25 | $200 |
| Styling (Tailwind theme + utilities) | 12 | 2,500 | ₨30,000 | $20 | $240 |
| Testing (Rust unit + frontend vitest) | 20 | 3,000 | ₨60,000 | $25 | $500 |
| CI/CD + code signing + distribution | 16 | 3,500 | ₨56,000 | $25 | $400 |
| Bug fixing + polish (20% buffer) | 52 | 3,000 | ₨156,000 | $25 | $1,300 |
| **Total from scratch** | **320h** | — | **₨978,000** | — | **$7,780** |

---

## 5. Monthly Running Costs (Post-Launch)

This is a **local desktop app** — no hosting, no database servers, no auth providers. Operating costs are minimal:

| Service | Free Tier | Paid Tier (when needed) |
|---------|-----------|-------------------------|
| **GitHub** (CI/CD) | Unlimited public repos, 2,000 min/mo Actions | $4/mo (Teams, more Actions minutes) |
| **Code signing cert** | — | $15–30/mo amortized ($180–360/yr OV cert) |
| **Domain** (for landing page) | — | $1.25/mo ($15/yr .com) |
| **Landing page hosting** (Vercel/Netlify) | $0 (hobby) | $0 (likely stays free for static site) |
| **Total monthly** | **$0** | **$5–35** |

If cloud sync is added later:

| Service | Free Tier | Paid Tier |
|---------|-----------|-----------|
| Supabase | $0 (500MB DB, 50K MAU) | $25/mo (Pro, 8GB, daily backups) |
| Clerk Auth | $0 (50K MRU) | $20/mo (Pro, MFA, custom domain) |
| Resend (invoice emails) | $0 (100/day) | $20/mo (50K emails) |
| Cloud sync ops | **$0** (free tier) | **$65/mo** (with Supabase + Clerk + Resend) |

---

## 6. Total Summary

| Scenario | Cost |
|----------|------|
| **Build remaining features** (local PK dev @ ₨2,500–3,500/hr) | **₨265,000** |
| **Build remaining features** (international @ $20–30/hr) | **$2,050** |
| **Full rebuild from scratch** (local PK dev) | **₨978,000** |
| **Full rebuild from scratch** (international) | **$7,780** |
| **Monthly ops (desktop-only, free tier)** | **$0** |
| **Monthly ops (with code signing + landing page)** | **$5** |
| **Monthly ops (with cloud sync, paid tiers)** | **$65** |

---

### Bottom Line

**The app is ~85% complete.** It's a functional, well-architected desktop invoicing tool. The backend is solid (clean architecture, decimal precision, decent error handling). The frontend works but lacks polish, shared types, state management, and tests. The 89 hours of remaining must-do work would bring it to production quality. A from-scratch rebuild would cost roughly ₨9.8L ($7,800) at mid-market rates — meaning the current codebase has an estimated replacement value of ~₨7.1L ($5,700) of already-completed work.
