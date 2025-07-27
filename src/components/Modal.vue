<template>
    <div v-if="isOpen" class="modal-overlay" @click="closeModal">
        >
        <div class="modal-content" @click.stop>
            <div class="modal-header">
                <h3>{{ title }}</h3>
                <button class="close-button" @click="closeModal">
                    &times;
                </button>
            </div>
            <div class="modal-body">
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

<style scoped>
    .modal-overlay {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background-color: rgba(0, 0, 0, 0.7);
        display: flex;
        justify-content: center;
        align-items: center;
        z-index: 1000;
    }

    .modal-content {
        background-color: var(--bg-secondary);
        border-radius: 8px;
        min-width: 400px;
        max-width: 90vw;
        max-height: 90vh;
        overflow-y: auto;
    }

    .modal-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 20px;
        border-bottom: 1px solid var(--bg-tertiary);
    }

    .modal-header h3 {
        margin: 0;
        color: var(--text-primary);
    }

    .close-button {
        background: none;
        border: none;
        font-size: 24px;
        color: var(--text-secondary);
        cursor: pointer;
        padding: 0;
        width: 30px;
        height: 30px;
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .close-button:hover {
        color: var(--text-primary);
    }

    .modal-body {
        padding: 20px;
    }
</style>
