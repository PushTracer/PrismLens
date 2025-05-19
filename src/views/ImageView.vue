<script setup lang="ts">
import { ref, onMounted } from "vue"; // ✨ 导入 onMounted
import { invoke } from "@tauri-apps/api/core"; // ✨ 导入 invoke
// import { NImage } from 'naive-ui'; // 导入 NImage 组件
// 导入 Tauri API 或者其他需要的库
// import { useRoute } from 'vue-router';
// import { useImageStore } from '@/store/images'; // 假设我们有这个 store

// 定义一个响应式变量来存放当前图片的路径或者数据
const currentImageUrl = ref<string | null>(null);
// ✨ 定义一个响应式变量来存放图片路径列表
const imageList = ref<string[]>([]);
// ✨ 定义一个响应式变量来存放当前选中的文件夹路径 (暂时硬编码一个用于测试)
const currentFolderPath = ref<string>("E:\图片"); // 👈 哥哥可以改成你电脑上存放图片的文件夹路径哦！

// TODO: 实现加载图片文件的逻辑
// 这部分需要调用 Rust 后端来读取文件
// 比如：
/*
async function loadImage(filePath: string) {
    try {
        // 调用 Tauri 命令读取图片数据
        // const imageData = await invoke('read_image_file', { path: filePath });
        // currentImageUrl.value = `data:image/png;base64,${imageData}`; // 假设返回的是 base64 编码
        // 或者直接使用文件路径，如果前端可以直接访问
        // currentImageUrl.value = filePath;

        // TODO: 更新 store 中的当前图片信息
        // const imageStore = useImageStore();
        // imageStore.setCurrentImageUrl(currentImageUrl.value);

    } catch (error) {
        console.error('Failed to load image:', error);
        currentImageUrl.value = null; // 加载失败则清空
    }
}
*/

// ✨ 实现调用后端命令读取图片列表的函数
async function loadImagesFromFolder(folderPath: string) {
	try {
		// 调用 Rust 后端的 read_image_list 命令
		const result = await invoke<string[]>("read_image_list", {
			dirPath: folderPath,
		});
		imageList.value = result; // 将获取到的图片路径列表赋值给 imageList
		console.log("图片列表:", imageList.value); // 打印出来看看对不对
		// 默认显示第一张图片 (如果列表不为空)
		if (imageList.value.length > 0) {
			selectImage(imageList.value[0]);
		}
	} catch (error) {
		console.error("Failed to read image list:", error);
		imageList.value = []; // 读取失败则清空列表
	}
}

// ✨ 实现选中图片并显示的功能
function selectImage(filePath: string) {
	// 使用自定义的 prism:// 协议来构建图片 URL
	currentImageUrl.value = `prism://${filePath}`;
	console.log("当前图片 URL:", currentImageUrl.value); // 打印出来看看 URL 对不对
}

// TODO: 根据路由参数或者 store 来获取要显示的图片路径
// const route = useRoute();
// const initialFilePath = route.query.path as string; // 假设图片路径通过路由参数传递

// 在组件加载时加载图片
onMounted(() => {
	// ✨ 在组件挂载后调用加载图片列表的函数
	if (currentFolderPath.value) {
		loadImagesFromFolder(currentFolderPath.value);
	}
});

// TODO: 添加图片操作相关的状态和方法 (旋转、缩放、拖拽等)
// const scale = ref(1);
// const rotation = ref(0);
// const position = ref({ x: 0, y: 0 });

// function zoomIn() { scale.value *= 1.1; }
// function zoomOut() { scale.value /= 1.1; }
// function rotateLeft() { rotation.value -= 90; }
// function rotateRight() { rotation.value += 90; }
// function startDrag(event: MouseEvent) { /* ... */ }
</script>

