use super::{crumble, r, saw, spring, GateDef, LevelData, SeqButtonDef};

/// Fase 10: Showdown Mundo 2 — Corda + Relógio 1→2→3 + Chão Quebradiço + Mola + Serra!
pub fn build(n: usize) -> LevelData {
    let timeout = if n >= 2 { 2.4 } else { 0.0 };
    let mut lvl = LevelData::base(
        9,
        "Fase 10: Showdown Mundo 2",
        "SHOWDOWN! Amarrados pela corda: chão quebradiço, mola, serra e botões 1→2→3 sincronizados!",
        n,
        (55.0, 540.0),
        (690.0, 238.0),
        r(1150.0, 386.0, 48.0, 64.0),
    );
    lvl.seq_step_timeout = timeout;
    lvl.rope_max_dist = if n >= 2 { Some(180.0) } else { None };
    lvl.solids = vec![
        r(0.0, 0.0, 28.0, 720.0),
        r(1252.0, 0.0, 28.0, 720.0),
        // Left start floor
        r(0.0, 620.0, 310.0, 100.0),
        // Upper left platform (button 1)
        r(120.0, 405.0, 200.0, 26.0),
        // Center safe island (button 2 + spring)
        r(520.0, 560.0, 220.0, 28.0),
        // Upper center (button 3 + key!)
        r(530.0, 290.0, 220.0, 28.0),
        // Right exit island
        r(960.0, 450.0, 320.0, 270.0),
        // Ceiling over exit gate
        r(1010.0, 290.0, 242.0, 26.0),
        // Pit floor
        r(310.0, 692.0, 650.0, 28.0),
    ];
    lvl.spikes = vec![r(310.0, 668.0, 650.0, 24.0)];
    lvl.gates = vec![GateDef {
        button_id: "seq_all".to_string(),
        x: 1020.0,
        y: 316.0,
        w: 26.0,
        h: 134.0,
        color: "#34d399".to_string(),
        is_bridge: false,
        solid: true,
    }];
    lvl.seq_buttons = vec![
        SeqButtonDef { order: 1, x: 190.0, y: 391.0, w: 54.0, h: 14.0, activated: false, prev_touched: false },
        SeqButtonDef { order: 2, x: 560.0, y: 546.0, w: 54.0, h: 14.0, activated: false, prev_touched: false },
        SeqButtonDef { order: 3, x: 575.0, y: 276.0, w: 54.0, h: 14.0, activated: false, prev_touched: false },
    ];
    lvl.crumbles = vec![
        crumble(355.0, 590.0, 125.0),
        crumble(790.0, 505.0, 130.0),
    ];
    lvl.springs = vec![
        spring(225.0, 604.0),
        spring(665.0, 544.0),
    ];
    lvl.saws = vec![saw(355.0, 455.0, 495.0, 455.0, 115.0)];
    lvl
}
