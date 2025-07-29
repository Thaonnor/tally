<template>
    <div v-if="isOpen" class="fixed inset-0 bg-black bg-opacity-70 flex justify-center items-center z-50" @click="closeModal">
        <div class="bg-gray-800 rounded-lg min-w-96 max-w-[90vw] max-h-[90vh] overflow-y-auto" @click.stop>
            <div class="flex justify-between items-center p-5 border-b border-gray-600">
                <h3 class="m-0 text-gray-50">{{ title }}</h3>
                <button class="bg-transparent border-none text-2xl text-gray-300 cursor-pointer p-0 w-8 h-8 flex justify-center items-center hover:text-gray-50" @click="closeModal">
                    &times;
                </button>
            </div>
            <div class="p-5">
                <slot></slot>
            </div>
        </div>
    </div>
</template>

<script>
    export default {
        name: 'Modal',
        props: {
            isOpen: {
                type: Boolean,
                required: true,
            },
            title: {
                type: String,
                default: 'Modal',
            },
        },
        methods: {
            closeModal() {
                this.$emit('close');
            },
        },
    };
</script>