<template>
	<div class="image-viewer-container">
		<!-- ✨ 添加图片列表区域 -->
		<div class="image-list-sidebar">
			<h3>图片列表</h3>
			<ul>
				<li
					v-for="imagePath in imageList"
					:key="imagePath"
					@click="selectImage(imagePath)"
				>
					{{ imagePath.split("\\").pop() }}
					<!-- 只显示文件名 -->
				</li>
			</ul>
		</div>

		<!-- TODO: 这里显示图片 -->
		<div v-if="currentImageUrl" class="image-wrapper">
			<!-- 使用 n-image 标签显示图片 -->
			<n-image
				:src="currentImageUrl"
				alt="查看的图片"
				class="displayed-image"
				draggable="false"
				preview-disabled
			/>
		</div>
		<div v-else class="placeholder">请选择一张图片来查看哦~ (｡･ω･｡)ﾉ♡</div>

		<!-- TODO: 添加图片操作的控制按钮或者滑块 -->
		<!--
        <div class="controls">
            <n-button @click="zoomIn">放大</n-button>
            <n-button @click="zoomOut">缩小</n-button>
            <n-button @click="rotateLeft">左转</n-button>
            <n-button @click="rotateRight">右转</n-button>
        </div>
        -->
	</div>
</template>

<style scoped>
.image-viewer-container {
	display: flex; /* ✨ 改为 flex 布局，让列表和图片并排 */
	flex-direction: row; /* ✨ 横向排列 */
	align-items: flex-start; /* ✨ 顶部对齐 */
	justify-content: flex-start; /* ✨ 左侧对齐 */
	width: 100%;
	height: 100%;
	overflow: hidden; /* 防止图片超出容器时出现滚动条 */
	background-color: var(--n-color); /* 使用 Naive UI 的背景色变量 */
}

/* ✨ 添加图片列表侧边栏样式 */
.image-list-sidebar {
	width: 200px; /* 列表宽度 */
	height: 100%;
	overflow-y: auto; /* 列表内容多时出现滚动条 */
	background-color: var(--n-color-alt); /* 使用 Naive UI 的备用背景色 */
	padding: 10px;
	box-sizing: border-box;
	border-right: 1px solid var(--n-border-color); /* 添加右边框 */
}

.image-list-sidebar h3 {
	margin-top: 0;
	color: var(--n-text-color);
}

.image-list-sidebar ul {
	list-style: none;
	padding: 0;
	margin: 0;
}

.image-list-sidebar li {
	padding: 5px;
	cursor: pointer;
	color: var(--n-text-color);
	white-space: nowrap; /* 防止文件名换行 */
	overflow: hidden; /* 隐藏超出部分 */
	text-overflow: ellipsis; /* 超出部分显示省略号 */
}

.image-list-sidebar li:hover {
	background-color: var(--n-action-color); /* 鼠标悬停时的背景色 */
}

.image-wrapper {
	flex-grow: 1; /* ✨ 让图片区域占据剩余空间 */
	max-width: calc(100% - 200px); /* ✨ 减去列表的宽度 */
	max-height: 100%;
	display: flex;
	align-items: center;
	justify-content: center;
	/* 允许图片在容器内自由缩放和定位 */
}

.displayed-image {
	max-width: 100%;
	max-height: 100%;
	object-fit: contain; /* 保持图片比例 */
	cursor: grab; /* 拖拽时的鼠标样式 */
	/* 添加过渡效果让操作更平滑 */
	transition: transform 0.1s ease-out;
}

.displayed-image:active {
	cursor: grabbing; /* 拖拽进行时的鼠标样式 */
}

.placeholder {
	flex-grow: 1; /* ✨ 让占位符也占据剩余空间 */
	font-size: 1.5em;
	color: var(--n-text-color); /* 使用 Naive UI 的文本颜色变量 */
	display: flex; /* ✨ 让占位符居中 */
	align-items: center;
	justify-content: center;
}

/* TODO: 控制按钮的样式 */
/*
.controls {
    position: absolute;
    bottom: 20px;
    left: 50%;
    transform: translateX(-50%);
    z-index: 10;
}
*/
</style>
