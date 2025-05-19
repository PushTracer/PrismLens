<script setup lang="ts">
import { ref, watch } from "vue";
import { NSwitch } from "naive-ui";
import { useThemeStore } from "../store";

const themeStore = useThemeStore();
const isDark = ref(themeStore.theme === "dark");

// 开关轨道样式
const railStyle = ({ checked }: { checked: boolean }) => {
  const style = {
    background: checked ? "#18a058" : "#2080f0",
    "--n-rail-color-hover": checked ? "#36ad6a" : "#4098fc",
  };
  return style;
};

// 主题切换处理
const handleThemeChange = (value: boolean) => {
  themeStore.theme = value ? "dark" : "light";
};

// 监听 store 中的主题变化
watch(() => themeStore.theme, (newTheme) => {
  isDark.value = newTheme === "dark";
});
</script>

<template>
	<div class="theme-switch">
		<n-switch
			v-model:value="isDark"
			:rail-style="railStyle"
			@update:value="handleThemeChange"
		>
			<template #checked> 🌙 </template>
			<template #unchecked> ☀️ </template>
		</n-switch>
	</div>
</template>

<style>
.theme-switch {
	position: fixed;
	top: 1rem;
	right: 1rem;
	z-index: 100;
}
</style>
