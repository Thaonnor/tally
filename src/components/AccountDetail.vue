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
        <div class="bg-gray-800 rounded-lg overflow-hidden overflow-x-auto">
            <table class="w-full min-w-max text-sm">
                <thead class="bg-gray-700">
                    <tr>
                        <th class="text-left px-2 py-2 text-gray-300 font-medium text-xs uppercase tracking-wide w-24">Date</th>
                        <th class="text-left px-2 py-2 text-gray-300 font-medium text-xs uppercase tracking-wide min-w-32">Description</th>
                        <th class="text-left px-2 py-2 text-gray-300 font-medium text-xs uppercase tracking-wide min-w-28">Payee</th>
                        <th class="text-left px-2 py-2 text-gray-300 font-medium text-xs uppercase tracking-wide min-w-32">Category</th>
                        <th class="text-right px-2 py-2 text-gray-300 font-medium text-xs uppercase tracking-wide w-24">Amount</th>
                        <th class="text-left px-2 py-2 text-gray-300 font-medium text-xs uppercase tracking-wide min-w-24">Memo</th>
                        <th class="text-center px-2 py-2 text-gray-300 font-medium text-xs uppercase tracking-wide w-16">Pend</th>
                        <th class="text-center px-2 py-2 text-gray-300 font-medium text-xs uppercase tracking-wide w-16">Clear</th>
                        <th class="text-center px-2 py-2 text-gray-300 font-medium text-xs uppercase tracking-wide w-20">Actions</th>
                    </tr>
                </thead>
                <tbody>
                    <tr
                        v-if="transactions.length === 0"
                        class="border-t border-gray-600"
                    >
                        <td class="px-2 py-3 text-gray-400 text-center" colspan="9">
                            No transactions yet
                        </td>
                    </tr>
                    <tr
                        v-for="transaction in transactions"
                        :key="transaction.id"
                        class="border-t border-gray-600 hover:bg-gray-750"
                        :class="{ 'bg-gray-750': editingTransactionId === transaction.id }"
                    >
                        <!-- Date -->
                        <td class="px-2 py-2 text-gray-50">
                            <input
                                v-if="editingTransactionId === transaction.id"
                                type="date"
                                v-model="editingTransaction.date"
                                @keyup.enter="saveTransaction"
                                @keyup.escape="cancelEdit"
                                class="w-full px-2 py-1 text-sm bg-gray-700 border border-gray-600 rounded text-gray-50 focus:outline-none focus:border-indigo-500"
                            />
                            <span v-else class="text-sm">{{ formatDate(transaction.date) }}</span>
                        </td>

                        <!-- Description -->
                        <td class="px-2 py-2 text-gray-50">
                            <input
                                v-if="editingTransactionId === transaction.id"
                                type="text"
                                v-model="editingTransaction.description"
                                placeholder="Description"
                                @keyup.enter="saveTransaction"
                                @keyup.escape="cancelEdit"
                                class="w-full px-2 py-1 text-sm bg-gray-700 border border-gray-600 rounded text-gray-50 focus:outline-none focus:border-indigo-500"
                            />
                            <span v-else class="text-sm">{{ transaction.description || '-' }}</span>
                        </td>

                        <!-- Payee -->
                        <td class="px-2 py-2 text-gray-50">
                            <input
                                v-if="editingTransactionId === transaction.id"
                                type="text"
                                v-model="editingTransaction.payee"
                                placeholder="Payee"
                                @keyup.enter="saveTransaction"
                                @keyup.escape="cancelEdit"
                                class="w-full px-2 py-1 text-sm bg-gray-700 border border-gray-600 rounded text-gray-50 focus:outline-none focus:border-indigo-500"
                            />
                            <span v-else class="text-sm">{{ transaction.payee || '-' }}</span>
                        </td>

                        <!-- Category -->
                        <td class="px-2 py-2 text-gray-300">
                            <select
                                v-if="editingTransactionId === transaction.id"
                                v-model="editingTransaction.category_id"
                                class="w-full px-2 py-1 text-sm bg-gray-700 border border-gray-600 rounded text-gray-50 focus:outline-none focus:border-indigo-500"
                            >
                                <option :value="null">Uncategorized</option>
                                <option 
                                    v-for="category in nonSystemCategories" 
                                    :key="category.id" 
                                    :value="category.id"
                                >
                                    {{ formatCategoryName(category) }}
                                </option>
                            </select>
                            <span v-else class="text-sm">{{ getCategoryName(transaction.category_id) }}</span>
                        </td>

                        <!-- Amount -->
                        <td class="px-2 py-2 text-right">
                            <input
                                v-if="editingTransactionId === transaction.id"
                                type="number"
                                step="0.01"
                                v-model.number="editingTransaction.amount"
                                @keyup.enter="saveTransaction"
                                @keyup.escape="cancelEdit"
                                class="w-full px-2 py-1 text-sm text-right bg-gray-700 border border-gray-600 rounded text-gray-50 focus:outline-none focus:border-indigo-500"
                            />
                            <span 
                                v-else
                                class="text-sm font-medium"
                                :class="
                                    transaction.amount >= 0
                                        ? 'text-emerald-500'
                                        : 'text-red-500'
                                "
                            >
                                {{ formatBalance(transaction.amount) }}
                            </span>
                        </td>

                        <!-- Memo -->
                        <td class="px-2 py-2 text-gray-300">
                            <input
                                v-if="editingTransactionId === transaction.id"
                                type="text"
                                v-model="editingTransaction.memo"
                                placeholder="Memo"
                                @keyup.enter="saveTransaction"
                                @keyup.escape="cancelEdit"
                                class="w-full px-2 py-1 text-xs bg-gray-700 border border-gray-600 rounded text-gray-50 focus:outline-none focus:border-indigo-500"
                            />
                            <span v-else class="text-xs text-gray-400">{{ transaction.memo || '-' }}</span>
                        </td>

                        <!-- Pending -->
                        <td class="px-2 py-2 text-center">
                            <input
                                v-if="editingTransactionId === transaction.id"
                                type="checkbox"
                                v-model="editingTransaction.pending"
                                class="w-3 h-3 accent-indigo-500"
                            />
                            <span 
                                v-else-if="transaction.pending" 
                                class="text-yellow-400 text-xs"
                                title="Pending"
                            >
                                ●
                            </span>
                            <span v-else class="text-gray-600 text-xs">○</span>
                        </td>

                        <!-- Cleared -->
                        <td class="px-2 py-2 text-center">
                            <input
                                v-if="editingTransactionId === transaction.id"
                                type="checkbox"
                                v-model="editingTransaction.cleared"
                                class="w-3 h-3 accent-indigo-500"
                            />
                            <span 
                                v-else-if="transaction.cleared" 
                                class="text-green-400 text-xs"
                                title="Cleared"
                            >
                                ●
                            </span>
                            <span v-else class="text-gray-600 text-xs">○</span>
                        </td>

                        <!-- Actions -->
                        <td class="px-1 py-2 text-center">
                            <div v-if="editingTransactionId === transaction.id" class="flex justify-center gap-1">
                                <!-- Save -->
                                <button
                                    @click="saveTransaction"
                                    class="text-green-400 hover:text-green-300 px-1 py-0.5 text-sm"
                                    title="Save changes"
                                >
                                    ✓
                                </button>
                                <!-- Cancel -->
                                <button
                                    @click="cancelEdit"
                                    class="text-gray-400 hover:text-gray-300 px-1 py-0.5 text-sm"
                                    title="Cancel editing"
                                >
                                    ✗
                                </button>
                            </div>
                            <div v-else class="flex justify-center gap-1">
                                <!-- Edit -->
                                <button
                                    @click="startEdit(transaction)"
                                    class="text-blue-400 hover:text-blue-300 px-1 py-0.5 text-xs"
                                    title="Edit transaction"
                                >
                                    ✏️
                                </button>
                                <!-- Delete -->
                                <button
                                    @click="deleteTransaction(transaction.id)"
                                    class="text-red-400 hover:text-red-300 px-1 py-0.5 text-xs"
                                    title="Delete transaction"
                                >
                                    🗑️
                                </button>
                            </div>
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
                @success="handleTransactionAdded"
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
                categories: [],
                editingTransactionId: null,
                editingTransaction: {},
            };
        },
        computed: {
            balanceColor() {
                return this.account.current_balance >= 0
                    ? 'text-emerald-500'
                    : 'text-red-500';
            },
            nonSystemCategories() {
                return this.categories.filter(category => !category.is_system_category);
            },
        },
        async mounted() {
            await this.loadAccount();
            await this.loadCategories();
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
                    const accountData = await invoke('get_account', {
                        id: parseInt(this.id),
                    });

                    if (accountData) {
                        this.account = {
                            id: accountData.id,
                            name: accountData.name,
                            type: accountData.account_type,
                            current_balance: accountData.current_balance || 0,
                            institution: accountData.institution,
                            display_order: accountData.display_order,
                            include_in_net_worth: accountData.include_in_net_worth,
                            account_number_last4: accountData.account_number_last4,
                            created_at: accountData.created_at,
                            updated_at: accountData.updated_at,
                            archived: accountData.archived,
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
            async loadCategories() {
                try {
                    this.categories = await invoke('get_categories');
                    console.log('Loaded categories for AccountDetail:', this.categories);
                } catch (error) {
                    console.error('Failed to load categories:', error);
                    this.categories = [];
                }
            },
            getCategoryName(categoryId) {
                if (!categoryId) return 'Uncategorized';
                
                const category = this.categories.find(cat => cat.id === categoryId);
                if (!category) return `Category ${categoryId}`;
                
                // If category has a parent, show it as "Parent > Child"
                if (category.parent_category_id) {
                    const parent = this.categories.find(cat => cat.id === category.parent_category_id);
                    if (parent) {
                        return `${parent.name} > ${category.name}`;
                    }
                }
                
                return category.name;
            },
            formatCategoryName(category) {
                // If category has a parent, show it as "Parent > Child"
                if (category.parent_category_id) {
                    const parent = this.categories.find(cat => cat.id === category.parent_category_id);
                    if (parent) {
                        return `${parent.name} > ${category.name}`;
                    }
                }
                return category.name;
            },
            startEdit(transaction) {
                this.editingTransactionId = transaction.id;
                // Create a copy of the transaction for editing
                this.editingTransaction = {
                    date: transaction.date,
                    amount: transaction.amount,
                    description: transaction.description || '',
                    payee: transaction.payee || '',
                    memo: transaction.memo || '',
                    category_id: transaction.category_id,
                    pending: transaction.pending,
                    cleared: transaction.cleared,
                };
            },
            async saveTransaction() {
                try {
                    const request = {
                        date: this.editingTransaction.date,
                        amount: this.editingTransaction.amount,
                        description: this.editingTransaction.description || null,
                        payee: this.editingTransaction.payee || null,
                        memo: this.editingTransaction.memo || null,
                        category_id: this.editingTransaction.category_id || null,
                        pending: this.editingTransaction.pending,
                        cleared: this.editingTransaction.cleared,
                    };

                    await invoke('update_transaction', {
                        transactionId: this.editingTransactionId,
                        request: request,
                    });

                    // Refresh transactions to show updated data
                    await this.loadTransactions();
                    
                    // Exit edit mode
                    this.editingTransactionId = null;
                    this.editingTransaction = {};
                    
                    console.log('Transaction updated successfully');
                } catch (error) {
                    console.error('Failed to update transaction:', error);
                    alert('Failed to update transaction. Please try again.');
                }
            },
            cancelEdit() {
                this.editingTransactionId = null;
                this.editingTransaction = {};
            },
            async deleteTransaction(transactionId) {
                if (!confirm('Are you sure you want to delete this transaction?')) {
                    return;
                }

                try {
                    await invoke('delete_transaction', {
                        transactionId: transactionId,
                    });

                    // Refresh transactions to remove deleted transaction
                    await this.loadTransactions();
                    
                    console.log('Transaction deleted successfully');
                } catch (error) {
                    console.error('Failed to delete transaction:', error);
                    alert('Failed to delete transaction. Please try again.');
                }
            },
            async handleTransactionAdded() {
                // Close modal and refresh transactions list
                this.showTransactionModal = false;
                await this.loadTransactions();
                console.log('New transaction added, refreshing list');
            },
        },
    };
</script>
