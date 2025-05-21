<script setup lang="ts">
import { convertFileSrc } from "@tauri-apps/api/core";
import { useImageStore } from "../store/images";
import ImageList from "../components/imagelist.vue";

const imageStore = useImageStore(); // 导入图片store
</script>
<template>
	<div class="imageview">
		<n-space vertical>
			<n-grid :cols="24" :x-gap="12">
				<n-grid-item :span="6">
					<!-- 图片列表 -->
					<ImageList />
				</n-grid-item>
				<n-grid-item :span="18">
					<n-card>
						<!-- 图片显示区域和操作按钮 -->
						<template v-if="imageStore.currentImage">
							<!-- 图片显示 -->
							<div>
								<img
									:src="convertFileSrc(imageStore.currentImage.path)"
									alt="Current Image"
									style="max-width: 100%; max-height: 600px"
								/>
							</div>
							<!-- 移除操作按钮 -->
							<!-- <ImageOperation /> -->
						</template>
						<template v-else>
							<n-empty description="请选择图片" />
						</template>
					</n-card>
				</n-grid-item>
			</n-grid>
		</n-space>
	</div>
</template>
<style scoped>
.imageview {
	width: 100%;
}
</style>
