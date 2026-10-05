// Cross-check General_Category and Script from `utf-mcp dump` against the
// JavaScript engine's ICU (Unicode version = process.versions.unicode), for every
// code point assigned by that version (surrogates excluded).
//
//   target/release/utf-mcp dump > target/dump.tsv && node checks/crosscheck_icu.mjs target/dump.tsv
//
// Written by the independent verifier (de-novo-feedback-1.md); adopted here with
// the Unicode version read from the runtime instead of fixed at 17.0.
import fs from 'fs';
const uv = process.versions.unicode.split('.').map(Number);
const lines = fs.readFileSync(process.argv[2], 'utf8').split('\n');
const h = lines[0].split('\t'); const I = n => h.indexOf(n); const cache = new Map();
const re = (k, v) => { const key = k + '=' + v; if (!cache.has(key)) { let r; try { r = new RegExp(`^\\p{${k}=${v}}$`, 'u'); } catch (e) { r = e.message; } cache.set(key, r); } return cache.get(key); };
const within = a => { const [x, y] = a.split('.').map(Number); return x < uv[0] || (x === uv[0] && y <= uv[1]); };
const bad = { gc: [], sc: [] }; let n = 0;
for (const l of lines.slice(1)) {
  if (!l) continue; const r = l.split('\t'); const age = r[I('age')];
  if (age === 'Unassigned' || !within(age)) continue;
  const cp = parseInt(r[0], 16); if (cp >= 0xD800 && cp <= 0xDFFF) continue;
  const ch = String.fromCodePoint(cp); n++;
  for (const [k, col, b] of [['General_Category', 'gc', 'gc'], ['Script', 'script', 'sc']]) {
    const x = re(k, r[I(col)]); if (typeof x === 'string' || !x.test(ch)) bad[b].push(r[0]);
  }
}
console.log(`node ${process.version} ICU unicode ${process.versions.unicode}: compared ${n} code points; gc mismatches ${bad.gc.length}, script mismatches ${bad.sc.length}`);
for (const k of ['gc', 'sc']) if (bad[k].length) console.log(k, bad[k].slice(0, 20).join(' '));
