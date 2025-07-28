<template>
    <div class="app-container">
        <Sidebar />

        <!-- Main Content -->
        <div class="main-content bg-red-500">
            <h1>Welcome to Tally!</h1>
            <p>Main content area goes here</p>
        </div>
    </div>
</template>

<script>
    import Sidebar from './components/Sidebar.vue';
    export default {
        name: 'App',
        components: {
            Sidebar,
        },
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
        --text-muted: #9ca3af;

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

    .main-content {
        flex: 1;
        /* background-color: var(--bg-primary); */
        color: var(--text-primary);
        padding: 20px;
    }
</style>
