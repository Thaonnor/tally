<template>
    <form @submit.prevent="handleSubmit" class="max-w-lg">
        <div class="mb-4">
            <label for="amount" class="block mb-2 font-medium text-gray-50"
                >Amount *</label
            >
            <!-- Add v-model.number="form.amount" -->
            <input
                id="amount"
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
            <!-- Add v-model."form.description" -->
            <input
                id="description"
                type="text"
                placeholder="What was this for?"
                class="w-full p-3 border border-gray-600 rounded bg-gray-900 focus:outline-none focus:border-indigo-500"
            />
        </div>

        <div class="mb-4">
            <label for="date" class="block mb-2 font-medium text-gray-50"
                >Date *</label
            >
            <!-- Add v-model."form.date" -->
            <input
                id="date"
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
                class="w-full p-3 pr-10 border border-gray-600 rounded bg-gray-900 text-gray-50 focus:outline-none focus:border-indigo-500 appearance-none"
            >
                <option value="1">Uncategorized</option>
                <option value="2">Gas</option>
                <option value="3">Dining Out</option>
            </select>
        </div>

        <div class="mb-4">
            <label for="memo" class="block mb-2 font-medium text-gray-50"
                >Memo</label
            >
            <textarea
                id="memo"
                rows="3"
                placeholder="Additional notes..."
                class="w-full p-3 border border-gray-600 rounded bg-gray-900 text-gray-50 focus:outline-none focus:border-indigo-500 resize-none"
            ></textarea>
        </div>

        <div class="mb-4">
            <div class="flex items-center gap-6">
                <label class="flex items-center">
                    <input type="checkbox" class="mr-2 accent-indigo-500 w-4 h-4" />
                    <span class="text-gray-50">Pending</span>
                </label>
                <label class="flex items-center">
                    <input type="checkbox" class="mr-2 accent-indigo-500 w-4 h-4" />
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
    export default {
        name: 'TransactionForm',
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
            handleSubmit() {
                console.log('Form submitted:', this.form);
                // we'll handle this later
            },
        },
    };
</script>
