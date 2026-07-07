import { build } from 'esbuild';
import sveltePlugin from 'esbuild-svelte';
import sveltePreprocess from 'svelte-preprocess';

export default {
  entryPoints: ['src/main.ts'],
  outfile: 'dist/main.js',
  bundle: true,
  target: 'node14.17', // Adjust as needed
  plugins: [
    sveltePlugin({
      preprocess: sveltePreprocess()
    })
  ],
  format: 'cjs'
};
