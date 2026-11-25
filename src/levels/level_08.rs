use super::{crumble, r, ButtonDef, GateDef, LevelData};

/// Fase 8: Chão Quebradiço — Duas rotas simultâneas!
/// Quem vai por baixo pisa no botão ciano (em cima do chão que desaba!) para abrir o portão da chave em cima;
/// quem vai por cima pisa no botão rosa para criar a ponte segura de volta!
pub fn build(n: usize) -> LevelData {
    let mut lvl = LevelData::base(
        7,
        "Fase 8: Chão Quebradiço",
        "Dividam-se! Baixo abre o portão ciano da chave em cima; cima libera a ponte rosa pro time de baixo!",
        n,
        (60.0, 520.0),
        (805.0, 320.0),
        r(1145.0, 446.0, 48.0, 64.0),
    );
    lvl.solids = vec![
        r(0.0, 0.0, 28.0, 720.0),
        r(1252.0, 0.0, 28.0, 720.0),
        // Start floor (left)
        r(0.0, 600.0, 220.0, 120.0),
        // Upper step to climb to top lane
        r(90.0, 505.0, 120.0, 24.0),
        r(190.0, 415.0, 130.0, 24.0),
        // Upper key room (sealed by cyan gate on left!)
        r(680.0, 375.0, 220.0, 26.0),
        r(680.0, 235.0, 220.0, 24.0),
        // Lower mid island (cyan button)
        r(570.0, 595.0, 130.0, 26.0),
        // Right exit platform
        r(1010.0, 510.0, 270.0, 210.0),
        // Pit floor
        r(220.0, 692.0, 790.0, 28.0),
    ];
    lvl.spikes = vec![r(220.0, 668.0, 790.0, 24.0)];
    lvl.crumbles = vec![
        // Top crumbling lane
        crumble(360.0, 405.0, 125.0),
        crumble(525.0, 390.0, 120.0),
        // Bottom crumbling lane
        crumble(265.0, 595.0, 120.0),
        crumble(420.0, 595.0, 115.0),
        // Final crumble from bottom island toward exit
        crumble(740.0, 560.0, 110.0),
    ];
    if n >= 2 {
        lvl.buttons = vec![
            ButtonDef {
                id: "top_key_gate".to_string(),
                x: 605.0,
                y: 581.0,
                w: 60.0,
                h: 14.0,
                color: "#22d3ee".to_string(),
                pressed: false,
            },
            ButtonDef {
                id: "bot_bridge".to_string(),
                x: 820.0,
                y: 361.0,
                w: 60.0,
                h: 14.0,
                color: "#f472b6".to_string(),
                pressed: false,
            },
        ];
        lvl.gates = vec![
            GateDef {
                button_id: "top_key_gate".to_string(),
                x: 690.0,
                y: 259.0,
                w: 24.0,
                h: 116.0,
                color: "#22d3ee".to_string(),
                is_bridge: false,
                solid: true,
            },
            GateDef {
                button_id: "bot_bridge".to_string(),
                x: 860.0,
                y: 530.0,
                w: 150.0,
                h: 24.0,
                color: "#f472b6".to_string(),
                is_bridge: true,
                solid: false,
            },
        ];
    } else {
        // Solo fallback bridge
        lvl.crumbles.push(crumble(875.0, 530.0, 115.0));
    }
    lvl
}
