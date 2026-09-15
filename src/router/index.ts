import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
	history: createWebHistory(),
	routes: [
		{
			path: "/",
			redirect: "/image",
		},
		{
			path: "/image",
			name: "image",
			component: () => import("../views/ImageView.vue"),
		},
	],
});

export default router;
