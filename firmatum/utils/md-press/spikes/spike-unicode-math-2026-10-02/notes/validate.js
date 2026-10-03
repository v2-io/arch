// Read JSON lines {id, tex} on stdin; render each with KaTeX (strict) and MathJax (TeX->MathML);
// print JSON lines for failures: {id, tex, katex, mathjax}
const katex = require('katex');
const {mathjax} = require('mathjax-full/js/mathjax.js');
const {TeX} = require('mathjax-full/js/input/tex.js');
const {liteAdaptor} = require('mathjax-full/js/adaptors/liteAdaptor.js');
const {RegisterHTMLHandler} = require('mathjax-full/js/handlers/html.js');
const {AllPackages} = require('mathjax-full/js/input/tex/AllPackages.js');
const {SerializedMmlVisitor} = require('mathjax-full/js/core/MmlTree/SerializedMmlVisitor.js');
const adaptor = liteAdaptor(); RegisterHTMLHandler(adaptor);
const tex = new TeX({packages: AllPackages.filter(p => p !== 'bussproofs')});
const html = mathjax.document('', {InputJax: tex});
const visitor = new SerializedMmlVisitor();
const {STATE} = require('mathjax-full/js/core/MathItem.js');
const rl = require('readline').createInterface({input: process.stdin});
let n = 0, bad = 0;
rl.on('line', line => {
  const r = JSON.parse(line); n++;
  let k = null, m = null;
  try { katex.renderToString(r.tex, {throwOnError: true, strict: 'ignore'}); } catch (e) { k = String(e.message).slice(0, 160); }
  try {
    const node = html.convert(r.tex, {display: false, end: STATE.CONVERT});
    const mml = visitor.visitTree(node);
    if (mml.includes('merror')) { const mm = mml.match(/data-mjx-error="([^"]*)"/); m = mm ? mm[1] : 'merror'; }
  } catch (e) { m = String(e.message).slice(0, 160); }
  if (k || m) { bad++; console.log(JSON.stringify({id: r.id, tex: r.tex, katex: k, mathjax: m})); }
});
rl.on('close', () => console.error(`checked ${n}, failing ${bad}`));
