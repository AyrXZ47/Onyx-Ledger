import { build } from 'esbuild';
import sveltePlugin from 'esbuild-svelte';
import sveltePreprocess from 'svelte-preprocess';

build({
  entryPoints: ['src/main.ts'],
  outfile: 'dist/main.js',
  bundle: true,
  external: ['obsidian'],
  format: 'cjs',
  target: 'es2020',
  logOverride: {
    'empty-import-meta': 'silent',
  },
  loader: {
    '.wasm': 'base64'
  },
  plugins: [
    sveltePlugin({
      preprocess: sveltePreprocess()
    })
  ]
}).catch(() => process.exit(1));
