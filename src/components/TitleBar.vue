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
import { PrismOutline } from "@vicons/ionicons5";
import ThemeSwitch from "./ThemeSwitch.vue";

// 获取当前窗口实例
const appWindow = getCurrentWindow();

const isMaximized = ref(false);
let unlistenResize: UnlistenFn | undefined;

/** 最小化窗口 */
const handleMinimize = async () => {
	await appWindow.minimize();
};

/** 最大化 / 还原窗口 */
const handleMaximize = async () => {
	await appWindow.toggleMaximize();
	isMaximized.value = await appWindow.isMaximized();
};

/** 关闭窗口 */
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
	<div class="title-bar">
		<div class="titlebar-left" data-tauri-drag-region>
			<div class="brand" data-tauri-drag-region>
				<n-icon class="brand-icon" size="16">
					<PrismOutline />
				</n-icon>
				<span class="title">PrismLens</span>
			</div>
		</div>

		<div class="window-controls">
			<ThemeSwitch />
			<span class="controls-divider"></span>
			<n-button
				quaternary
				circle
				size="small"
				class="control-btn"
				:focusable="false"
				@click="handleMinimize"
			>
				<n-icon size="16"><MinusOutlined /></n-icon>
			</n-button>
			<n-button
				quaternary
				circle
				size="small"
				class="control-btn"
				:focusable="false"
				@click="handleMaximize"
			>
				<n-icon size="15">
					<FullscreenExitOutlined v-if="isMaximized" />
					<FullscreenOutlined v-else />
				</n-icon>
			</n-button>
			<n-button
				quaternary
				circle
				size="small"
				class="control-btn close-btn"
				:focusable="false"
				@click="handleClose"
			>
				<n-icon size="16"><CloseOutlined /></n-icon>
			</n-button>
		</div>
	</div>
</template>

<style scoped>
/* 样式位于 assets/scss/layout/_titlebar.scss */
</style>
