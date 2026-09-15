<script setup lang="ts">
import { computed, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { dirname, join } from "@tauri-apps/api/path";
import { save } from "@tauri-apps/plugin-dialog";
import { useMessage } from "naive-ui";
import {
	AddOutline,
	ChevronDownOutline,
	DownloadOutline,
	RefreshOutline,
	RemoveOutline,
	ResizeOutline,
} from "@vicons/ionicons5";
import { useImageStore } from "../store/images";

const imageStore = useImageStore();
const message = useMessage();

// 调整大小的输入框绑定状态
const resizeWidth = ref<number | null>(null);
const resizeHeight = ref<number | null>(null);

const zoomPercent = computed(() =>
	Math.round(imageStore.view.scale * 100)
);

// 可导出的格式
const exportOptions = [
	{ label: "PNG", key: "png" },
	{ label: "JPG", key: "jpg" },
	{ label: "WebP", key: "webp" },
	{ label: "GIF", key: "gif" },
	{ label: "BMP", key: "bmp" },
	{ label: "ICO", key: "ico" },
];

/**
 * 弹出"另存为"对话框，让用户显式选择输出路径
 * @param suffix 文件名后缀
 * @param extension 目标扩展名
 */
async function pickOutputPath(suffix: string, extension: string) {
	const currentImage = imageStore.currentImage;
	if (!currentImage) return null;

	const baseName = currentImage.name.replace(/\.[^.]+$/, "");
	const dirPath = await dirname(currentImage.path);
	const defaultPath = await join(
		dirPath,
		`${baseName}${suffix}.${extension}`
	);

	return save({
		defaultPath,
		filters: [{ name: extension.toUpperCase(), extensions: [extension] }],
	});
}

/**
 * 转换图片格式（另存为，不修改源文件）
 * @param format 目标格式 (jpg, png, webp等)
 */
async function convertFormat(format: string) {
	const currentImage = imageStore.currentImage;
	if (!currentImage) {
		message.warning("请先选择图片");
		return;
	}

	const outputPath = await pickOutputPath("", format);
	if (!outputPath) return;

	try {
		const result = await invoke<string>("convert_image_format_command", {
			path: currentImage.path,
			format,
			outputPath,
		});
		message.success("已另存为：" + result);
	} catch (error) {
		message.error("转换失败：" + error);
	}
}

/** 下拉菜单导出 */
function handleExportSelect(key: string | number) {
	convertFormat(String(key));
}

/**
 * 调整图片大小（另存为，不修改源文件）
 * @param width 目标宽度
 * @param height 目标高度
 */
async function resizeImage(width: number, height: number) {
	const currentImage = imageStore.currentImage;
	if (!currentImage) {
		message.warning("请先选择图片");
		return;
	}

	// 检查宽度和高度是否有效
	if (!width || !height || width <= 0 || height <= 0) {
		message.warning("请输入有效的宽度和高度");
		return;
	}

	const extension = currentImage.format || "png";
	const outputPath = await pickOutputPath(
		`_${width}x${height}`,
		extension
	);
	if (!outputPath) return;

	try {
		const result = await invoke<string>("resize_image_command", {
			path: currentImage.path,
			width,
			height,
			outputPath,
		});
		message.success("已另存为：" + result);
	} catch (error) {
		message.error("调整大小失败：" + error);
	}
}
</script>

<template>
	<div class="image-operation">
		<!-- 旋转（仅视图变换，不改动文件） -->
		<div class="op-group">
			<span class="op-label">
				<n-icon size="14"><RefreshOutline /></n-icon>
				旋转
			</span>
			<n-button-group size="small">
				<n-button
					title="顺时针旋转 90°"
					@click="imageStore.rotateView(90)"
				>
					90°
				</n-button>
				<n-button
					title="顺时针旋转 180°"
					@click="imageStore.rotateView(180)"
				>
					180°
				</n-button>
				<n-button
					title="顺时针旋转 270°"
					@click="imageStore.rotateView(270)"
				>
					270°
				</n-button>
			</n-button-group>
		</div>

		<span class="op-divider"></span>

		<!-- 缩放（仅视图变换） -->
		<div class="op-group">
			<span class="op-label">缩放</span>
			<n-button-group size="small">
				<n-button :focusable="false" @click="imageStore.zoomOut()">
					<template #icon>
						<n-icon><RemoveOutline /></n-icon>
					</template>
				</n-button>
				<n-button
					class="zoom-value"
					title="适应窗口"
					:focusable="false"
					@click="imageStore.fitView()"
				>
					{{ zoomPercent }}%
				</n-button>
				<n-button :focusable="false" @click="imageStore.zoomIn()">
					<template #icon>
						<n-icon><AddOutline /></n-icon>
					</template>
				</n-button>
			</n-button-group>
			<n-button size="small" quaternary @click="imageStore.resetView()">
				重置
			</n-button>
		</div>

		<span class="op-divider"></span>

		<!-- 导出格式（另存为） -->
		<div class="op-group">
			<span class="op-label">
				<n-icon size="14"><DownloadOutline /></n-icon>
				导出
			</span>
			<n-dropdown
				trigger="click"
				:options="exportOptions"
				@select="handleExportSelect"
			>
				<n-button size="small" secondary>
					<template #icon>
						<n-icon><DownloadOutline /></n-icon>
					</template>
					另存为
					<n-icon class="export-caret" size="14">
						<ChevronDownOutline />
					</n-icon>
				</n-button>
			</n-dropdown>
		</div>

		<span class="op-divider"></span>

		<!-- 调整尺寸（另存为） -->
		<div class="op-group">
			<span class="op-label">
				<n-icon size="14"><ResizeOutline /></n-icon>
				尺寸
			</span>
			<n-input-number
				v-model:value="resizeWidth"
				size="small"
				placeholder="宽"
				:min="1"
				class="size-input"
			/>
			<span class="op-times">×</span>
			<n-input-number
				v-model:value="resizeHeight"
				size="small"
				placeholder="高"
				:min="1"
				class="size-input"
			/>
			<n-button
				size="small"
				type="primary"
				@click="resizeImage(resizeWidth || 0, resizeHeight || 0)"
			>
				另存
			</n-button>
		</div>
	</div>
</template>

<style scoped>
/* 样式位于 assets/scss/components/_imageoperation.scss */
</style>
