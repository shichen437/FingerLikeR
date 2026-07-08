import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import "./index.css";
import { initializeStore } from "./store/store";

initializeStore().then(() => {
  createApp(App).use(router).mount("#app");
});
