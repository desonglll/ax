import { reactRouter } from "@react-router/dev/vite";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [tailwindcss(), reactRouter()],
  server: {
    host: "0.0.0.0",
    // port: 5173,
  },
  resolve: {
    tsconfigPaths: true,
  },
});
