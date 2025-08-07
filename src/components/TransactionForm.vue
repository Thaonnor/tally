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
            <label for="payee" class="block mb-2 font-medium text-gray-50"
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
                <option 
                    v-for="category in nonSystemCategories" 
                    :key="category.id" 
                    :value="category.id"
                >
                    {{ formatCategoryName(category) }}
                </option>
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
                    categoryId: null,
                    memo: '',
                    pending: false,
                    cleared: false,
                },
                categories: [],
                categoriesLoading: true,
            };
        },
        async mounted() {
            await this.loadCategories();
        },
        computed: {
            nonSystemCategories() {
                // Filter out system categories since we show "Uncategorized" separately
                return this.categories.filter(category => !category.is_system_category);
            }
        },
        methods: {
            async loadCategories() {
                try {
                    this.categoriesLoading = true;
                    this.categories = await invoke('get_categories');
                    console.log('Loaded categories for transaction form:', this.categories);
                } catch (error) {
                    console.error('Failed to load categories:', error);
                    this.categories = [];
                } finally {
                    this.categoriesLoading = false;
                }
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
            async handleSubmit() {
                try {
                    console.log('Submitting transaction:', this.form);

                    const request = {
                        account_id: this.accountId,
                        date: this.form.date,
                        amount: this.form.amount,
                        description: this.form.description || null,
                        payee: this.form.payee || null,
                        memo: this.form.memo || null,
                        category_id: this.form.categoryId || null,
                        pending: this.form.pending,
                        cleared: this.form.cleared,
                    };

                    const result = await invoke('add_transaction', { request });

                    console.log('Transaction added with ID:', result);
                    this.$emit('success');
                } catch (error) {
                    console.error('Failed to add transaction:', error);
                }
            },
        },
    };
</script>
