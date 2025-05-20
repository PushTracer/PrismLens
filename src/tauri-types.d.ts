/**
 * @description 图片信息接口，对应 Rust 中的 ImageInfo 结构体
 */
export interface ImageInfo {
  path: string;   // 图片路径
  name: string;   // 图片名称
  width: number;  // 图片宽度
  height: number; // 图片高度
  format: string; // 图片格式
  size: number;   // 图片大小（字节）
  modified: number; // 最后修改时间
}