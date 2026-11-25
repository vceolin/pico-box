const canvas = document.getElementById('gameCanvas');
const ctx = canvas.getContext('2d');
const levelNameEl = document.getElementById('levelName');
const levelHintEl = document.getElementById('levelHint');
const levelBtnsEl = document.getElementById('levelBtns');
const qrOverlay = document.getElementById('qrOverlay');
const qrUrlEl = document.getElementById('qrUrl');
const restartBtn = document.getElementById('restartBtn');
const qrToggleBtn = document.getElementById('qrToggleBtn');
const soundBtn = document.getElementById('soundBtn');

let ws = null;
let latest = null;
let soundOn = true;
let audioCtx = null;
let userHidQr = false;

function playSfx(kind) {
  if (!soundOn || !kind) return;
  try {
    if (!audioCtx) audioCtx = new (window.AudioContext || window.webkitAudioContext)();
    const tone = (f, d, type = 'triangle', g = 0.12, delay = 0) => {
      const osc = audioCtx.createOscillator();
      const gain = audioCtx.createGain();
      osc.type = type;
      osc.frequency.value = f;
      const t = audioCtx.currentTime + delay;
      gain.gain.setValueAtTime(g, t);
      gain.gain.exponentialRampToValueAtTime(0.001, t + d);
      osc.connect(gain).connect(audioCtx.destination);
      osc.start(t);
      osc.stop(t + d);
    };
    if (kind === 'jump') tone(420, 0.09, 'sine', 0.1);
    else if (kind === 'spring') { tone(320, 0.08, 'sine', 0.12); tone(760, 0.14, 'triangle', 0.14, 0.05); }
    else if (kind === 'seq_ok') tone(840, 0.11, 'triangle', 0.13);
    else if (kind === 'seq_fail') { tone(210, 0.12, 'sawtooth', 0.11); tone(160, 0.16, 'sawtooth', 0.11, 0.09); }
    else if (kind === 'crumble') tone(140, 0.12, 'square', 0.08);
    else if (kind === 'join') tone(660, 0.12);
    else if (kind === 'key') { tone(784, 0.1); tone(1046, 0.16, 'triangle', 0.13, 0.08); }
    else if (kind === 'unlock') { tone(587, 0.1); tone(880, 0.18, 'triangle', 0.13, 0.08); }
    else if (kind === 'door') tone(920, 0.09, 'sine', 0.1);
    else if (kind === 'die') { tone(240, 0.14, 'sawtooth', 0.1); tone(150, 0.2, 'sawtooth', 0.1, 0.1); }
    else if (kind === 'clear') { [523, 659, 784, 1046].forEach((f, i) => tone(f, 0.18, 'triangle', 0.14, i * 0.08)); }
  } catch {}
}

function send(obj) {
  if (ws && ws.readyState === WebSocket.OPEN) {
    ws.send(JSON.stringify(obj));
  }
}

restartBtn.addEventListener('click', () => send({ type: 'restart' }));
qrToggleBtn.addEventListener('click', () => {
  const nextHidden = !qrOverlay.classList.contains('hidden');
  qrOverlay.classList.toggle('hidden', nextHidden);
  userHidQr = nextHidden;
});
soundBtn.addEventListener('click', () => {
  soundOn = !soundOn;
  soundBtn.textContent = soundOn ? '🔊' : '🔇';
});

// Local PC Keyboard play (Arrows + Space/Up, and WASD for a 2nd keyboard player)
const kbArrows = { joined: false, left: false, right: false, jump: false };
const kbWasd = { joined: false, left: false, right: false, jump: false };
let lastKb = kbArrows;

function updateKb(kb, id, name) {
  lastKb = kb;
  if (!kb.joined) {
    kb.joined = true;
    send({ type: 'join', player_id: id, name });
  }
  send({ type: 'input', player_id: id, left: kb.left, right: kb.right, jump: kb.jump });
}

