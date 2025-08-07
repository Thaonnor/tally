<template>
    <div class="flex-1 bg-gray-900 text-gray-50 p-5">
        <div class="flex justify-between items-center mb-6">
            <h1 class="text-3xl font-bold">Account Management</h1>
            <button 
                @click="showAddModal = true"
                class="px-6 py-3 bg-indigo-500 text-white rounded font-medium hover:bg-indigo-600 transition-colors"
            >
                Add New Account
            </button>
        </div>

        <!-- Loading State -->
        <div v-if="loading" class="text-center py-8">
            <p class="text-gray-400">Loading accounts...</p>
        </div>

        <!-- Empty State -->
        <div v-else-if="accounts.length === 0" class="text-center py-12">
            <p class="text-gray-400 text-lg mb-4">No accounts found</p>
            <p class="text-gray-500 mb-6">Get started by adding your first account</p>
            <button 
                @click="showAddModal = true"
                class="px-6 py-3 bg-indigo-500 text-white rounded font-medium hover:bg-indigo-600 transition-colors"
            >
                Add Your First Account
            </button>
        </div>

        <!-- Accounts Table -->
        <div v-else class="bg-gray-800 rounded-lg overflow-hidden">
            <table class="w-full">
                <thead class="bg-gray-700">
                    <tr>
                        <th class="px-6 py-4 text-left font-medium text-gray-300">Account Name</th>
                        <th class="px-6 py-4 text-left font-medium text-gray-300">Type</th>
                        <th class="px-6 py-4 text-left font-medium text-gray-300">Institution</th>
                        <th class="px-6 py-4 text-right font-medium text-gray-300">Balance</th>
                        <th class="px-6 py-4 text-center font-medium text-gray-300">Net Worth</th>
                        <th class="px-6 py-4 text-center font-medium text-gray-300">Actions</th>
                    </tr>
                </thead>
                <tbody>
                    <tr 
                        v-for="account in accounts" 
                        :key="account.id"
                        class="border-t border-gray-700 hover:bg-gray-700/50 transition-colors"
                    >
                        <td class="px-6 py-4">
                            <div class="flex flex-col">
                                <span class="font-medium text-gray-50">{{ account.name }}</span>
                                <span v-if="account.account_number_last4" class="text-sm text-gray-400">
                                    ...{{ account.account_number_last4 }}
                                </span>
                            </div>
                        </td>
                        <td class="px-6 py-4">
                            <span class="inline-block px-2 py-1 bg-gray-600 text-gray-300 text-sm rounded capitalize">
                                {{ account.account_type.replace('_', ' ') }}
                            </span>
                        </td>
                        <td class="px-6 py-4 text-gray-300">
                            {{ account.institution || '—' }}
                        </td>
                        <td class="px-6 py-4 text-right font-mono">
                            <span :class="balanceClass(account.current_balance)">
                                {{ formatCurrency(account.current_balance) }}
                            </span>
                        </td>
                        <td class="px-6 py-4 text-center">
                            <span v-if="account.include_in_net_worth" class="text-green-400">✓</span>
                            <span v-else class="text-gray-500">—</span>
                        </td>
                        <td class="px-6 py-4 text-center">
                            <div class="flex justify-center gap-2">
                                <button 
                                    @click="viewAccount(account.id)"
                                    class="px-3 py-1 bg-blue-600 text-white text-sm rounded hover:bg-blue-700 transition-colors"
                                    title="View Transactions"
                                >
                                    View
                                </button>
                                <button 
                                    @click="editAccount(account)"
                                    class="px-3 py-1 bg-gray-600 text-white text-sm rounded hover:bg-gray-700 transition-colors"
                                    title="Edit Account"
                                >
                                    Edit
                                </button>
                                <button 
                                    @click="archiveAccount(account)"
                                    class="px-3 py-1 bg-orange-600 text-white text-sm rounded hover:bg-orange-700 transition-colors"
                                    title="Archive Account"
                                >
                                    Archive
                                </button>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>

        <!-- Add Account Modal -->
        <Modal 
            :isOpen="showAddModal" 
            title="Add New Account" 
            @close="showAddModal = false"
        >
            <AccountForm 
                mode="create"
                @submit="handleAddAccount"
                @cancel="showAddModal = false"
            />
        </Modal>

        <!-- Edit Account Modal -->
        <Modal 
            :isOpen="showEditModal" 
            title="Edit Account" 
            @close="showEditModal = false"
        >
            <AccountForm 
                mode="edit"
                :account="selectedAccount"
                @submit="handleEditAccount"
                @cancel="showEditModal = false"
            />
        </Modal>

        <!-- Archive Account Confirmation Modal -->
        <Modal 
            :isOpen="showArchiveModal" 
            title="📁 Archive Account" 
            @close="showArchiveModal = false"
        >
            <div class="max-w-md">
                <div class="mb-4">
                    <p class="text-yellow-400 font-medium mb-3">
                        ⚠️ This will hide the account from your active accounts list.
                    </p>
                    
                    <div class="bg-gray-700 p-4 rounded mb-4">
                        <p class="text-gray-300 mb-2">This will archive:</p>
                        <ul class="text-gray-300 space-y-1">
                            <li>• The account "{{ selectedAccount?.name }}"</li>
                            <li>• Account will be hidden from lists and reports</li>
                            <li>• All transaction history will be preserved</li>
                        </ul>
                    </div>
                    
                    <p class="text-gray-300 mb-3">
                        Type the account name to confirm archiving:
                    </p>
                </div>

                <div class="mb-4">
                    <input
                        v-model="archiveConfirmationText"
                        type="text"
                        :placeholder="selectedAccount?.name"
                        class="w-full p-3 border border-gray-600 rounded bg-gray-900 text-gray-50 text-base focus:outline-none focus:border-red-500"
                        @keyup.enter="confirmArchive"
                    />
                    <p v-if="archiveError" class="text-red-400 text-sm mt-2">{{ archiveError }}</p>
                </div>

                <div class="flex gap-3 justify-end">
                    <button
                        @click="showArchiveModal = false"
                        class="px-6 py-3 bg-gray-600 text-white rounded font-medium hover:bg-gray-700 transition-colors"
                    >
                        Cancel
                    </button>
                    <button
                        @click="confirmArchive"
                        :disabled="!canArchive"
                        :class="[
                            'px-6 py-3 rounded font-medium transition-colors',
                            canArchive 
                                ? 'bg-orange-600 text-white hover:bg-orange-700' 
                                : 'bg-gray-500 text-gray-300 cursor-not-allowed'
                        ]"
                    >
                        Archive Account
                    </button>
                </div>
            </div>
        </Modal>
    </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/core';
