import { reactRouter } from "@react-router/dev/vite";
import tailwindcss from "@tailwindcss/vite";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { defineConfig } from "vite";

// The backend writes its actually-bound port (which may be random when
// PORT=0) to `.server-port` at the repo root. Pick it up at dev-server start
// so the frontend follows the backend automatically; VITE_API_URL still wins
// if set explicitly.
const readBackendPort = (): string => {
  const portFile = resolve(__dirname, "../../.server-port");
  try {
    if (existsSync(portFile)) {
      const port = readFileSync(portFile, "utf-8").trim();
      if (/^\d+$/.test(port)) return port;
    }
  } catch {
    // fall through to the default
  }
  return "8000";
};

export default defineConfig({
  plugins: [tailwindcss(), reactRouter()],
  server: {
    host: "0.0.0.0",
    proxy: {
      "/api": `http://127.0.0.1:${readBackendPort()}`,
      "/stats": `http://127.0.0.1:${readBackendPort()}`,
    },
  },
  resolve: {
    tsconfigPaths: true,
  },
});
