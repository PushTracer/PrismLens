<script setup lang="ts">
import { convertFileSrc } from "@tauri-apps/api/core";
import { useMessage } from "naive-ui";
import { useVirtualList } from "@vueuse/core";
import { computed } from "vue";
import {
	FolderOpenOutline,
	ImageOutline,
	RefreshOutline,
} from "@vicons/ionicons5";
import { useImageStore } from "../store/images";

const message = useMessage();
const imageStore = useImageStore();

/** 选择图片目录 */
async function selectDirectory() {
	try {
		await imageStore.openDirectoryDialog();
	} catch (error) {
		message.error("选择目录失败：" + error);
	}
}

/** 重新扫描当前目录 */
async function refreshDirectory() {
	try {
		await imageStore.refreshDirectory();
		message.success("已刷新图片列表");
	} catch (error) {
		message.error("刷新失败：" + error);
	}
}

/** 切换当前显示的图片 */
async function showImage(path: string) {
	if (imageStore.currentImage?.path === path) return;
	try {
		await imageStore.selectImage(path);
	} catch (error) {
		message.error("获取图片信息失败：" + error);
	}
}

const imageListRef = computed(() => imageStore.imageList);
const { list, containerProps, wrapperProps } = useVirtualList(imageListRef, {
	itemWidth: 176,
});
</script>

<template>
	<div class="imagelist">
		<div class="toolbar">
			<n-button type="primary" size="small" block @click="selectDirectory">
				<template #icon>
					<n-icon><FolderOpenOutline /></n-icon>
				</template>
				打开文件夹
			</n-button>
			<div class="toolbar-row">
				<n-tooltip :show-arrow="false" :delay="300">
					<template #trigger>
						<n-button
							size="small"
							quaternary
							:disabled="imageStore.imageList.length === 0"
							@click="refreshDirectory"
						>
							<template #icon>
								<n-icon><RefreshOutline /></n-icon>
							</template>
						</n-button>
					</template>
					刷新列表
				</n-tooltip>
				<span v-if="imageStore.imageList.length > 0" class="image-count">
					{{ imageStore.imageList.length }} 张
				</span>
			</div>
		</div>

		<div v-bind="containerProps" class="imagelist-container">
			<div v-bind="wrapperProps" class="imagelist-content">
				<div
					v-for="item in list"
					:key="item.index"
					class="image-item"
					:class="{
						active:
							imageStore.currentImage?.path === item.data.path,
					}"
					:title="item.data.name"
					@click="showImage(item.data.path)"
				>
					<img
						:src="convertFileSrc(item.data.path)"
						alt="列表图片"
						class="thumbnail"
						loading="lazy"
						draggable="false"
					/>
					<span class="image-item__name">{{ item.data.name }}</span>
				</div>
			</div>
		</div>

		<div v-if="imageStore.imageList.length === 0" class="list-empty">
			<n-icon size="22"><ImageOutline /></n-icon>
			<span>暂无图片，点击左侧按钮打开文件夹</span>
		</div>
	</div>
</template>

<style scoped>
/* 样式位于 assets/scss/components/_imagelist.scss */
</style>
