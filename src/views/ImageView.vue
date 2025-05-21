<script setup lang="ts">
import { convertFileSrc } from "@tauri-apps/api/core";
import { useImageStore } from "../store/images";
import ImageList from "../components/imagelist.vue";

const imageStore = useImageStore(); // 导入图片store
</script>
<template>
	<div class="imageview">
		<n-grid :cols="24" :x-gap="12">
			<n-grid-item :span="6">
				<!-- 图片列表 -->
				<ImageList />
			</n-grid-item>
			<n-grid-item :span="18" class="image-display-area">
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
					<n-empty description="请选择图片" />
				</template>
			</n-grid-item>
		</n-grid>
	</div>
</template>
<style scoped></style>
