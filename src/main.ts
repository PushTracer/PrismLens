import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import store from "./store";
// 引入 SASS 主文件
import "./assets/scss/main.scss";

const app = createApp(App);

app.use(router).use(store).mount("#app");
