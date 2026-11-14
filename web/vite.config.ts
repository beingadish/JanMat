import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// In dev, the app calls the API under the `/api` prefix. We proxy that to the
// running janmat-api backend (default 127.0.0.1:3000) and strip the prefix, so
// no CORS config is needed on the Rust side. Override the target with
// VITE_DEV_API_TARGET when the backend runs elsewhere (e.g. a container).
const apiTarget = process.env.VITE_DEV_API_TARGET ?? "http://127.0.0.1:3000";

export default defineConfig({
  base: "./",
  plugins: [react()],
  server: {
    host: true,
    port: 5173,
    proxy: {
      "/api": {
        target: apiTarget,
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api/, ""),
      },
    },
  },
});
