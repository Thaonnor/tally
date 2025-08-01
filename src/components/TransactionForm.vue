<template>
    <form @submit.prevent="handleSubmit" class="max-w-lg">
        <div class="mb-4">
            <label for="amount" class="block mb-2 font-medium text-gray-50">Amount *</label>
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
            <label for="description" class="block mb-2 font-medium text-gray-50">Description</label>
            <!-- Add v-model."form.description" -->
            <input 
                id="description"
                type="text"
                placeholder="What was this for?"
                class="w-full p-3 border border-gray-600 rounded bg-gray-900 focus:outline-none focus:border-indigo-500"
            />
        </div>

        <div class="mb-4">
            <label for="date" class="block mb-2 font-medium text-gray-50">Date *</label>
            <!-- Add v-model."form.date" -->
            <input 
                id="date"
                type="date"
                required
                class="w-full p-3 border border-gray-600 rounded bg-gray-900 text-gray-50 focus:outline-none focus:border-indigo-500"
            />
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
                    date: new Date().toISOString().split('T')[0]
                }
            }
        },
        methods: {
            handleSubmit() {
                console.log('Form submitted:', this.form);
                // we'll handle this later
            }
        }
    };
</script>
