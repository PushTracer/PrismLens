import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import type { ImageBasicInfo, ImageInfo } from "../tauri-types";

/** 视图变换状态（仅影响展示，不写文件） */
export interface ViewState {
	/** 顺时针旋转角度（0/90/180/270） */
	rotation: number;
	/** 缩放比例，1 表示适应窗口 */
	scale: number;
	/** 平移偏移（像素） */
	x: number;
	y: number;
}

const MIN_SCALE = 0.1;
const MAX_SCALE = 8;
const ZOOM_STEP = 1.25;

function createViewState(): ViewState {
	return { rotation: 0, scale: 1, x: 0, y: 0 };
}

/**
 * 图片相关的状态管理
 * 所有 Tauri 命令调用收敛在这里，组件只负责交互与消息提示
 */
export const useImageStore = defineStore("image", {
	state: () => ({
		imageList: [] as ImageBasicInfo[],
		currentImage: null as ImageInfo | null,
		view: createViewState(),
	}),

	actions: {
		/**
		 * 扫描目录下的所有图片并更新列表
		 * @param dirPath 目录路径
		 */
		async loadDirectoryImages(dirPath: string) {
			const images = await invoke<ImageBasicInfo[]>(
				"get_directory_images_command",
				{ dirPath }
			);
			this.imageList = images;
			return images;
		},

		/**
		 * 读取单张图片的完整信息并设为当前图片
		 * @param path 图片路径
		 */
		async selectImage(path: string) {
			const image = await invoke<ImageInfo>("read_image_info_command", {
				path,
			});
			this.currentImage = image;
			this.resetView();
			return image;
		},

		/** 重置视图变换 */
		resetView() {
			this.view = createViewState();
		},

		/** 旋转视图（不改动文件） */
		rotateView(angle: number) {
			this.view.rotation = (this.view.rotation + angle + 360) % 360;
		},

		/** 设置缩放比例（带上下限） */
		setScale(scale: number) {
			this.view.scale = Math.min(MAX_SCALE, Math.max(MIN_SCALE, scale));
		},

		/** 放大 */
		zoomIn() {
			this.setScale(this.view.scale * ZOOM_STEP);
		},

		/** 缩小 */
		zoomOut() {
			this.setScale(this.view.scale / ZOOM_STEP);
		},

		/** 平移到指定偏移 */
		panTo(x: number, y: number) {
			this.view.x = x;
			this.view.y = y;
		},
	},
});
