import { useMessage as naiveUseMessage } from "naive-ui";

/**
 * 本文件用于封装全局消息提示的函数
 * 用于定制消息
 * 暂时无用
 */

/**
 * 获取 Naive UI 的全局消息提示实例
 * 该函数应在 setup 语法糖或组件 setup 内部调用
 * 返回的 message 实例可用于全局消息提示（success、error、warning、info）
 */
export function useMessage() {
	return naiveUseMessage();
}

/**
 * 显示成功消息
 * @param content 消息内容
 */
export function showSuccess(content: string) {
	const message = naiveUseMessage();
	message.success(content);
}

/**
 * 显示错误消息
 * @param content 消息内容
 */
export function showError(content: string) {
	const message = naiveUseMessage();
	message.error(content);
}

/**
 * 显示警告消息
 * @param content 消息内容
 */
export function showWarning(content: string) {
	const message = naiveUseMessage();
	message.warning(content);
}

/**
 * 显示普通信息消息
 * @param content 消息内容
 */
export function showInfo(content: string) {
	const message = naiveUseMessage();
	message.info(content);
}
