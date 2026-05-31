// Drive headless Edge over CDP to verify the configurator -> renderer handoff
// in a SINGLE browsing session (sessionStorage persists across the same-origin nav).
const fs = require('fs');
const PORT = process.argv[2] || '38090';
const OUT = process.argv[3] || 'handoff.png';
const CDP = process.env.CDP_PORT || '9222';
const BASE = `http://localhost:${PORT}`;
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

async function main() {
  // find a page target
  let targets = [];
  for (let i = 0; i < 20; i++) {
    try { targets = await (await fetch(`http://localhost:${CDP}/json/list`)).json(); } catch {}
    if (targets.find((t) => t.type === 'page')) break;
    await sleep(300);
  }
  const target = targets.find((t) => t.type === 'page');
  if (!target) throw new Error('no page target');

  const ws = new WebSocket(target.webSocketDebuggerUrl);
  let id = 0; const pending = new Map();
  await new Promise((res, rej) => { ws.onopen = res; ws.onerror = rej; });
  ws.onmessage = (m) => {
    const msg = JSON.parse(m.data);
    if (msg.id && pending.has(msg.id)) { pending.get(msg.id)(msg.result); pending.delete(msg.id); }
  };
  const send = (method, params = {}) =>
    new Promise((res) => { const i = ++id; pending.set(i, res); ws.send(JSON.stringify({ id: i, method, params })); });
  const evalJs = async (expression) =>
    (await send('Runtime.evaluate', { expression, returnByValue: true, awaitPromise: true })).result?.value;

  await send('Page.enable');
  await send('Runtime.enable');

  // 1) configurator — let WASM init + first render
  await send('Page.navigate', { url: `${BASE}/configurator/` });
  await sleep(6000);

  // 2) pick a distinctive design: sunflowers image + triangle pattern
  await evalJs(`document.querySelectorAll('.samples button')[1].click()`); // sunflowers
  await sleep(2500);
  await evalJs(`document.getElementById('pat-next').click()`);             // diamond -> hexagonal
  await evalJs(`document.getElementById('pat-next').click()`);             // hexagonal -> triangle
  await sleep(2500);

  const design = await evalJs(`sessionStorage.getItem('sg-design')`);
  const parsed = design ? JSON.parse(design) : {};
  console.log('stored design: pattern=' + parsed.pattern + ', colours=' + (parsed.palette?.length / 3) +
    ', img=' + (parsed.img || '').slice(0, 24) + '…');

  // 3) navigate to renderer in the SAME tab (sessionStorage carries over)
  await send('Page.navigate', { url: `${BASE}/rendering/` });
  await sleep(14000);

  const status = await evalJs(
    `(() => { const s = document.getElementById('status'); return { hidden: s.classList.contains('hidden'),
       text: s.querySelector('strong')?.textContent, dl: !document.getElementById('download-btn').disabled }; })()`);
  console.log('renderer status:', JSON.stringify(status));

  const shot = await send('Page.captureScreenshot', { format: 'png' });
  fs.writeFileSync(OUT, Buffer.from(shot.data, 'base64'));
  console.log('screenshot ->', OUT);
  ws.close();
}
main().catch((e) => { console.error('ERR', e); process.exit(1); });
