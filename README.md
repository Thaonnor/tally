# Tally

A modern personal finance tracking application built with Tauri, Vue.js, and Rust.

## Features

- **Account Management**: Track multiple financial accounts (checking, savings, credit cards, etc.)
- **Transaction Tracking**: Record and categorize financial transactions
- **Balance Management**: Real-time balance updates and account reconciliation
- **Cross-Platform**: Runs natively on Windows, macOS, and Linux
- **Secure Local Storage**: SQLite database stored locally on your device
- **Modern UI**: Clean, responsive interface built with Vue 3 and TailwindCSS

## Technology Stack

- **Frontend**: Vue 3 + Vue Router + Vite + TailwindCSS
- **Backend**: Rust with Tauri framework
- **Database**: SQLite with sqlx
- **Styling**: TailwindCSS with custom CSS variables for theming

## Prerequisites

- [Node.js](https://nodejs.org/) (v16 or higher)
- [Rust](https://rustup.rs/) (latest stable)
- [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)

## Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd tally
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Start the development server:
   ```bash
   npm run tauri:dev
   ```

## Development

### Available Scripts

**Frontend Development:**
- `npm run dev` - Start Vite development server (port 1420)
- `npm run build` - Build frontend for production

**Tauri Development:**
- `npm run tauri:dev` - Start Tauri development with hot reload
- `npm run tauri:build` - Build Tauri application for production

**Tauri CLI:**
- `npm run tauri` - Access Tauri CLI directly

### Project Structure

```
├── src/                    # Vue.js frontend
│   ├── components/         # Vue components
│   ├── router/            # Vue Router configuration
│   └── App.vue            # Root component
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── main.rs        # Tauri app entry point
│   │   └── database.rs    # Database operations
│   └── Cargo.toml         # Rust dependencies
└── package.json           # Node.js dependencies
```

## Database

The application uses SQLite for local data storage with the following schema:

- **accounts** - Financial accounts with balance tracking
- **transactions** - Financial transactions linked to accounts  
- **categories** - Transaction categorization system
- **transfers** - Inter-account transfers

All monetary values are stored as integer cents for precision and converted to dollars in the UI.

## Building for Production

To create a production build:

```bash
npm run tauri:build
```

This will generate platform-specific installers in the `src-tauri/target/release/bundle/` directory.

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Architecture

### Currency Handling
All monetary values are stored as integer cents in the database and converted to/from dollars in the Rust layer for precision.

### Tauri Commands
The backend exposes database operations to the frontend through async Tauri commands:

- `get_accounts()` - Retrieve all non-archived accounts
- `add_account()` - Create new account
- `get_account()` - Get specific account details
- `add_transaction()` - Create transaction
- `get_transactions()` - Get paginated account transactions

### State Management
SqlitePool is managed as Tauri state and shared across all commands for efficient database access.

## License

This project is licensed under the MIT License - see the LICENSE file for details.