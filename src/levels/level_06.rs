use super::{r, LevelData};

/// Fase 6: A Corda — Todos amarrados por uma corda elástica!
pub fn build(n: usize) -> LevelData {
    let mut lvl = LevelData::base(
        5,
        "Fase 6: A Corda",
        "Vocês estão amarrados juntos! Pulem em sincronia sobre os espinhos!",
        n,
        (60.0, 520.0),
        (795.0, 355.0),
        r(1135.0, 476.0, 48.0, 64.0),
    );
    lvl.solids = vec![
        r(0.0, 0.0, 28.0, 720.0),
        r(1252.0, 0.0, 28.0, 720.0),
        r(0.0, 600.0, 280.0, 120.0),
        r(400.0, 560.0, 190.0, 160.0),
        r(700.0, 510.0, 190.0, 210.0),
        r(995.0, 540.0, 285.0, 180.0),
        r(715.0, 405.0, 160.0, 24.0),
    ];
    lvl.spikes = vec![
        r(280.0, 692.0, 120.0, 28.0),
        r(590.0, 692.0, 110.0, 28.0),
        r(890.0, 692.0, 105.0, 28.0),
    ];
    lvl.rope_max_dist = Some(150.0);
    lvl
}
