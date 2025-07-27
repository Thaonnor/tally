<template>
    <li class="account-item" @click="handleClick">
        <span class="account-name">{{ name }}</span>
        <span class="account-balance" :class="{ negative: balance < 0 }">
            {{ formatBalance(balance) }}
        </span>
    </li>
</template>

<script>
    export default {
        name: 'AccountItem',
        props: {
            name: {
                type: String,
                required: true,
            },
            balance: {
                type: Number,
                required: true,
            },
        },
        methods: {
            formatBalance(amount) {
                return new Intl.NumberFormat('en-US', {
                    style: 'currency',
                    currency: 'USD',
                }).format(amount);
            },
            handleClick() {
                // Will emit events or handle navigation later
                console.log(`Clicked on account: ${this.name}`);
            },
        },
    };
</script>

<style scoped>
    .account-item {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 12px 20px;
        cursor: pointer;
        transition: background-color 0.2s;
    }

    .account-item:hover {
        background-color: var(--bg-tertiary);
    }

    .account-name {
        font-weight: 500;
    }

    .account-balance {
        font-weight: 600;
        color: var(--success);
    }

    .negative {
        color: var(--danger) !important;
    }
</style>
