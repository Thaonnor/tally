<template>
    <form @submit.prevent="handleSubmit" class="max-w-2xl">
        <!-- Basic Information Section -->
        <div class="mb-6">
            <h3 class="text-lg font-medium text-gray-50 mb-4">Basic Information</h3>
            
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
                <div>
                    <label for="name" class="block mb-2 font-medium text-gray-50">Account Name *</label>
                    <input
                        id="name"
                        v-model="form.name"
                        type="text"
                        required
                        placeholder="e.g., Chase Checking"
                        class="w-full p-3 border-gray-600 rounded bg-gray-900 text-gray-50 text-base focus:outline-none focus:border-indigo-500"
                    />
                </div>
                
                <div>
                    <label for="type" class="block mb-2 font-medium text-gray-50">Account Type *</label>
                    <select id="type" v-model="form.type" required class="w-full p-3 pr-10 border border-gray-600 rounded bg-gray-900 text-gray-50 text-base focus:outline-none focus:border-indigo-500 appearance-none">
                        <option value="">Select account type</option>
                        <option value="checking">Checking</option>
                        <option value="savings">Savings</option>
                        <option value="credit_card">Credit Card</option>
                        <option value="investment">Investment</option>
                        <option value="balance_only">Balance Only</option>
                    </select>
                </div>
            </div>

            <div>
                <label for="institution" class="block mb-2 font-medium text-gray-50">Institution</label>
                <input
                    id="institution"
                    v-model="form.institution"
                    type="text"
                    placeholder="e.g., Bank of America"
                    class="w-full p-3 border border-gray-600 rounded bg-gray-900 text-gray-50 text-base focus:outline-none focus:border-indigo-500"
                />
            </div>
        </div>

        <!-- Financial Details Section -->
        <div class="mb-6">
            <h3 class="text-lg font-medium text-gray-50 mb-4">Financial Details</h3>
            
            <!-- Starting Balance (Create mode only) -->
            <div v-if="mode === 'create'" class="mb-4">
                <label for="balance" class="block mb-2 font-medium text-gray-50">Starting Balance</label>
                <input
                    id="balance"
                    v-model.number="form.currentBalance"
                    type="number"
                    step="0.01"
                    placeholder="0.00"
                    class="w-full p-3 border border-gray-600 rounded bg-gray-900 text-gray-50 text-base focus:outline-none focus:border-indigo-500 appearance-none"
                />
                <p class="text-sm text-gray-400 mt-1">Optional: Set the initial balance for this account</p>
            </div>

            <!-- Current Balance (Edit mode - read-only) -->
            <div v-else class="mb-4">
                <label class="block mb-2 font-medium text-gray-50">Current Balance</label>
                <div class="w-full p-3 border border-gray-600 rounded bg-gray-800 text-gray-300 text-base font-mono">
                    {{ formatCurrency(account.current_balance) }}
                </div>
                <p class="text-sm text-gray-400 mt-1">Balance is calculated from transactions and cannot be edited directly</p>
            </div>

            <div>
                <label class="flex items-center gap-3">
                    <input
                        type="checkbox"
                        v-model="form.includeInNetWorth"
                        class="w-4 h-4 text-indigo-600 bg-gray-900 border-gray-600 rounded focus:ring-indigo-500 focus:ring-2"
                    />
                    <span class="font-medium text-gray-50">Include in Net Worth</span>
                </label>
                <p class="text-sm text-gray-400 mt-1 ml-7">Uncheck for liability accounts (loans, credit cards, etc.)</p>
            </div>
        </div>

        <!-- Additional Settings Section -->
        <div class="mb-6">
            <h3 class="text-lg font-medium text-gray-50 mb-4">Additional Settings</h3>
            
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                    <label for="displayOrder" class="block mb-2 font-medium text-gray-50">Display Order</label>
                    <input
                        id="displayOrder"
                        v-model.number="form.displayOrder"
                        type="number"
                        placeholder="Auto"
                        class="w-full p-3 border border-gray-600 rounded bg-gray-900 text-gray-50 text-base focus:outline-none focus:border-indigo-500 appearance-none"
                    />
                    <p class="text-sm text-gray-400 mt-1">Lower numbers appear first</p>
                </div>

                <div>
                    <label for="accountNumber" class="block mb-2 font-medium text-gray-50">Last 4 of Account Number</label>
                    <input
                        id="accountNumber"
                        v-model="form.accountNumberLast4"
                        type="text"
                        maxlength="4"
                        placeholder="1234"
                        @input="validateAccountNumber"
                        class="w-full p-3 border border-gray-600 rounded bg-gray-900 text-gray-50 text-base focus:outline-none focus:border-indigo-500 appearance-none"
                    />
                    <p class="text-sm text-gray-400 mt-1">For account identification</p>
                    <p v-if="accountNumberError" class="text-sm text-red-400 mt-1">{{ accountNumberError }}</p>
                </div>
            </div>
        </div>

        <div class="flex gap-3 justify-end mt-6">
            <button
                type="button"
                @click="$emit('cancel')"
                class="px-6 py-3 border-none rounded font-medium cursor-pointer transition-colors bg-gray-600 text-gray-50 hover:bg-gray-500"
            >
                Cancel
            </button>
            <button type="submit" class="px-6 py-3 border-none rounded font-medium cursor-pointer transition-colors bg-indigo-500 text-white hover:bg-indigo-600">
                {{ mode === 'edit' ? 'Update Account' : 'Add Account' }}
            </button>
        </div>
    </form>
