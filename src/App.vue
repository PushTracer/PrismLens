<script setup lang="ts">
// import { ref } from "vue";
// import { invoke } from "@tauri-apps/api/core";
import { NConfigProvider, darkTheme } from "naive-ui";
import ThemeSwitch from "./components/themeswitch.vue";
import { useThemeStore } from "./store";

const themeStore = useThemeStore();
// const greetMsg = ref("");
// const name = ref("");

// async function greet() {
// 	greetMsg.value = await invoke("greet", { name: name.value });
// }
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
		<n-layout>
			<TitleBar />
			<n-space>
				<ThemeSwitch />
				<n-card>
					<router-view></router-view>
					<router-link to="/image">跳转</router-link>
				</n-card>
			</n-space>
		</n-layout>
	</n-config-provider>
</template>

<style scoped>
.n-layout {
	min-height: 100vh;
}
</style>
