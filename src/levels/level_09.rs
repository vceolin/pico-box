use super::{r, saw, spring, ButtonDef, GateDef, LevelData};

/// Fase 9: Molas & Lâminas — Cooperação com molas e portões!
/// Alguém segura o botão rosa embaixo para abrir o teto da mola enquanto o amigo voa desviando da serra!
pub fn build(n: usize) -> LevelData {
    let mut lvl = LevelData::base(
        8,
        "Fase 9: Molas & Lâminas",
        "Segure o botão rosa para liberar o salto da mola! Depois abra a ponte amarela para quem ficou!",
        n,
        (65.0, 560.0),
        (805.0, 190.0),
        r(1135.0, 356.0, 48.0, 64.0),
    );
    lvl.solids = vec![
        r(0.0, 640.0, 1280.0, 80.0),
        r(0.0, 0.0, 28.0, 720.0),
        r(1252.0, 0.0, 28.0, 720.0),
        // Mid high platform (reached by left spring)
        r(360.0, 430.0, 260.0, 28.0),
        // Top key platform (reached by 2nd spring!)
        r(700.0, 240.0, 210.0, 28.0),
        // Right exit ledge
        r(980.0, 420.0, 272.0, 28.0),
    ];
    lvl.springs = vec![
        spring(245.0, 624.0),
        spring(545.0, 414.0),
        spring(880.0, 624.0),
    ];
    lvl.saws = vec![
        saw(390.0, 398.0, 515.0, 398.0, 130.0),
        saw(440.0, 608.0, 795.0, 608.0, 175.0),
    ];
    if n >= 2 {
        lvl.buttons = vec![
            // Bottom-left button opens the hatch above the 1st spring!
            ButtonDef {
                id: "spring_hatch".to_string(),
                x: 110.0,
                y: 626.0,
                w: 60.0,
                h: 14.0,
                color: "#f472b6".to_string(),
                pressed: false,
            },
            // Upper-mid button creates a safe bridge over the bottom saw for the teammate!
            ButtonDef {
                id: "safe_bridge".to_string(),
                x: 410.0,
                y: 416.0,
                w: 60.0,
                h: 14.0,
                color: "#facc15".to_string(),
                pressed: false,
            },
        ];
        lvl.gates = vec![
            GateDef {
                button_id: "spring_hatch".to_string(),
                x: 215.0,
                y: 470.0,
                w: 110.0,
                h: 24.0,
                color: "#f472b6".to_string(),
                is_bridge: false,
                solid: true,
            },
            GateDef {
                button_id: "safe_bridge".to_string(),
                x: 620.0,
                y: 425.0,
                w: 360.0,
                h: 22.0,
                color: "#facc15".to_string(),
                is_bridge: true,
                solid: false,
            },
        ];
    }
    lvl
}
