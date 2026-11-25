const joinBox = document.getElementById('joinBox');
const padTop = document.getElementById('padTop');
const padArea = document.getElementById('padArea');
const nameInput = document.getElementById('nameInput');
const joinBtn = document.getElementById('joinBtn');
const colorDot = document.getElementById('colorDot');
const playerLabel = document.getElementById('playerLabel');
const padRestartBtn = document.getElementById('padRestartBtn');

const btnLeft = document.getElementById('btnLeft');
const btnRight = document.getElementById('btnRight');
const btnJump = document.getElementById('btnJump');

let playerId = localStorage.getItem('picobox_pid');
if (!playerId) {
  playerId = 'g_' + Math.random().toString(36).slice(2, 10);
  localStorage.setItem('picobox_pid', playerId);
}
let savedName = localStorage.getItem('picobox_name') || '';
nameInput.value = savedName;

let ws = null;
let joined = false;
const touchInput = { left: false, right: false, jump: false };
const keyInput = { left: false, right: false, jump: false };
const padInput = { left: false, right: false, jump: false };
const sentInput = { left: false, right: false, jump: false };

function send(obj) {
  if (ws && ws.readyState === WebSocket.OPEN) {
    ws.send(JSON.stringify(obj));
  }
}

function syncInput() {
  if (!joined) return;
  const left = touchInput.left || keyInput.left || padInput.left;
  const right = touchInput.right || keyInput.right || padInput.right;
  const jump = touchInput.jump || keyInput.jump || padInput.jump;

  btnLeft.classList.toggle('pressed', left);
  btnRight.classList.toggle('pressed', right);
  btnJump.classList.toggle('pressed', jump);

  if (left !== sentInput.left || right !== sentInput.right || jump !== sentInput.jump) {
    sentInput.left = left;
    sentInput.right = right;
    sentInput.jump = jump;
    send({ type: 'input', left, right, jump });
  }
}

function bindTouchButton(el, key) {
  const activePointers = new Set();

  const update = () => {
    const down = activePointers.size > 0;
    if (touchInput[key] !== down) {
      touchInput[key] = down;
      syncInput();
    }
  };

  el.addEventListener('pointerdown', (e) => {
    e.preventDefault();
    el.setPointerCapture(e.pointerId);
    activePointers.add(e.pointerId);
    update();
  });

  const release = (e) => {
    if (activePointers.delete(e.pointerId)) {
      update();
    }
  };

  el.addEventListener('pointerup', release);
  el.addEventListener('pointercancel', release);
  el.addEventListener('lostpointercapture', release);
}

bindTouchButton(btnLeft, 'left');
bindTouchButton(btnRight, 'right');
bindTouchButton(btnJump, 'jump');

// Keyboard support on the controller page (Arrows + Space/Up, or WASD + Space)
window.addEventListener('keydown', (e) => {
  if (!joined || document.activeElement === nameInput) return;
  if (['ArrowLeft', 'ArrowRight', 'ArrowUp', 'ArrowDown', 'Space'].includes(e.code)) {
    e.preventDefault();
  }
  if (e.code === 'ArrowLeft' || e.code === 'KeyA') keyInput.left = true;
  if (e.code === 'ArrowRight' || e.code === 'KeyD') keyInput.right = true;
  if (e.code === 'Space' || e.code === 'ArrowUp' || e.code === 'KeyW') keyInput.jump = true;
  if (e.code === 'KeyR') send({ type: 'restart' });
  syncInput();
});

window.addEventListener('keyup', (e) => {
  if (!joined) return;
  if (e.code === 'ArrowLeft' || e.code === 'KeyA') keyInput.left = false;
  if (e.code === 'ArrowRight' || e.code === 'KeyD') keyInput.right = false;
  if (e.code === 'Space' || e.code === 'ArrowUp' || e.code === 'KeyW') keyInput.jump = false;
  syncInput();
});

// Joystick / Gamepad support on the controller page
let prevPadRestart = false;
function pollGamepad() {
  requestAnimationFrame(pollGamepad);
  if (!joined || !navigator.getGamepads) return;
  const pads = navigator.getGamepads();
  let left = false, right = false, jump = false, restart = false;
  for (const gp of pads) {
    if (!gp) continue;
    const ax = gp.axes[0] ?? 0;
    if (ax < -0.35 || gp.buttons[14]?.pressed) left = true;
    if (ax > 0.35 || gp.buttons[15]?.pressed) right = true;
    if (gp.buttons[0]?.pressed || gp.buttons[1]?.pressed || gp.buttons[2]?.pressed || gp.buttons[12]?.pressed) {
      jump = true;
    }
    if (gp.buttons[8]?.pressed || gp.buttons[9]?.pressed) restart = true;
  }
  if (restart && !prevPadRestart) send({ type: 'restart' });
  prevPadRestart = restart;

  if (padInput.left !== left || padInput.right !== right || padInput.jump !== jump) {
    padInput.left = left;
    padInput.right = right;
    padInput.jump = jump;
    syncInput();
  }
}
requestAnimationFrame(pollGamepad);

padRestartBtn.addEventListener('click', () => send({ type: 'restart' }));

function doJoin() {
  const name = nameInput.value.trim() || 'Gato';
  localStorage.setItem('picobox_name', name);
  savedName = name;
  joined = true;
  send({ type: 'join', player_id: playerId, name });
}

joinBtn.addEventListener('click', doJoin);
nameInput.addEventListener('keydown', (e) => { if (e.key === 'Enter') doJoin(); });

function connect() {
  const proto = location.protocol === 'https:' ? 'wss:' : 'ws:';
  ws = new WebSocket(`${proto}//${location.host}/ws`);
  ws.onopen = () => {
    if (joined && savedName) {
      send({ type: 'join', player_id: playerId, name: savedName });
    }
  };
  ws.onmessage = (e) => {
    try {
      const msg = JSON.parse(e.data);
      if (msg.type === 'joined' && msg.player) {
        joinBox.classList.add('hidden');
        padTop.classList.remove('hidden');
        padArea.classList.remove('hidden');
        colorDot.style.background = msg.player.color;
        playerLabel.textContent = msg.player.name;
      }
    } catch {}
  };
  ws.onclose = () => setTimeout(connect, 1500);
}

connect();
