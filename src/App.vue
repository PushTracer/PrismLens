<script setup lang="ts">
import { NConfigProvider, darkTheme } from "naive-ui";
import TitleBar from "./components/TitleBar.vue";

import { useThemeStore } from "./store";

const themeStore = useThemeStore();
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
		<div class="app-shell">
			<TitleBar />
			<main class="content">
				<!-- Naive UI 消息容器 -->
				<n-message-provider placement="bottom">
					<router-view></router-view>
				</n-message-provider>
			</main>
		</div>
	</n-config-provider>
</template>

<style scoped>
.content {
	margin-top: var(--titlebar-height);
	height: calc(100vh - var(--titlebar-height));
	overflow: hidden;
}
</style>
