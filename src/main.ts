import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import { createPinia } from "pinia";
// 引入 SASS 主文件
import "./assets/scss/main.scss";

const app = createApp(App);

app.use(createPinia());
app.use(router);

app.mount("#app");
