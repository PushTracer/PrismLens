import { watch } from "vue";
import { defineStore } from "pinia";
import { useOsTheme } from "naive-ui";
import type { GlobalThemeOverrides } from "naive-ui";
import { useStorage } from "@vueuse/core";

export type ThemeMode = "light" | "dark";

const FONT_FAMILY = '"Lato", "PingFang SC", "Microsoft YaHei", sans-serif';

/**
 * 主题相关的状态管理
 * 主题值写入 localStorage，并同步到 <html data-theme>，供全局 SCSS 变量使用
 */
export const useThemeStore = defineStore("theme", () => {
	const osTheme = useOsTheme();
	const theme = useStorage<ThemeMode>("color-scheme", osTheme.value ?? "light");

	// 亮色主题配置
	const lightThemeOverrides: GlobalThemeOverrides = {
		common: {
			primaryColor: "#8B5CF6",
			primaryColorHover: "#A78BFA",
			primaryColorPressed: "#7C3AED",
			primaryColorSuppl: "#9333EA",
			textColor1: "#1F2937",
			textColor2: "#4B5563",
			textColor3: "#9CA3AF",
			bodyColor: "#F5F6FA",
			cardColor: "#FFFFFF",
			borderRadius: "10px",
			borderRadiusSmall: "8px",
			fontFamily: FONT_FAMILY,
		},
		Card: {
			borderRadius: "12px",
			color: "#FFFFFF",
			borderColor: "#E5E7EB",
		},
		Button: {
			borderRadiusMedium: "8px",
			borderRadiusSmall: "6px",
			heightMedium: "36px",
			heightSmall: "30px",
		},
		Input: {
			borderRadius: "8px",
		},
		Layout: {
			color: "#F5F6FA",
			headerColor: "#FFFFFF",
			footerColor: "#FFFFFF",
		},
		Tooltip: {
			borderRadius: "8px",
		},
	};

	// 暗色主题配置
	const darkThemeOverrides: GlobalThemeOverrides = {
		common: {
			primaryColor: "#A78BFA",
			primaryColorHover: "#C4B5FD",
			primaryColorPressed: "#8B5CF6",
			primaryColorSuppl: "#7C3AED",
			textColor1: "#F3F4F6",
			textColor2: "#CBD5E1",
			textColor3: "#8B93A1",
			bodyColor: "#0F1115",
			cardColor: "#171A21",
			borderRadius: "10px",
			borderRadiusSmall: "8px",
			fontFamily: FONT_FAMILY,
		},
		Card: {
			borderRadius: "12px",
			color: "#171A21",
			borderColor: "#2A2F3A",
		},
		Button: {
			borderRadiusMedium: "8px",
			borderRadiusSmall: "6px",
			heightMedium: "36px",
			heightSmall: "30px",
		},
		Input: {
			borderRadius: "8px",
		},
		Layout: {
			color: "#0F1115",
			headerColor: "#171A21",
			footerColor: "#171A21",
		},
		Tooltip: {
			borderRadius: "8px",
		},
	};

	watch(
		theme,
		(value) => {
			document.documentElement.dataset.theme = value;
		},
		{ immediate: true }
	);

	return {
		theme,
		lightThemeOverrides,
		darkThemeOverrides,
	};
});
