<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

async function greet() {
	greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <main class="container">
    <h1>PrismLens</h1>

    <router-view></router-view>

    <p>现代化的图片查看工具，让每一张图片都绽放光彩~</p>

    <div class="row">
      <input id="greet-input" v-model="name" placeholder="传递信息..." />
      <n-button type="primary" @click="greet">发送</n-button>
    </div>
    <p>{{ greetMsg }}</p>
  </main>
  <div class="demo-section">
    <h2>样式展示</h2>
    <div class="text-demo">
      <p>这是基础字体效果</p>
      <code>这是等宽字体效果</code>
    </div>
    <div class="heading-demo">
      <h1>一级标题</h1>
      <h2>二级标题</h2>
      <h3>三级标题</h3>
      <h4>四级标题</h4>
      <h5>五级标题</h5>
      <h6>六级标题</h6>
    </div>
    <div class="link-demo">
      <a href="#">这是一个链接效果</a>
    </div>
  </div>
</template>

<style lang="scss">
@use "sass:color";

.container {
	margin: 0 auto;
	padding: 2rem;
	text-align: center;
}

.demo-section {
	max-width: 800px;
	margin: 2rem auto;
	padding: 2rem;
	border-radius: 8px;
	box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);

	.text-demo {
		margin: 1rem 0;

		code {
			background: #f5f5f5;
			padding: 0.2em 0.4em;
			border-radius: 3px;
		}
	}

	.heading-demo {
		margin: 2rem 0;
		text-align: left;

		h1,
		h2,
		h3,
		h4,
		h5,
		h6 {
			margin: 0.5rem 0;
		}
	}

	.link-demo {
		margin: 1rem 0;

		a {
			&:hover {
				color: #4a90e2;
			}
		}
	}
}

.row {
	display: flex;
	gap: 1rem;
	justify-content: center;
	align-items: center;
	margin: 1rem 0;
}

input {
	padding: 0.5rem 1rem;
	border-radius: 4px;
	border: 1px solid #ddd;

	&:focus {
		outline: none;
		border-color: #4a90e2;
	}
}

// 移除原生button的样式，因为我们使用n-button了
button {
	background: #4a90e2;
	color: white;
	border: none;
	cursor: pointer;
	opacity: 1; // 添加初始透明度
	transition: all 0.3s ease; // 添加过渡效果

	&:hover {
		background: color.adjust(#4a90e2, $lightness: -10%);
	}
}
</style>
<style scoped></style>
