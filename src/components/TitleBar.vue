<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useThemeStore } from "../store";
import { MinusOutlined, FullscreenOutlined, CloseOutlined } from "@vicons/antd";

// 获取当前窗口实例
const appWindow = getCurrentWindow();
// 获取主题store
const themeStore = useThemeStore();

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
	if (await appWindow.isMaximized()) {
		await appWindow.unmaximize();
	} else {
		await appWindow.maximize();
	}
};

/**
 * 处理窗口关闭
 */
const handleClose = async () => {
	await appWindow.close();
};
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
					<FullscreenOutlined />
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
/* Styles moved to _titlebar.scss */
</style>
