<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useMessage } from "naive-ui"; // 导入 useMessage
import { useImageStore } from "../store/images";

const imageStore = useImageStore(); // 导入图片store
const message = useMessage(); // 创建 message 实例

// 添加调整大小的输入框绑定的状态
const resizeWidth = ref<number | null>(null);
const resizeHeight = ref<number | null>(null);

/**
 * 旋转图片
 * @param angle 旋转角度 (90, 180, 270)
 */
async function rotateImage(angle: number) {
	if (!imageStore.currentImage) {
		message.warning("请先选择图片");
		return;
	}

	try {
		const result = await invoke("rotate_image_command", {
			path: imageStore.currentImage.path,
			angle,
		});
		message.success("旋转成功：" + result);
	} catch (error) {
		message.error("旋转失败：" + error);
	}
}

/**
 * 转换图片格式
 * @param format 目标格式 (jpg, png, webp等)
 */
async function convertFormat(format: string) {
	if (!imageStore.currentImage) {
		message.warning("请先选择图片");
		return;
	}

	try {
		const result = await invoke("convert_image_format_command", {
			path: imageStore.currentImage.path,
			format,
		});
		message.success("转换成功：" + result);
	} catch (error) {
		message.error("转换失败：" + error);
	}
}

/**
 * 调整图片大小
 * @param width 目标宽度
 * @param height 目标高度
 */
async function resizeImage(width: number, height: number) {
	if (!imageStore.currentImage) {
		message.warning("请先选择图片");
		return;
	}

	// 检查宽度和高度是否有效
	if (!width || !height || width <= 0 || height <= 0) {
		message.warning("请输入有效的宽度和高度");
		return;
	}

	try {
		const result = await invoke("resize_image_command", {
			path: imageStore.currentImage.path,
			width,
			height,
		});
		message.success("调整大小成功：" + result);
	} catch (error) {
		message.error("调整大小失败：" + error);
	}
}
</script>
<template>
	<div>
		<!-- 图片操作按钮 -->
		<n-space>
			<!-- 旋转 -->
			<n-button tertiary type="primary" @click="rotateImage(90)"
				>旋转 90°</n-button
			>
			<n-button tertiary type="primary" @click="rotateImage(180)"
				>旋转 180°</n-button
			>
			<n-button tertiary type="primary" @click="rotateImage(270)"
				>旋转 270°</n-button
			>

			<!-- 转换格式 -->
			<n-button tertiary type="info" @click="convertFormat('png')"
				>转为 PNG</n-button
			>
			<n-button tertiary type="info" @click="convertFormat('jpg')"
				>转为 JPG</n-button
			>
			<n-button tertiary type="info" @click="convertFormat('webp')"
				>转为 WebP</n-button
			>

			<!-- 调整大小 -->
			<n-space>
				<n-input-number v-model:value="resizeWidth" placeholder="宽度" />
				<n-input-number v-model:value="resizeHeight" placeholder="高度" />
				<n-button
					type="primary"
					@click="resizeImage(resizeWidth || 0, resizeHeight || 0)"
					>调整大小</n-button
				>
			</n-space>
		</n-space>
	</div>
</template>
<style scoped>
.n-space {
	margin: 12px 0;
}
</style>
