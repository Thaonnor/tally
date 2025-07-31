<template>
    <li
        class="flex justify-between items-center px-5 py-3 cursor-pointer transition-colors hover:bg-gray-600"
        :class="{ 'bg-gray-600 text-gray-50': isActive }"
        @click="handleClick"
    >
        <span class="font-medium">{{ name }}</span>
        <span
            class="font-semibold text-emerald-500"
            :class="{ 'text-red-500': balance < 0 }"
        >
            {{ formatBalance(balance) }}
        </span>
    </li>
</template>

<script>
    export default {
        name: 'AccountItem',
        props: {
            id: {
                type: Number,
                required: true,
            },
            name: {
                type: String,
                required: true,
            },
            balance: {
                type: Number,
                required: true,
            },
        },
        computed: {
            isActive() {
                return this.$route.path === `/account/${this.id}`;
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
                this.$router.push(`/account/${this.id}`);
            },
        },
    };
</script>
