<script setup lang="ts">
import { ref } from "vue";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import type { ImageInfo } from "../tauri-types";
import { useMessage } from "naive-ui"; // 导入 useMessage

const imageList = ref<ImageInfo[]>([]);
const currentImage = ref<ImageInfo | null>(null);
const message = useMessage(); // 创建 message 实例

/*
 * 选择目录
 */
async function selectDirectory() {
	try {
		// 使用 dialog.open 而不是直接使用 open
		const selected = await open({
			directory: true,
			multiple: false,
		});

		if (selected) {
			const result = await invoke("get_directory_images_command", {
				dirPath: selected,
			});
			imageList.value = result as ImageInfo[];
		}
	} catch (error) {
		message.error("选择目录失败：" + error);
	}
}

/**
 * 显示图片信息
 * @param image 图片信息对象
 */
async function showImageInfo(path: string) {
	try {
		const result = await invoke("read_image_info_command", {
			path: path,
		});
		currentImage.value = result as ImageInfo;
	} catch (error) {
		message.error("获取图片信息失败：" + error);
	}
}

/**
 * 旋转图片
 * @param angle 旋转角度 (90, 180, 270)
 */
async function rotateImage(angle: number) {
	if (!currentImage.value) {
		message.warning("请先选择图片");
		return;
	}

	try {
		const result = await invoke("rotate_image_command", {
			path: currentImage.value.path,
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
	if (!currentImage.value) {
		message.warning("请先选择图片");
		return;
	}

	try {
		const result = await invoke("convert_image_format_command", {
			path: currentImage.value.path,
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
	if (!currentImage.value) {
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
			path: currentImage.value.path,
			width,
			height,
		});
		message.success("调整大小成功：" + result);
	} catch (error) {
		message.error("调整大小失败：" + error);
	}
}

// 添加调整大小的输入框绑定的状态
const resizeWidth = ref<number | null>(null);
const resizeHeight = ref<number | null>(null);
</script>
<template>
	<n-space vertical>
		<n-grid :cols="24" :x-gap="12">
			<n-grid-item :span="18">
				<n-card>
					<!-- 图片显示区域和操作按钮 -->
					<template v-if="currentImage">
						<!-- 图片显示 -->
						<div>
							<img
								:src="convertFileSrc(currentImage.path)"
								alt="Current Image"
								style="max-width: 100%; max-height: 600px"
							/>
						</div>

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
								<n-input-number
									v-model:value="resizeWidth"
									placeholder="宽度"
								/>
								<n-input-number
									v-model:value="resizeHeight"
									placeholder="高度"
								/>
								<n-button
									type="primary"
									@click="resizeImage(resizeWidth || 0, resizeHeight || 0)"
									>调整大小</n-button
								>
							</n-space>
						</n-space>
					</template>
					<template v-else>
						<n-empty description="请选择图片" />
					</template>
				</n-card>
			</n-grid-item>
			<n-grid-item :span="6">
				<n-card>
					<!-- 图片列表或详细信息区域 -->
					<n-button type="primary" @click="selectDirectory">选择目录</n-button>
					<!-- 这里可以放置图片列表 -->
					<n-list clickable hoverable>
						<n-list-item
							v-for="image in imageList"
							:key="image.path"
							@click="showImageInfo(image.path)"
						>
							{{ image.name }}
						</n-list-item>
					</n-list>
				</n-card>
			</n-grid-item>
		</n-grid>
	</n-space>
</template>
<style scoped>
.n-space {
	margin: 12px 0;
}
</style>
