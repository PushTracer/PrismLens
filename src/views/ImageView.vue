<script setup lang="ts">
import { computed, ref } from "vue";
import { convertFileSrc } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { dirname } from "@tauri-apps/api/path";
import { Image } from "@vicons/ionicons5";
import { useMessage } from "naive-ui";
import { useResizeObserver } from "@vueuse/core";
import { useImageStore } from "../store/images";
import ImageList from "../components/ImageList.vue";
import ImageOperation from "../components/ImageOperation.vue";

const message = useMessage();
const imageStore = useImageStore();

const viewportRef = ref<HTMLElement | null>(null);
const viewportSize = ref({ width: 0, height: 0 });
const naturalSize = ref({ width: 0, height: 0 });

// 监听显示区域尺寸，用于计算图片适应窗口的显示大小
useResizeObserver(viewportRef, (entries) => {
	const rect = entries[0].contentRect;
	viewportSize.value = { width: rect.width, height: rect.height };
});

/**
 * 图片显示尺寸：先按旋转后的外接框适应窗口，再乘以用户缩放比例
 */
const displaySize = computed(() => {
	const { width: vw, height: vh } = viewportSize.value;
	const { width: iw, height: ih } = naturalSize.value;
	if (!vw || !vh || !iw || !ih) return { width: 0, height: 0 };

	const rotated = imageStore.view.rotation % 180 !== 0;
	const footprintWidth = rotated ? ih : iw;
	const footprintHeight = rotated ? iw : ih;
	const fit = Math.min(vw / footprintWidth, vh / footprintHeight, 1);

	return {
		width: iw * fit * imageStore.view.scale,
		height: ih * fit * imageStore.view.scale,
	};
});

const imageStyle = computed(() => ({
	width: displaySize.value.width
		? `${displaySize.value.width}px`
		: undefined,
	height: displaySize.value.height
		? `${displaySize.value.height}px`
		: undefined,
	transform: `translate(${imageStore.view.x}px, ${imageStore.view.y}px) rotate(${imageStore.view.rotation}deg)`,
}));

/** 图片加载完成，记录原始尺寸 */
function handleImageLoad(event: Event) {
	const img = event.target as HTMLImageElement;
	naturalSize.value = { width: img.naturalWidth, height: img.naturalHeight };
}

// ---- 拖拽平移 ----
const dragging = ref(false);
let dragStart = { x: 0, y: 0, offsetX: 0, offsetY: 0 };

function handlePointerDown(event: PointerEvent) {
	if (!imageStore.currentImage) return;
	dragging.value = true;
	dragStart = {
		x: event.clientX,
		y: event.clientY,
		offsetX: imageStore.view.x,
		offsetY: imageStore.view.y,
	};
	(event.currentTarget as HTMLElement).setPointerCapture(event.pointerId);
}

function handlePointerMove(event: PointerEvent) {
	if (!dragging.value) return;
	imageStore.panTo(
		dragStart.offsetX + (event.clientX - dragStart.x),
		dragStart.offsetY + (event.clientY - dragStart.y)
	);
}

function handlePointerUp(event: PointerEvent) {
	if (!dragging.value) return;
	dragging.value = false;
	(event.currentTarget as HTMLElement).releasePointerCapture(event.pointerId);
}

/** 滚轮缩放视图 */
function handleWheel(event: WheelEvent) {
	if (!imageStore.currentImage) return;
	if (event.deltaY < 0) {
		imageStore.zoomIn();
	} else {
		imageStore.zoomOut();
	}
}

/**
 * 选择单张图片，并自动加载其所在目录的图片列表
 */
async function selectImage() {
	const selected = await open({
		directory: false,
		multiple: false,
		// 文件类型过滤器
		filters: [
			{
				name: "图片文件",
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
		title: "选择图片",
	});

	if (!selected) return;

	try {
		await imageStore.selectImage(selected);
		const dirPath = await dirname(selected);
		await imageStore.loadDirectoryImages(dirPath);
	} catch (error) {
		message.error("选择文件失败：" + error);
	}
}
</script>

<template>
	<div class="imageview">
		<n-layout vertical>
			<div
				ref="viewportRef"
				class="image-display-area"
				:class="{
					pannable: !!imageStore.currentImage,
					grabbing: dragging,
				}"
				@pointerdown="handlePointerDown"
				@pointermove="handlePointerMove"
				@pointerup="handlePointerUp"
				@pointercancel="handlePointerUp"
				@wheel.prevent="handleWheel"
			>
				<!-- 图片显示区域：变换只影响展示，不改动文件 -->
				<div v-if="imageStore.currentImage" class="imageshow">
					<img
						:src="convertFileSrc(imageStore.currentImage.path)"
						alt="Current Image"
						:style="imageStyle"
						draggable="false"
						@load="handleImageLoad"
					/>
				</div>
				<n-empty v-else description="无内容">
					<template #icon>
						<n-icon size="40">
							<Image />
						</n-icon>
					</template>
					<template #extra>
						<n-button type="primary" @click="selectImage"> 选择 </n-button>
					</template>
				</n-empty>
			</div>
			<!-- 图片操作 -->
			<ImageOperation v-if="imageStore.currentImage" />
			<div class="imagelist">
				<ImageList />
			</div>
		</n-layout>
	</div>
</template>

<style scoped></style>