</template>

<script>
    export default {
        name: 'AccountForm',
        props: {
            mode: {
                type: String,
                default: 'create', // 'create' or 'edit'
                validator: (value) => ['create', 'edit'].includes(value),
            },
            account: {
                type: Object,
                default: () => ({}),
            },
        },
        data() {
            return {
                form: {
                    name: '',
                    type: '',
                    institution: '',
                    currentBalance: null,
                    displayOrder: null,
                    accountNumberLast4: '',
                    includeInNetWorth: true,
                },
                accountNumberError: '',
            };
        },
        mounted() {
            // Pre-populate form if editing
            if (this.mode === 'edit' && this.account) {
                this.form = {
                    name: this.account.name || '',
                    type: this.account.account_type || '',
                    institution: this.account.institution || '',
                    currentBalance: this.account.current_balance,
                    displayOrder: this.account.display_order,
                    accountNumberLast4: this.account.account_number_last4 || '',
                    includeInNetWorth: this.account.include_in_net_worth ?? true,
                };
            }
        },
        methods: {
            handleSubmit() {
                // Validate before submit
                if (this.form.accountNumberLast4 && !this.isValidAccountNumber(this.form.accountNumberLast4)) {
                    this.accountNumberError = 'Account number must be exactly 4 digits';
                    return;
                }
                
                this.accountNumberError = '';
                this.$emit('submit', { ...this.form });
            },
            validateAccountNumber() {
                const value = this.form.accountNumberLast4;
                
                if (!value) {
                    this.accountNumberError = '';
                    return;
                }
                
                if (!this.isValidAccountNumber(value)) {
                    this.accountNumberError = 'Only 4 digits allowed (0-9)';
                } else {
                    this.accountNumberError = '';
                }
            },
            isValidAccountNumber(value) {
                return /^\d{4}$/.test(value);
            },
            formatCurrency(amount) {
                if (amount === null || amount === undefined) return '$0.00';
                return new Intl.NumberFormat('en-US', {
                    style: 'currency',
                    currency: 'USD',
                }).format(amount);
            },
        },
    };
</script>

<style scoped>
    .account-form {
        max-width: 400px;
    }

    .form-group {
        margin-bottom: 20px;
    }

    .form-group label {
        display: block;
        margin-bottom: 8px;
        font-weight: 500;
        color: var(--text-primary);
    }

    .form-input {
        width: 100%;
        padding: 12px;
        border: 1px solid var(--bg-tertiary);
        border-radius: 4px;
        background-color: var(--bg-primary);
        color: var(--text-primary);
        font-size: var(--text-body);
    }

    .form-input:focus {
        outline: none;
        border-color: var(--primary);
    }

    .form-input select,
    select.form-input {
        appearance: none; /* Remove default browser styling */
        background-image: url("data:image/svg+xml;charset=UTF-8,%3csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='%23d1d5db' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3e%3cpolyline points='6,9 12,15 18,9'%3e%3c/polyline%3e%3c/svg%3e");
        background-repeat: no-repeat;
        background-position: right 12px center;
        background-size: 16px;
        padding-right: 40px;
    }

    .form-input select:focus,
    select.form-input:focus {
        background-image: url("data:image/svg+xml;charset=UTF-8,%3csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='%236366f1' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3e%3cpolyline points='6,9 12,15 18,9'%3e%3c/polyline%3e%3c/svg%3e");
    }

    .form-actions {
        display: flex;
        gap: 12px;
        justify-content: flex-end;
        margin-top: 24px;
    }

    .btn {
        padding: 12px 24px;
        border: none;
        border-radius: 4px;
        font-weight: 500;
        cursor: pointer;
        transition: background-color 0.2s;
    }

    .btn-primary {
        background-color: var(--primary);
        color: white;
    }

    .btn-primary:hover {
        background-color: #5855eb;
    }

    .btn-secondary {
        background-color: var(--bg-tertiary);
        color: var(--text-primary);
    }

    .btn-secondary:hover {
        background-color: #4b5563;
    }
</style>
