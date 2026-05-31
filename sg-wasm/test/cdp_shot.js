// Generic CDP screenshot: navigate, run optional setup JS, wait, capture.
const fs = require('fs');
const URL = process.argv[2];
const OUT = process.argv[3];
const SETUP = process.argv[4] || '';
const WAIT = +(process.argv[5] || 8000);
const PREWAIT = +(process.argv[6] || 6000);
const CDP = process.env.CDP_PORT || '9222';
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

async function main() {
  let targets = [];
  for (let i = 0; i < 20; i++) {
    try { targets = await (await fetch(`http://localhost:${CDP}/json/list`)).json(); } catch {}
    if (targets.find((t) => t.type === 'page')) break;
    await sleep(300);
  }
  const target = targets.find((t) => t.type === 'page');
  const ws = new WebSocket(target.webSocketDebuggerUrl);
  let id = 0; const pending = new Map();
  await new Promise((res, rej) => { ws.onopen = res; ws.onerror = rej; });
  ws.onmessage = (m) => { const msg = JSON.parse(m.data); if (msg.id && pending.has(msg.id)) { pending.get(msg.id)(msg.result); pending.delete(msg.id); } };
  const send = (method, params = {}) => new Promise((res) => { const i = ++id; pending.set(i, res); ws.send(JSON.stringify({ id: i, method, params })); });
  const evalJs = (expression) => send('Runtime.evaluate', { expression, returnByValue: true, awaitPromise: true });

  await send('Page.enable');
  await send('Page.navigate', { url: URL });
  await sleep(PREWAIT);
  if (SETUP) { const r = await evalJs(SETUP); console.log('setup ->', JSON.stringify(r.result?.value)); }
  await sleep(WAIT);
  const shot = await send('Page.captureScreenshot', { format: 'png' });
  fs.writeFileSync(OUT, Buffer.from(shot.data, 'base64'));
  console.log('screenshot ->', OUT);
  ws.close();
}
main().catch((e) => { console.error('ERR', e); process.exit(1); });
