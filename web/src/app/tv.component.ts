import { AfterViewInit, Component, ElementRef, OnDestroy, OnInit, ViewChild, signal } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatCardModule } from '@angular/material/card';

@Component({
  selector: 'app-pico-tv',
  standalone: true,
  imports: [CommonModule, MatButtonModule, MatIconModule, MatCardModule],
  template: `
    <div class="tv-wrap">
      <header class="tv-bar">
        <div class="tv-left">
          <span class="tv-title">🐱 PicoBox</span>
          <span class="tv-lvl">{{ levelTitle() }}</span>
          <span class="tv-hint">{{ levelHint() }}</span>
        </div>

        <div class="tv-controls">
          <div class="lvl-btns">
            @for (i of levelList(); track i) {
              <button
                mat-stroked-button
                class="lvl-btn"
                [class.active]="i === currentLevel()"
                (click)="send({ type: 'set_level', index: i })"
              >
                {{ i + 1 }}
              </button>
            }
          </div>
          <button mat-stroked-button (click)="send({ type: 'restart' })">
            <mat-icon>refresh</mat-icon> Reiniciar
          </button>
          <button mat-stroked-button (click)="toggleQr()">
            <mat-icon>qr_code_2</mat-icon> QR
          </button>
          <button mat-icon-button (click)="soundOn.set(!soundOn())" aria-label="Som">
            <mat-icon>{{ soundOn() ? 'volume_up' : 'volume_off' }}</mat-icon>
          </button>
        </div>
      </header>

      <div class="stage-wrap">
        <canvas #gameCanvas width="1280" height="720"></canvas>
        @if (showQr()) {
          <mat-card appearance="outlined" class="qr-overlay">
            <strong>Entre pelo Celular 📱</strong>
            <img src="/qr.svg" alt="QR Code" />
            <div class="qr-url">{{ joinUrl() }}</div>
            <div class="qr-sub">Celular, Controle (Joystick) ou Teclado (Setas + Espaço / WASD)</div>
          </mat-card>
        }
      </div>
    </div>
  `,
  styles: [`
    .tv-wrap { height: 100vh; display: flex; flex-direction: column; padding: 12px 18px 16px; gap: 10px; }
    .tv-bar { display: flex; flex-wrap: wrap; align-items: center; justify-content: space-between; gap: 10px; }
    .tv-left { display: flex; align-items: center; gap: 10px; flex-wrap: wrap; }
    .tv-title { font-size: 1.35rem; font-weight: 900; color: #fb923c; }
    .tv-lvl { font-weight: 700; color: #e2e8f0; }
    .tv-hint { color: #94a3b8; font-size: 0.92rem; }
    .tv-controls { display: flex; align-items: center; gap: 8px; flex-wrap: wrap; }
    .lvl-btns { display: flex; gap: 4px; flex-wrap: wrap; }
    .lvl-btn { min-width: 36px !important; padding: 0 8px !important; height: 34px !important; }
    .lvl-btn.active { background: #f97316 !important; color: #fff !important; border-color: #f97316 !important; }
    .stage-wrap { flex: 1; position: relative; display: grid; place-items: center; min-height: 0; }
    canvas { max-width: 100%; max-height: 100%; aspect-ratio: 16 / 9; border-radius: 18px; box-shadow: 0 20px 60px rgba(0,0,0,0.55); background: #fff9f0; display: block; }
    .qr-overlay { position: absolute; top: 16px; right: 16px; width: 215px; padding: 14px; border-radius: 18px !important; text-align: center; display: flex; flex-direction: column; align-items: center; gap: 8px; background: rgba(15, 23, 42, 0.94); }
    .qr-overlay img { width: 164px; height: 164px; background: #fff; padding: 8px; border-radius: 12px; }
    .qr-url { font-family: ui-monospace, monospace; font-size: 0.78rem; color: #38bdf8; word-break: break-all; }
    .qr-sub { font-size: 0.74rem; opacity: 0.7; }
  `],
})
export class TvComponent implements OnInit, AfterViewInit, OnDestroy {
  @ViewChild('gameCanvas') canvasRef!: ElementRef<HTMLCanvasElement>;