window.addEventListener('keydown', (e) => {
  if (['ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight', 'Space'].includes(e.code)) e.preventDefault();
  if (e.code === 'ArrowLeft') { kbArrows.left = true; updateKb(kbArrows, 'kb_arrows', 'Teclado'); }
  if (e.code === 'ArrowRight') { kbArrows.right = true; updateKb(kbArrows, 'kb_arrows', 'Teclado'); }
  if (e.code === 'ArrowUp') { kbArrows.jump = true; updateKb(kbArrows, 'kb_arrows', 'Teclado'); }

  if (e.code === 'KeyA') { kbWasd.left = true; updateKb(kbWasd, 'kb_wasd', 'Teclado 2'); }
  if (e.code === 'KeyD') { kbWasd.right = true; updateKb(kbWasd, 'kb_wasd', 'Teclado 2'); }
  if (e.code === 'KeyW') { kbWasd.jump = true; updateKb(kbWasd, 'kb_wasd', 'Teclado 2'); }

  if (e.code === 'Space') {
    const target = kbArrows.joined ? kbArrows : (kbWasd.joined ? kbWasd : kbArrows);
    const id = target === kbArrows ? 'kb_arrows' : 'kb_wasd';
    const name = target === kbArrows ? 'Teclado' : 'Teclado 2';
    target.jump = true;
    updateKb(target, id, name);
  }
  if (e.code === 'KeyR') send({ type: 'restart' });
});

window.addEventListener('keyup', (e) => {
  if (e.code === 'ArrowLeft') { kbArrows.left = false; updateKb(kbArrows, 'kb_arrows', 'Teclado'); }
  if (e.code === 'ArrowRight') { kbArrows.right = false; updateKb(kbArrows, 'kb_arrows', 'Teclado'); }
  if (e.code === 'ArrowUp') { kbArrows.jump = false; updateKb(kbArrows, 'kb_arrows', 'Teclado'); }

  if (e.code === 'KeyA') { kbWasd.left = false; updateKb(kbWasd, 'kb_wasd', 'Teclado 2'); }
  if (e.code === 'KeyD') { kbWasd.right = false; updateKb(kbWasd, 'kb_wasd', 'Teclado 2'); }
  if (e.code === 'KeyW') { kbWasd.jump = false; updateKb(kbWasd, 'kb_wasd', 'Teclado 2'); }

  if (e.code === 'Space') {
    if (kbArrows.joined && kbArrows.jump) { kbArrows.jump = false; updateKb(kbArrows, 'kb_arrows', 'Teclado'); }
    if (kbWasd.joined && kbWasd.jump) { kbWasd.jump = false; updateKb(kbWasd, 'kb_wasd', 'Teclado 2'); }
  }
});

// Multi-Gamepad / Joystick support (Xbox, PlayStation, Moonlight controllers via Web Gamepad API)
const gamepadsState = new Map(); // index -> { joined, left, right, jump, prevRestart }

function pollGamepads() {
  const pads = navigator.getGamepads ? navigator.getGamepads() : [];
  for (let i = 0; i < pads.length; i++) {
    const gp = pads[i];
    if (!gp) continue;

    const axisX = gp.axes[0] ?? 0;
    const dpadLeft = Boolean(gp.buttons[14]?.pressed);
    const dpadRight = Boolean(gp.buttons[15]?.pressed);
    const left = axisX < -0.35 || dpadLeft;
    const right = axisX > 0.35 || dpadRight;
    const jump = Boolean(
      gp.buttons[0]?.pressed || // A / Cross
      gp.buttons[1]?.pressed || // B / Circle
      gp.buttons[2]?.pressed || // X / Square
      gp.buttons[12]?.pressed   // D-pad Up
    );
    const restartBtn = Boolean(gp.buttons[8]?.pressed || gp.buttons[9]?.pressed); // Back/Select or Start/Options

    let st = gamepadsState.get(i);
    if (!st) {
      st = { joined: false, left: false, right: false, jump: false, prevRestart: false };
      gamepadsState.set(i, st);
    }

    if (restartBtn && !st.prevRestart) {
      send({ type: 'restart' });
    }
    st.prevRestart = restartBtn;

    const anyActive = left || right || jump;
    const id = `gp_${i + 1}`;
    const name = `Controle ${i + 1}`;

    if (!st.joined && anyActive) {
      st.joined = true;
      send({ type: 'join', player_id: id, name });
    }

    if (st.joined && (st.left !== left || st.right !== right || st.jump !== jump)) {
      st.left = left;
      st.right = right;
      st.jump = jump;
      send({ type: 'input', player_id: id, left, right, jump });
    }
  }
}

