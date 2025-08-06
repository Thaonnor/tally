import { createRouter, createWebHistory } from 'vue-router';
import Dashboard from '../components/Dashboard.vue';
import AccountDetail from '../components/AccountDetail.vue';
import AccountManagement from '../components/AccountManagement.vue';

const routes = [
    {
        path: '/',
        name: 'Dashboard',
        component: Dashboard,
    },
    {
        path: '/accounts',
        name: 'AccountManagement',
        component: AccountManagement,
    },
    {
        path: '/account/:id',
        name: 'AccountDetail',
        component: AccountDetail,
        props: true,
    },
];

const router = createRouter({
    history: createWebHistory(),
    routes,
});

export default router;
