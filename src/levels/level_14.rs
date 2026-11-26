use super::{r, rope_len_for, wind, LevelData};

/// Fase 14: Corrente de Ar — Ventanias ascendentes levantam a equipe inteira presa na corda!
pub fn build(n: usize) -> LevelData {
    let mut lvl = LevelData::base(
        13,
        "Fase 14: Corrente de Ar",
        format!("Vento ({n}P)! Pulem juntos na corrente de ar para voar por cima do muro!"),
        n,
        (50.0, 520.0),
        (645.0, 145.0),
        r(1140.0, 466.0, 48.0, 64.0),
    );
    lvl.rope_max_dist = rope_len_for(n);
    lvl.solids = vec![
        r(0.0, 0.0, 28.0, 720.0),
        r(1252.0, 0.0, 28.0, 720.0),
        // Left start platform
        r(0.0, 600.0, 250.0, 120.0),
        // Tall central wall you MUST fly over using wind!
        r(590.0, 310.0, 110.0, 410.0),
        // Safe landing perch on top of the wall
        r(590.0, 310.0, 110.0, 26.0),
        // Right exit platform
        r(990.0, 530.0, 290.0, 190.0),
        // Pit floors
        r(250.0, 692.0, 340.0, 28.0),
        r(700.0, 692.0, 290.0, 28.0),
    ];
    lvl.spikes = vec![
        r(250.0, 668.0, 340.0, 24.0),
        r(700.0, 668.0, 290.0, 24.0),
    ];
    lvl.winds = vec![
        wind(280.0, 190.0, 270.0, 470.0, -1850.0),
        wind(735.0, 210.0, 220.0, 450.0, -1750.0),
    ];
    lvl
}
