import { defineConfig } from 'vite';
import { ripple } from 'vite-plugin-ripple';
import tailwindcss from '@tailwindcss/vite';

export default defineConfig({
	plugins: [ripple(), tailwindcss()],
	build: {
		target: 'esnext',
        outDir: "../../dist",
        emptyOutDir: true,
        rollupOptions: {
            input: {
                folder: "/folder/index.html",
                login: "/login/index.html"
            }
        }
	},
    root: "./src",
    appType: "mpa"
});