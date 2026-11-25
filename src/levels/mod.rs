mod level_01;
mod level_02;
mod level_03;
mod level_04;
mod level_05;
mod level_06;
mod level_07;
mod level_08;
mod level_09;
mod level_10;
mod level_11;

use serde::Serialize;

pub const WORLD_W: f32 = 1280.0;
pub const WORLD_H: f32 = 720.0;

#[derive(Debug, Clone, Serialize)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Debug, Clone, Serialize)]
pub struct ButtonDef {
    pub id: String,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: String,
    pub pressed: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct GateDef {
    pub button_id: String,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: String,
    pub is_bridge: bool,
    pub solid: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct PushBlockDef {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub vx: f32,
    pub vy: f32,
    pub required: usize,
    pub current_pushers: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct ElevatorDef {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub start_y: f32,
    pub target_y: f32,
    pub required: usize,
    pub riders: usize,
    pub dy: f32,
}

#[derive(Debug, Clone, Serialize)]
pub struct SeqButtonDef {
    pub order: usize,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub activated: bool,
    #[serde(skip)]
    pub prev_touched: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct CrumbleDef {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    /// 0 = solid, 1 = shaking, 2 = gone
    pub state: u8,
    pub timer: f32,
}

#[derive(Debug, Clone, Serialize)]
pub struct SpringDef {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub power: f32,
    pub anim: f32,
}

#[derive(Debug, Clone, Serialize)]
pub struct SawDef {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub speed: f32,
    pub t: f32,
    pub dir: f32,
}

#[derive(Debug, Clone, Serialize)]
pub struct LevelData {
    pub index: usize,
    pub name: String,
    pub hint: String,
    pub adapted_for: usize,
    pub spawn: (f32, f32),
    pub solids: Vec<Rect>,
    pub spikes: Vec<Rect>,
    pub buttons: Vec<ButtonDef>,
    pub gates: Vec<GateDef>,
    pub push_blocks: Vec<PushBlockDef>,
    pub elevators: Vec<ElevatorDef>,
    pub seq_buttons: Vec<SeqButtonDef>,
    /// If > 0, once button 1 is pressed, players have this many seconds per step before the sequence resets!
    pub seq_step_timeout: f32,
    pub seq_timer: f32,
    pub crumbles: Vec<CrumbleDef>,
    pub springs: Vec<SpringDef>,
    pub saws: Vec<SawDef>,
    pub rope_max_dist: Option<f32>,
    pub key_pos: (f32, f32),
    pub door: Rect,
}

impl LevelData {
    pub fn base(
        index: usize,
        name: impl Into<String>,
        hint: impl Into<String>,
        adapted_for: usize,
        spawn: (f32, f32),
        key_pos: (f32, f32),
        door: Rect,
    ) -> Self {
        Self {
            index,
            name: name.into(),
            hint: hint.into(),
            adapted_for,
            spawn,
            solids: vec![],
            spikes: vec![],
            buttons: vec![],
            gates: vec![],
            push_blocks: vec![],
            elevators: vec![],
            seq_buttons: vec![],
            seq_step_timeout: 0.0,
            seq_timer: 0.0,
            crumbles: vec![],
            springs: vec![],
            saws: vec![],
            rope_max_dist: None,
            key_pos,
            door,
        }
    }
}

pub fn r(x: f32, y: f32, w: f32, h: f32) -> Rect {
    Rect { x, y, w, h }
}

pub fn crumble(x: f32, y: f32, w: f32) -> CrumbleDef {
    CrumbleDef {
        x,
        y,
        w,
        h: 24.0,
        state: 0,
        timer: 0.0,
    }
}

pub fn spring(x: f32, y: f32) -> SpringDef {
    SpringDef {
        x,
        y,
        w: 48.0,
        h: 16.0,
        power: -790.0,
        anim: 0.0,
    }
}

pub fn saw(x1: f32, y1: f32, x2: f32, y2: f32, speed: f32) -> SawDef {
    SawDef {
        x: x1,
        y: y1,
        radius: 22.0,
        x1,
        y1,
        x2,
        y2,
        speed,
        t: 0.0,
        dir: 1.0,
    }
}

const LEVELS: &[fn(usize) -> LevelData] = &[
    level_01::build,
    level_02::build,
    level_03::build,
    level_04::build,
    level_05::build,
    level_06::build,
    level_07::build,
    level_08::build,
    level_09::build,
    level_10::build,
    level_11::build,
];

pub fn level_count() -> usize {
    LEVELS.len()
}

pub fn build_level(index: usize, player_count: usize) -> LevelData {
    let n = player_count.clamp(1, 8);
    LEVELS[index % LEVELS.len()](n)
}