function drawRoundedRect(x, y, w, h, r) {
  ctx.beginPath();
  ctx.roundRect(x, y, w, h, r);
  ctx.fill();
}

function drawCatPlayer(p) {
  const s = 36;
  const x = p.x;
  const y = p.y;

  // Cat ears
  ctx.fillStyle = p.color;
  ctx.beginPath();
  ctx.moveTo(x + 4, y + 8);
  ctx.lineTo(x + 8, y - 6);
  ctx.lineTo(x + 15, y + 6);
  ctx.fill();

  ctx.beginPath();
  ctx.moveTo(x + s - 4, y + 8);
  ctx.lineTo(x + s - 8, y - 6);
  ctx.lineTo(x + s - 15, y + 6);
  ctx.fill();

  // Body
  drawRoundedRect(x, y, s, s, 8);
  ctx.strokeStyle = 'rgba(0,0,0,0.22)';
  ctx.lineWidth = 2;
  ctx.stroke();

  // Eyes looking in facing direction
  const eyeShift = p.facing * 3;
  ctx.fillStyle = '#ffffff';
  ctx.beginPath();
  ctx.arc(x + 12 + eyeShift, y + 15, 4.5, 0, Math.PI * 2);
  ctx.arc(x + 24 + eyeShift, y + 15, 4.5, 0, Math.PI * 2);
  ctx.fill();

  ctx.fillStyle = '#0f172a';
  ctx.beginPath();
  ctx.arc(x + 13 + eyeShift * 1.3, y + 15, 2.3, 0, Math.PI * 2);
  ctx.arc(x + 25 + eyeShift * 1.3, y + 15, 2.3, 0, Math.PI * 2);
  ctx.fill();

  // Cute mouth
  ctx.strokeStyle = '#0f172a';
  ctx.lineWidth = 1.8;
  ctx.beginPath();
  ctx.arc(x + 18 + eyeShift, y + 22, 3, 0.1 * Math.PI, 0.9 * Math.PI);
  ctx.stroke();

  // Name tag
  ctx.font = 'bold 13px system-ui, sans-serif';
  ctx.textAlign = 'center';
  ctx.fillStyle = '#1e293b';
  ctx.fillText(p.name, x + s / 2, y - 12);
}

function drawKey(x, y) {
  ctx.save();
  ctx.translate(x, y);
  ctx.fillStyle = '#facc15';
  ctx.strokeStyle = '#a16207';
  ctx.lineWidth = 3;
  // Ring
  ctx.beginPath();
  ctx.arc(-8, 0, 9, 0, Math.PI * 2);
  ctx.fill();
  ctx.stroke();
  // Shaft + teeth
  ctx.fillRect(0, -3, 18, 6);
  ctx.strokeRect(0, -3, 18, 6);
  ctx.fillRect(10, 3, 4, 6);
  ctx.fillRect(15, 3, 4, 6);
  ctx.restore();
}

function drawSpikes(sp) {
  ctx.fillStyle = '#ef4444';
  const toothW = 20;
  const count = Math.max(1, Math.floor(sp.w / toothW));
  const actualW = sp.w / count;
  for (let i = 0; i < count; i++) {
    const sx = sp.x + i * actualW;
    ctx.beginPath();
    ctx.moveTo(sx, sp.y + sp.h);
    ctx.lineTo(sx + actualW / 2, sp.y);
    ctx.lineTo(sx + actualW, sp.y + sp.h);
    ctx.closePath();
    ctx.fill();
  }
}

