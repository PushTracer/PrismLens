<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type { UnlistenFn } from "@tauri-apps/api/event";
import {
	MinusOutlined,
	FullscreenOutlined,
	CloseOutlined,
	FullscreenExitOutlined,
} from "@vicons/antd";
import { useThemeStore } from "../store";

// 获取当前窗口实例
const appWindow = getCurrentWindow();
// 获取主题store
const themeStore = useThemeStore();

const isMaximized = ref(false);
let unlistenResize: UnlistenFn | undefined;

/**
 * 处理窗口最小化
 */
const handleMinimize = async () => {
	await appWindow.minimize();
};

/**
 * 处理窗口最大化/还原
 */
const handleMaximize = async () => {
	await appWindow.toggleMaximize();
	const maximized = await appWindow.isMaximized();
	isMaximized.value = maximized;
};

/**
 * 处理窗口关闭
 */
const handleClose = async () => {
	await appWindow.close();
};

onMounted(async () => {
	isMaximized.value = await appWindow.isMaximized();
	// 监听窗口尺寸变化，同步最大化图标状态
	unlistenResize = await appWindow.onResized(async () => {
		isMaximized.value = await appWindow.isMaximized();
	});
});

onUnmounted(() => {
	unlistenResize?.();
});
</script>

<template>
	<div class="title-bar" data-tauri-drag-region>
		<div class="title">PrismLens</div>
		<div class="window-controls">
			<n-button
				quaternary
				circle
				size="small"
				class="control-btn"
				:focusable="false"
				@click="handleMinimize"
			>
				<n-icon :color="themeStore.theme === 'dark' ? '#FFFFFF' : '#2C3E50'">
					<MinusOutlined />
				</n-icon>
			</n-button>
			<n-button
				quaternary
				circle
				size="small"
				class="control-btn"
				:focusable="false"
				@click="handleMaximize"
			>
				<n-icon :color="themeStore.theme === 'dark' ? '#FFFFFF' : '#2C3E50'">
					<FullscreenExitOutlined v-if="isMaximized" />
					<FullscreenOutlined v-else />
				</n-icon>
			</n-button>
			<n-button
				quaternary
				circle
				size="small"
				class="close-btn"
				:focusable="false"
				@click="handleClose"
			>
				<n-icon :color="themeStore.theme === 'dark' ? '#FFFFFF' : '#2C3E50'">
					<CloseOutlined />
				</n-icon>
			</n-button>
		</div>
	</div>
</template>

<style scoped>
/* 样式位于 assets/scss/layout/_titlebar.scss */
</style>
