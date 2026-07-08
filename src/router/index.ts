import { createRouter, createWebHistory } from "vue-router";
import Index from "../views/index.vue";
import About from "../views/about/about.vue";
import ClickMode from "../views/clickMode/clickMode.vue";
import MainPanel from "../views/mainPanel/mainPanel.vue";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      component: Index,
      redirect: { name: "mainPanel" },
      children: [
        {
          path: "mainPanel",
          name: "mainPanel",
          component: MainPanel,
        },
        {
          path: "clickMode",
          name: "clickMode",
          component: ClickMode,
        },
        {
          path: "about",
          name: "about",
          component: About,
        },
      ],
    },
  ],
});

export default router;

