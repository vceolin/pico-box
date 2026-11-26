use crate::levels::{build_level, level_count, LevelData, Rect, WORLD_H, WORLD_W};
use serde::Serialize;

pub const PLAYER_SIZE: f32 = 36.0;
const MOVE_SPEED: f32 = 275.0;
const GRAVITY: f32 = 1350.0;
const JUMP_VEL: f32 = -545.0;
const MAX_FALL: f32 = 820.0;
const BLOCK_PUSH_SPEED: f32 = 125.0;
const ELEVATOR_SPEED: f32 = 120.0;

const PALETTE: &[&str] = &[
    "#ff6b6b", // Coral Red
    "#4ecdc4", // Mint Cyan
    "#ffe66d", // Sunny Yellow
    "#a78bfa", // Lavender Purple
    "#ff9f43", // Tangerine
    "#f472b6", // Bubblegum Pink
    "#60a5fa", // Sky Blue
    "#34d399", // Emerald
];

#[derive(Debug, Clone, Default)]
pub struct PlayerInput {
    pub left: bool,
    pub right: bool,
    pub jump: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerState {
    pub id: String,
    pub name: String,
    pub color: String,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub facing: i8, // -1 | 1
    pub on_ground: bool,
    pub in_door: bool,
    pub connected: bool,
    #[serde(skip)]
    pub input: PlayerInput,
    #[serde(skip)]
    pub coyote_timer: f32,
    #[serde(skip)]
    pub jump_buffer: f32,
    #[serde(skip)]
    pub prev_jump: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyState {
    pub x: f32,
    pub y: f32,
    pub holder_id: Option<String>,
    pub used: bool,
}

pub struct GameWorld {
    pub level_index: usize,
    pub level: LevelData,
    pub players: Vec<PlayerState>,
    pub key: KeyState,
    pub door_open: bool,
    pub status: String, // "playing" | "cleared" | "victory"
    pub banner: Option<String>,
    pub status_timer: f32,
    pub sfx_event: Option<&'static str>,
}

fn overlaps(ax: f32, ay: f32, aw: f32, ah: f32, bx: f32, by: f32, bw: f32, bh: f32) -> bool {
    ax < bx + bw && ax + aw > bx && ay < by + bh && ay + ah > by
}

impl GameWorld {
    pub fn new() -> Self {
        let level = build_level(0, 1);
        let key = KeyState {
            x: level.key_pos.0,
            y: level.key_pos.1,
            holder_id: None,
            used: false,
        };
        Self {
            level_index: 0,
            level,
            players: Vec::new(),
            key,
            door_open: false,
            status: "playing".to_string(),
            banner: None,
            status_timer: 0.0,
            sfx_event: None,
        }
    }

    fn connected_count(&self) -> usize {
        self.players.iter().filter(|p| p.connected).count().max(1)
    }

    pub fn load_level(&mut self, idx: usize) {
        self.level_index = idx % level_count();
        let count = self.connected_count();
        self.level = build_level(self.level_index, count);
        self.key = KeyState {
            x: self.level.key_pos.0,
            y: self.level.key_pos.1,
            holder_id: None,
            used: false,
        };
        self.door_open = false;
        self.status = "playing".to_string();
        self.banner = None;
        self.status_timer = 0.0;
        self.reset_player_positions();
    }

    fn adapt_if_fresh(&mut self) {
        let count = self.connected_count();
        if self.status == "playing"
            && !self.door_open
            && self.key.holder_id.is_none()
            && self.level.adapted_for != count
        {
            self.load_level(self.level_index);
        }
    }

    pub fn reset_level(&mut self, reason: Option<&str>) {
        let idx = self.level_index;
        self.load_level(idx);
        if let Some(msg) = reason {
            self.banner = Some(msg.to_string());
            self.status_timer = 1.6;
        }
    }

    fn reset_player_positions(&mut self) {
        let (sx, sy) = self.level.spawn;
        for (i, p) in self.players.iter_mut().enumerate() {
            p.x = sx + (i as f32) * (PLAYER_SIZE + 8.0);
            p.y = sy;
            p.vx = 0.0;
            p.vy = 0.0;
            p.on_ground = false;
            p.in_door = false;
            p.coyote_timer = 0.0;
            p.jump_buffer = 0.0;
        }
    }

    pub fn join_player(&mut self, id: String, name: String) -> PlayerState {
        let clean = name.trim().chars().take(14).collect::<String>();
        let clean = if clean.is_empty() {
            format!("Gato {}", self.players.len() + 1)
        } else {
            clean
        };

        if let Some(p) = self.players.iter_mut().find(|p| p.id == id) {
            p.name = clean;
            p.connected = true;
            let ret = p.clone();
            self.adapt_if_fresh();
            return ret;
        }

        let color = PALETTE[self.players.len() % PALETTE.len()].to_string();
        let (sx, sy) = self.level.spawn;
        let offset = (self.players.len() as f32) * (PLAYER_SIZE + 8.0);
        let player = PlayerState {
            id,
            name: clean,
            color,
            x: (sx + offset).min(WORLD_W - 100.0),
            y: sy,
            vx: 0.0,
            vy: 0.0,
            facing: 1,
            on_ground: false,
            in_door: false,
            connected: true,
            input: PlayerInput::default(),
            coyote_timer: 0.0,
            jump_buffer: 0.0,
            prev_jump: false,
        };
        self.players.push(player.clone());
        self.adapt_if_fresh();
        self.sfx_event = Some("join");
        player
    }

    pub fn set_connected(&mut self, id: &str, connected: bool) {
        if let Some(p) = self.players.iter_mut().find(|p| p.id == id) {
            p.connected = connected;
            if !connected {
                p.input = PlayerInput::default();
            }
        }
        self.adapt_if_fresh();
    }

    pub fn set_input(&mut self, id: &str, left: bool, right: bool, jump: bool) {
        if let Some(p) = self.players.iter_mut().find(|p| p.id == id) {
            p.input.left = left;
            p.input.right = right;
            p.input.jump = jump;
        }
    }

    pub fn step(&mut self, dt: f32) {
        self.sfx_event = None;

        if self.status == "cleared" {
            self.status_timer -= dt;
            if self.status_timer <= 0.0 {
                if self.level_index + 1 < level_count() {
                    self.load_level(self.level_index + 1);
                } else {
                    self.status = "victory".to_string();
                    self.banner = Some("🏆 VOCÊS ZERARAM O PICOBOX!".to_string());
                }
            }
            return;
        }

        if self.status == "victory" {
            return;
        }

        if self.status_timer > 0.0 {
            self.status_timer -= dt;
            if self.status_timer <= 0.0 {
                self.banner = None;
            }
        }

        let active_count = self
            .players
            .iter()
            .filter(|p| p.connected && !p.in_door)
            .count()
            .max(1);

        // 1. Update buttons (pressed by any active player or push block)
        for btn in &mut self.level.buttons {
            let mut pressed = false;
            for p in &self.players {
                if p.connected && !p.in_door
                    && overlaps(p.x, p.y, PLAYER_SIZE, PLAYER_SIZE, btn.x, btn.y - 4.0, btn.w, btn.h + 6.0)
                {
                    pressed = true;
                    break;
                }
            }
            if !pressed {
                for b in &self.level.push_blocks {
                    if overlaps(b.x, b.y, b.w, b.h, btn.x, btn.y - 4.0, btn.w, btn.h + 6.0) {
                        pressed = true;
                        break;
                    }
                }
            }
            btn.pressed = pressed;
        }

        // 1b. Update sequence buttons (1 -> 2 -> 3...) with optional relay countdown!
        if !self.level.seq_buttons.is_empty() {
            let activated_count = self
                .level
                .seq_buttons
                .iter()
                .filter(|b| b.activated)
                .count();
            let all_done = activated_count == self.level.seq_buttons.len();

            if activated_count > 0 && !all_done && self.level.seq_step_timeout > 0.0 {
                self.level.seq_timer -= dt;
                if self.level.seq_timer <= 0.0 {
                    for sb in &mut self.level.seq_buttons {
                        sb.activated = false;
                    }
                    self.level.seq_timer = 0.0;
                    self.sfx_event = Some("seq_fail");
                    self.banner = Some("⏱️ Tempo esgotado! Coordenem juntos!".to_string());
                    self.status_timer = 1.4;
                }
            }

            let next_expected = self
                .level
                .seq_buttons
                .iter()
                .filter(|b| b.activated)
                .count()
                + 1;
            let mut wrong_hit = false;
            let mut right_hit = false;
            for sb in &mut self.level.seq_buttons {
                let touched = self.players.iter().any(|p| {
                    p.connected
                        && !p.in_door
                        && overlaps(p.x, p.y, PLAYER_SIZE, PLAYER_SIZE, sb.x, sb.y - 4.0, sb.w, sb.h + 6.0)
                });
                let just_stepped = touched && !sb.prev_touched;
                sb.prev_touched = touched;
                if sb.activated || !just_stepped {
                    continue;
                }
                if sb.order == next_expected {
                    sb.activated = true;
                    right_hit = true;
                } else if sb.order > next_expected {
                    wrong_hit = true;
                }
            }
            if wrong_hit {
                for sb in &mut self.level.seq_buttons {
                    sb.activated = false;
                }
                self.level.seq_timer = 0.0;
                self.sfx_event = Some("seq_fail");
                self.banner = Some("❌ Ordem errada! Comece pelo 1!".to_string());
                self.status_timer = 1.4;
            } else if right_hit {
                self.sfx_event = Some("seq_ok");
                let now_done = self.level.seq_buttons.iter().all(|b| b.activated);
                if now_done {
                    self.level.seq_timer = 0.0;
                } else if self.level.seq_step_timeout > 0.0 {
                    self.level.seq_timer = self.level.seq_step_timeout;
                }
            }
        }

        // 1c. Update crumbling platforms
        for c in &mut self.level.crumbles {
            if c.state == 0 {
                let stood_on = self.players.iter().any(|p| {
                    p.connected
                        && !p.in_door
                        && p.x + PLAYER_SIZE > c.x + 2.0
                        && p.x < c.x + c.w - 2.0
                        && (p.y + PLAYER_SIZE - c.y).abs() <= 5.0
                });
                if stood_on {
                    c.state = 1;
                    c.timer = 0.85;
                }
            } else if c.state == 1 {
                c.timer -= dt;
                if c.timer <= 0.0 {
                    c.state = 2;
                    c.timer = 2.8;
                    self.sfx_event = Some("crumble");
                }
            } else if c.state == 2 {
                c.timer -= dt;
                if c.timer <= 0.0 {
                    c.state = 0;
                    c.timer = 0.0;
                }
            }
        }

        // 1d. Update moving saws
        for sw in &mut self.level.saws {
            let dist = ((sw.x2 - sw.x1).powi(2) + (sw.y2 - sw.y1).powi(2)).sqrt().max(1.0);
            sw.t += (sw.speed / dist) * dt * sw.dir;
            if sw.t >= 1.0 {
                sw.t = 1.0;
                sw.dir = -1.0;
            } else if sw.t <= 0.0 {
                sw.t = 0.0;
                sw.dir = 1.0;
            }
            sw.x = sw.x1 + (sw.x2 - sw.x1) * sw.t;
            sw.y = sw.y1 + (sw.y2 - sw.y1) * sw.t;
        }

        // 2. Update gates/bridges from buttons (including special "seq_all")
        let seq_all_done = !self.level.seq_buttons.is_empty()
            && self.level.seq_buttons.iter().all(|b| b.activated);
        for gate in &mut self.level.gates {
            let pressed = if gate.button_id == "seq_all" {
                seq_all_done
            } else {
                self.level
                    .buttons
                    .iter()
                    .find(|b| b.id == gate.button_id)
                    .map(|b| b.pressed)
                    .unwrap_or(false)
            };
            gate.solid = if gate.is_bridge { pressed } else { !pressed };
        }

        // 3. Update elevators (count riders, move smoothly, carry riders)
        for elev in &mut self.level.elevators {
            let mut riders = 0;
            for p in &self.players {
                if p.connected && !p.in_door {
                    let on_top = p.x + PLAYER_SIZE > elev.x + 2.0
                        && p.x < elev.x + elev.w - 2.0
                        && (p.y + PLAYER_SIZE - elev.y).abs() <= 6.0;
                    if on_top {
                        riders += 1;
                    }
                }
            }
            elev.riders = riders;
            let needed = elev.required.min(active_count).max(1);
            let prev_y = elev.y;
            if riders >= needed {
                elev.y = (elev.y - ELEVATOR_SPEED * dt).max(elev.target_y);
            } else {
                elev.y = (elev.y + ELEVATOR_SPEED * dt).min(elev.start_y);
            }
            elev.dy = elev.y - prev_y;

            if elev.dy.abs() > 0.0001 {
                for p in &mut self.players {
                    if p.connected && !p.in_door {
                        let on_top = p.x + PLAYER_SIZE > elev.x + 2.0
                            && p.x < elev.x + elev.w - 2.0
                            && (p.y + PLAYER_SIZE - prev_y).abs() <= 6.0;
                        if on_top {
                            p.y += elev.dy;
                        }
                    }
                }
            }
        }

        // 4. Update push blocks (supports push chains: players lined up behind each other OR stacked!)
        let static_solids = self.collect_static_solids();
        for bi in 0..self.level.push_blocks.len() {
            let b = self.level.push_blocks[bi].clone();

            // Fixed-point propagation for pushing RIGHT
            let mut right_pushers = vec![false; self.players.len()];
            for (idx, p) in self.players.iter().enumerate() {
                if !p.connected || p.in_door || !p.input.right {
                    continue;
                }
                let vert_overlap = p.y + PLAYER_SIZE > b.y + 4.0 && p.y < b.y + b.h - 4.0;
                if vert_overlap && (p.x + PLAYER_SIZE - b.x).abs() <= 6.0 {
                    right_pushers[idx] = true;
                }
            }
            loop {
                let mut added = false;
                for i in 0..self.players.len() {
                    if right_pushers[i] || !self.players[i].connected || self.players[i].in_door || !self.players[i].input.right {
                        continue;
                    }
                    let pi = &self.players[i];
                    for j in 0..self.players.len() {
                        if !right_pushers[j] { continue; }
                        let pj = &self.players[j];
                        let vert_overlap = pi.y + PLAYER_SIZE > pj.y + 4.0 && pi.y < pj.y + PLAYER_SIZE - 4.0;
                        if vert_overlap && (pi.x + PLAYER_SIZE - pj.x).abs() <= 8.0 {
                            right_pushers[i] = true;
                            added = true;
                            break;
                        }
                    }
                }
                if !added { break; }
            }

            // Fixed-point propagation for pushing LEFT
            let mut left_pushers = vec![false; self.players.len()];
            for (idx, p) in self.players.iter().enumerate() {
                if !p.connected || p.in_door || !p.input.left {
                    continue;
                }
                let vert_overlap = p.y + PLAYER_SIZE > b.y + 4.0 && p.y < b.y + b.h - 4.0;
                if vert_overlap && (p.x - (b.x + b.w)).abs() <= 6.0 {
                    left_pushers[idx] = true;
                }
            }
            loop {
                let mut added = false;
                for i in 0..self.players.len() {
                    if left_pushers[i] || !self.players[i].connected || self.players[i].in_door || !self.players[i].input.left {
                        continue;
                    }
                    let pi = &self.players[i];
                    for j in 0..self.players.len() {
                        if !left_pushers[j] { continue; }
                        let pj = &self.players[j];
                        let vert_overlap = pi.y + PLAYER_SIZE > pj.y + 4.0 && pi.y < pj.y + PLAYER_SIZE - 4.0;
                        if vert_overlap && (pi.x - (pj.x + PLAYER_SIZE)).abs() <= 8.0 {
                            left_pushers[i] = true;
                            added = true;
                            break;
                        }
                    }
                }
                if !added { break; }
            }

            let push_right = right_pushers.iter().filter(|&&v| v).count();
            let push_left = left_pushers.iter().filter(|&&v| v).count();
            let needed = b.required.min(active_count).max(1);
            let mut new_vx = 0.0;
            let pushers;
            if push_right >= needed && push_right > push_left {
                new_vx = BLOCK_PUSH_SPEED;
                pushers = push_right;
            } else if push_left >= needed && push_left > push_right {
                new_vx = -BLOCK_PUSH_SPEED;
                pushers = push_left;
            } else {
                pushers = push_right.max(push_left);
            }

            let blk = &mut self.level.push_blocks[bi];
            blk.current_pushers = pushers;
            blk.vx = new_vx;

            // Move X & resolve against static solids
            let old_x = blk.x;
            blk.x += blk.vx * dt;
            for s in &static_solids {
                if overlaps(blk.x, blk.y, blk.w, blk.h, s.x, s.y, s.w, s.h) {
                    if blk.vx > 0.0 {
                        blk.x = s.x - blk.w;
                    } else if blk.vx < 0.0 {
                        blk.x = s.x + s.w;
                    }
                }
            }
            let dx = blk.x - old_x;

            // Carry any players standing on top of the push block
            if dx.abs() > 0.0001 {
                let bx = blk.x;
                let by = blk.y;
                let bw = blk.w;
                for p in &mut self.players {
                    if p.connected && !p.in_door {
                        let on_top = p.x + PLAYER_SIZE > old_x + 2.0
                            && p.x < old_x + bw - 2.0
                            && (p.y + PLAYER_SIZE - by).abs() <= 5.0;
                        if on_top {
                            p.x += dx;
                        }
                    }
                }
                let _ = bx;
            }

            // Gravity on push block
            blk.vy = (blk.vy + GRAVITY * dt).min(MAX_FALL);
            blk.y += blk.vy * dt;
            for s in &static_solids {
                if overlaps(blk.x, blk.y, blk.w, blk.h, s.x, s.y, s.w, s.h) {
                    if blk.vy > 0.0 {
                        blk.y = s.y - blk.h;
                        blk.vy = 0.0;
                    }
                }
            }
        }

        // Collect all solid rects (static + push blocks + elevators + active crumbling platforms)
        let mut all_solids = static_solids;
        for b in &self.level.push_blocks {
            all_solids.push(Rect { x: b.x, y: b.y, w: b.w, h: b.h });
        }
        for e in &self.level.elevators {
            all_solids.push(Rect { x: e.x, y: e.y, w: e.w, h: e.h });
        }
        for c in &self.level.crumbles {
            if c.state != 2 {
                all_solids.push(Rect { x: c.x, y: c.y, w: c.w, h: c.h });
            }
        }

        // 5. Step players (horizontal + vertical + stacking!)
        // First apply carry velocity from players standing on other players' heads
        let prev_positions: Vec<(f32, f32, f32, bool)> = self
            .players
            .iter()
            .map(|p| (p.x, p.y, p.vx, p.connected && !p.in_door))
            .collect();

        for i in 0..self.players.len() {
            if !self.players[i].connected || self.players[i].in_door {
                continue;
            }
            let p = &mut self.players[i];
            let mut dir = 0.0;
            if p.input.left {
                dir -= 1.0;
                p.facing = -1;
            }
            if p.input.right {
                dir += 1.0;
                p.facing = 1;
            }
            let has_rope = self.level.rope_max_dist.is_some();
            if has_rope && !p.on_ground {
                // Preserve pendulum swing momentum in mid-air while still allowing responsive air pumping!
                p.vx *= 0.992;
                if dir != 0.0 {
                    p.vx = (p.vx + dir * 720.0 * dt).clamp(-560.0, 560.0);
                }
            } else {
                p.vx = dir * MOVE_SPEED;
            }

            // Carry horizontal movement if standing on a teammate's head
            let mut carry_vx = 0.0;
            for (j, &(ox, oy, ovx, oactive)) in prev_positions.iter().enumerate() {
                if i != j && oactive {
                    let on_head = p.x + PLAYER_SIZE > ox + 3.0
                        && p.x < ox + PLAYER_SIZE - 3.0
                        && (p.y + PLAYER_SIZE - oy).abs() <= 4.5;
                    if on_head {
                        carry_vx = ovx;
                        break;
                    }
                }
            }

            // Jump buffer & coyote time
            if p.input.jump && !p.prev_jump {
                p.jump_buffer = 0.11;
            } else {
                p.jump_buffer = (p.jump_buffer - dt).max(0.0);
            }
            p.prev_jump = p.input.jump;

            if p.on_ground {
                p.coyote_timer = 0.09;
            } else {
                p.coyote_timer = (p.coyote_timer - dt).max(0.0);
            }

            if p.jump_buffer > 0.0 && p.coyote_timer > 0.0 {
                p.vy = JUMP_VEL;
                p.on_ground = false;
                p.coyote_timer = 0.0;
                p.jump_buffer = 0.0;
                self.sfx_event = Some("jump");
            }

            // Move X
            let total_vx = p.vx + carry_vx;
            p.x += total_vx * dt;
            for s in &all_solids {
                if overlaps(p.x, p.y, PLAYER_SIZE, PLAYER_SIZE, s.x, s.y, s.w, s.h) {
                    if total_vx > 0.0 {
                        p.x = s.x - PLAYER_SIZE;
                    } else if total_vx < 0.0 {
                        p.x = s.x + s.w;
                    }
                }
            }

            // Move Y
            p.vy = (p.vy + GRAVITY * dt).min(MAX_FALL);
            p.y += p.vy * dt;
            p.on_ground = false;
            for s in &all_solids {
                if overlaps(p.x, p.y, PLAYER_SIZE, PLAYER_SIZE, s.x, s.y, s.w, s.h) {
                    if p.vy > 0.0 {
                        p.y = s.y - PLAYER_SIZE;
                        p.vy = 0.0;
                        p.on_ground = true;
                    } else if p.vy < 0.0 {
                        p.y = s.y + s.h;
                        p.vy = 0.0;
                    }
                }
            }
        }

        // 6. Player-to-player solid collision & head stacking (2 passes for stable towers)
        // Re-clamp against ceiling/walls so jumping under a platform never teleports the top player through it!
        for _ in 0..2 {
            let len = self.players.len();
            for i in 0..len {
                for j in (i + 1)..len {
                    if !self.players[i].connected
                        || self.players[i].in_door
                        || !self.players[j].connected
                        || self.players[j].in_door
                    {
                        continue;
                    }
                    let (ax, ay, avy) = (self.players[i].x, self.players[i].y, self.players[i].vy);
                    let (bx, by, bvy) = (self.players[j].x, self.players[j].y, self.players[j].vy);
                    if !overlaps(ax, ay, PLAYER_SIZE, PLAYER_SIZE, bx, by, PLAYER_SIZE, PLAYER_SIZE) {
                        continue;
                    }

                    let ox = (ax + PLAYER_SIZE).min(bx + PLAYER_SIZE) - ax.max(bx);
                    let oy = (ay + PLAYER_SIZE).min(by + PLAYER_SIZE) - ay.max(by);

                    if oy <= ox + 4.0 {
                        // Determine which player is on top and which is on bottom
                        let (top_idx, bot_idx) = if ay < by { (i, j) } else { (j, i) };
                        let top_vy = if ay < by { avy } else { bvy };

                        if top_vy >= -10.0 {
                            let target_top_y = self.players[bot_idx].y - PLAYER_SIZE;
                            let top_x = self.players[top_idx].x;

                            // Check if pushing top player up would shove them into a ceiling solid
                            let mut ceiling_bottom: Option<f32> = None;
                            for s in &all_solids {
                                if overlaps(top_x, target_top_y, PLAYER_SIZE, PLAYER_SIZE, s.x, s.y, s.w, s.h) {
                                    let sb = s.y + s.h;
                                    ceiling_bottom = Some(match ceiling_bottom {
                                        Some(cur) => cur.max(sb),
                                        None => sb,
                                    });
                                }
                            }

                            if let Some(cb) = ceiling_bottom {
                                // Bonk! Keep top player under the ceiling and stop bottom player's upward jump
                                self.players[top_idx].y = cb;
                                self.players[top_idx].vy = self.players[top_idx].vy.max(0.0);
                                self.players[bot_idx].y = cb + PLAYER_SIZE;
                                self.players[bot_idx].vy = self.players[bot_idx].vy.max(0.0);
                            } else {
                                self.players[top_idx].y = target_top_y;
                                self.players[top_idx].vy = 0.0;
                                self.players[top_idx].on_ground = true;
                            }
                        }
                    } else {
                        // Gentle horizontal separation, clamped against solid walls
                        let push = ox * 0.5;
                        let (left_idx, right_idx) = if ax < bx { (i, j) } else { (j, i) };
                        let new_lx = self.players[left_idx].x - push;
                        let new_rx = self.players[right_idx].x + push;
                        let ly = self.players[left_idx].y;
                        let ry = self.players[right_idx].y;

                        let left_blocked = all_solids
                            .iter()
                            .any(|s| overlaps(new_lx, ly, PLAYER_SIZE, PLAYER_SIZE, s.x, s.y, s.w, s.h));
                        let right_blocked = all_solids
                            .iter()
                            .any(|s| overlaps(new_rx, ry, PLAYER_SIZE, PLAYER_SIZE, s.x, s.y, s.w, s.h));

                        if !left_blocked {
                            self.players[left_idx].x = new_lx;
                        }
                        if !right_blocked {
                            self.players[right_idx].x = new_rx;
                        }
                    }
                }
            }
        }

        // 6b. Spring pads!
        for sp in &mut self.level.springs {
            sp.anim = (sp.anim - dt * 4.0).max(0.0);
            for p in &mut self.players {
                if p.connected
                    && !p.in_door
                    && p.vy >= -60.0
                    && overlaps(p.x, p.y, PLAYER_SIZE, PLAYER_SIZE, sp.x, sp.y - 4.0, sp.w, sp.h + 4.0)
                {
                    p.vy = sp.power;
                    p.on_ground = false;
                    sp.anim = 1.0;
                    self.sfx_event = Some("spring");
                }
            }
        }

        // 6b2. Bouncy mid-air bumpers!
        for bm in &mut self.level.bumpers {
            bm.anim = (bm.anim - dt * 5.0).max(0.0);
            for p in &mut self.players {
                if !p.connected || p.in_door {
                    continue;
                }
                let pcx = p.x + PLAYER_SIZE * 0.5;
                let pcy = p.y + PLAYER_SIZE * 0.5;
                let dx = pcx - bm.x;
                let dy = pcy - bm.y;
                let dist = (dx * dx + dy * dy).sqrt();
                let min_d = bm.radius + PLAYER_SIZE * 0.5;
                if dist < min_d && dist > 0.001 {
                    let nx = dx / dist;
                    let ny = dy / dist;
                    // Push out slightly & launch with upward bias so bumpers feel joyful and forgiving!
                    p.x += nx * (min_d - dist + 2.0);
                    p.y += ny * (min_d - dist + 2.0);
                    p.vx = nx * bm.power * 0.72;
                    p.vy = (ny * bm.power).min(-bm.power * 0.78);
                    p.on_ground = false;
                    bm.anim = 1.0;
                    self.sfx_event = Some("spring");
                }
            }
        }

        // 6b3. Wind updrafts!
        for w in &self.level.winds {
            for p in &mut self.players {
                if p.connected
                    && !p.in_door
                    && overlaps(p.x, p.y, PLAYER_SIZE, PLAYER_SIZE, w.x, w.y, w.w, w.h)
                {
                    p.vy = (p.vy + w.force_y * dt).max(-420.0);
                    p.on_ground = false;
                }
            }
        }

        // 6c. Heave-Ho style Pendulum Rope / Tether physics between adjacent connected players
        if let Some(max_d) = self.level.rope_max_dist {
            let active_indices: Vec<usize> = self
                .players
                .iter()
                .enumerate()
                .filter(|(_, p)| p.connected && !p.in_door)
                .map(|(idx, _)| idx)
                .collect();
            if active_indices.len() >= 2 {
                // Allow dangling players to pump their swing with left/right input
                for pair in active_indices.windows(2) {
                    let a = pair[0];
                    let b = pair[1];
                    let dx = self.players[b].x - self.players[a].x;
                    let dy = self.players[b].y - self.players[a].y;
                    let dist = (dx * dx + dy * dy).sqrt();
                    if dist >= max_d * 0.92 && dist > 1.0 {
                        // Tangent vector perpendicular to rope
                        let tx = -dy / dist;
                        let ty = dx / dist;
                        for &(idx, other) in &[(a, b), (b, a)] {
                            if !self.players[idx].on_ground && self.players[idx].y > self.players[other].y + 12.0 {
                                let pump = if self.players[idx].input.right {
                                    1.0
                                } else if self.players[idx].input.left {
                                    -1.0
                                } else {
                                    0.0
                                };
                                if pump != 0.0 {
                                    // Ensure tangent points in the horizontal direction of pump
                                    let sign = if tx * pump >= 0.0 { 1.0 } else { -1.0 };
                                    self.players[idx].vx += tx * sign * 560.0 * dt;
                                    self.players[idx].vy += ty * sign * 380.0 * dt;
                                }
                            }
                        }
                    }
                }

                // Multi-iteration Position-Based Dynamics + Radial Velocity Projection
                for _ in 0..5 {
                    for pair in active_indices.windows(2) {
                        let a = pair[0];
                        let b = pair[1];
                        let ax = self.players[a].x + PLAYER_SIZE * 0.5;
                        let ay = self.players[a].y + PLAYER_SIZE * 0.5;
                        let bx = self.players[b].x + PLAYER_SIZE * 0.5;
                        let by = self.players[b].y + PLAYER_SIZE * 0.5;
                        let dx = bx - ax;
                        let dy = by - ay;
                        let dist = (dx * dx + dy * dy).sqrt();

                        if dist > max_d && dist > 0.001 {
                            let excess = dist - max_d;
                            let nx = dx / dist;
                            let ny = dy / dist;

                            // Grounded player acts as a heavy anchor so the dangling player can swing cleanly!
                            let a_grounded = self.players[a].on_ground;
                            let b_grounded = self.players[b].on_ground;
                            let (wa, wb) = match (a_grounded, b_grounded) {
                                (true, false) => (0.12, 0.88),
                                (false, true) => (0.88, 0.12),
                                _ => (0.5, 0.5),
                            };

                            let move_ax = nx * excess * wa;
                            let move_ay = ny * excess * wa;
                            let move_bx = -nx * excess * wb;
                            let move_by = -ny * excess * wb;

                            // Resolve X and Y separately against solids so players slide smoothly over ledge lips!
                            let try_ax = self.players[a].x + move_ax;
                            if !all_solids.iter().any(|s| overlaps(try_ax, self.players[a].y, PLAYER_SIZE, PLAYER_SIZE, s.x, s.y, s.w, s.h)) {
                                self.players[a].x = try_ax;
                            }
                            let try_ay = self.players[a].y + move_ay;
                            if !all_solids.iter().any(|s| overlaps(self.players[a].x, try_ay, PLAYER_SIZE, PLAYER_SIZE, s.x, s.y, s.w, s.h)) {
                                self.players[a].y = try_ay;
                            }

                            let try_bx = self.players[b].x + move_bx;
                            if !all_solids.iter().any(|s| overlaps(try_bx, self.players[b].y, PLAYER_SIZE, PLAYER_SIZE, s.x, s.y, s.w, s.h)) {
                                self.players[b].x = try_bx;
                            }
                            let try_by = self.players[b].y + move_by;
                            if !all_solids.iter().any(|s| overlaps(self.players[b].x, try_by, PLAYER_SIZE, PLAYER_SIZE, s.x, s.y, s.w, s.h)) {
                                self.players[b].y = try_by;
                            }

                            // Project out ONLY the outward radial velocity -> preserves 100% of tangential swing momentum!
                            let rel_vx = self.players[b].vx - self.players[a].vx;
                            let rel_vy = self.players[b].vy - self.players[a].vy;
                            let rel_vn = rel_vx * nx + rel_vy * ny;
                            if rel_vn > 0.0 {
                                self.players[a].vx += nx * rel_vn * wa;
                                self.players[a].vy += ny * rel_vn * wa;
                                self.players[b].vx -= nx * rel_vn * wb;
                                self.players[b].vy -= ny * rel_vn * wb;
                            }
                        }
                    }
                }
            }
        }

        // 7. Hazards (spikes, saws, or falling off map)
        let mut died_name: Option<String> = None;
        for p in &self.players {
            if !p.connected || p.in_door {
                continue;
            }
            if p.y > WORLD_H + 50.0 {
                died_name = Some(p.name.clone());
                break;
            }
            for sp in &self.level.spikes {
                if overlaps(p.x + 4.0, p.y + 4.0, PLAYER_SIZE - 8.0, PLAYER_SIZE - 4.0, sp.x, sp.y, sp.w, sp.h) {
                    died_name = Some(p.name.clone());
                    break;
                }
            }
            if died_name.is_none() {
                let pcx = p.x + PLAYER_SIZE * 0.5;
                let pcy = p.y + PLAYER_SIZE * 0.5;
                for sw in &self.level.saws {
                    let dist = ((pcx - sw.x).powi(2) + (pcy - sw.y).powi(2)).sqrt();
                    if dist < sw.radius + PLAYER_SIZE * 0.42 {
                        died_name = Some(p.name.clone());
                        break;
                    }
                }
            }
            if died_name.is_some() {
                break;
            }
        }
        if let Some(name) = died_name {
            self.sfx_event = Some("die");
            self.reset_level(Some(&format!("💀 {name} caiu! Tentem de novo!")));
            return;
        }

        // 8. Key pickup & follow
        if !self.key.used {
            if self.key.holder_id.is_none() {
                for p in &self.players {
                    if p.connected
                        && !p.in_door
                        && overlaps(p.x, p.y, PLAYER_SIZE, PLAYER_SIZE, self.key.x - 14.0, self.key.y - 14.0, 28.0, 28.0)
                    {
                        self.key.holder_id = Some(p.id.clone());
                        self.sfx_event = Some("key");
                        break;
                    }
                }
            }
            if let Some(hid) = self.key.holder_id.clone() {
                if let Some(holder) = self.players.iter().find(|p| p.id == hid && p.connected) {
                    let target_x = holder.x + PLAYER_SIZE * 0.5 - (holder.facing as f32) * 32.0;
                    let target_y = holder.y + 8.0;
                    self.key.x += (target_x - self.key.x) * (10.0 * dt).min(1.0);
                    self.key.y += (target_y - self.key.y) * (10.0 * dt).min(1.0);
                } else {
                    self.key.holder_id = None;
                }
            }
        }

        // 9. Door unlock & players entering door
        let d = self.level.door.clone();
        if !self.door_open {
            if let Some(hid) = self.key.holder_id.clone() {
                if let Some(holder) = self.players.iter().find(|p| p.id == hid && p.connected) {
                    if overlaps(holder.x, holder.y, PLAYER_SIZE, PLAYER_SIZE, d.x, d.y, d.w, d.h) {
                        self.door_open = true;
                        self.key.used = true;
                        self.key.holder_id = None;
                        self.sfx_event = Some("unlock");
                    }
                }
            }
        }

        if self.door_open {
            for p in &mut self.players {
                if p.connected
                    && !p.in_door
                    && overlaps(p.x, p.y, PLAYER_SIZE, PLAYER_SIZE, d.x, d.y, d.w, d.h)
                {
                    p.in_door = true;
                    p.vx = 0.0;
                    p.vy = 0.0;
                    self.sfx_event = Some("door");
                }
            }
            let total_connected = self.players.iter().filter(|p| p.connected).count();
            let all_in = total_connected > 0
                && self.players.iter().filter(|p| p.connected).all(|p| p.in_door);
            if all_in {
                self.status = "cleared".to_string();
                self.banner = Some(format!("🎉 {} CONCLUÍDA!", self.level.name));
                self.status_timer = 2.4;
                self.sfx_event = Some("clear");
            }
        }
    }

    fn collect_static_solids(&self) -> Vec<Rect> {
        let mut out = self.level.solids.clone();
        for g in &self.level.gates {
            if g.solid {
                out.push(Rect { x: g.x, y: g.y, w: g.w, h: g.h });
            }
        }
        out
    }

    pub fn snapshot(&self) -> serde_json::Value {
        serde_json::json!({
            "worldW": WORLD_W,
            "worldH": WORLD_H,
            "levelIndex": self.level_index,
            "levelCount": level_count(),
            "level": self.level,
            "players": self.players,
            "key": self.key,
            "doorOpen": self.door_open,
            "status": self.status,
            "banner": self.banner,
            "sfx": self.sfx_event,
        })
    }
}
