<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useThemeStore } from "../store";
import {
	MinusOutlined,
	FullscreenOutlined,
	CloseOutlined,
	FullscreenExitOutlined,
} from "@vicons/antd";
import { ref } from "vue";

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
		isMaximized.value = false; // 手动更新状态
	} else {
		await appWindow.maximize();
		isMaximized.value = true; // 手动更新状态
	}
};

/**
 * 处理窗口关闭
 */
const handleClose = async () => {
	await appWindow.close();
};

const isMaximized = ref(false);

// // 存储解除监听的函数
// const unlisten = ref<(() => void)[]>([]);

// onMounted(async () => {
// 	isMaximized.value = await appWindow.isMaximized();
// 	// 监听窗口状态变化
// 	unlisten.value.push(
// 		await appWindow.listen("tauri://window-event", (event: any) => {
// 			if (event.event === "maximize") {
// 				isMaximized.value = true;
// 				console.log("maximize event triggered");
// 			} else if (event.event === "unmaximize") {
// 				isMaximized.value = false;
// 				console.log("unmaximize event triggered");
// 			}
// 		})
// 	);
// });

// onUnmounted(() => {
// 	// 解除所有事件监听
// 	unlisten.value.forEach((fn) => fn());
// });
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
/* Styles moved to _titlebar.scss */
</style>
