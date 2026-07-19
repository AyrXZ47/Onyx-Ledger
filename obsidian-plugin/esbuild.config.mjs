import { build } from 'esbuild';
import sveltePlugin from 'esbuild-svelte';
import sveltePreprocess from 'svelte-preprocess';
import fs from 'fs';

build({
  entryPoints: ['src/main.ts'],
  // Compilamos directo a la raíz para que Obsidian encuentre main.js
  outfile: 'main.js',
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
}).then(() => {
  // Magia de empaquetado: Obsidian SOLO lee 'styles.css'. 
  // Esbuild genera 'main.css' o 'dist/main.css'. Lo renombramos a la fuerza.
  if (fs.existsSync('main.css')) {
    fs.renameSync('main.css', 'styles.css');
    console.log('⚡ CSS inyectado y renombrado a styles.css con éxito.');
  } else if (fs.existsSync('dist/main.css')) {
    fs.renameSync('dist/main.css', 'styles.css');
    console.log('⚡ CSS inyectado y renombrado a styles.css con éxito.');
  }
}).catch(() => process.exit(1));
