<script setup lang="ts">
import type { ImageInfo } from "../tauri-types";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { useMessage } from "naive-ui"; // 导入 useMessage
import { useImageStore } from "../store/images";

const message = useMessage(); // 创建 message 实例
const imageStore = useImageStore(); // 导入图片store

/*
 * 选择目录
 */
async function selectDirectory() {
	try {
		// 使用tauri的dialog插件，实现文件路径选择
		const selected = await open({
			directory: true,
			multiple: false,
		});

		if (selected) {
			const result = await invoke("get_directory_images_command", {
				dirPath: selected,
			});
			imageStore.imageList = result as ImageInfo[];
		}
	} catch (error) {
		message.error("选择目录失败：" + error);
	}
}

/**
 * 更新当前选择图片
 * @param path 图片路径
 */
async function showImage(path: string) {
	try {
		const result = await invoke("read_image_info_command", {
			path: path,
		});
		imageStore.currentImage = result as ImageInfo;
	} catch (error) {
		message.error("获取图片信息失败：" + error);
	}
}
</script>
<template>
	<div class="imagelist">
		<n-button type="primary" @click="selectDirectory">选择目录</n-button>
		<!-- 这里可以放置图片列表 -->
		<n-list clickable hoverable>
			<n-list-item
				v-for="image in imageStore.imageList"
				:key="image.path"
				@click="showImage(image.path)"
			>
				{{ image.name }}
			</n-list-item>
		</n-list>
	</div>
</template>
<style scoped></style>
