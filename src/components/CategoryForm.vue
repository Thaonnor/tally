<template>
    <form @submit.prevent="handleSubmit" class="space-y-4">
        <!-- Category Name -->
        <div>
            <label for="name" class="block mb-2 font-medium text-gray-50">
                Category Name *
            </label>
            <input
                id="name"
                v-model="form.name"
                type="text"
                required
                placeholder="e.g., Groceries, Gas, Entertainment"
                class="w-full p-3 border border-gray-600 rounded bg-gray-900 text-gray-50 focus:outline-none focus:border-indigo-500"
            />
        </div>

        <!-- Parent Category -->
        <div>
            <label for="parentCategory" class="block mb-2 font-medium text-gray-50">
                Parent Category
            </label>
            <select
                id="parentCategory"
                v-model.number="form.parentCategoryId"
                class="w-full p-3 pr-10 border border-gray-600 rounded bg-gray-900 text-gray-50 focus:outline-none focus:border-indigo-500 appearance-none"
            >
                <option :value="null">None (Top Level)</option>
                <option 
                    v-for="category in availableParentCategories" 
                    :key="category.id" 
                    :value="category.id"
                >
                    {{ category.name }}
                </option>
            </select>
            <p class="text-gray-400 text-sm mt-1">
                Optional: Choose a parent category to create a subcategory
            </p>
        </div>

        <!-- Display Order -->
        <div>
            <label for="displayOrder" class="block mb-2 font-medium text-gray-50">
                Display Order
            </label>
            <input
                id="displayOrder"
                v-model.number="form.displayOrder"
                type="number"
                min="0"
                placeholder="e.g., 1, 10, 100"
                class="w-full p-3 border border-gray-600 rounded bg-gray-900 text-gray-50 focus:outline-none focus:border-indigo-500"
            />
            <p class="text-gray-400 text-sm mt-1">
                Optional: Lower numbers appear first in lists
            </p>
        </div>

        <!-- Default Type Settings -->
        <div>
            <label class="block mb-3 font-medium text-gray-50">
                Default Spending Type
            </label>
            <div class="space-y-2">
                <label class="flex items-center">
                    <input
                        v-model="defaultTypeSelection"
                        type="radio"
                        value="none"
                        class="mr-3 text-indigo-500 focus:ring-indigo-500 focus:ring-2"
                    />
                    <span class="text-gray-300">No default (flexible)</span>
                </label>
                <label class="flex items-center">
                    <input
                        v-model="defaultTypeSelection"
                        type="radio"
                        value="discretionary"
                        class="mr-3 text-indigo-500 focus:ring-indigo-500 focus:ring-2"
                    />
                    <span class="text-gray-300">Discretionary spending</span>
                </label>
                <label class="flex items-center">
                    <input
                        v-model="defaultTypeSelection"
                        type="radio"
                        value="fixed"
                        class="mr-3 text-indigo-500 focus:ring-indigo-500 focus:ring-2"
                    />
                    <span class="text-gray-300">Fixed expenses</span>
                </label>
            </div>
            <p class="text-gray-400 text-sm mt-2">
                This helps categorize transactions for budgeting and reporting
            </p>
        </div>

        <!-- Form Actions -->
        <div class="flex gap-3 pt-4">
            <button
                type="button"
                @click="$emit('cancel')"
                class="flex-1 px-6 py-3 bg-gray-600 text-white rounded font-medium hover:bg-gray-700 transition-colors"
            >
                Cancel
            </button>
            <button
                type="submit"
                :disabled="!form.name.trim()"
                :class="[
                    'flex-1 px-6 py-3 rounded font-medium transition-colors',
                    form.name.trim()
                        ? 'bg-indigo-500 text-white hover:bg-indigo-600'
                        : 'bg-gray-500 text-gray-300 cursor-not-allowed'
                ]"
            >
                {{ mode === 'create' ? 'Add Category' : 'Update Category' }}
            </button>
        </div>
    </form>
</template>

<script>
export default {
    name: 'CategoryForm',
    props: {
        mode: {
            type: String,
            required: true,
            validator: (value) => ['create', 'edit'].includes(value),
        },
        category: {
            type: Object,
            default: null,
        },
        categories: {
            type: Array,
            default: () => [],
        },
    },
    data() {
        return {
            form: {
                name: '',
                parentCategoryId: null,
                displayOrder: null,
                defaultDiscretionary: null,
                defaultFixed: null,
            },
            defaultTypeSelection: 'none', // Used for the radio buttons
        };
    },
    computed: {
        availableParentCategories() {
            // Filter out system categories and the current category (if editing)
            return this.categories.filter(cat => {
                // Don't show system categories as parent options
                if (cat.is_system_category) return false;
                
                // Don't show the current category as a parent option (prevents circular references)
                if (this.mode === 'edit' && this.category && cat.id === this.category.id) {
                    return false;
                }
                
                return true;
            });
        },
    },
    watch: {
        category: {
            handler(newCategory) {
                if (newCategory) {
                    this.populateForm(newCategory);
                }
            },
            immediate: true,
        },
        defaultTypeSelection(newValue) {
            // Update the form based on radio button selection
            this.form.defaultDiscretionary = newValue === 'discretionary' ? true : null;
            this.form.defaultFixed = newValue === 'fixed' ? true : null;
        },
    },
    methods: {
        populateForm(category) {
            this.form.name = category.name || '';
            this.form.parentCategoryId = category.parent_category_id || null;
            this.form.displayOrder = category.display_order || null;
            this.form.defaultDiscretionary = category.default_discretionary;
            this.form.defaultFixed = category.default_fixed;

            // Set the radio button selection based on the category data
            if (category.default_discretionary === true) {
                this.defaultTypeSelection = 'discretionary';
            } else if (category.default_fixed === true) {
                this.defaultTypeSelection = 'fixed';
            } else {
                this.defaultTypeSelection = 'none';
            }
        },
        handleSubmit() {
            if (!this.form.name.trim()) {
                return;
            }

            // Prepare the data to emit
            const formData = {
                name: this.form.name.trim(),
                parentCategoryId: this.form.parentCategoryId,
                displayOrder: this.form.displayOrder,
                defaultDiscretionary: this.form.defaultDiscretionary,
                defaultFixed: this.form.defaultFixed,
            };

            this.$emit('submit', formData);
        },
        resetForm() {
            this.form = {
                name: '',
                parentCategoryId: null,
                displayOrder: null,
                defaultDiscretionary: null,
                defaultFixed: null,
            };
            this.defaultTypeSelection = 'none';
        },
    },
    mounted() {
        // If this is a create form, reset to defaults
        if (this.mode === 'create') {
            this.resetForm();
        }
        // If this is an edit form and we have category data, populate it
        else if (this.mode === 'edit' && this.category) {
            this.populateForm(this.category);
        }
    },
};
</script>