function renderFrame() {
  requestAnimationFrame(renderFrame);
  pollGamepads();
  if (!latest) return;

  const { level, players, key, doorOpen, banner } = latest;

  // Background
  ctx.fillStyle = '#fff9f0';
  ctx.fillRect(0, 0, 1280, 720);

  // Subtle background grid
  ctx.strokeStyle = 'rgba(249, 115, 22, 0.06)';
  ctx.lineWidth = 1;
  for (let gx = 0; gx < 1280; gx += 40) {
    ctx.beginPath(); ctx.moveTo(gx, 0); ctx.lineTo(gx, 720); ctx.stroke();
  }
  for (let gy = 0; gy < 720; gy += 40) {
    ctx.beginPath(); ctx.moveTo(0, gy); ctx.lineTo(1280, gy); ctx.stroke();
  }

  // Solids
  for (const s of level.solids) {
    ctx.fillStyle = '#f97316';
    drawRoundedRect(s.x, s.y, s.w, s.h, 4);
    ctx.fillStyle = '#fb923c';
    ctx.fillRect(s.x, s.y, s.w, Math.min(8, s.h));
  }

  // Spikes
  for (const sp of level.spikes) drawSpikes(sp);

  // Buttons
  for (const b of level.buttons) {
    const h = b.pressed ? 5 : b.h;
    const y = b.y + (b.h - h);
    ctx.fillStyle = b.color;
    drawRoundedRect(b.x, y, b.w, h, 4);
    ctx.fillStyle = '#334155';
    ctx.fillRect(b.x - 4, b.y + b.h - 4, b.w + 8, 4);
  }

  // Numbered Sequence Buttons (1 -> 2 -> 3...)
  for (const sb of (level.seq_buttons || [])) {
    const h = sb.activated ? 5 : sb.h;
    const y = sb.y + (sb.h - h);
    ctx.fillStyle = sb.activated ? '#10b981' : '#f59e0b';
    drawRoundedRect(sb.x, y, sb.w, h, 4);
    ctx.fillStyle = '#334155';
    ctx.fillRect(sb.x - 4, sb.y + sb.h - 4, sb.w + 8, 4);
    ctx.fillStyle = sb.activated ? '#047857' : '#92400e';
    ctx.font = '900 18px system-ui, sans-serif';
    ctx.textAlign = 'center';
    ctx.fillText(`${sb.order}`, sb.x + sb.w / 2, sb.y - 8);
  }

  // Crumbling platforms
  for (const c of (level.crumbles || [])) {
    if (c.state === 2) {
      ctx.save();
      ctx.setLineDash([6, 6]);
      ctx.strokeStyle = 'rgba(146, 64, 14, 0.35)';
      ctx.lineWidth = 2;
      ctx.strokeRect(c.x + 2, c.y + 2, c.w - 4, c.h - 4);
      ctx.restore();
    } else {
      const shakeX = c.state === 1 ? (Math.sin(performance.now() * 0.09) * 3.5) : 0;
      ctx.fillStyle = c.state === 1 ? '#b45309' : '#92400e';
      drawRoundedRect(c.x + shakeX, c.y, c.w, c.h, 6);
      ctx.strokeStyle = '#451a03';
      ctx.lineWidth = 2;
      ctx.stroke();
    }
  }

  // Spring pads
  for (const sp of (level.springs || [])) {
    const stretch = sp.anim * 10;
    ctx.fillStyle = '#22c55e';
    drawRoundedRect(sp.x, sp.y - stretch, sp.w, sp.h + stretch, 6);
    ctx.fillStyle = '#15803d';
    ctx.fillRect(sp.x + 4, sp.y + sp.h - 5, sp.w - 8, 5);
    ctx.fillStyle = '#ffffff';
    ctx.font = '900 13px system-ui, sans-serif';
    ctx.textAlign = 'center';
    ctx.fillText('▲', sp.x + sp.w / 2, sp.y - stretch + 12);
  }

  // Moving spinning saws
  const sawAngle = performance.now() * 0.012;
  for (const sw of (level.saws || [])) {
    ctx.save();
    ctx.translate(sw.x, sw.y);
    ctx.rotate(sawAngle);
    ctx.fillStyle = '#ef4444';
    const teeth = 10;
    ctx.beginPath();
    for (let i = 0; i < teeth * 2; i++) {
      const ang = (i * Math.PI) / teeth;
      const rad = i % 2 === 0 ? sw.radius + 4 : sw.radius - 5;
      if (i === 0) ctx.moveTo(Math.cos(ang) * rad, Math.sin(ang) * rad);
      else ctx.lineTo(Math.cos(ang) * rad, Math.sin(ang) * rad);
    }
    ctx.closePath();
    ctx.fill();
    ctx.fillStyle = '#1e293b';
    ctx.beginPath();
    ctx.arc(0, 0, 7, 0, Math.PI * 2);
    ctx.fill();
    ctx.restore();
  }

  // Gates / Bridges
  for (const g of level.gates) {
    if (g.solid) {
      ctx.fillStyle = g.color;
      drawRoundedRect(g.x, g.y, g.w, g.h, 6);
      ctx.strokeStyle = 'rgba(0,0,0,0.25)';
      ctx.lineWidth = 2;
      ctx.stroke();
    } else {
      ctx.save();
      ctx.setLineDash([8, 8]);
      ctx.strokeStyle = g.color;
      ctx.lineWidth = 3;
      ctx.strokeRect(g.x + 2, g.y + 2, g.w - 4, g.h - 4);
      ctx.restore();
    }
  }

  // Elevators
  const activeCount = Math.max(1, players.filter((p) => p.connected && !p.inDoor).length);
  for (const e of level.elevators) {
    ctx.fillStyle = '#6366f1';
    drawRoundedRect(e.x, e.y, e.w, e.h, 8);
    const needed = Math.min(e.required, activeCount);
    ctx.fillStyle = '#ffffff';
    ctx.font = 'bold 16px system-ui, sans-serif';
    ctx.textAlign = 'center';
    ctx.fillText(`⬆ ${e.riders}/${needed}`, e.x + e.w / 2, e.y + 18);
  }

  // Push blocks
  for (const b of level.push_blocks) {
    const needed = Math.min(b.required, activeCount);
    const moving = b.current_pushers >= needed;
    ctx.fillStyle = moving ? '#10b981' : '#0ea5e9';
    drawRoundedRect(b.x, b.y, b.w, b.h, 10);
    ctx.strokeStyle = moving ? '#047857' : '#0369a1';
    ctx.lineWidth = 4;
    ctx.stroke();
    ctx.fillStyle = '#ffffff';
    ctx.font = '900 24px system-ui, sans-serif';
    ctx.textAlign = 'center';
    ctx.fillText(`${b.current_pushers}/${needed}`, b.x + b.w / 2, b.y + b.h / 2 + 8);
  }

  // Exit Door
  const d = level.door;
  ctx.fillStyle = doorOpen ? '#1e293b' : '#b45309';
  drawRoundedRect(d.x, d.y, d.w, d.h, [18, 18, 0, 0]);
  ctx.strokeStyle = doorOpen ? '#34d399' : '#78350f';
  ctx.lineWidth = 4;
  ctx.stroke();
  if (!doorOpen) {
    ctx.fillStyle = '#facc15';
    ctx.beginPath();
    ctx.arc(d.x + d.w - 12, d.y + d.h / 2 + 4, 5, 0, Math.PI * 2);
    ctx.fill();
  } else {
    ctx.fillStyle = '#34d399';
    ctx.font = 'bold 13px system-ui, sans-serif';
    ctx.textAlign = 'center';
    ctx.fillText('ENTREM!', d.x + d.w / 2, d.y - 8);
  }

  // Floating Key
  if (!key.used) {
    const bob = Math.sin(performance.now() / 240) * 4;
    drawKey(key.x, key.y + bob);
  }

  // Rope between tethered players (sagging curve when slack, taut red-orange when stretched!)
  if (level.rope_max_dist) {
    const active = players.filter((p) => p.connected && !p.inDoor);
    if (active.length >= 2) {
      ctx.save();
      ctx.lineCap = 'round';
      for (let i = 0; i < active.length - 1; i++) {
        const ax = active[i].x + 18;
        const ay = active[i].y + 22;
        const bx = active[i + 1].x + 18;
        const by = active[i + 1].y + 22;
        const dist = Math.hypot(bx - ax, by - ay);
        const tension = Math.min(1, dist / level.rope_max_dist);
        const sag = Math.max(0, (1 - tension) * 34);
        const mx = (ax + bx) / 2;
        const my = (ay + by) / 2 + sag;

        // Outer border for crisp contrast
        ctx.strokeStyle = '#451a03';
        ctx.lineWidth = 8;
        ctx.beginPath();
        ctx.moveTo(ax, ay);
        ctx.quadraticCurveTo(mx, my, bx, by);
        ctx.stroke();

        // Inner rope color (shifts to red when stretched tight)
        ctx.strokeStyle = tension > 0.85 ? '#ef4444' : '#f59e0b';
        ctx.lineWidth = 4.5;
        ctx.beginPath();
        ctx.moveTo(ax, ay);
        ctx.quadraticCurveTo(mx, my, bx, by);
        ctx.stroke();
      }
      ctx.restore();
    }
  }

  // Players
  for (const p of players) {
    if (p.connected && !p.inDoor) drawCatPlayer(p);
  }

  // Sequence countdown bar (when timed sequence is active!)
  if (level.seq_timer > 0 && level.seq_step_timeout > 0) {
    const pct = Math.max(0, Math.min(1, level.seq_timer / level.seq_step_timeout));
    ctx.fillStyle = 'rgba(15, 23, 42, 0.78)';
    drawRoundedRect(490, 18, 300, 24, 12);
    ctx.fillStyle = pct < 0.35 ? '#ef4444' : '#10b981';
    drawRoundedRect(494, 22, 292 * pct, 16, 8);
    ctx.fillStyle = '#ffffff';
    ctx.font = '900 13px system-ui, sans-serif';
    ctx.textAlign = 'center';
    ctx.fillText(`⏱️ ${level.seq_timer.toFixed(1)}s`, 640, 35);
  }

  // Banner overlay
  if (banner) {
    ctx.fillStyle = 'rgba(15, 23, 42, 0.85)';
    drawRoundedRect(290, 40, 700, 66, 18);
    ctx.fillStyle = '#ffffff';
    ctx.font = '900 28px system-ui, sans-serif';
    ctx.textAlign = 'center';
    ctx.fillText(banner, 640, 83);
  }
}

