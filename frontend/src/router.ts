import { createWebHistory, createRouter } from "vue-router";

import TestView from "./views/TestView.vue";
import TestView2 from "./views/TestView2.vue";

const routes = [
  { path: "/", component: TestView },
  { path: "/test", component: TestView2 },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
