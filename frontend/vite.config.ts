import tailwindcss from "@tailwindcss/vite";
import vue from "@vitejs/plugin-vue";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { defineConfig } from "vite";

/**
 * The backend writes its bound port to `<repo>/.server-port` on startup.
 * Resolved per request, so restarting the backend on a new port needs no
 * Vite restart.
 */
const backendTarget = () => {
  const portFile = resolve(__dirname, "../.server-port");
  let port = process.env.BACKEND_PORT || "8000";
  if (existsSync(portFile)) {
    const value = readFileSync(portFile, "utf8").trim();
    if (/^\d+$/.test(value)) port = value;
  }
  return `http://127.0.0.1:${port}`;
};

export default defineConfig({
  plugins: [vue(), tailwindcss()],
  server: {
    host: "0.0.0.0",
    proxy: {
      "/api": {
        target: backendTarget(),
        changeOrigin: true,
        // Re-resolve the target on every request so a backend restart on a new port just works.
        configure: proxy => {
          const forward = proxy.web.bind(proxy) as (...args: unknown[]) => unknown;
          Object.assign(proxy, {
            web: (req: unknown, res: unknown, options: Record<string, unknown> = {}, ...rest: unknown[]) =>
              forward(req, res, { ...options, target: backendTarget() }, ...rest),
          });
        },
      },
    },
  },
});
