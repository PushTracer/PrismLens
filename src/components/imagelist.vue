<script setup lang="ts">
import type { ImageInfo } from "../tauri-types";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { useMessage } from "naive-ui";
import { useImageStore } from "../store/images";
import { useVirtualList } from "@vueuse/core";
import { computed } from "vue";

const message = useMessage();
const imageStore = useImageStore();

/**
 * 选择图片目录
 * 使用Tauri的dialog插件打开目录选择器，获取用户选择的目录
 * 然后调用后端命令获取目录中的所有图片信息
 */
async function selectDirectory() {
	try {
		const selected = await open({
			directory: true,
			multiple: false,
		});

		if (selected) {
			const result = await invoke("get_directory_images_command", {
				dirPath: selected,
			});
			imageStore.imageList = result as ImageInfo[];

			// 如果有图片，自动选择第一张
			if (imageStore.imageList.length > 0) {
				showImage(imageStore.imageList[0].path);
			}
		}
	} catch (error) {
		message.error("选择目录失败：" + error);
	}
}

/**
 * 显示选中的图片
 * 根据图片路径获取详细信息并更新当前选中的图片
 * @param path 图片文件路径
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

const imageListRef = computed(() => imageStore.imageList);
const { list, containerProps, wrapperProps } = useVirtualList(imageListRef, {
	itemWidth: 200,
});
</script>

<template>
	<div class="imagelist">
		<!-- <div class="toolbar">
			<n-button type="primary" @click="selectDirectory">选择图片目录</n-button>
			<div class="image-count" v-if="imageStore.imageList.length > 0">
				共 {{ imageStore.imageList.length }} 张图片
			</div>
		</div> -->

		<div v-bind="containerProps">
			<div v-bind="wrapperProps" class="imagelist-content">
				<n-card v-for="item in list" :key="item.index" class="image-item">
					<img
						:src="convertFileSrc(item.data.path)"
						@click="showImage(item.data.path)"
						alt="列表图片"
						class="thumbnail"
					/>
				</n-card>
			</div>
		</div>
	</div>
</template>
<style scoped>
/* 样式已移至 _imagelist.scss */
</style>
