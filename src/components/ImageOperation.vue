<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { dirname, join } from "@tauri-apps/api/path";
import { save } from "@tauri-apps/plugin-dialog";
import { useMessage } from "naive-ui";
import { useImageStore } from "../store/images";

const imageStore = useImageStore();
const message = useMessage();

// 调整大小的输入框绑定状态
const resizeWidth = ref<number | null>(null);
const resizeHeight = ref<number | null>(null);

/**
 * 弹出"另存为"对话框，让用户显式选择输出路径
 * @param suffix 文件名后缀
 * @param extension 目标扩展名
 */
async function pickOutputPath(suffix: string, extension: string) {
	const currentImage = imageStore.currentImage;
	if (!currentImage) return null;

	const baseName = currentImage.name.replace(/\.[^.]+$/, "");
	const dirPath = await dirname(currentImage.path);
	const defaultPath = await join(
		dirPath,
		`${baseName}${suffix}.${extension}`
	);

	return save({
		defaultPath,
		filters: [{ name: extension.toUpperCase(), extensions: [extension] }],
	});
}

/**
 * 转换图片格式（另存为，不修改源文件）
 * @param format 目标格式 (jpg, png, webp等)
 */
async function convertFormat(format: string) {
	const currentImage = imageStore.currentImage;
	if (!currentImage) {
		message.warning("请先选择图片");
		return;
	}

	const outputPath = await pickOutputPath("", format);
	if (!outputPath) return;

	try {
		const result = await invoke<string>("convert_image_format_command", {
			path: currentImage.path,
			format,
			outputPath,
		});
		message.success("已另存为：" + result);
	} catch (error) {
		message.error("转换失败：" + error);
	}
}

/**
 * 调整图片大小（另存为，不修改源文件）
 * @param width 目标宽度
 * @param height 目标高度
 */
async function resizeImage(width: number, height: number) {
	const currentImage = imageStore.currentImage;
	if (!currentImage) {
		message.warning("请先选择图片");
		return;
	}

	// 检查宽度和高度是否有效
	if (!width || !height || width <= 0 || height <= 0) {
		message.warning("请输入有效的宽度和高度");
		return;
	}

	const extension = currentImage.format || "png";
	const outputPath = await pickOutputPath(
		`_${width}x${height}`,
		extension
	);
	if (!outputPath) return;

	try {
		const result = await invoke<string>("resize_image_command", {
			path: currentImage.path,
			width,
			height,
			outputPath,
		});
		message.success("已另存为：" + result);
	} catch (error) {
		message.error("调整大小失败：" + error);
	}
}
</script>

<template>
	<div class="image-operation">
		<n-space>
			<!-- 旋转（仅视图变换，不改动文件） -->
			<n-button
				tertiary
				type="primary"
				@click="imageStore.rotateView(90)"
			>
				旋转 90°
			</n-button>
			<n-button
				tertiary
				type="primary"
				@click="imageStore.rotateView(180)"
			>
				旋转 180°
			</n-button>
			<n-button
				tertiary
				type="primary"
				@click="imageStore.rotateView(270)"
			>
				旋转 270°
			</n-button>

			<!-- 缩放（仅视图变换） -->
			<n-button tertiary @click="imageStore.zoomOut()">缩小</n-button>
			<n-text depth="3" class="zoom-text">
				{{ Math.round(imageStore.view.scale * 100) }}%
			</n-text>
			<n-button tertiary @click="imageStore.zoomIn()">放大</n-button>
			<n-button quaternary @click="imageStore.resetView()">重置</n-button>

			<!-- 转换格式（另存为） -->
			<n-button tertiary type="info" @click="convertFormat('png')">
				另存为 PNG
			</n-button>
			<n-button tertiary type="info" @click="convertFormat('jpg')">
				另存为 JPG
			</n-button>
			<n-button tertiary type="info" @click="convertFormat('webp')">
				另存为 WebP
			</n-button>

			<!-- 调整大小（另存为） -->
			<n-input-number v-model:value="resizeWidth" placeholder="宽度" />
			<n-input-number v-model:value="resizeHeight" placeholder="高度" />
			<n-button
				type="primary"
				@click="resizeImage(resizeWidth || 0, resizeHeight || 0)"
			>
				调整大小并另存
			</n-button>
		</n-space>
	</div>
</template>

<style scoped>
.image-operation {
	display: flex;
	justify-content: center;
	padding: 6px 12px;
	flex: none;
}

.zoom-text {
	min-width: 48px;
	text-align: center;
}
</style>
