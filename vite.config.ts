import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import AutoImport from "unplugin-auto-import/vite";
import { NaiveUiResolver } from "unplugin-vue-components/resolvers";
import Components from "unplugin-vue-components/vite";

// 由于该指令未使用，将其移除
const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
	base: "./",
	plugins: [
		vue(),
		AutoImport({
			imports: [
				"vue",
				{
					"naive-ui": [
						"useDialog",
						"useMessage",
						"useNotification",
						"useLoadingBar",
					],
				},
			],
		}),
		Components({
			resolvers: [NaiveUiResolver()],
			// 确保 naive-ui 组件被正确解析
			dts: true,
		}),
	],

	// 打包相关配置
	build: {
		// 确保 CSS 被正确提取
		cssCodeSplit: true,
		// 优化 CSS 提取
		assetsInlineLimit: 4096,
		rollupOptions: {
			output: {
				manualChunks: {
					naive: ["naive-ui"],
					vue: ["vue", "vue-router", "pinia"],
				},
				// 确保资源文件名格式一致
				assetFileNames: (assetInfo) => {
					const info = assetInfo.name.split(".");
					let extType = info[info.length - 1];
					if (/\.(css|scss|sass)$/.test(assetInfo.name)) {
						extType = "css";
					}
					return `assets/${extType}/[name]-[hash][extname]`;
				},
				// 确保 chunk 文件名格式一致
				chunkFileNames: "assets/js/[name]-[hash].js",
				// 确保入口文件名格式一致
				entryFileNames: "assets/js/[name]-[hash].js",
			},
		},
	},

	// Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
	//
	// 1. prevent vite from obscuring rust errors
	clearScreen: false,
	// 2. tauri expects a fixed port, fail if that port is not available
	server: {
		port: 1420,
		strictPort: true,
		host: host || false,
		hmr: host
			? {
					protocol: "ws",
					host,
					port: 1421,
			  }
			: undefined,
		watch: {
			// 3. tell vite to ignore watching `src-tauri`
			ignored: ["**/src-tauri/**"],
		},
	},
}));
