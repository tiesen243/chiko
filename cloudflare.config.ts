import { bindings, defineConfig } from "cf/config";

export default defineConfig({
  worker: {
    name: "chiko",
    compatibilityDate: "2026-11-11",
    entrypoint: "build/index.js",
    env: {
      DB: bindings.d1({
        id: "aefeb332-0bad-4fc8-b08f-3cde37e6bbbd",
        name: "chiko-db",
      }),
    },
  },
});
