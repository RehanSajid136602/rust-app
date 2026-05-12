# Zahra Invoice Desktop

A modern Tauri v2 + Rust + Vue 3 desktop application for generating and managing invoices and quotations.

## 🚀 Quick Start

### Prerequisites

- **Node.js** 18+ (for frontend)
- **Rust** 1.77+ (for backend)
- **Python** 3.10+ (optional, for PDF generation bridge)

### Development

```bash
# Install frontend dependencies
npm install

# Run in development mode (starts both frontend and Rust backend)
npm run tauri dev
```

### Build

```bash
# Build for current platform
npm run tauri build

# Output: dist/zahra-invoice-desktop/
```

---

## 📁 Project Structure

```
zahra-invoice-desktop/
├── src/                    # Vue 3 frontend
│   ├── components/         # Reusable UI components
│   ├── pages/              # Page components
│   ├── composables/        # Reusable logic
│   ├── stores/             # Pinia state management
│   └── types/              # TypeScript types
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── commands/       # Tauri command handlers
│   │   ├── services/       # Business logic
│   │   ├── repositories/   # Data access layer
│   │   ├── models/         # Domain models
│   │   └── database/       # SQLite connection
│   ├── capabilities/       # Tauri security config
│   └── migrations/         # Database migrations
└── original-python/        # Backup of original app
```

---

## 📚 Documentation

- **[ARCHITECTURE.md](./ARCHITECTURE.md)** - Technical architecture and design decisions
- **[MIGRATION.md](./MIGRATION.md)** - Migration progress tracking
- **[TODO.md](./TODO.md)** - Detailed task list

---

## 🛠️ Tech Stack

### Backend
- **Rust** - Systems programming language
- **Tauri v2** - Desktop application framework
- **rusqlite** - SQLite database wrapper
- **rust_decimal** - Precise currency calculations
- **tokio** - Async runtime

### Frontend
- **Vue 3** - Progressive framework
- **TypeScript** - Type-safe JavaScript
- **Tailwind CSS v4** - Utility-first CSS
- **Pinia** - State management
- **Headless UI** - Accessible components

### Database
- **SQLite** with WAL mode
- **Migrations** for schema versioning

---

## 📋 Features

### Implemented
- [ ] Invoice creation and management
- [ ] Quotation creation and management
- [ ] Client management with ledgers
- [ ] Product catalog with fuzzy search
- [ ] Excel export
- [ ] PDF generation (via Python bridge)
- [ ] Catalog import (PDF/Excel)
- [ ] Company settings

### Planned
- [ ] Native PDF generation (Rust)
- [ ] Dark mode
- [ ] Multi-window support
- [ ] Cloud sync

---

## 🔧 Development

### Commands

```bash
# Development
npm run tauri dev

# Build
npm run tauri build

# Rust tests
cd src-tauri && cargo test

# Frontend tests
npm run test

# Linting
npm run lint
cd src-tauri && cargo clippy
```

### Database

The application uses SQLite with migrations. The database file is stored in the app data directory:

- **Linux:** `~/.config/com.zahraenterprises.invoice/invoice_app.db`
- **Windows:** `%APPDATA%\com.zahraenterprises.invoice\invoice_app.db`
- **macOS:** `~/Library/Application Support/com.zahraenterprises.invoice/invoice_app.db`

---

## 📦 Distribution

### Linux
- AppImage
- Debian package (.deb)

### Windows
- MSI installer
- NSIS installer

### macOS
- DMG
- App Bundle

---

## 🔒 Security

This app uses Tauri v2's capability-based security model. All dangerous operations are denied by default and must be explicitly enabled in `src-tauri/capabilities/`.

---

## 📝 Migration Notes

This is a **migration** of an existing Python PyQt6 application. The original app is preserved at:

```
/home/rehan/Documents/Invoice-Python/invoice-app/
```

**DO NOT modify the original app during migration.**

See [MIGRATION.md](./MIGRATION.md) for progress tracking.

---

## 🤝 Contributing

1. Check [TODO.md](./TODO.md) for available tasks
2. Create a feature branch
3. Make changes
4. Run tests (`cargo test` + `npm run test`)
5. Submit pull request

---

## 📄 License

Proprietary - Zahra Enterprises

---

## 📞 Support

For issues or questions, contact the development team.

---

## 🙏 Acknowledgments

Original Python application developed by the Zahra Enterprises team.
