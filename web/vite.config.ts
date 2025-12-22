import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import checker from "vite-plugin-checker";
import tailwindcss from "@tailwindcss/vite";
import router from "@tanstack/router-plugin/vite";
import path from "node:path";

export default defineConfig({
  plugins: [
    router({
      target: "react",
      autoCodeSplitting: true,
    }),
    tailwindcss(),
    checker({
      typescript: {
        tsconfigPath: "tsconfig.app.json",
      },
    }),
    react(),
  ],
  resolve: {
    alias: {
      "@": path.join(__dirname, "src"),
    },
  },
  envDir: "..",
  envPrefix: "PUBLIC_",
});