  levelTitle = signal('Fase 1');
  levelHint = signal('');
  levelList = signal<number[]>([]);
  currentLevel = signal(0);
  joinUrl = signal(window.location.origin + '/');
  showQr = signal(true);
  soundOn = signal(true);

  private ws: WebSocket | null = null;
  private reconnectTimer: any = null;
  private rafId = 0;
  private latest: any = null;
  private audioCtx: AudioContext | null = null;
  private userHidQr = false;
  private lastHudKey = '';

  private kbArrows = { joined: false, left: false, right: false, jump: false };
  private kbWasd = { joined: false, left: false, right: false, jump: false };
  private gamepadsState = new Map<number, { joined: boolean; left: boolean; right: boolean; jump: boolean; prevRestart: boolean }>();

  private onKeyDown = (e: KeyboardEvent) => {
    if (['ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight', 'Space'].includes(e.code)) e.preventDefault();
    if (e.code === 'ArrowLeft') { this.kbArrows.left = true; this.updateKb(this.kbArrows, 'kb_arrows', 'Teclado'); }
    if (e.code === 'ArrowRight') { this.kbArrows.right = true; this.updateKb(this.kbArrows, 'kb_arrows', 'Teclado'); }
    if (e.code === 'ArrowUp') { this.kbArrows.jump = true; this.updateKb(this.kbArrows, 'kb_arrows', 'Teclado'); }
    if (e.code === 'KeyA') { this.kbWasd.left = true; this.updateKb(this.kbWasd, 'kb_wasd', 'Teclado 2'); }
    if (e.code === 'KeyD') { this.kbWasd.right = true; this.updateKb(this.kbWasd, 'kb_wasd', 'Teclado 2'); }
    if (e.code === 'KeyW') { this.kbWasd.jump = true; this.updateKb(this.kbWasd, 'kb_wasd', 'Teclado 2'); }
    if (e.code === 'Space') {
      const target = this.kbArrows.joined ? this.kbArrows : (this.kbWasd.joined ? this.kbWasd : this.kbArrows);
      target.jump = true;
      this.updateKb(target, target === this.kbArrows ? 'kb_arrows' : 'kb_wasd', target === this.kbArrows ? 'Teclado' : 'Teclado 2');
    }
    if (e.code === 'KeyR') this.send({ type: 'restart' });
  };

  private onKeyUp = (e: KeyboardEvent) => {
    if (e.code === 'ArrowLeft') { this.kbArrows.left = false; this.updateKb(this.kbArrows, 'kb_arrows', 'Teclado'); }
    if (e.code === 'ArrowRight') { this.kbArrows.right = false; this.updateKb(this.kbArrows, 'kb_arrows', 'Teclado'); }
    if (e.code === 'ArrowUp') { this.kbArrows.jump = false; this.updateKb(this.kbArrows, 'kb_arrows', 'Teclado'); }
    if (e.code === 'KeyA') { this.kbWasd.left = false; this.updateKb(this.kbWasd, 'kb_wasd', 'Teclado 2'); }
    if (e.code === 'KeyD') { this.kbWasd.right = false; this.updateKb(this.kbWasd, 'kb_wasd', 'Teclado 2'); }
    if (e.code === 'KeyW') { this.kbWasd.jump = false; this.updateKb(this.kbWasd, 'kb_wasd', 'Teclado 2'); }
    if (e.code === 'Space') {
      if (this.kbArrows.joined && this.kbArrows.jump) { this.kbArrows.jump = false; this.updateKb(this.kbArrows, 'kb_arrows', 'Teclado'); }
      if (this.kbWasd.joined && this.kbWasd.jump) { this.kbWasd.jump = false; this.updateKb(this.kbWasd, 'kb_wasd', 'Teclado 2'); }
    }
  };

