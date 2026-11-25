import { Component, OnDestroy, OnInit, signal } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { MatCardModule } from '@angular/material/card';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';

@Component({
  selector: 'app-pico-play',
  standalone: true,
  imports: [
    CommonModule,
    FormsModule,
    MatCardModule,
    MatButtonModule,
    MatIconModule,
    MatFormFieldModule,
    MatInputModule,
  ],
  template: `
    <div class="pad-body">
      @if (!joined()) {
        <mat-card appearance="outlined" class="join-box">
          <h1>🐱 PicoBox</h1>
          <p>Digite seu apelido e use o celular, teclado (Setas/WASD + Espaço) ou controle!</p>
          <mat-form-field appearance="outline" subscriptSizing="dynamic">
            <mat-label>Seu apelido</mat-label>
            <input matInput maxlength="14" [ngModel]="name()" (ngModelChange)="name.set($event)" (keydown.enter)="doJoin()" />
          </mat-form-field>
          <button mat-flat-button class="join-btn" (click)="doJoin()">Entrar na Fase</button>
        </mat-card>
      } @else {
        <div class="pad-top">
          <div class="player-tag">
            <span class="color-dot" [style.background]="playerColor()"></span>
            <span>{{ playerLabel() }}</span>
          </div>
          <button mat-stroked-button (click)="send({ type: 'restart' })">
            <mat-icon>refresh</mat-icon> Reiniciar Fase
          </button>
        </div>

        <div class="pad-area">
          <div class="dpad">
            <button
              type="button"
              class="ctrl-btn"
              [class.pressed]="pressedLeft()"
              (pointerdown)="onPtrDown($event, 'left')"
              (pointerup)="onPtrUp($event, 'left')"
              (pointercancel)="onPtrUp($event, 'left')"
              (lostpointercapture)="onPtrUp($event, 'left')"
            >
              ◀
            </button>
            <button
              type="button"
              class="ctrl-btn"
              [class.pressed]="pressedRight()"
              (pointerdown)="onPtrDown($event, 'right')"
              (pointerup)="onPtrUp($event, 'right')"
              (pointercancel)="onPtrUp($event, 'right')"
              (lostpointercapture)="onPtrUp($event, 'right')"
            >
              ▶
            </button>
          </div>
          <button
            type="button"
            class="ctrl-btn jump-btn"
            [class.pressed]="pressedJump()"
            (pointerdown)="onPtrDown($event, 'jump')"
            (pointerup)="onPtrUp($event, 'jump')"
            (pointercancel)="onPtrUp($event, 'jump')"
            (lostpointercapture)="onPtrUp($event, 'jump')"
          >
            PULAR ⬆
          </button>
        </div>
      }
    </div>
  `,
  styles: [`
    .pad-body {
      height: 100dvh;
      overflow: hidden;
      display: flex;
      flex-direction: column;
      padding: calc(10px + env(safe-area-inset-top)) calc(14px + env(safe-area-inset-right)) calc(14px + env(safe-area-inset-bottom)) calc(14px + env(safe-area-inset-left));
      gap: 10px;
      touch-action: none;
    }
    .join-box {
      margin: auto;
      width: 100%;
      max-width: 390px;
      padding: 24px;
      border-radius: 22px !important;
      display: flex;
      flex-direction: column;
      gap: 14px;
      text-align: center;
      background: #1e293b;
    }
    .join-box h1 { margin: 0; font-size: 1.65rem; color: #fb923c; font-weight: 900; }
    .join-box p { margin: 0; opacity: 0.72; font-size: 0.94rem; }
    .join-btn { height: 48px !important; font-size: 1.05rem !important; border-radius: 14px !important; }
    .pad-top { display: flex; align-items: center; justify-content: space-between; gap: 10px; }
    .player-tag { display: inline-flex; align-items: center; gap: 8px; font-weight: 800; font-size: 1.05rem; }
    .color-dot { width: 18px; height: 18px; border-radius: 5px; display: inline-block; }
    .pad-area { flex: 1; display: flex; align-items: stretch; justify-content: space-between; gap: 16px; min-height: 0; }
    .dpad { display: flex; gap: 14px; flex: 1.15; }
    .ctrl-btn {
      flex: 1;
      border-radius: 24px;
      border: 3px solid #334155;
      background: #1e293b;
      color: #f8fafc;
      font-size: clamp(2rem, 6vw, 3.2rem);
      font-weight: 900;
      display: grid;
      place-items: center;
      touch-action: none;
      cursor: pointer;
      transition: transform 0.05s ease, background 0.05s ease;
    }
    .ctrl-btn.pressed { transform: scale(0.95); background: #334155; border-color: #fb923c; }
    .jump-btn { flex: 0.9; background: linear-gradient(135deg, #f97316, #ea580c); border-color: #fdba74; color: #fff; font-size: clamp(1.3rem, 4vw, 2rem); }
    .jump-btn.pressed { background: #c2410c; transform: scale(0.95); }
  `],
})
export class PlayComponent implements OnInit, OnDestroy {
  name = signal(localStorage.getItem('picobox_name') || '');
  joined = signal(false);
  playerColor = signal('#f97316');
  playerLabel = signal('Jogador');

  pressedLeft = signal(false);
  pressedRight = signal(false);
  pressedJump = signal(false);

  private ws: WebSocket | null = null;
  private reconnectTimer: any = null;
  private rafId = 0;
  private playerId: string;

