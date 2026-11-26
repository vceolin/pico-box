use super::{bumper, r, rope_len_for, LevelData};

/// Fase 13: Quique & Impulso — Bumpers flutuantes no meio do abismo, adaptado para 2P-8P!
pub fn build(n: usize) -> LevelData {
    let mid_w = (150.0 + (n.saturating_sub(2) as f32) * 18.0).min(230.0);
    let mut lvl = LevelData::base(
        12,
        "Fase 13: Quique & Impulso",
        format!("Balanço + Bumpers ({n}P)! Quique nos bumpers roxos no ar para voar até a chave e puxar a equipe!"),
        n,
        (50.0, 440.0),
        (640.0, 220.0),
        r(1135.0, 436.0, 48.0, 64.0),
    );
    lvl.rope_max_dist = rope_len_for(n);
    lvl.solids = vec![
        r(0.0, 0.0, 28.0, 720.0),
        r(1252.0, 0.0, 28.0, 720.0),
        // Left launch platform
        r(0.0, 520.0, 240.0, 200.0),
        // Mid rest platform (widens for larger groups)
        r(565.0 - (mid_w - 150.0) * 0.5, 510.0, mid_w, 28.0),
        // Right exit platform
        r(1010.0, 500.0, 270.0, 220.0),
        // Pit floor
        r(240.0, 692.0, 770.0, 28.0),
    ];
    lvl.spikes = vec![r(240.0, 668.0, 770.0, 24.0)];
    lvl.bumpers = vec![
        bumper(395.0, 485.0, 690.0),
        bumper(640.0, 365.0, 660.0),
        bumper(870.0, 470.0, 690.0),
    ];
    lvl
}
