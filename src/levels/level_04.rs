use super::{r, ElevatorDef, LevelData};

/// Fase 4: Elevador Sincronizado — Exige todos os jogadores em cima e alarga automaticamente.
pub fn build(n: usize) -> LevelData {
    let req = n.clamp(1, 8);
    let elev_w = (160.0 + (req as f32) * 24.0).min(300.0);
    let stack = n.clamp(1, 3);
    let key_shelf_y = (260.0 - 88.0 - (stack.saturating_sub(1) as f32) * 34.0).max(110.0);

    let mut lvl = LevelData::base(
        3,
        "Fase 4: Elevador",
        format!("Subam os {req} jogadores juntos no elevador!"),
        n,
        (55.0, 540.0),
        (890.0, key_shelf_y - 46.0),
        r(1120.0, 196.0, 48.0, 64.0),
    );
    lvl.solids = vec![
        r(0.0, 0.0, 28.0, 720.0),
        r(1252.0, 0.0, 28.0, 720.0),
        r(0.0, 620.0, 360.0, 100.0),
        r(720.0, 260.0, 560.0, 34.0),
        r(800.0, key_shelf_y, 200.0, 24.0),
        r(360.0, 690.0, 920.0, 30.0),
    ];
    lvl.spikes = vec![r(360.0, 666.0, 892.0, 24.0)];
    lvl.elevators = vec![ElevatorDef {
        x: 390.0,
        y: 600.0,
        w: elev_w,
        h: 26.0,
        start_y: 600.0,
        target_y: 260.0,
        required: req,
        riders: 0,
        dy: 0.0,
    }];
    lvl
}
