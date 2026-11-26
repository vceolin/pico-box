use super::{bumper, r, LevelData};

/// Fase 13: Quique & Impulso — Bumpers flutuantes no meio do abismo!
/// Balance na corda para quicar nos bumpers roxos, pegar a chave no alto e puxar a equipe!
pub fn build(n: usize) -> LevelData {
    let mut lvl = LevelData::base(
        12,
        "Fase 13: Quique & Impulso",
        "Balanço + Bumpers! Quique nos bumpers roxos no ar para voar até a chave e puxar os amigos!",
        n,
        (70.0, 440.0),
        (640.0, 220.0),
        r(1135.0, 436.0, 48.0, 64.0),
    );
    lvl.rope_max_dist = if n >= 2 { Some(235.0) } else { None };
    lvl.solids = vec![
        r(0.0, 0.0, 28.0, 720.0),
        r(1252.0, 0.0, 28.0, 720.0),
        // Left launch platform
        r(0.0, 520.0, 240.0, 200.0),
        // Small mid rest platform
        r(565.0, 510.0, 150.0, 28.0),
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