import Modal from './Modal.vue';
import AccountForm from './AccountForm.vue';

export default {
    name: 'AccountManagement',
    components: {
        Modal,
        AccountForm,
    },
    data() {
        return {
            accounts: [],
            loading: true,
            showAddModal: false,
            showEditModal: false,
            showArchiveModal: false,
            selectedAccount: null,
            archiveConfirmationText: '',
            archiveError: '',
        };
    },
    async mounted() {
        await this.loadAccounts();
    },
    computed: {
        canArchive() {
            return this.archiveConfirmationText === this.selectedAccount?.name;
        },
    },
    methods: {
        async loadAccounts() {
            try {
                this.loading = true;
                this.accounts = await invoke('get_accounts');
                console.log('Loaded accounts:', this.accounts);
            } catch (error) {
                console.error('Failed to load accounts:', error);
            } finally {
                this.loading = false;
            }
        },
        async handleAddAccount(accountData) {
            try {
                console.log('Adding account:', accountData);

                const request = {
                    name: accountData.name,
                    account_type: accountData.type,
                    institution: accountData.institution || null,
                    current_balance: accountData.currentBalance || null,
                    display_order: accountData.displayOrder || null,
                    include_in_net_worth: accountData.includeInNetWorth ?? null, // Use nullish coalescing
                    account_number_last4: accountData.accountNumberLast4 || null,
                };

                const accountId = await invoke('add_account', { request });
                console.log('Account added with ID:', accountId);

                // Refresh the account list
                await this.loadAccounts();

                // Notify parent to refresh sidebar
                this.$emit('account-updated');

                // Close the modal
                this.showAddModal = false;
            } catch (error) {
                console.error('Failed to add account:', error);
                // TODO: Show error message to user
            }
        },
        editAccount(account) {
            this.selectedAccount = { ...account };
            this.showEditModal = true;
        },
        async handleEditAccount(accountData) {
            try {
                console.log('Updating account:', this.selectedAccount.id, accountData);

                const request = {
                    name: accountData.name,
                    account_type: accountData.type,
                    institution: accountData.institution || null,
                    current_balance: accountData.currentBalance || null,
                    display_order: accountData.displayOrder || null,
                    include_in_net_worth: accountData.includeInNetWorth ?? null,
                    account_number_last4: accountData.accountNumberLast4 || null,
                };

                await invoke('update_account', { 
                    accountId: this.selectedAccount.id, 
                    request 
                });

                console.log('Account updated successfully');

                // Close modal and refresh data
                this.showEditModal = false;
                await this.loadAccounts(); // Refresh account list
                this.$emit('account-updated'); // Notify parent to refresh sidebar
            } catch (error) {
                console.error('Failed to update account:', error);
                // TODO: Show error message to user in modal
            }
        },
        viewAccount(accountId) {
            this.$router.push(`/account/${accountId}`);
        },
        archiveAccount(account) {
            this.selectedAccount = { ...account };
            this.archiveConfirmationText = '';
            this.archiveError = '';
            this.showArchiveModal = true;
        },
        async confirmArchive() {
            if (!this.canArchive) {
                this.archiveError = 'Account name does not match';
                return;
            }

            try {
                console.log('Archiving account:', this.selectedAccount.id);
                await invoke('archive_account', { accountId: this.selectedAccount.id });
                
                this.showArchiveModal = false;
                await this.loadAccounts(); // Refresh list
                this.$emit('account-archived'); // Notify parent to refresh sidebar
                console.log('Account archived successfully');
            } catch (error) {
                console.error('Failed to archive account:', error);
                this.archiveError = 'Failed to archive account. Please try again.';
            }
        },
        formatCurrency(amount) {
            if (amount === null || amount === undefined) return '—';
            return new Intl.NumberFormat('en-US', {
                style: 'currency',
                currency: 'USD',
            }).format(amount);
        },
        balanceClass(balance) {
            if (balance === null || balance === undefined) return 'text-gray-400';
            return balance >= 0 ? 'text-green-400' : 'text-red-400';
        },
    },
};
</script>