/**
 * @description 图片基础信息接口，对应 Rust 中的 ImageBasicInfo 结构体
 */
export interface ImageBasicInfo {
	path: string; // 图片路径
	name: string; // 图片名称
	size: number; // 图片大小（字节）
	modified: number; // 最后修改时间
}

/**
 * @description 图片完整信息接口，对应 Rust 中的 ImageInfo 结构体
 */
export interface ImageInfo extends ImageBasicInfo {
	width: number; // 图片宽度
	height: number; // 图片高度
	format: string; // 图片格式
}
