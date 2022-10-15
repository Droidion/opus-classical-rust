import typescript from '@rollup/plugin-typescript';
import svelte from 'rollup-plugin-svelte';
import sveltePreprocess from 'svelte-preprocess';
import commonjs from '@rollup/plugin-commonjs';
import resolve from '@rollup/plugin-node-resolve';
import { terser } from 'rollup-plugin-terser';
import gzipPlugin from 'rollup-plugin-gzip';
import brotli from "rollup-plugin-brotli";

export default {
    input: 'frontend/scripts/main.ts',
    strictDeprecations: true,
    output: {
        file: 'static/bundle.js',
        format: 'es',
        sourcemap: true,
    },
    plugins: [
        svelte({
            preprocess: sveltePreprocess(),
        }),
        resolve({
            browser: true,
            dedupe: ['svelte']
        }),
        commonjs(),
        typescript(),
        terser(),
        gzipPlugin({
            additionalFiles: ['static/bundle.css']
        }),
        brotli({
            additional: ['static/bundle.css']
        })
    ],
};