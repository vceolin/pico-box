use super::{r, rope_len_for, LevelData};

/// Fase 11: Balanço em Dupla (Inspirado em Heave Ho!)
/// Escala o comprimento da corda e a largura da ilha central para 2P-8P!
pub fn build(n: usize) -> LevelData {
    let mid_w = (170.0 + (n.saturating_sub(2) as f32) * 18.0).min(240.0);
    let key_y = (495.0 + (n.saturating_sub(2) as f32) * 14.0).min(565.0);
    let mut lvl = LevelData::base(
        10,
        "Fase 11: Balanço em Dupla",
        format!("Heave Ho ({n}P)! Ancorem na beirada enquanto a corrente se balança (← / →) até a chave!"),
        n,
        (55.0, 340.0),
        (385.0, key_y),
        r(1125.0, 376.0, 48.0, 64.0),
    );
    lvl.rope_max_dist = rope_len_for(n);
    lvl.solids = vec![
        r(0.0, 0.0, 28.0, 720.0),
        r(1252.0, 0.0, 28.0, 720.0),
        // High left anchor cliff
        r(0.0, 420.0, 275.0, 300.0),
        // Mid-chasm island (scales wider for larger groups)
        r(535.0, 450.0, mid_w, 30.0),
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
