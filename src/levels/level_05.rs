use super::{r, ButtonDef, ElevatorDef, GateDef, LevelData, PushBlockDef};

/// Fase 5: Showdown Mundo 1 — Bloco pesado + botão + elevador + torre humana!
pub fn build(n: usize) -> LevelData {
    let coop_n = n.max(2);
    let block_req = coop_n.clamp(2, 4).min(n);
    let elev_req = n.clamp(1, 6);
    let elev_w = (155.0 + (elev_req as f32) * 16.0).min(210.0);
    let stack = n.clamp(1, 3);
    let key_y = (465.0 - 92.0 - (stack.saturating_sub(1) as f32) * 34.0).max(290.0);

    let mut lvl = LevelData::base(
        4,
        "Fase 5: Showdown Mundo 1",
        format!("Adaptada para {n} jogador(es): bloco ({block_req}), elevador ({elev_req}) e torre!"),
        n,
        (50.0, 540.0),
        (180.0, key_y - 52.0),
        r(1145.0, 246.0, 48.0, 64.0),
    );
    lvl.solids = vec![
        r(0.0, 0.0, 28.0, 720.0),
        r(1252.0, 0.0, 28.0, 720.0),
        r(0.0, 620.0, 520.0, 100.0),
        r(520.0, 620.0, 260.0, 100.0),
        r(995.0, 310.0, 285.0, 34.0),
        r(350.0, 465.0, 150.0, 26.0),
        r(90.0, key_y, 220.0, 28.0),
        r(780.0, 690.0, 500.0, 30.0),
    ];
    lvl.spikes = vec![r(780.0, 666.0, 472.0, 24.0)];
    lvl.buttons = vec![ButtonDef {
        id: "gate_final".to_string(),
        x: 610.0,
        y: 606.0,
        w: 64.0,
        h: 14.0,
        color: "#facc15".to_string(),
        pressed: false,
    }];
    lvl.gates = vec![GateDef {
        button_id: "gate_final".to_string(),
        x: 1055.0,
        y: 180.0,
        w: 26.0,
        h: 130.0,
        color: "#facc15".to_string(),
        is_bridge: false,
        solid: true,
    }];
    lvl.push_blocks = vec![PushBlockDef {
        x: 280.0,
        y: 540.0,
        w: 80.0,
        h: 80.0,
        vx: 0.0,
        vy: 0.0,
        required: block_req,
        current_pushers: 0,
    }];
    lvl.elevators = vec![ElevatorDef {
        x: 785.0,
        y: 600.0,
        w: elev_w,
        h: 26.0,
        start_y: 600.0,
        target_y: 310.0,
        required: elev_req,
        riders: 0,
        dy: 0.0,
    }];
    lvl
}