let lastHudKey = '';
function updateHud(state) {
  if (state.sfx) playSfx(state.sfx);

  const connected = state.players.filter((p) => p.connected).length;
  if (connected >= 2 && !userHidQr) {
    qrOverlay.classList.add('hidden');
  }

  const hudKey = `${state.levelIndex}:${state.level.adapted_for}`;
  if (hudKey !== lastHudKey) {
    lastHudKey = hudKey;
    levelNameEl.textContent = `${state.level.name} (${state.level.adapted_for}P)`;
    levelHintEl.textContent = `— ${state.level.hint}`;
    levelBtnsEl.innerHTML = Array.from({ length: state.levelCount }, (_, i) =>
      `<button class="btn ${i === state.levelIndex ? 'active' : ''}" data-lvl="${i}">${i + 1}</button>`
    ).join('');
    levelBtnsEl.querySelectorAll('[data-lvl]').forEach((b) => {
      b.addEventListener('click', () => send({ type: 'set_level', index: Number(b.dataset.lvl) }));
    });
  }
}

async function init() {
  try {
    const info = await (await fetch('/api/info')).json();
    qrUrlEl.textContent = info.joinUrl;
  } catch {}

  function connect() {
    const proto = location.protocol === 'https:' ? 'wss:' : 'ws:';
    ws = new WebSocket(`${proto}//${location.host}/ws`);
    ws.onopen = () => send({ type: 'watch_tv' });
    ws.onmessage = (e) => {
      try {
        const msg = JSON.parse(e.data);
        if (msg.level) {
          latest = msg;
          updateHud(msg);
        }
      } catch {}
    };
    ws.onclose = () => setTimeout(connect, 1500);
  }
  connect();
  requestAnimationFrame(renderFrame);
}

init();
