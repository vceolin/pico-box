use super::{r, ButtonDef, GateDef, LevelData};

/// Fase 2: A Ponte — Botões que seguram a ponte + prateleira da chave adaptada ao grupo.
pub fn build(n: usize) -> LevelData {
    let stack = n.clamp(1, 4);
    let extra_layers = stack.saturating_sub(1) as f32;
    let key_shelf_y = (600.0 - 95.0 - extra_layers * 35.0).max(320.0);

    let mut lvl = LevelData::base(
        1,
        "Fase 2: A Ponte",
        format!("Pise no botão para segurar a ponte! Chave adaptada para {stack} jogador(es)."),
        n,
        (55.0, 520.0),
        (705.0, key_shelf_y - 52.0),
        r(1130.0, 496.0, 48.0, 64.0),
    );
    lvl.solids = vec![
        r(0.0, 0.0, 28.0, 720.0),
        r(1252.0, 0.0, 28.0, 720.0),
        r(0.0, 600.0, 370.0, 120.0),
        r(610.0, 600.0, 220.0, 120.0),
        r(960.0, 560.0, 320.0, 160.0),
        r(630.0, key_shelf_y, 180.0, 28.0),
    ];
    lvl.spikes = vec![
        r(370.0, 692.0, 240.0, 28.0),
        r(830.0, 692.0, 130.0, 28.0),
    ];
    lvl.buttons = vec![
        ButtonDef {
            id: "bridge1".to_string(),
            x: 240.0,
            y: 586.0,
            w: 64.0,
            h: 14.0,
            color: "#22d3ee".to_string(),
            pressed: false,
        },
        ButtonDef {
            id: "bridge2".to_string(),
            x: 690.0,
            y: 586.0,
            w: 64.0,
            h: 14.0,
            color: "#f472b6".to_string(),
            pressed: false,
        },
    ];
    lvl.gates = vec![
        GateDef {
            button_id: "bridge1".to_string(),
            x: 370.0,
            y: 600.0,
            w: 240.0,
            h: 26.0,
            color: "#22d3ee".to_string(),
            is_bridge: true,
            solid: false,
        },
        GateDef {
            button_id: "bridge2".to_string(),
            x: 370.0,
            y: 600.0,
            w: 240.0,
            h: 26.0,
            color: "#f472b6".to_string(),
            is_bridge: true,
            solid: false,
        },
    ];
    lvl
}
