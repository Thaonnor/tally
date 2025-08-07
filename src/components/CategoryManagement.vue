<template>
    <div class="flex-1 bg-gray-900 text-gray-50 p-5">
        <div class="flex justify-between items-center mb-6">
            <h1 class="text-3xl font-bold">Category Management</h1>
            <button 
                @click="showAddModal = true"
                class="px-6 py-3 bg-indigo-500 text-white rounded font-medium hover:bg-indigo-600 transition-colors"
            >
                Add New Category
            </button>
        </div>

        <!-- Loading State -->
        <div v-if="loading" class="text-center py-8">
            <p class="text-gray-400">Loading categories...</p>
        </div>

        <!-- Empty State -->
        <div v-else-if="categories.length === 0" class="text-center py-12">
            <p class="text-gray-400 text-lg mb-4">No categories found</p>
            <p class="text-gray-500 mb-6">Get started by adding your first category</p>
            <button 
                @click="showAddModal = true"
                class="px-6 py-3 bg-indigo-500 text-white rounded font-medium hover:bg-indigo-600 transition-colors"
            >
                Add Your First Category
            </button>
        </div>

        <!-- Categories Table -->
        <div v-else class="bg-gray-800 rounded-lg overflow-hidden">
            <table class="w-full">
                <thead class="bg-gray-700">
                    <tr>
                        <th class="px-6 py-4 text-left font-medium text-gray-300">Category Name</th>
                        <th class="px-6 py-4 text-left font-medium text-gray-300">Parent Category</th>
                        <th class="px-6 py-4 text-center font-medium text-gray-300">Display Order</th>
                        <th class="px-6 py-4 text-center font-medium text-gray-300">Default Type</th>
                        <th class="px-6 py-4 text-center font-medium text-gray-300">System</th>
                        <th class="px-6 py-4 text-center font-medium text-gray-300">Actions</th>
                    </tr>
                </thead>
                <tbody>
                    <tr 
                        v-for="category in categories" 
                        :key="category.id"
                        class="border-t border-gray-700 hover:bg-gray-700/50 transition-colors"
                    >
                        <td class="px-6 py-4">
                            <span class="font-medium text-gray-50">{{ category.name }}</span>
                        </td>
                        <td class="px-6 py-4 text-gray-300">
                            {{ getParentCategoryName(category.parent_category_id) || '—' }}
                        </td>
                        <td class="px-6 py-4 text-center text-gray-300">
                            {{ category.display_order || '—' }}
                        </td>
                        <td class="px-6 py-4 text-center">
                            <span v-if="category.default_discretionary" class="inline-block px-2 py-1 bg-blue-600 text-blue-100 text-sm rounded">
                                Discretionary
                            </span>
                            <span v-else-if="category.default_fixed" class="inline-block px-2 py-1 bg-orange-600 text-orange-100 text-sm rounded">
                                Fixed
                            </span>
                            <span v-else class="text-gray-500">—</span>
                        </td>
                        <td class="px-6 py-4 text-center">
                            <span v-if="category.is_system_category" class="text-yellow-400">⚠️</span>
                            <span v-else class="text-gray-500">—</span>
                        </td>
                        <td class="px-6 py-4 text-center">
                            <div class="flex justify-center gap-2">
                                <button 
                                    v-if="!category.is_system_category"
                                    @click="editCategory(category)"
                                    class="px-3 py-1 bg-gray-600 text-white text-sm rounded hover:bg-gray-700 transition-colors"
                                    title="Edit Category"
                                >
                                    Edit
                                </button>
                                <button 
                                    v-if="!category.is_system_category"
                                    @click="archiveCategory(category)"
                                    class="px-3 py-1 bg-orange-600 text-white text-sm rounded hover:bg-orange-700 transition-colors"
                                    title="Archive Category"
                                >
                                    Archive
                                </button>
                                <span v-if="category.is_system_category" class="text-gray-500 text-sm px-3 py-1">
                                    Protected
                                </span>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>

        <!-- Add Category Modal -->
        <Modal 
            :isOpen="showAddModal" 
            title="Add New Category" 
            @close="showAddModal = false"
        >
            <CategoryForm 
                mode="create"
                :categories="categories"
                @submit="handleAddCategory"
                @cancel="showAddModal = false"
            />
        </Modal>

        <!-- Edit Category Modal -->
        <Modal 
            :isOpen="showEditModal" 
            title="Edit Category" 
            @close="showEditModal = false"
        >
            <CategoryForm 
                mode="edit"
                :category="selectedCategory"
                :categories="categories"
                @submit="handleEditCategory"
                @cancel="showEditModal = false"
            />
        </Modal>

        <!-- Archive Category Confirmation Modal -->
        <Modal 
            :isOpen="showArchiveModal" 
            title="🗂️ Archive Category" 
            @close="showArchiveModal = false"
        >
            <div class="max-w-md">
                <div class="mb-4">
                    <p class="text-yellow-400 font-medium mb-3">
                        ⚠️ This will hide the category from your active categories list.
                    </p>
                    
                    <div class="bg-gray-700 p-4 rounded mb-4">
                        <p class="text-gray-300 mb-2">This will archive:</p>
                        <ul class="text-gray-300 space-y-1">
                            <li>• The category "{{ selectedCategory?.name }}"</li>
                            <li>• Category will be hidden from lists and forms</li>
                            <li>• All transaction history will be preserved</li>
                        </ul>
                    </div>
                    
                    <p class="text-gray-300 mb-3">
                        Type the category name to confirm archiving:
                    </p>
                </div>

                <div class="mb-4">
                    <input
                        v-model="archiveConfirmationText"
                        type="text"
                        :placeholder="selectedCategory?.name"
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
                        Archive Category
                    </button>
                </div>
            </div>
        </Modal>
    </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/core';
