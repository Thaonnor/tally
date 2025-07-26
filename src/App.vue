<template>
    <div class="app-container">
        <!-- Sidebar -->
        <div class="sidebar">
            <div class="app-title">
                <h2>Tally</h2>
            </div>

            <nav class="main-nav">
                <ul>
                    <li><a href="#" class="nav-link active">Dashboard</a></li>
                    <li><a href="#" class="nav-link">Reports</a></li>
                    <li><a href="#" class="nav-link">Settings</a></li>
                </ul>
            </nav>

            <div class="accounts-section">
                <h3>Accounts</h3>
                <ul class="account-list">
                    <li class="account-item">
                        <span class="account-name">Chase Checking</span>
                        <span class="account-balance">$1,250.75</span>
                    </li>
                    <li class="account-item">
                        <span class="account-name">Savings</span>
                        <span class="account-balance">$5,000.00</span>
                    </li>
                    <li class="account-item">
                        <span class="account-name">Credit Card</span>
                        <span class="account-balance negative">-$340.20</span>
                    </li>
                </ul>
            </div>
        </div>

        <!-- Main Content -->
        <div class="main-content">
            <h1>Welcome to Tally!</h1>
            <p>Main content area goes here</p>
        </div>
    </div>
</template>

<script>
    export default {
        name: 'App',
    };
</script>

<style>
    :root {
        /* Typography */
        --font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI',
            sans-serif;

        /* Font sizes (1.25 ratio, 16px base) */
        --text-h1: 2.44rem; /* 39px - Page titles */
        --text-h2: 1.95rem; /* 31px - Section headers */
        --text-h3: 1.56rem; /* 25px - Subsection headers */
        --text-h4: 1.25rem; /* 20px - Card titles */
        --text-body: 1rem; /* 16px - Main content */
        --text-small: 0.8rem; /* 13px - Secondary info */
        --text-caption: 0.64rem; /* 10px - Form help text */

        /* Background colors */
        --bg-primary: #111827;
        --bg-secondary: #1f2937;
        --bg-tertiary: #374151;

        /* Text colors */
        --text-primary: #f9fafb;
        --text-secondary: #d1d5db;
        -text-muted: #9ca3af;

        /* Brand colors */
        --primary: #6366f1;
        --secondary: #8b5cf6;

        /* Status colors */
        --success: #10b981;
        --warning: #f59e0b;
        --danger: #ef4444;
    }

    * {
        margin: 0;
        padding: 0;
        box-sizing: border-box;
    }

    body,
    html {
        margin: 0;
        padding: 0;
        height: 100%;
    }

    body {
        font-family: var(--font-family);
        font-feature-settings: 'tnum' 1; /* Enables tabular numbers for financial data */
        font-size: var(--text-body);
        line-height: 1.5;
    }

    h1 {
        font-size: var(--text-h1);
        font-weight: 700;
    }
    h2 {
        font-size: var(--text-h2);
        font-weight: 600;
    }
    h3 {
        font-size: var(--text-h3);
        font-weight: 600;
    }
    h4 {
        font-size: var(--text-h4);
        font-weight: 500;
    }

    .app-container {
        display: flex;
        height: 100vh;
    }

    .sidebar {
        width: 280px;
        background-color: var(--bg-secondary);
        color: #f9fafb;
        padding: 20px;
        display: flex;
        flex-direction: column;
    }

    .app-title {
        padding: 24px 20px 16px;
        border-bottom: 1px solid var(--bg-tertiary);
    }

    .app-title h2 {
        font-size: 24px;
        font-weight: 600;
        color: var(--primary);
    }

    .main-nav {
        padding: 16px 0;
        border-bottom: 1px solid var(--bg-tertiary);
    }

    .main-nav ul {
        list-style: none;
    }

    .nav-link {
        display: block;
        padding: 12px 20px;
        color: var(--text-secondary);
        text-decoration: none;
        transition: background-color 0.2s;
    }

    .nav-link:hover,
    .nav-link.active {
        background-color: var(--bg-tertiary);
        color: var(--text-primary);
    }

    .accounts-section {
        flex: 1;
        padding: 16px 0;
    }

    .accounts-section h3 {
        padding: 0 20px 12px;
        font-size: 14px;
        font-weight: 600;
        color: var(--text-muted);
        text-transform: uppercase;
        letter-spacing: 0.05em;
    }

    .account-list {
        list-style: none;
    }

    .account-item {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 12px 20px;
        cursor: pointer;
        transition: background-color 0.2s;
    }

    .account-item:hover {
        background-color: var(--bg-tertiary);
    }

    .account-name {
        font-weight: 500;
    }

    .account-balance {
        font-weight: 600;
        color: var(--success);
    }

    .account-balance.negative {
        color: var(--danger);
    }

    /* Responsive adjustments */
    @media (min-width: 1920px) {
        .sidebar {
            width: 320px;
        }
    }

    @media (max-width: 1366px) {
        .sidebar {
            width: 240px;
        }
    }

    .main-content {
        flex: 1;
        background-color: var(--bg-primary);
        color: var(--text-primary);
        padding: 20px;
    }
</style>
