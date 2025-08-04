<template>
    <div class="flex-1 bg-gray-900 text-gray-50 p-6">
        <!-- Account Header -->
        <div class="border-b border-gray-600 pb-6 mb-6">
            <div class="flex justify-between items-center">
                <div>
                    <h1 class="text-3xl font-semibold text-gray-50">
                        {{ account.name }}
                    </h1>
                    <p class="text-gray-400 text-sm mt-1">
                        {{ account.type.toUpperCase() }} - Last updated:
                        {{ formatDate(account.updated_at) }}
                    </p>
                </div>
                <div class="text-right">
                    <div class="text-2xl font-bold" :class="balanceColor">
                        {{ formatBalance(account.current_balance) }}
                    </div>
                    <button
                        @click="showTransactionModal = true"
                        class="bg-indigo-500 hover:bg-indigo-600 text-white px-4 py-2 rounded mt-2"
                    >
                        Add Transaction
                    </button>
                </div>
            </div>
        </div>

        <!-- Transaction Table -->
        <div class="bg-gray-800 rounded-lg overflow-hidden">
            <table class="w-full">
                <thead class="bg-gray-700">
                    <tr>
                        <th class="text-left p-4 text-gray-300 font-medium">
                            Date
                        </th>
                        <th class="text-left p-4 text-gray-300 font-medium">
                            Description
                        </th>
                        <th class="text-left p-4 text-gray-300 font-medium">
                            Category
                        </th>
                        <th class="text-left p-4 text-gray-300 font-medium">
                            Amount
                        </th>
                        <th class="text-left p-4 text-gray-300 font-medium">
                            Balance
                        </th>
                        <th class="text-left p-4 text-gray-300 font-medium">
                            Status
                        </th>
                    </tr>
                </thead>
                <tbody>
                    <tr
                        v-if="transactions.length === 0"
                        class="border-t border-gray-600"
                    >
                        <td class="p-4 text-gray-400" colspan="6">
                            No transactions yet
                        </td>
                    </tr>
                    <tr
                        v-for="transaction in transactions"
                        :key="transaction.id"
                        class="border-t border-gray-600 hover:bg-gray-750"
                    >
                        <td class="p-4 text-gray-50">
                            {{ formatDate(transaction.date) }}
                        </td>
                        <td class="p-4 text-gray-50">
                            {{ transaction.payee || 'No Payee' }}
                        </td>
                        <td class="p-4 text-gray-300">
                            {{
                                transaction.category_id
                                    ? 'Category ' + transaction.category_id
                                    : 'Uncategorized'
                            }}
                        </td>
                        <td
                            class="p-4 text-gray-50"
                            :class="
                                transaction.amount >= 0
                                    ? 'text-emerald-500'
                                    : 'text-red-500'
                            "
                        >
                            {{ formatBalance(transaction.amount) }}
                        </td>
                        <td class="p-4 text-gray-300">-</td>
                        <td class="p-4 text-gray-300">
                            <span
                                v-if="transaction.pending"
                                class="text-yellow-400"
                                >Pending</span
                            >
                            <span
                                v-if="transaction.reconciled"
                                class="text-green-400"
                                >Reconciled</span
                            >
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>

        <Modal
            :isOpen="showTransactionModal"
            title="Add Transaction"
            @close="showTransactionModal = false"
        >
            <TransactionForm
                :accountId="account.id"
                @cancel="showTransactionModal = false"
            />
        </Modal>
    </div>
</template>

<script>
    import { invoke } from '@tauri-apps/api/core';
    import TransactionForm from './TransactionForm.vue';
    import Modal from './Modal.vue';

    export default {
        name: 'AccountDetail',
        components: {
            TransactionForm,
            Modal,
        },
        props: ['id'], // This comes from the router
        data() {
            return {
                showTransactionModal: false,
                account: {
                    id: null,
                    name: 'Loading...',
                    type: '',
                    current_balance: 0,
                    updated_at: new Date(),
                },
                transactions: [],
            };
        },
        computed: {
            balanceColor() {
                return this.account.current_balance >= 0
                    ? 'text-emerald-500'
                    : 'text-red-500';
            },
        },
        async mounted() {
            await this.loadAccount();
            await this.loadTransactions();
        },
        watch: {
            // Watch for route change
            $route(to, from) {
                this.loadAccount();
            },
        },
        methods: {
            async loadAccount() {
                try {
                    const accountData = await invoke('get_account_by_id', {
                        id: parseInt(this.id),
                    });

                    if (accountData) {
                        this.account = {
                            id: accountData[0],
                            name: accountData[1],
                            type: accountData[2],
                            current_balance: accountData[3] || 0,
                            updated_at: new Date(),
                        };
                    }
                } catch (error) {
                    console.error('Failed to load account:', error);
                }
            },
            async loadTransactions() {
                try {
                    console.log(
                        'Loading transactions for accountID:',
                        parseInt(this.id)
                    );
                    this.transactions = await invoke('get_transactions', {
                        accountId: parseInt(this.id),
                        limit: 50,
                        offset: 0,
                    });
                    console.log(
                        'Raw transactions response:',
                        this.transactions
                    );
                    console.log(
                        'Number of transactions:',
                        this.transactions.length
                    );

                    // Log each transaction individually
                    this.transactions.forEach((txn, index) => {
                        console.log(`Transaction ${index}:`, txn);
                    });
                } catch (error) {
                    console.log('Failed to load transactions:', error);
                }
            },
            formatBalance(amount) {
                return new Intl.NumberFormat('en-US', {
                    style: 'currency',
                    currency: 'USD',
                }).format(amount);
            },
            formatDate(date) {
                return new Date(date).toLocaleDateString();
            },
        },
    };
</script>
