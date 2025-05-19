import { defineStore } from "pinia";
import { GlobalThemeOverrides } from "naive-ui";
import { useStorage } from "@vueuse/core";
import { useOsTheme } from "naive-ui";

/**
 * 主题相关的状态管理
 */
export const useThemeStore = defineStore("theme", () => {
	const theme = useStorage("color-scheme", useOsTheme().value);

	// 亮色主题配置
	const lightThemeOverrides: GlobalThemeOverrides = {
		common: {
			// 主色调：优雅的紫色
			primaryColor: "#8B5CF6",
			primaryColorHover: "#A78BFA",
			primaryColorPressed: "#7C3AED",
			primaryColorSuppl: "#9333EA",
			textColor1: "#2C3E50",
			textColor2: "#34495E",
			textColor3: "#7F8C8D",
			// 添加背景色配置
			bodyColor: "#F8F9FA",
			cardColor: "#FFFFFF",
		},
		Card: {
			borderRadius: "8px",
			paddingMedium: "24px",
			color: "#FFFFFF",
			borderColor: "#E3F2FD",
		},
		Button: {
			borderRadiusMedium: "4px",
			heightMedium: "40px",
			textColor: "#FFFFFF",
			border: "1px solid #8B5CF6", // 更新边框颜色
		},
		Input: {
			borderRadius: "4px",
			border: "1px solid #E3F2FD",
		},
		Layout: {
			color: "#F8F9FA", // 设置布局背景色
			headerColor: "#FFFFFF",
			footerColor: "#FFFFFF",
		},
	};

	// 暗色主题配置
	const darkThemeOverrides: GlobalThemeOverrides = {
		common: {
			primaryColor: "#A78BFA", // 暗色主题下稍微亮一点的紫色
			primaryColorHover: "#C4B5FD",
			primaryColorPressed: "#8B5CF6",
			primaryColorSuppl: "#7C3AED",
			textColor1: "#FFFFFF",
			textColor2: "#E0E0E0",
			textColor3: "#BDBDBD",
		},
		Card: {
			borderRadius: "8px",
			paddingMedium: "24px",
			color: "#1A1A1A",
			borderColor: "#333333",
		},
		Button: {
			borderRadiusMedium: "4px",
			heightMedium: "40px",
			textColor: "#FFFFFF",
			border: "1px solid #A78BFA", // 更新边框颜色
		},
		Input: {
			borderRadius: "4px",
			border: "1px solid #333333",
		},
		Layout: {
			color: "#121212", // 设置暗色布局背景色
			headerColor: "#1A1A1A",
			footerColor: "#1A1A1A",
		},
	};

	return {
		theme,
		lightThemeOverrides,
		darkThemeOverrides,
	};
});
