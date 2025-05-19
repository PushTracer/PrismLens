import { defineStore } from "pinia";
import { GlobalThemeOverrides } from "naive-ui";
import { useStorage } from "@vueuse/core";
import { useOsTheme } from "naive-ui";

/**
 * 主题相关的状态管理
 */
export const useThemeStore = defineStore("theme", () => {
	// 使用 vueuse 的 useStorage 来持久化主题设置
	const theme = useStorage("color-scheme", useOsTheme().value);

	// 亮色主题配置
	const lightThemeOverrides: GlobalThemeOverrides = {
		common: {
			// 主色调：天蓝色
			primaryColor: "#4A90E2",
			primaryColorHover: "#357ABD",
			primaryColorPressed: "#2A6298",
			primaryColorSuppl: "#6BA7E8",
			textColor1: "#2C3E50",
			textColor2: "#34495E",
			textColor3: "#7F8C8D",
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
			border: "1px solid #4A90E2",
		},
		Input: {
			borderRadius: "4px",
			border: "1px solid #E3F2FD",
		},
	};

	// 暗色主题配置
	const darkThemeOverrides: GlobalThemeOverrides = {
		common: {
			primaryColor: "#6BA7E8",
			primaryColorHover: "#89B8ED",
			primaryColorPressed: "#4A90E2",
			primaryColorSuppl: "#357ABD",
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
			border: "1px solid #6BA7E8",
		},
		Input: {
			borderRadius: "4px",
			border: "1px solid #333333",
		},
	};

	return {
		theme,
		lightThemeOverrides,
		darkThemeOverrides,
	};
});
