import { createWebHistory, createRouter } from "vue-router";

import Page from "./Page.vue";
import Login from "./Login.vue";

import TestView from "./views/TestView.vue";
import TestView2 from "./views/TestView2.vue";
import Brainlog from "./views/Brainlog.vue";
import Quest from "./views/Quest.vue";
import QuestItem from "./views/QuestItem.vue";

const routes = [
  { 
    path: "/",
    component: Page,
    children: [
      { path: "/", redirect: "/test" },
      { path: "/test", component: TestView },
      { path: "/test2", component: TestView2 },
      { path: "/brainlog", component: Brainlog },
      { path: "/quest", component: Quest },
      { path: "/quest/items", component: QuestItem },
    ], 
  },
  { 
    path: "/login",
    component: Login,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

// https://stackoverflow.com/a/46461592
router.beforeEach(function (to, from, next) {
  const is_logged_in = Boolean(localStorage.getItem("API_KEY"));
  if ((to.path !== "/login" && to.path !== "login") && !is_logged_in) { // not logged in, not trying to access login page
    next({ path: "/login" });
  } else if ((to.path === "/login" || to.path === "login") && is_logged_in) { // already logged in, trying to access login page
    next({ path: "/" });
  } else { // logged in, normal navigation
    next();
  }
});

export default router;
