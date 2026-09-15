<script setup lang="ts">
import { NConfigProvider, darkTheme } from "naive-ui";
import ThemeSwitch from "./components/ThemeSwitch.vue";
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
		<n-layout class="full-screen-layout">
			<TitleBar />
			<div class="content">
				<!-- nativeUI消息包裹 -->
				<n-message-provider placement="bottom">
					<ThemeSwitch />

					<router-view></router-view>
				</n-message-provider>
			</div>
		</n-layout>
	</n-config-provider>
</template>

<style scoped>
.content {
	margin-top: 35px;
	height: calc(100vh - 35px);
	/* 减去标题栏高度 */
	overflow: auto;
	/* 内容可滚动 */
}
</style>