import Modal from './Modal.vue';
import CategoryForm from './CategoryForm.vue';

export default {
    name: 'CategoryManagement',
    components: {
        Modal,
        CategoryForm,
    },
    data() {
        return {
            categories: [],
            loading: true,
            showAddModal: false,
            showEditModal: false,
            showArchiveModal: false,
            selectedCategory: null,
            archiveConfirmationText: '',
            archiveError: '',
        };
    },
    async mounted() {
        await this.loadCategories();
    },
    computed: {
        canArchive() {
            return this.archiveConfirmationText === this.selectedCategory?.name;
        },
    },
    methods: {
        async loadCategories() {
            try {
                this.loading = true;
                this.categories = await invoke('get_categories');
                console.log('Loaded categories:', this.categories);
            } catch (error) {
                console.error('Failed to load categories:', error);
            } finally {
                this.loading = false;
            }
        },
        getParentCategoryName(parentId) {
            if (!parentId) return null;
            const parent = this.categories.find(cat => cat.id === parentId);
            return parent ? parent.name : `Category ${parentId}`;
        },
        async handleAddCategory(categoryData) {
            try {
                console.log('Adding category:', categoryData);

                const request = {
                    name: categoryData.name,
                    display_order: categoryData.displayOrder || null,
                    parent_category_id: categoryData.parentCategoryId || null,
                    default_discretionary: categoryData.defaultDiscretionary || null,
                    default_fixed: categoryData.defaultFixed || null,
                };

                const categoryId = await invoke('add_category', { request });
                console.log('Category added with ID:', categoryId);

                // Refresh the category list
                await this.loadCategories();

                // Notify parent to refresh any category-dependent components
                this.$emit('category-updated');

                // Close the modal
                this.showAddModal = false;
            } catch (error) {
                console.error('Failed to add category:', error);
                // TODO: Show error message to user
            }
        },
        editCategory(category) {
            this.selectedCategory = { ...category };
            this.showEditModal = true;
        },
        async handleEditCategory(categoryData) {
            try {
                console.log('Updating category:', this.selectedCategory.id, categoryData);

                const request = {
                    name: categoryData.name,
                    display_order: categoryData.displayOrder || null,
                    parent_category_id: categoryData.parentCategoryId || null,
                    default_discretionary: categoryData.defaultDiscretionary || null,
                    default_fixed: categoryData.defaultFixed || null,
                };

                await invoke('update_category', { 
                    categoryId: this.selectedCategory.id, 
                    request 
                });

                console.log('Category updated successfully');

                // Close modal and refresh data
                this.showEditModal = false;
                await this.loadCategories(); // Refresh category list
                this.$emit('category-updated'); // Notify parent
            } catch (error) {
                console.error('Failed to update category:', error);
                // TODO: Show error message to user in modal
            }
        },
        archiveCategory(category) {
            this.selectedCategory = { ...category };
            this.archiveConfirmationText = '';
            this.archiveError = '';
            this.showArchiveModal = true;
        },
        async confirmArchive() {
            if (!this.canArchive) {
                this.archiveError = 'Category name does not match';
                return;
            }

            try {
                console.log('Archiving category:', this.selectedCategory.id);
                await invoke('archive_category', { categoryId: this.selectedCategory.id });
                
                this.showArchiveModal = false;
                await this.loadCategories(); // Refresh list
                this.$emit('category-updated'); // Notify parent
                console.log('Category archived successfully');
            } catch (error) {
                console.error('Failed to archive category:', error);
                this.archiveError = 'Failed to archive category. Please try again.';
            }
        },
    },
};
</script>