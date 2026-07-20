import { build } from 'esbuild';
import sveltePlugin from 'esbuild-svelte';
import sveltePreprocess from 'svelte-preprocess';

build({
  entryPoints: ['src/main.ts'],
  outfile: 'dist/main.js', // Volvemos a mandarlo a dist (o a la raíz si prefieres)
  bundle: true,
  external: ['obsidian'],
  format: 'cjs',
  target: 'es2020',
  loader: {
    '.wasm': 'base64'
  },
  logOverride: { 'empty-import-meta': 'silent' },
  plugins: [
    sveltePlugin({
      preprocess: sveltePreprocess()
    })
  ]
}).catch(() => process.exit(1));
