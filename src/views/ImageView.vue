<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from "vue";
import { convertFileSrc } from "@tauri-apps/api/core";
import { useMessage } from "naive-ui";
import { useResizeObserver } from "@vueuse/core";
import {
	ChevronBackOutline,
	ChevronForwardOutline,
	FolderOpenOutline,
	ImageOutline,
} from "@vicons/ionicons5";
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

/** 人类可读的文件大小 */
function formatSize(bytes: number): string {
	if (!bytes) return "0 B";
	const units = ["B", "KB", "MB", "GB"];
	const i = Math.floor(Math.log(bytes) / Math.log(1024));
	return `${(bytes / Math.pow(1024, i)).toFixed(i === 0 ? 0 : 1)} ${units[i]}`;
}

/** 时间戳（秒）格式化为本地时间 */
function formatTime(seconds: number): string {
	if (!seconds) return "-";
	const date = new Date(seconds * 1000);
	const pad = (n: number) => String(n).padStart(2, "0");
	return `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(
		date.getDate()
	)} ${pad(date.getHours())}:${pad(date.getMinutes())}`;
}

/** 当前图片的元信息条目 */
const infoItems = computed(() => {
	const img = imageStore.currentImage;
	if (!img) return [];
	return [
		{ label: "尺寸", value: `${img.width} × ${img.height}` },
		{ label: "格式", value: img.format.toUpperCase() },
		{ label: "大小", value: formatSize(img.size) },
		{ label: "修改", value: formatTime(img.modified) },
	];
});

/** 当前图片在列表中的位置文本 */
const positionText = computed(() => {
	const total = imageStore.imageList.length;
	const index = imageStore.currentIndex;
	if (index < 0 || total === 0) return "";
	return `${index + 1} / ${total}`;
});

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

/** 选择单张图片 */
async function selectImage() {
	try {
		await imageStore.openImageDialog();
	} catch (error) {
		message.error("选择文件失败：" + error);
	}
}

/** 选择目录 */
async function selectDirectory() {
	try {
		await imageStore.openDirectoryDialog();
	} catch (error) {
		message.error("选择目录失败：" + error);
	}
}

/** 切换上一张 / 下一张 */
async function stepImage(delta: number) {
	try {
		await imageStore.stepImage(delta);
	} catch (error) {
		message.error("切换图片失败：" + error);
	}
}

/** 键盘快捷键：方向键切换、+/- 缩放、0 重置 */
function handleKeydown(event: KeyboardEvent) {
	if (!imageStore.currentImage) return;

	switch (event.key) {
		case "ArrowLeft":
			event.preventDefault();
			stepImage(-1);
			break;
		case "ArrowRight":
			event.preventDefault();
			stepImage(1);
			break;
		case "+":
		case "=":
			event.preventDefault();
			imageStore.zoomIn();
			break;
		case "-":
		case "_":
			event.preventDefault();
			imageStore.zoomOut();
			break;
		case "0":
			event.preventDefault();
			imageStore.resetView();
			break;
	}
}

// 预加载相邻图片，切换时更顺滑
watch(
	() => imageStore.currentIndex,
	(index) => {
		if (index < 0) return;
		[index - 1, index + 1].forEach((i) => {
			const item = imageStore.imageList[i];
			if (!item) return;
			const preload = new Image();
			preload.src = convertFileSrc(item.path);
		});
	}
);

onMounted(() => window.addEventListener("keydown", handleKeydown));
onUnmounted(() => window.removeEventListener("keydown", handleKeydown));
</script>

<template>
	<div class="imageview">
		<div class="viewer-wrap">
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
				<template v-if="imageStore.currentImage">
					<!-- 图片显示区域：变换只影响展示，不改动文件 -->
					<div class="imageshow">
						<img
							:src="convertFileSrc(imageStore.currentImage.path)"
							alt="Current Image"
							:style="imageStyle"
							draggable="false"
							@load="handleImageLoad"
						/>
					</div>

					<!-- 图片信息浮层 -->
					<div class="image-info">
						<div class="image-info__head">
							<span class="image-info__name">
								{{ imageStore.currentImage.name }}
							</span>
							<span v-if="positionText" class="image-info__position">
								{{ positionText }}
							</span>
						</div>
						<div class="image-info__meta">
							<span
								v-for="item in infoItems"
								:key="item.label"
								class="image-info__item"
							>
								<em>{{ item.label }}</em>
								{{ item.value }}
							</span>
						</div>
					</div>

					<!-- 上一张 / 下一张 -->
					<n-button
						v-if="imageStore.canStepBack"
						circle
						class="nav-btn nav-btn--prev"
						:focusable="false"
						title="上一张 (←)"
						@pointerdown.stop
						@click.stop="stepImage(-1)"
					>
						<n-icon size="20"><ChevronBackOutline /></n-icon>
					</n-button>
					<n-button
						v-if="imageStore.canStepForward"
						circle
						class="nav-btn nav-btn--next"
						:focusable="false"
						title="下一张 (→)"
						@pointerdown.stop
						@click.stop="stepImage(1)"
					>
						<n-icon size="20"><ChevronForwardOutline /></n-icon>
					</n-button>
				</template>

				<!-- 空状态 -->
				<div v-else class="empty-state">
					<div class="empty-state__icon">
						<n-icon size="34"><ImageOutline /></n-icon>
					</div>
					<h2 class="empty-state__title">还没有打开图片</h2>
					<p class="empty-state__desc">
						选择一张图片开始浏览，或直接打开整个文件夹
					</p>
					<div class="empty-state__actions">
						<n-button type="primary" @click="selectImage">
							<template #icon>
								<n-icon><ImageOutline /></n-icon>
							</template>
							选择图片
						</n-button>
						<n-button secondary @click="selectDirectory">
							<template #icon>
								<n-icon><FolderOpenOutline /></n-icon>
							</template>
							打开文件夹
						</n-button>
					</div>
					<p class="empty-state__hint">
						支持 JPG / PNG / GIF / WebP / BMP / ICO / TIFF
					</p>
				</div>
			</div>
		</div>

		<!-- 图片操作 -->
		<ImageOperation v-if="imageStore.currentImage" />

		<div class="imagelist-wrap">
			<ImageList />
		</div>
	</div>
</template>

<style scoped>
/* 样式位于 assets/scss/pages/_imageview.scss */
</style>
