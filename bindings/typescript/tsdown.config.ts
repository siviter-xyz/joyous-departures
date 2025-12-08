import { defineConfig } from "tsdown";

export default defineConfig({
  entry: ["src/index.ts"],
  format: ["esm", "cjs"],
  outDir: "dist",
  // Use fixed extensions (.mjs, .cjs) for better compatibility
  fixedExtension: true,
  dts: {
    sourcemap: true,
  },
  sourcemap: true,
  clean: true,
  splitting: false,
  treeshake: true,
  minify: false, // Keep readable for debugging, consumers can minify
  external: [
    // External dependencies - don't bundle these
    // These are relative paths that will be resolved at runtime
    /^\.\.\/pkg\//,
  ],
  // Platform-specific settings
  platform: "node",
  // tsdown automatically handles __dirname and import.meta.url shims
});

