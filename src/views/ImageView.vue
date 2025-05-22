<script setup lang="ts">
import { convertFileSrc } from "@tauri-apps/api/core";
import { useImageStore } from "../store/images";
import ImageList from "../components/imagelist.vue";
import { Image } from "@vicons/ionicons5";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import type { ImageInfo } from "../tauri-types";
import { useMessage } from "naive-ui";
import { dirname } from "@tauri-apps/api/path"; // 导入dirname函数

const message = useMessage();
const imageStore = useImageStore();

async function selectImage() {
	// 使用tauri的dialog插件，实现文件路径选择
	const selected = await open({
		directory: false,
		multiple: false,
		// 文件类型过滤器
		filters: [
			{
				name: "图片文件",
				// 支持的图片格式扩展名
				extensions: [
					"jpg",
					"jpeg",
					"png",
					"gif",
					"webp",
					"bmp",
					"ico",
					"tiff",
					"tif",
				],
			},
		],
		// 对话框标题
		title: "选择图片",
	});

	try {
		if (selected) {
			// 更新当前选择图片
			const result01 = await invoke("read_image_info_command", {
				path: selected,
			});
			imageStore.currentImage = result01 as ImageInfo;

			// 获取选中文件的目录路径
			const dirPath = await dirname(selected as string);

			// 使用目录路径获取目录下的所有图片
			const result02 = await invoke("get_directory_images_command", {
				dirPath: dirPath,
			});
			imageStore.imageList = result02 as ImageInfo[];
		}
	} catch (error) {
		message.error("选择文件失败：" + error);
	}
}
</script>
<template>
	<div class="imageview">
		<n-layout vertical>
			<div class="image-display-area">
				<!-- 图片显示区域和操作按钮 -->
				<template v-if="imageStore.currentImage">
					<!-- 图片显示 -->
					<div class="imageshow">
						<img
							:src="convertFileSrc(imageStore.currentImage.path)"
							alt="Current Image"
						/>
					</div>
					<!-- 移除操作按钮 -->
					<!-- <ImageOperation /> -->
				</template>
				<template v-else>
					<n-empty description="无内容">
						<template #icon>
							<n-icon size="40">
								<Image />
							</n-icon>
						</template>
						<template #extra>
							<n-button type="primary" @click="selectImage"> 选择 </n-button>
						</template>
					</n-empty>
				</template>
			</div>
			<div class="imagelist">
				<ImageList />
			</div>
		</n-layout>
	</div>
</template>
<style scoped></style>
