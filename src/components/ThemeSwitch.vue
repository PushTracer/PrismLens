<script setup lang="ts">
import { computed } from "vue";
import { MoonOutline, SunnyOutline } from "@vicons/ionicons5";
import { useThemeStore } from "../store";

const themeStore = useThemeStore();
const isDark = computed(() => themeStore.theme === "dark");

function toggleTheme() {
	themeStore.theme = isDark.value ? "light" : "dark";
}
</script>

<template>
	<n-tooltip :show-arrow="false" :delay="300">
		<template #trigger>
			<n-button
				quaternary
				circle
				class="theme-toggle"
				:focusable="false"
				@click="toggleTheme"
			>
				<n-icon size="18">
					<SunnyOutline v-if="isDark" />
					<MoonOutline v-else />
				</n-icon>
			</n-button>
		</template>
		{{ isDark ? "切换到浅色模式" : "切换到深色模式" }}
	</n-tooltip>
</template>

<style scoped>
.theme-toggle {
	color: var(--text-2);
}

.theme-toggle:hover {
	color: var(--accent);
	background-color: var(--accent-soft) !important;
}
</style>
