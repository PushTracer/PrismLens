import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import { dirname } from "@tauri-apps/api/path";
import { open } from "@tauri-apps/plugin-dialog";
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

/** 后端支持的图片扩展名 */
export const IMAGE_EXTENSIONS = [
	"jpg",
	"jpeg",
	"png",
	"gif",
	"webp",
	"bmp",
	"ico",
	"tiff",
	"tif",
];

const MIN_SCALE = 0.1;
const MAX_SCALE = 8;
const ZOOM_STEP = 1.25;

function createViewState(): ViewState {
	return { rotation: 0, scale: 1, x: 0, y: 0 };
}

/**
 * 图片相关的状态管理
 * 所有 Tauri 命令 / 对话框调用收敛在这里，组件只负责交互与消息提示
 */
export const useImageStore = defineStore("image", {
	state: () => ({
		imageList: [] as ImageBasicInfo[],
		currentImage: null as ImageInfo | null,
		/** 当前图片所在目录，用于刷新列表 */
		currentDirectory: "" as string,
		view: createViewState(),
	}),

	getters: {
		/** 当前图片在列表中的下标，未命中返回 -1 */
		currentIndex: (state): number => {
			if (!state.currentImage) return -1;
			const path = state.currentImage.path;
			return state.imageList.findIndex((img) => img.path === path);
		},
		/** 是否存在上一张 / 下一张 */
		canStepBack: (state): boolean => {
			if (!state.currentImage) return false;
			const index = state.imageList.findIndex(
				(img) => img.path === state.currentImage!.path
			);
			return index > 0;
		},
		canStepForward: (state): boolean => {
			if (!state.currentImage) return false;
			const index = state.imageList.findIndex(
				(img) => img.path === state.currentImage!.path
			);
			return index >= 0 && index < state.imageList.length - 1;
		},
	},

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
			this.currentDirectory = dirPath;
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

		/** 选择单张图片，并自动加载其所在目录 */
		async openImageDialog() {
			const selected = await open({
				directory: false,
				multiple: false,
				title: "选择图片",
				filters: [{ name: "图片文件", extensions: IMAGE_EXTENSIONS }],
			});

			if (!selected || Array.isArray(selected)) return false;

			await this.selectImage(selected);
			const dirPath = await dirname(selected);
			await this.loadDirectoryImages(dirPath);
			return true;
		},

		/** 选择目录，加载目录下所有图片并默认显示第一张 */
		async openDirectoryDialog() {
			const selected = await open({
				directory: true,
				multiple: false,
				title: "选择图片目录",
			});

			if (!selected || Array.isArray(selected)) return false;

			const images = await this.loadDirectoryImages(selected);
			if (images.length > 0) {
				await this.selectImage(images[0].path);
			}
			return true;
		},

		/** 按列表下标切换图片 */
		async selectByIndex(index: number) {
			if (index < 0 || index >= this.imageList.length) return;
			await this.selectImage(this.imageList[index].path);
		},

		/** 相对切换图片，delta 为 -1（上一张）/ 1（下一张） */
		async stepImage(delta: number) {
			const index = this.currentIndex;
			if (index < 0) return;
			await this.selectByIndex(index + delta);
		},

		/** 重新扫描当前目录，尽量保留当前选中的图片 */
		async refreshDirectory() {
			if (!this.currentDirectory) return this.imageList;

			const currentPath = this.currentImage?.path;
			const images = await this.loadDirectoryImages(this.currentDirectory);

			if (currentPath && images.some((img) => img.path === currentPath)) {
				await this.selectImage(currentPath);
			}
			return images;
		},

		/** 重置视图变换 */
		resetView() {
			this.view = createViewState();
		},

		/** 适应窗口：重置缩放与平移，保留旋转 */
		fitView() {
			this.view.scale = 1;
			this.view.x = 0;
			this.view.y = 0;
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
