use super::{r, LevelData};

/// Fase 11: Balanço em Dupla (Inspirado em Heave Ho!)
/// Um jogador serve de âncora na beirada enquanto o outro se balança no pêndulo da corda
/// para pegar a chave suspensa no abismo e se lançar para a outra plataforma!
pub fn build(n: usize) -> LevelData {
    let mut lvl = LevelData::base(
        10,
        "Fase 11: Balanço em Dupla",
        "Heave Ho! Um segura na beirada enquanto o outro se balança na corda (← / →) para pegar a chave e atravessar!",
        n,
        (75.0, 340.0),
        (410.0, 515.0),
        r(1125.0, 376.0, 48.0, 64.0),
    );
    lvl.rope_max_dist = if n >= 2 { Some(225.0) } else { None };
    lvl.solids = vec![
        r(0.0, 0.0, 28.0, 720.0),
        r(1252.0, 0.0, 28.0, 720.0),
        // High left anchor cliff
        r(0.0, 420.0, 275.0, 300.0),
        // Mid-chasm island (landing & second anchor!)
        r(535.0, 450.0, 170.0, 30.0),
        // High right exit cliff
        r(920.0, 440.0, 360.0, 280.0),
        // Pit floor
        r(275.0, 692.0, 645.0, 28.0),
    ];
    lvl.spikes = vec![r(275.0, 668.0, 645.0, 24.0)];
    if n == 1 {
        // Solo stepping stones so 1P testing still works
        lvl.solids.push(r(350.0, 540.0, 120.0, 24.0));
        lvl.solids.push(r(760.0, 460.0, 110.0, 24.0));
    }
    lvl
}
