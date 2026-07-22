import tailwindcss from "@tailwindcss/vite";
import vue from "@vitejs/plugin-vue";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { defineConfig } from "vite";

const readBackendPort = () => {
  const portFile = resolve(__dirname, "../../.server-port");
  if (existsSync(portFile)) {
    const port = readFileSync(portFile, "utf8").trim();
    if (/^\d+$/.test(port)) return port;
  }
  return "8000";
};

export default defineConfig({
  plugins: [vue(), tailwindcss()],
  server: {
    host: "0.0.0.0",
    proxy: {
      "/api": `http://127.0.0.1:${readBackendPort()}`,
      "/stats": `http://127.0.0.1:${readBackendPort()}`,
    },
  },
});
