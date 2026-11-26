use super::{bumper, r, saw, wind, ButtonDef, GateDef, LevelData};

/// Fase 15: Showdown Mundo 3 — Heave Ho Final!
/// Corda + Corrente de Ar + Bumper Flutuante + Botão sob Penhasco + Serra Giratória!
pub fn build(n: usize) -> LevelData {
    let mut lvl = LevelData::base(
        14,
        "Fase 15: Showdown Mundo 3",
        "SHOWDOWN HEAVE HO! Corda, vento, bumper, serra e mergulho no botão — tudo junto!",
        n,
        (60.0, 480.0),
        (650.0, 185.0),
        r(1150.0, 356.0, 48.0, 64.0),
    );
    lvl.rope_max_dist = if n >= 2 { Some(235.0) } else { None };
    lvl.solids = vec![
        r(0.0, 0.0, 28.0, 720.0),
        r(1252.0, 0.0, 28.0, 720.0),
        // Left start cliff
        r(0.0, 560.0, 220.0, 160.0),
        // Mid anchor island
        r(545.0, 410.0, 180.0, 30.0),
        // Lower secret button ledge underneath mid island
        r(595.0, 575.0, 90.0, 22.0),
        // Right exit cliff + ceiling over gate
        r(970.0, 420.0, 310.0, 300.0),
        r(1010.0, 260.0, 242.0, 26.0),
        // Pit floor
        r(220.0, 692.0, 750.0, 28.0),
    ];
    lvl.spikes = vec![r(220.0, 668.0, 750.0, 24.0)];
    lvl.winds = vec![wind(250.0, 220.0, 240.0, 440.0, -1820.0)];
    lvl.bumpers = vec![
        bumper(635.0, 295.0, 620.0),
        bumper(845.0, 460.0, 700.0),
    ];
    lvl.saws = vec![saw(755.0, 350.0, 920.0, 350.0, 125.0)];

    if n >= 2 {
        lvl.buttons = vec![ButtonDef {
            id: "w3_final_gate".to_string(),
            x: 610.0,
            y: 561.0,
            w: 60.0,
            h: 14.0,
            color: "#f472b6".to_string(),
            pressed: false,
        }];
        lvl.gates = vec![GateDef {
            button_id: "w3_final_gate".to_string(),
            x: 1020.0,
            y: 286.0,
            w: 26.0,
            h: 134.0,
            color: "#f472b6".to_string(),
            is_bridge: false,
            solid: true,
        }];
    }
    lvl
}
