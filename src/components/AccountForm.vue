<template>
    <form @submit.prevent="handleSubmit" class="max-w-sm">
        <div class="mb-5">
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

        <div class="mb-5">
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

        <div class="mb-5">
            <label for="institution" class="block mb-2 font-medium text-gray-50">Institution</label>
            <input
                id="institution"
                v-model="form.institution"
                type="text"
                placeholder="e.g., Bank of America"
                class="w-full p-3 border border-gray-600 rounded bg-gray-900 text-gray-50 text-base focus:outline-none focus:border-indigo-500"
            />
        </div>

        <div class="mb-5">
            <label for="balance" class="block mb-2 font-medium text-gray-50">Starting Balance</label>
            <input
                id="balance"
                v-model.number="form.currentBalance"
                type="number"
                step="0.01"
                placeholder="0.00"
                class="w-full p-3 border border-gray-600 rounded bg-gray-900 text-gray-50 text-base focus:outline-none focus:border-indigo-500 appearance-none"
            />
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
                },
            };
        },
        mounted() {
            // Pre-populate form if editing
            if (this.mode === 'edit' && this.account) {
                this.form = { ...this.account };
            }
        },
        methods: {
            handleSubmit() {
                this.$emit('submit', { ...this.form });
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
