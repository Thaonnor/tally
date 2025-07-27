<template>
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
                <AccountItem name="Chase Checking" :balance="1250.75" />
                <AccountItem name="Savings Account" :balance="5000.00" />
                <AccountItem name="Credit Card" :balance="Number(-340.20)" />
            </ul>
        </div>
    </div>
</template>

<script>
    import AccountItem from './AccountItem.vue';
    export default {
        name: 'Sidebar',
        components: {
            AccountItem,
        },
    };
</script>

<style scoped>
    .sidebar {
        width: 280px;
        background-color: var(--bg-secondary);
        color: var(--text-primary);
        padding: 0; /* Remove the 20px padding */
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
</style>
