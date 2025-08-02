<template>
    <form @submit.prevent="handleSubmit" class="max-w-lg">
        <div class="mb-4">
            <label for="amount" class="block mb-2 font-medium text-gray-50"
                >Amount *</label
            >
            <input
                id="amount"
                v-model.number="form.amount"
                type="number"
                step="0.01"
                required
                placeholder="0.00"
                class="w-full p-3 border border-gray-600 rounded bg-gray-900 focus:outline-none focus:border-indigo-500"
            />
        </div>

        <div class="mb-4">
            <label for="description" class="block mb-2 font-medium text-gray-50"
                >Description</label
            >
            <input
                id="description"
                v-model="form.description"
                type="text"
                placeholder="What was this for?"
                class="w-full p-3 border border-gray-600 rounded bg-gray-900 focus:outline-none focus:border-indigo-500"
            />
        </div>

        <div class="mb-4">
            <label for="date" class="block mb-2 font-medium text-gray-50"
                >Date *</label
            >
            <input
                id="date"
                v-model="form.date"
                type="date"
                required
                class="w-full p-3 border border-gray-600 rounded bg-gray-900 text-gray-50 focus:outline-none focus:border-indigo-500"
            />
        </div>

        <div class="mb-4">
            <label for="payee" class="block mb-2 font-medium text-gray 50"
                >Payee</label
            >
            <input
                id="payee"
                v-model="form.payee"
                type="text"
                placeholder="e.g. Grocery Store, John Doe"
                class="w-full p-3 border border-gray-600 rounded bg-gray-900 text-gray-50 focus:outline-none focus:border-indigo-500"
            />
        </div>

        <div class="mb-4">
            <label for="category" class="block mb-2 font-medium text-gray-50"
                >Category</label
            >
            <select
                id="category"
                v-model.number="form.categoryId"
                class="w-full p-3 pr-10 border border-gray-600 rounded bg-gray-900 text-gray-50 focus:outline-none focus:border-indigo-500 appearance-none"
            >
                <option :value="null">Uncategorized</option>
                <option :value="1">Gas</option>
                <option :value="2">Dining Out</option>
            </select>
        </div>

        <div class="mb-4">
            <label for="memo" class="block mb-2 font-medium text-gray-50"
                >Memo</label
            >
            <textarea
                id="memo"
                v-model="form.memo"
                rows="3"
                placeholder="Additional notes..."
                class="w-full p-3 border border-gray-600 rounded bg-gray-900 text-gray-50 focus:outline-none focus:border-indigo-500 resize-none"
            ></textarea>
        </div>

        <div class="mb-4">
            <div class="flex items-center gap-6">
                <label class="flex items-center">
                    <input
                        type="checkbox"
                        v-model="form.pending"
                        class="mr-2 accent-indigo-500 w-4 h-4"
                    />
                    <span class="text-gray-50">Pending</span>
                </label>
                <label class="flex items-center">
                    <input
                        type="checkbox"
                        v-model="form.cleared"
                        class="mr-2 accent-indigo-500 w-4 h-4"
                    />
                    <span class="text-gray-50">Cleared</span>
                </label>
            </div>
        </div>

        <div class="flex gap-3 justify-end mt-6">
            <button
                type="button"
                @click="$emit('cancel')"
                class="px-4 py-2 bg-gray-600 text-gray-50 rounded hover:bg-gray-500"
            >
                Cancel
            </button>
            <button
                type="submit"
                class="px-4 py-2 bg-indigo-500 text-white rounded hover:bg-indigo-600"
            >
                Add Transaction
            </button>
        </div>
    </form>
</template>

<script>
    import { invoke } from '@tauri-apps/api/core';

    export default {
        name: 'TransactionForm',
        props: {
            accountId: {
                type: Number,
                required: true,
            },
        },
        data() {
            return {
                form: {
                    amount: null,
                    description: '',
                    date: new Date().toISOString().split('T')[0],
                    payee: '',
                    categoryId: '',
                    memo: '',
                    pending: false,
                    cleared: false,
                },
            };
        },
        methods: {
            async handleSubmit() {
                try {
                    console.log('Submitting transaction:', this.form);

                    const result = await invoke('add_transaction', {
                        accountId: this.accountId,
                        date: this.form.date,
                        amount: this.form.amount,
                        description: this.form.description || null,
                        payee: this.form.payee || null,
                        memo: this.form.memo || null,
                        categoryId: this.form.categoryId || null,
                        pending: this.form.pending,
                        cleared: this.form.cleared,
                    });

                    console.log('Transaction added with ID:', result);
                    this.$emit('success');
                } catch (error) {
                    console.error('Failed to add transaction:', error);
                }
            },
        },
    };
</script>
