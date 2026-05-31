/* @ts-self-types="./sg_wasm.d.ts" */

/**
 * Compute an `n_colors` palette from the image (downsampled KMeans). Returns flat RGB bytes.
 * @param {Uint8Array} pixels
 * @param {number} iw
 * @param {number} ih
 * @param {number} ch
 * @param {number} n_colors
 * @param {number} seed
 * @returns {Uint8Array}
 */
function compute_palette(pixels, iw, ih, ch, n_colors, seed) {
    const ptr0 = passArray8ToWasm0(pixels, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.compute_palette(ptr0, len0, iw, ih, ch, n_colors, seed);
    var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v2;
}
exports.compute_palette = compute_palette;

/**
 * Report tile count for a config (for tuning).
 * @param {string} pattern
 * @param {number} panel_w
 * @param {number} panel_h
 * @param {number} tile_size
 * @returns {number}
 */
function count_tiles(pattern, panel_w, panel_h, tile_size) {
    const ptr0 = passStringToWasm0(pattern, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.count_tiles(ptr0, len0, panel_w, panel_h, tile_size);
    return ret >>> 0;
}
exports.count_tiles = count_tiles;

/**
 * Build the 3D stained-glass GLB using a supplied palette. Returns GLB bytes.
 * @param {Uint8Array} pixels
 * @param {number} img_w
 * @param {number} img_h
 * @param {number} channels
 * @param {string} pattern
 * @param {number} panel_w
 * @param {number} panel_h
 * @param {number} tile_size
 * @param {number} lead_gap
 * @param {number} glass_depth
 * @param {number} frame_height
 * @param {number} glass_alpha
 * @param {Uint8Array} palette
 * @returns {Uint8Array}
 */
function render_glb(pixels, img_w, img_h, channels, pattern, panel_w, panel_h, tile_size, lead_gap, glass_depth, frame_height, glass_alpha, palette) {
    const ptr0 = passArray8ToWasm0(pixels, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(pattern, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passArray8ToWasm0(palette, wasm.__wbindgen_malloc);
    const len2 = WASM_VECTOR_LEN;
    const ret = wasm.render_glb(ptr0, len0, img_w, img_h, channels, ptr1, len1, panel_w, panel_h, tile_size, lead_gap, glass_depth, frame_height, glass_alpha, ptr2, len2);
    var v4 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v4;
}
exports.render_glb = render_glb;

/**
 * Rasterize the flat front view using a supplied palette. Returns RGBA (out_w x out_h).
 * @param {Uint8Array} pixels
 * @param {number} img_w
 * @param {number} img_h
 * @param {number} channels
 * @param {string} pattern
 * @param {number} panel_w
 * @param {number} panel_h
 * @param {number} tile_size
 * @param {number} lead_gap
 * @param {number} glass_alpha
 * @param {Uint8Array} palette
 * @param {number} out_w
 * @param {number} out_h
 * @returns {Uint8Array}
 */
function render_preview(pixels, img_w, img_h, channels, pattern, panel_w, panel_h, tile_size, lead_gap, glass_alpha, palette, out_w, out_h) {
    const ptr0 = passArray8ToWasm0(pixels, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(pattern, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passArray8ToWasm0(palette, wasm.__wbindgen_malloc);
    const len2 = WASM_VECTOR_LEN;
    const ret = wasm.render_preview(ptr0, len0, img_w, img_h, channels, ptr1, len1, panel_w, panel_h, tile_size, lead_gap, glass_alpha, ptr2, len2, out_w, out_h);
    var v4 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v4;
}
exports.render_preview = render_preview;
function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./sg_wasm_bg.js": import0,
    };
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

const wasmPath = `${__dirname}/sg_wasm_bg.wasm`;
const wasmBytes = require('fs').readFileSync(wasmPath);
const wasmModule = new WebAssembly.Module(wasmBytes);
let wasmInstance = new WebAssembly.Instance(wasmModule, __wbg_get_imports());
let wasm = wasmInstance.exports;
wasm.__wbindgen_start();
