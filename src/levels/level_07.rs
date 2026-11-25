use super::{r, GateDef, LevelData, SeqButtonDef};

/// Fase 7: Ordem Sincronizada — Escala o tempo E a quantidade de botões conforme o número de jogadores!
/// - 2P: 4 botões, 3.5s por passo (cada um cuida de um lado)
/// - 3P: 4 botões, 2.5s por passo
/// - 4P: 4 botões, 1.8s por passo
/// - 5P+: 5 botões (ou 6 com 6P+), 1.8s por passo!
pub fn build(n: usize) -> LevelData {
    let timeout = match n {
        1 => 0.0,
        2 => 3.5,
        3 => 2.5,
        _ => 1.8,
    };
    let total_btns = if n >= 6 {
        6
    } else if n >= 5 {
        5
    } else {
        4
    };

    let hint = if n == 1 {
        "Pise nos botões em ordem (1 → 4)! (Com 2+ jogadores, ativa o cronômetro de equipe!)".to_string()
    } else {
        format!("Dividam-se! Vocês têm {timeout:.1}s entre cada número (1 → {total_btns})!")
    };

    let mut lvl = LevelData::base(
        6,
        "Fase 7: Ordem Sincronizada",
        hint,
        n,
        (580.0, 560.0),
        (1080.0, 416.0),
        r(1165.0, 406.0, 48.0, 64.0),
    );
    lvl.seq_step_timeout = timeout;
    lvl.solids = vec![
        r(0.0, 640.0, 1280.0, 80.0),
        r(0.0, 0.0, 28.0, 720.0),
        r(1252.0, 0.0, 28.0, 720.0),
        // Steps to climb left & right upper corners
        r(240.0, 545.0, 130.0, 24.0),
        r(70.0, 450.0, 210.0, 26.0),  // Top-left (Button 3)
        r(530.0, 545.0, 140.0, 24.0), // Mid-center (Button 6 for 6P+)
        r(720.0, 455.0, 210.0, 26.0), // Top-right (Button 4)
        // High center platform (Button 5 for 5P+)
        r(400.0, 360.0, 200.0, 26.0),
        // Exit room (far right, sealed by seq_all gate)
        r(990.0, 470.0, 290.0, 170.0),
        r(990.0, 310.0, 262.0, 26.0),
    ];
    lvl.gates = vec![GateDef {
        button_id: "seq_all".to_string(),
        x: 1000.0,
        y: 336.0,
        w: 26.0,
        h: 134.0,
        color: "#34d399".to_string(),
        is_bridge: false,
        solid: true,
    }];
    let mut seq = vec![
        SeqButtonDef { order: 1, x: 75.0, y: 626.0, w: 54.0, h: 14.0, activated: false, prev_touched: false },
        SeqButtonDef { order: 2, x: 875.0, y: 626.0, w: 54.0, h: 14.0, activated: false, prev_touched: false },
        SeqButtonDef { order: 3, x: 110.0, y: 436.0, w: 54.0, h: 14.0, activated: false, prev_touched: false },
        SeqButtonDef { order: 4, x: 825.0, y: 441.0, w: 54.0, h: 14.0, activated: false, prev_touched: false },
    ];
    if total_btns >= 5 {
        seq.push(SeqButtonDef { order: 5, x: 473.0, y: 346.0, w: 54.0, h: 14.0, activated: false, prev_touched: false });
    }
    if total_btns >= 6 {
        seq.push(SeqButtonDef { order: 6, x: 573.0, y: 531.0, w: 54.0, h: 14.0, activated: false, prev_touched: false });
    }
    lvl.seq_buttons = seq;
    lvl
}
