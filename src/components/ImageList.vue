<script setup lang="ts">
import { convertFileSrc } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { useMessage } from "naive-ui";
import { useVirtualList } from "@vueuse/core";
import { computed } from "vue";
import { useImageStore } from "../store/images";

const message = useMessage();
const imageStore = useImageStore();

/**
 * 选择图片目录，加载目录下的所有图片并默认显示第一张
 */
async function selectDirectory() {
	try {
		const selected = await open({
			directory: true,
			multiple: false,
		});

		if (!selected) return;

		const images = await imageStore.loadDirectoryImages(selected);
		if (images.length > 0) {
			await imageStore.selectImage(images[0].path);
		}
	} catch (error) {
		message.error("选择目录失败：" + error);
	}
}

/**
 * 切换当前显示的图片
 * @param path 图片文件路径
 */
async function showImage(path: string) {
	try {
		await imageStore.selectImage(path);
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
		<div class="toolbar">
			<n-button type="primary" size="small" @click="selectDirectory">
				选择图片目录
			</n-button>
			<div v-if="imageStore.imageList.length > 0" class="image-count">
				共 {{ imageStore.imageList.length }} 张图片
			</div>
		</div>

		<div v-bind="containerProps" class="imagelist-container">
			<div v-bind="wrapperProps" class="imagelist-content">
				<n-card
					v-for="item in list"
					:key="item.index"
					class="image-item"
					@click="showImage(item.data.path)"
				>
					<img
						:src="convertFileSrc(item.data.path)"
						alt="列表图片"
						class="thumbnail"
					/>
				</n-card>
			</div>
		</div>
	</div>
</template>

<style scoped></style>
