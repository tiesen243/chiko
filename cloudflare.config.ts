import { defineConfig } from "cf/config";

export default defineConfig({
	worker: {
		name: "chiko",
		compatibilityDate: "2026-11-11",
		entrypoint: "build/index.js",
	},
});
