<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { NConfigProvider, darkTheme } from "naive-ui";
import ThemeSwitch from "./components/themeswitch.vue";
import { useThemeStore } from "./store";

const themeStore = useThemeStore();
const greetMsg = ref("");
const name = ref("");

async function greet() {
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <n-config-provider
    :theme="themeStore.theme === 'dark' ? darkTheme : null"
    :theme-overrides="
      themeStore.theme === 'dark'
        ? themeStore.darkThemeOverrides
        : themeStore.lightThemeOverrides
    "
  >
    <n-space vertical>
      <ThemeSwitch />

      <n-card>
        <h1>PrismLens</h1>
        <router-view></router-view>

        <n-text>现代化的图片查看工具，让每一张图片都绽放光彩~</n-text>

        <n-space justify="center" align="center">
          <n-input v-model:value="name" placeholder="传递信息..." />
          <n-button type="primary" @click="greet">发送</n-button>
        </n-space>
        <n-text>{{ greetMsg }}</n-text>
      </n-card>

      <n-card title="样式展示">
        <n-space vertical>
          <n-text>这是基础字体效果</n-text>
          <n-space vertical align="start">
            <n-h1>一级标题</n-h1>
            <n-h2>二级标题</n-h2>
            <n-h3>三级标题</n-h3>
            <n-h4>四级标题</n-h4>
            <n-h5>五级标题</n-h5>
            <n-h6>六级标题</n-h6>
          </n-space>

          <n-a href="#">这是一个链接效果</n-a>
        </n-space>
      </n-card>
    </n-space>
  </n-config-provider>
</template>

<style scoped></style>
