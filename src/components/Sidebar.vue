<template>
    <div class="w-60 xl:w-80 2xl:w-80 bg-gray-800 text-gray-50 flex flex-col">
        <div class="px-5 pt-4 pb-4 border-b border-gray-600">
            <h2 class="text-2xl font-semibold text-indigo-500">Tally</h2>
        </div>

        <nav class="py-4 border-b border-gray-600">
            <ul class="list-none">
                <li>
                    <a
                        href="#"
                        class="block px-5 py-3 text-gray-300 no-underline transition-colors hover:bg-gray-600 hover:text-gray-50 bg-gray-600 text-gray-50"
                        >Dashboard</a
                    >
                </li>
                <li>
                    <a
                        href="#"
                        class="block px-5 py-3 text-gray-300 no-underline transition-colors hover:bg-gray-600 hover:text-gray"
                        >Reports</a
                    >
                </li>
                <li>
                    <a
                        href="#"
                        class="block px-5 py-3 text-gray-300 no-underline transition-colors hover:bg-gray-600 hover:text-gray"
                        >Settings</a
                    >
                </li>
            </ul>
        </nav>

        <div class="flex-1 py-4">
            <div class="flex justify-between items-center px-5 pb-3">
                <h3
                    class="text-sm font-semibold text-gray-400 uppercase tracking-wide"
                >
                    Accounts
                </h3>
                <button
                    @click="showAddModal = true"
                    class="bg-indigo-500 text-white border-none rounded w-6 h-6 cursor-pointer"
                >
                    +
                </button>
            </div>
            <ul class="list-none">
                <AccountItem
                    v-for="account in accounts"
                    :key="account[0]"
                    :name="account[1]"
                    :balance="account[3] || 0"
                />
            </ul>
        </div>
    </div>

    <Modal
        :isOpen="showAddModal"
        title="Add Account"
        @close="showAddModal = false"
    >
        <AccountForm
            mode="create"
            @submit="handleAddAccount"
            @cancel="showAddModal = false"
        />
    </Modal>
</template>

<script>
    import AccountItem from './AccountItem.vue';
    import Modal from './Modal.vue';
    import AccountForm from './AccountForm.vue';
    import { invoke } from '@tauri-apps/api/core';
    export default {
        name: 'Sidebar',
        components: {
            AccountItem,
            Modal,
            AccountForm,
        },
        data() {
            return {
                showAddModal: false,
                accounts: [],
            };
        },
        async mounted() {
            await this.loadAccounts();
        },
        methods: {
            handleAddAccount(accountData) {
                this.showAddModal = false;
            },
            async loadAccounts() {
                try {
                    console.log('Loading accounts...');
                    this.accounts = await invoke('get_accounts');
                    console.log('Loaded accounts:', this.accounts);
                } catch (error) {
                    console.error('Failed to load accounts:', error);
                    // Fallback to mock data if invoke fails
                    this.accounts = [
                        [1, 'Chase Checking', 'checking', 1250.75],
                        [2, 'Savings Account', 'savings', 5000.0],
                    ];
                    console.log('Using mock data instead');
                }
            },
        },
    };
</script>