  private ptrs = { left: new Set<number>(), right: new Set<number>(), jump: new Set<number>() };
  private touchInput = { left: false, right: false, jump: false };
  private keyInput = { left: false, right: false, jump: false };
  private padInput = { left: false, right: false, jump: false };
  private sent = { left: false, right: false, jump: false };
  private prevPadRestart = false;

  private onKeyDown = (e: KeyboardEvent) => {
    if (!this.joined()) return;
    if (['ArrowLeft', 'ArrowRight', 'ArrowUp', 'ArrowDown', 'Space'].includes(e.code)) e.preventDefault();
    if (e.code === 'ArrowLeft' || e.code === 'KeyA') this.keyInput.left = true;
    if (e.code === 'ArrowRight' || e.code === 'KeyD') this.keyInput.right = true;
    if (e.code === 'Space' || e.code === 'ArrowUp' || e.code === 'KeyW') this.keyInput.jump = true;
    if (e.code === 'KeyR') this.send({ type: 'restart' });
    this.syncInput();
  };

  private onKeyUp = (e: KeyboardEvent) => {
    if (!this.joined()) return;
    if (e.code === 'ArrowLeft' || e.code === 'KeyA') this.keyInput.left = false;
    if (e.code === 'ArrowRight' || e.code === 'KeyD') this.keyInput.right = false;
    if (e.code === 'Space' || e.code === 'ArrowUp' || e.code === 'KeyW') this.keyInput.jump = false;
    this.syncInput();
  };

  constructor() {
    let pid = localStorage.getItem('picobox_pid');
    if (!pid) {
      pid = 'g_' + Math.random().toString(36).slice(2, 10);
      localStorage.setItem('picobox_pid', pid);
    }
    this.playerId = pid;
  }

  ngOnInit() {
    window.addEventListener('keydown', this.onKeyDown);
    window.addEventListener('keyup', this.onKeyUp);
    this.connect();

    const poll = () => {
      this.rafId = requestAnimationFrame(poll);
      if (!this.joined() || !navigator.getGamepads) return;
      const pads = navigator.getGamepads();
      let left = false, right = false, jump = false, restart = false;
      for (const gp of pads) {
        if (!gp) continue;
        const ax = gp.axes[0] ?? 0;
        if (ax < -0.35 || gp.buttons[14]?.pressed) left = true;
        if (ax > 0.35 || gp.buttons[15]?.pressed) right = true;
        if (gp.buttons[0]?.pressed || gp.buttons[1]?.pressed || gp.buttons[2]?.pressed || gp.buttons[12]?.pressed) jump = true;
        if (gp.buttons[8]?.pressed || gp.buttons[9]?.pressed) restart = true;
      }
      if (restart && !this.prevPadRestart) this.send({ type: 'restart' });
      this.prevPadRestart = restart;
      if (this.padInput.left !== left || this.padInput.right !== right || this.padInput.jump !== jump) {
        this.padInput = { left, right, jump };
        this.syncInput();
      }
    };
    this.rafId = requestAnimationFrame(poll);
  }

  ngOnDestroy() {
    cancelAnimationFrame(this.rafId);
    clearTimeout(this.reconnectTimer);
    window.removeEventListener('keydown', this.onKeyDown);
    window.removeEventListener('keyup', this.onKeyUp);
    this.ws?.close();
  }

  private connect() {
    const proto = location.protocol === 'https:' ? 'wss:' : 'ws:';
    this.ws = new WebSocket(`${proto}//${location.host}/ws`);
    this.ws.onopen = () => {
      if (this.joined() && this.name().trim()) {
        this.send({ type: 'join', player_id: this.playerId, name: this.name().trim() });
      }
    };
    this.ws.onmessage = (e) => {
      try {
        const msg = JSON.parse(e.data);
        if (msg.type === 'joined' && msg.player) {
          this.joined.set(true);
          this.playerColor.set(msg.player.color);
          this.playerLabel.set(msg.player.name);
        }
      } catch {}
    };
    this.ws.onclose = () => {
      this.reconnectTimer = setTimeout(() => this.connect(), 1500);
    };
  }

  send(obj: any) {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(obj));
    }
  }

  doJoin() {
    const clean = this.name().trim() || 'Gato';
    this.name.set(clean);
    localStorage.setItem('picobox_name', clean);
    this.send({ type: 'join', player_id: this.playerId, name: clean });
  }

  onPtrDown(e: PointerEvent, key: 'left' | 'right' | 'jump') {
    e.preventDefault();
    (e.currentTarget as HTMLElement).setPointerCapture(e.pointerId);
    this.ptrs[key].add(e.pointerId);
    this.touchInput[key] = this.ptrs[key].size > 0;
    this.syncInput();
  }

  onPtrUp(e: PointerEvent, key: 'left' | 'right' | 'jump') {
    if (this.ptrs[key].delete(e.pointerId)) {
      this.touchInput[key] = this.ptrs[key].size > 0;
      this.syncInput();
    }
  }

  private syncInput() {
    if (!this.joined()) return;
    const left = this.touchInput.left || this.keyInput.left || this.padInput.left;
    const right = this.touchInput.right || this.keyInput.right || this.padInput.right;
    const jump = this.touchInput.jump || this.keyInput.jump || this.padInput.jump;
    this.pressedLeft.set(left);
    this.pressedRight.set(right);
    this.pressedJump.set(jump);
    if (left !== this.sent.left || right !== this.sent.right || jump !== this.sent.jump) {
      this.sent = { left, right, jump };
      this.send({ type: 'input', left, right, jump });
    }
  }
}