  async ngOnInit() {
    try {
      const info = await (await fetch('/api/info')).json();
      if (info.joinUrl) this.joinUrl.set(info.joinUrl);
    } catch {}
    window.addEventListener('keydown', this.onKeyDown);
    window.addEventListener('keyup', this.onKeyUp);
    this.connect();
  }

  ngAfterViewInit() {
    const loop = () => {
      this.rafId = requestAnimationFrame(loop);
      this.pollGamepads();
      this.renderCanvas();
    };
    this.rafId = requestAnimationFrame(loop);
  }

  ngOnDestroy() {
    cancelAnimationFrame(this.rafId);
    clearTimeout(this.reconnectTimer);
    window.removeEventListener('keydown', this.onKeyDown);
    window.removeEventListener('keyup', this.onKeyUp);
    this.ws?.close();
  }

  toggleQr() {
    const next = !this.showQr();
    this.showQr.set(next);
    this.userHidQr = !next;
  }

  send(obj: any) {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(obj));
    }
  }

  private updateKb(kb: { joined: boolean; left: boolean; right: boolean; jump: boolean }, id: string, name: string) {
    if (!kb.joined) {
      kb.joined = true;
      this.send({ type: 'join', player_id: id, name });
    }
    this.send({ type: 'input', player_id: id, left: kb.left, right: kb.right, jump: kb.jump });
  }

  private pollGamepads() {
    const pads = navigator.getGamepads ? navigator.getGamepads() : [];
    for (let i = 0; i < pads.length; i++) {
      const gp = pads[i];
      if (!gp) continue;
      const ax = gp.axes[0] ?? 0;
      const left = ax < -0.35 || Boolean(gp.buttons[14]?.pressed);
      const right = ax > 0.35 || Boolean(gp.buttons[15]?.pressed);
      const jump = Boolean(gp.buttons[0]?.pressed || gp.buttons[1]?.pressed || gp.buttons[2]?.pressed || gp.buttons[12]?.pressed);
      const restartBtn = Boolean(gp.buttons[8]?.pressed || gp.buttons[9]?.pressed);

      let st = this.gamepadsState.get(i);
      if (!st) {
        st = { joined: false, left: false, right: false, jump: false, prevRestart: false };
        this.gamepadsState.set(i, st);
      }
      if (restartBtn && !st.prevRestart) this.send({ type: 'restart' });
      st.prevRestart = restartBtn;

      const id = `gp_${i + 1}`;
      if (!st.joined && (left || right || jump)) {
        st.joined = true;
        this.send({ type: 'join', player_id: id, name: `Controle ${i + 1}` });
      }
      if (st.joined && (st.left !== left || st.right !== right || st.jump !== jump)) {
        st.left = left; st.right = right; st.jump = jump;
        this.send({ type: 'input', player_id: id, left, right, jump });
      }
    }
  }

  private connect() {
    const proto = location.protocol === 'https:' ? 'wss:' : 'ws:';
    this.ws = new WebSocket(`${proto}//${location.host}/ws`);
    this.ws.onopen = () => this.send({ type: 'watch_tv' });
    this.ws.onmessage = (e) => {
      try {
        const msg = JSON.parse(e.data);
        if (msg.level) {
          this.latest = msg;
          this.updateHud(msg);
        }
      } catch {}
    };
    this.ws.onclose = () => {
      this.reconnectTimer = setTimeout(() => this.connect(), 1500);
    };
  }

  private updateHud(st: any) {
    if (st.sfx) this.playSfx(st.sfx);
    const connected = st.players.filter((p: any) => p.connected).length;
    if (connected >= 2 && !this.userHidQr && this.showQr()) {
      this.showQr.set(false);
    }
    const key = `${st.levelIndex}:${st.level.adapted_for}`;
    if (key !== this.lastHudKey) {
      this.lastHudKey = key;
      this.currentLevel.set(st.levelIndex);
      this.levelTitle.set(`${st.level.name} (${st.level.adapted_for}P)`);
      this.levelHint.set(`— ${st.level.hint}`);
      if (this.levelList().length !== st.levelCount) {
        this.levelList.set(Array.from({ length: st.levelCount }, (_, i) => i));
      }
    }
  }

  private playSfx(kind: string) {
    if (!this.soundOn()) return;
    try {
      if (!this.audioCtx) this.audioCtx = new AudioContext();
      const tone = (f: number, d: number, type: OscillatorType = 'triangle', g = 0.12, delay = 0) => {
        const osc = this.audioCtx!.createOscillator();
        const gain = this.audioCtx!.createGain();
        osc.type = type;
        osc.frequency.value = f;
        const t = this.audioCtx!.currentTime + delay;
        gain.gain.setValueAtTime(g, t);
        gain.gain.exponentialRampToValueAtTime(0.001, t + d);
        osc.connect(gain).connect(this.audioCtx!.destination);
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
      else if (kind === 'clear') [523, 659, 784, 1046].forEach((f, i) => tone(f, 0.18, 'triangle', 0.14, i * 0.08));
    } catch {}
  }

  private renderCanvas() {
    if (!this.latest || !this.canvasRef) return;
    const ctx = this.canvasRef.nativeElement.getContext('2d')!;
    const { level, players, key, doorOpen, banner } = this.latest;

    const rrect = (x: number, y: number, w: number, h: number, r: number | number[]) => {
      ctx.beginPath();
      ctx.roundRect(x, y, w, h, r);
      ctx.fill();
    };

    ctx.fillStyle = '#fff9f0';
    ctx.fillRect(0, 0, 1280, 720);

    ctx.strokeStyle = 'rgba(249, 115, 22, 0.06)';
    ctx.lineWidth = 1;
    for (let gx = 0; gx < 1280; gx += 40) { ctx.beginPath(); ctx.moveTo(gx, 0); ctx.lineTo(gx, 720); ctx.stroke(); }
    for (let gy = 0; gy < 720; gy += 40) { ctx.beginPath(); ctx.moveTo(0, gy); ctx.lineTo(1280, gy); ctx.stroke(); }

    for (const s of level.solids) {
      ctx.fillStyle = '#f97316';
      rrect(s.x, s.y, s.w, s.h, 4);
      ctx.fillStyle = '#fb923c';
      ctx.fillRect(s.x, s.y, s.w, Math.min(8, s.h));
    }

    for (const sp of level.spikes) {
      ctx.fillStyle = '#ef4444';
      const count = Math.max(1, Math.floor(sp.w / 20));
      const aw = sp.w / count;
      for (let i = 0; i < count; i++) {
        const sx = sp.x + i * aw;
        ctx.beginPath();
        ctx.moveTo(sx, sp.y + sp.h);
        ctx.lineTo(sx + aw / 2, sp.y);
        ctx.lineTo(sx + aw, sp.y + sp.h);
        ctx.closePath();
        ctx.fill();
      }
    }

    for (const b of level.buttons) {
      const h = b.pressed ? 5 : b.h;
      ctx.fillStyle = b.color;
      rrect(b.x, b.y + (b.h - h), b.w, h, 4);
      ctx.fillStyle = '#334155';
      ctx.fillRect(b.x - 4, b.y + b.h - 4, b.w + 8, 4);
    }

    for (const sb of (level.seq_buttons || [])) {
      const h = sb.activated ? 5 : sb.h;
      ctx.fillStyle = sb.activated ? '#10b981' : '#f59e0b';
      rrect(sb.x, sb.y + (sb.h - h), sb.w, h, 4);
      ctx.fillStyle = '#334155';
      ctx.fillRect(sb.x - 4, sb.y + sb.h - 4, sb.w + 8, 4);
      ctx.fillStyle = sb.activated ? '#047857' : '#92400e';
      ctx.font = '900 18px system-ui, sans-serif';
      ctx.textAlign = 'center';
      ctx.fillText(`${sb.order}`, sb.x + sb.w / 2, sb.y - 8);
    }

    for (const c of (level.crumbles || [])) {
      if (c.state === 2) {
        ctx.save();
        ctx.setLineDash([6, 6]);
        ctx.strokeStyle = 'rgba(146, 64, 14, 0.35)';
        ctx.lineWidth = 2;
        ctx.strokeRect(c.x + 2, c.y + 2, c.w - 4, c.h - 4);
        ctx.restore();
      } else {
        const shakeX = c.state === 1 ? Math.sin(performance.now() * 0.09) * 3.5 : 0;
        ctx.fillStyle = c.state === 1 ? '#b45309' : '#92400e';
        rrect(c.x + shakeX, c.y, c.w, c.h, 6);
        ctx.strokeStyle = '#451a03';
        ctx.lineWidth = 2;
        ctx.stroke();
      }
    }

    for (const sp of (level.springs || [])) {
      const stretch = sp.anim * 10;
      ctx.fillStyle = '#22c55e';
      rrect(sp.x, sp.y - stretch, sp.w, sp.h + stretch, 6);
      ctx.fillStyle = '#15803d';
      ctx.fillRect(sp.x + 4, sp.y + sp.h - 5, sp.w - 8, 5);
      ctx.fillStyle = '#fff';
      ctx.font = '900 13px system-ui, sans-serif';
      ctx.textAlign = 'center';
      ctx.fillText('▲', sp.x + sp.w / 2, sp.y - stretch + 12);
    }

    const sawAngle = performance.now() * 0.012;
    for (const sw of (level.saws || [])) {
      ctx.save();
      ctx.translate(sw.x, sw.y);
      ctx.rotate(sawAngle);
      ctx.fillStyle = '#ef4444';
      ctx.beginPath();
      for (let i = 0; i < 20; i++) {
        const ang = (i * Math.PI) / 10;
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

    for (const g of level.gates) {
      if (g.solid) {
        ctx.fillStyle = g.color;
        rrect(g.x, g.y, g.w, g.h, 6);
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

    const activeCount = Math.max(1, players.filter((p: any) => p.connected && !p.inDoor).length);
    for (const e of level.elevators) {
      ctx.fillStyle = '#6366f1';
      rrect(e.x, e.y, e.w, e.h, 8);
      ctx.fillStyle = '#fff';
      ctx.font = 'bold 16px system-ui, sans-serif';
      ctx.textAlign = 'center';
      ctx.fillText(`⬆ ${e.riders}/${Math.min(e.required, activeCount)}`, e.x + e.w / 2, e.y + 18);
    }

    for (const b of level.push_blocks) {
      const needed = Math.min(b.required, activeCount);
      const moving = b.current_pushers >= needed;
      ctx.fillStyle = moving ? '#10b981' : '#0ea5e9';
      rrect(b.x, b.y, b.w, b.h, 10);
      ctx.strokeStyle = moving ? '#047857' : '#0369a1';
      ctx.lineWidth = 4;
      ctx.stroke();
      ctx.fillStyle = '#fff';
      ctx.font = '900 24px system-ui, sans-serif';
      ctx.textAlign = 'center';
      ctx.fillText(`${b.current_pushers}/${needed}`, b.x + b.w / 2, b.y + b.h / 2 + 8);
    }

    const d = level.door;
    ctx.fillStyle = doorOpen ? '#1e293b' : '#b45309';
    rrect(d.x, d.y, d.w, d.h, [18, 18, 0, 0]);
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

    if (!key.used) {
      const bob = Math.sin(performance.now() / 240) * 4;
      ctx.save();
      ctx.translate(key.x, key.y + bob);
      ctx.fillStyle = '#facc15';
      ctx.strokeStyle = '#a16207';
      ctx.lineWidth = 3;
      ctx.beginPath(); ctx.arc(-8, 0, 9, 0, Math.PI * 2); ctx.fill(); ctx.stroke();
      ctx.fillRect(0, -3, 18, 6); ctx.strokeRect(0, -3, 18, 6);
      ctx.fillRect(10, 3, 4, 6); ctx.fillRect(15, 3, 4, 6);
      ctx.restore();
    }

    if (level.rope_max_dist) {
      const active = players.filter((p: any) => p.connected && !p.inDoor);
      if (active.length >= 2) {
        ctx.save();
        ctx.lineCap = 'round';
        for (let i = 0; i < active.length - 1; i++) {
          const ax = active[i].x + 18, ay = active[i].y + 22;
          const bx = active[i + 1].x + 18, by = active[i + 1].y + 22;
          const tension = Math.min(1, Math.hypot(bx - ax, by - ay) / level.rope_max_dist);
          const mx = (ax + bx) / 2, my = (ay + by) / 2 + Math.max(0, (1 - tension) * 34);
          ctx.strokeStyle = '#451a03'; ctx.lineWidth = 8;
          ctx.beginPath(); ctx.moveTo(ax, ay); ctx.quadraticCurveTo(mx, my, bx, by); ctx.stroke();
          ctx.strokeStyle = tension > 0.85 ? '#ef4444' : '#f59e0b'; ctx.lineWidth = 4.5;
          ctx.beginPath(); ctx.moveTo(ax, ay); ctx.quadraticCurveTo(mx, my, bx, by); ctx.stroke();
        }
        ctx.restore();
      }
    }

    for (const p of players) {
      if (!p.connected || p.inDoor) continue;
      const s = 36, x = p.x, y = p.y;
      ctx.fillStyle = p.color;
      ctx.beginPath(); ctx.moveTo(x + 4, y + 8); ctx.lineTo(x + 8, y - 6); ctx.lineTo(x + 15, y + 6); ctx.fill();
      ctx.beginPath(); ctx.moveTo(x + s - 4, y + 8); ctx.lineTo(x + s - 8, y - 6); ctx.lineTo(x + s - 15, y + 6); ctx.fill();
      rrect(x, y, s, s, 8);
      ctx.strokeStyle = 'rgba(0,0,0,0.22)'; ctx.lineWidth = 2; ctx.stroke();
      const es = p.facing * 3;
      ctx.fillStyle = '#fff';
      ctx.beginPath(); ctx.arc(x + 12 + es, y + 15, 4.5, 0, Math.PI * 2); ctx.arc(x + 24 + es, y + 15, 4.5, 0, Math.PI * 2); ctx.fill();
      ctx.fillStyle = '#0f172a';
      ctx.beginPath(); ctx.arc(x + 13 + es * 1.3, y + 15, 2.3, 0, Math.PI * 2); ctx.arc(x + 25 + es * 1.3, y + 15, 2.3, 0, Math.PI * 2); ctx.fill();
      ctx.strokeStyle = '#0f172a'; ctx.lineWidth = 1.8;
      ctx.beginPath(); ctx.arc(x + 18 + es, y + 22, 3, 0.1 * Math.PI, 0.9 * Math.PI); ctx.stroke();
      ctx.font = 'bold 13px system-ui, sans-serif'; ctx.textAlign = 'center'; ctx.fillStyle = '#1e293b';
      ctx.fillText(p.name, x + s / 2, y - 12);
    }

    if (level.seq_timer > 0 && level.seq_step_timeout > 0) {
      const pct = Math.max(0, Math.min(1, level.seq_timer / level.seq_step_timeout));
      ctx.fillStyle = 'rgba(15, 23, 42, 0.78)';
      rrect(490, 18, 300, 24, 12);
      ctx.fillStyle = pct < 0.35 ? '#ef4444' : '#10b981';
      rrect(494, 22, 292 * pct, 16, 8);
      ctx.fillStyle = '#fff'; ctx.font = '900 13px system-ui, sans-serif'; ctx.textAlign = 'center';
      ctx.fillText(`⏱️ ${level.seq_timer.toFixed(1)}s`, 640, 35);
    }

    if (banner) {
      ctx.fillStyle = 'rgba(15, 23, 42, 0.85)';
      rrect(290, 40, 700, 66, 18);
      ctx.fillStyle = '#fff'; ctx.font = '900 28px system-ui, sans-serif'; ctx.textAlign = 'center';
      ctx.fillText(banner, 640, 83);
    }
  }
}
