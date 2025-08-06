<template>
    <div class="w-60 xl:w-80 2xl:w-80 bg-gray-800 text-gray-50 flex flex-col">
        <div class="px-5 pt-4 pb-4 border-b border-gray-600">
            <h2 class="text-2xl font-semibold text-indigo-500">Tally</h2>
        </div>

        <nav class="py-4 border-b border-gray-600">
            <ul class="list-none">
                <li>
                    <router-link
                        to="/"
                        class="block px-5 py-3 text-gray-300 no-underline transition-colors hover:bg-gray-600 hover:text-gray-50"
                        active-class="bg-gray-600 text-gray-50"
                        >Dashboard</router-link
                    >
                </li>
                <li>
                    <router-link
                        to="/accounts"
                        class="block px-5 py-3 text-gray-300 no-underline transition-colors hover:bg-gray-600 hover:text-gray-50"
                        active-class="bg-gray-600 text-gray-50"
                        >Accounts</router-link
                    >
                </li>
                <li>
                    <router-link
                        to="/reports"
                        class="block px-5 py-3 text-gray-300 no-underline transition-colors hover:bg-gray-600 hover:text-gray-50"
                        active-class="bg-gray-600 text-gray-50"
                        >Reports</router-link
                    >
                </li>
                <li>
                    <router-link
                        to="/settings"
                        class="block px-5 py-3 text-gray-300 no-underline transition-colors hover:bg-gray-600 hover:text-gray-50"
                        active-class="bg-gray-600 text-gray-50"
                        >Settings</router-link
                    >
                </li>
            </ul>
        </nav>

        <div class="flex-1 py-4">
            <div class="px-5 pb-3">
                <h3
                    class="text-sm font-semibold text-gray-400 uppercase tracking-wide"
                >
                    Accounts
                </h3>
            </div>
            <ul class="list-none">
                <AccountItem
                    v-for="account in accounts"
                    :key="account.id"
                    :id="account.id"
                    :name="account.name"
                    :balance="account.current_balance || 0"
                />
            </ul>
        </div>
    </div>
</template>

<script>
    import AccountItem from './AccountItem.vue';
    import { invoke } from '@tauri-apps/api/core';
    export default {
        name: 'Sidebar',
        components: {
            AccountItem,
        },
        data() {
            return {
                accounts: [],
            };
        },
        async mounted() {
            await this.loadAccounts();
        },
        methods: {
            async loadAccounts() {
                try {
                    this.accounts = await invoke('get_accounts');
                    console.log('Loaded accounts:', this.accounts);
                } catch (error) {
                    console.error('Failed to load accounts:', error);
                    // Fallback to mock data if invoke fails
                    this.accounts = [
                        {
                            id: 1,
                            name: 'Chase Checking',
                            account_type: 'checking',
                            current_balance: 1250.75,
                            institution: 'Chase',
                            display_order: 1,
                            archived: false,
                            include_in_net_worth: true,
                            account_number_last4: null,
                            created_at: '2024-01-01 00:00:00',
                            updated_at: '2024-01-01 00:00:00'
                        },
                        {
                            id: 2,
                            name: 'Savings Account',
                            account_type: 'savings',
                            current_balance: 5000.0,
                            institution: null,
                            display_order: 2,
                            archived: false,
                            include_in_net_worth: true,
                            account_number_last4: null,
                            created_at: '2024-01-01 00:00:00',
                            updated_at: '2024-01-01 00:00:00'
                        }
                    ];
                }
            },
            async refreshAccounts() {
                // Public method that can be called from parent components
                await this.loadAccounts();
            },
        },
    };
</script>
