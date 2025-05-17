<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

async function greet() {
	// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
	greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
	<main class="container">
		<h1>Head Line</h1>

		<p>Click on the Tauri, Vite, and Vue logos to learn more.</p>

		<form class="row" @submit.prevent="greet">
			<input id="greet-input" v-model="name" placeholder="传递信息..." />
			<button type="submit">发送</button>
		</form>
		<p>{{ greetMsg }}</p>
	</main>
	<n-button type="primary"> Primary </n-button>
	<div style="font-family: Lato">Primary</div>
	<div>Primary</div>
</template>

<style scoped></style>
