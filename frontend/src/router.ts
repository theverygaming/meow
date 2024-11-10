import { createWebHistory, createRouter } from "vue-router";

import TestView from "./views/TestView.vue";
import TestView2 from "./views/TestView2.vue";
import Brainlog from "./views/Brainlog.vue";

const routes = [
  { path: "/", component: TestView },
  { path: "/test", component: TestView2 },
  { path: "/brainlog", component: Brainlog },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
