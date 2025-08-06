# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

**Frontend Development:**
- `npm run dev` - Start Vite development server (port 1420)
- `npm run build` - Build frontend for production

**Tauri Development:**
- `npm run tauri:dev` - Start Tauri development with hot reload
- `npm run tauri:build` - Build Tauri application for production

**Basic Commands:**
- `npm run tauri` - Access Tauri CLI directly

## Project Architecture

**Technology Stack:**
- **Frontend**: Vue 3 + Vue Router + Vite + TailwindCSS
- **Backend**: Rust with Tauri framework
- **Database**: SQLite with sqlx
- **Styling**: TailwindCSS with custom CSS variables for theming

**Application Structure:**
This is a Tauri-based personal finance tracking application with a Vue.js frontend and Rust backend.

**Frontend Architecture (src/):**
- `App.vue` - Root component with sidebar layout and router outlet
- `components/` - Vue components for UI (Dashboard, AccountDetail, forms, etc.)
- `router/index.js` - Vue Router configuration with Dashboard and AccountDetail routes
- Uses a CSS custom property system for consistent theming and typography

**Backend Architecture (src-tauri/):**
- `main.rs` - Tauri application entry point with command handlers and database initialization
- `database.rs` - Complete database layer with SQLite operations, data models, and currency handling
- Database creates `tally.db` SQLite file automatically on first run

**Database Schema:**
- `accounts` - Financial accounts with balance tracking (stored as integer cents)
- `transactions` - Financial transactions linked to accounts
- `categories` - Transaction categorization system
- `transfers` - Inter-account transfers

**Key Architectural Patterns:**
- **Currency Handling**: All monetary values stored as integer cents in database, converted to/from dollars in Rust layer
- **Tauri Commands**: Async handlers in main.rs expose database operations to frontend via `#[tauri::command]`
- **State Management**: SqlitePool managed as Tauri state, shared across all commands
- **Error Handling**: Database errors converted to strings for frontend consumption

**Available Tauri Commands:**
- `get_accounts()` - Retrieve all non-archived accounts
- `add_account(name, account_type, institution?, current_balance?)` - Create new account
- `get_account(id)` - Get specific account details
- `add_transaction(account_id, date, amount, description?, payee?, memo?, category_id?, pending, cleared)` - Create transaction
- `get_transactions(account_id, limit, offset)` - Get paginated account transactions

**Important Implementation Details:**
- Database connection pool initialized on app startup with automatic table creation
- All monetary amounts use cents-to-dollars conversion functions for precision
- Transactions support comprehensive metadata (payee, memo, categories, reconciliation status)
- Accounts support institutional tracking and net worth inclusion flags