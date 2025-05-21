<script setup lang="ts">
// import { ref } from "vue";
import { NConfigProvider, darkTheme } from "naive-ui";
import ThemeSwitch from "./components/themeswitch.vue";
import TitleBar from "./components/titlebar.vue";

import { useThemeStore } from "./store";

const themeStore = useThemeStore();


</script>

<template>
  <n-config-provider :theme="themeStore.theme === 'dark' ? darkTheme : null" :theme-overrides="themeStore.theme === 'dark'
      ? themeStore.darkThemeOverrides
      : themeStore.lightThemeOverrides
    ">
    <n-layout class="full-screen-layout">
      <TitleBar />
      <div class="content">
        <n-space>
          <!-- nativeUI消息包裹 -->
          <n-message-provider placement="bottom">
            <ThemeSwitch />
            <n-card>
              <router-view></router-view>
              <router-link to="/image">跳转图像编辑</router-link>
              <br></br>
              <router-link to="/">跳转主页</router-link>
            </n-card>
          </n-message-provider>
        </n-space>
      </div>
    </n-layout>
  </n-config-provider>
</template>

<style scoped>
.n-card {

  :deep(.n-card__content),
  :deep(.n-card__footer) {
    padding: 0 !important;
  }
}

.content {
  margin-top: 35px;
  height: calc(100vh - 35px);
  /* 减去标题栏高度 */
  overflow: auto;
  /* 内容可滚动 */
}
</style>
