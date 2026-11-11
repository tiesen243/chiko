import { bindings, defineConfig } from "cf/config";

export default defineConfig({
  worker: {
    name: "chiko",
    compatibilityDate: "2026-11-11",
    entrypoint: "build/index.js",
    env: {
      DB: bindings.d1({
        id: "00f0f4ec-705c-4c72-9998-fcdcf43675ea",
        name: "chiko-db",
      }),
    },
  },
});
