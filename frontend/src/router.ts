import { createWebHistory, createRouter } from "vue-router";

import MainPage from "./MainPage.vue";
import LoginView from "./LoginView.vue";

import TestView from "./views/TestView.vue";
import TestView2 from "./views/TestView2.vue";
import BrainlogView from "./views/BrainlogView.vue";
import QuestView from "./views/QuestView.vue";
import QuestItemView from "./views/QuestItemView.vue";

const routes = [
  { 
    path: "/",
    component: MainPage,
    children: [
      { path: "/", redirect: "/test" },
      { path: "/test", component: TestView },
      { path: "/test2", component: TestView2 },
      { path: "/brainlog", component: BrainlogView },
      { path: "/quest", component: QuestView },
      { path: "/quest/items", component: QuestItemView },
    ], 
  },
  { 
    path: "/login",
    component: LoginView,
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
