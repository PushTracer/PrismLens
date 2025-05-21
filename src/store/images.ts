import { defineStore } from "pinia";
import type { ImageInfo } from "../tauri-types";

/**
 * 使用选项式定义 Pinia 会自动处理响应性
 * 图片相关的状态管理
 */
export const useImageStore = defineStore("image", {
	state: () => ({
		imageList: [] as ImageInfo[],
		currentImage: null as ImageInfo | null,
	}),
});
