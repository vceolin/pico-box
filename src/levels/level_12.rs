use super::{r, rope_len_for, ButtonDef, GateDef, LevelData};

/// Fase 12: Mergulho no Abismo — A chave e o botão ciano ficam pendurados SOB o precipício!
/// Escala o comprimento da corda para formar uma corrente humana de 2P-8P!
pub fn build(n: usize) -> LevelData {
    let mut lvl = LevelData::base(
        11,
        "Fase 12: Mergulho no Abismo",
        format!("Mergulho ({n}P)! Ancorem lá em cima enquanto a corrente desce para pegar a chave sob o penhasco!"),
        n,
        (55.0, 290.0),
        (560.0, 475.0),
        r(1140.0, 316.0, 48.0, 64.0),
    );
    lvl.rope_max_dist = rope_len_for(n);
    lvl.solids = vec![
        r(0.0, 0.0, 28.0, 720.0),
        r(1252.0, 0.0, 28.0, 720.0),
        // High left cliff
        r(0.0, 370.0, 310.0, 40.0),
        // Mid high overhang (walk on top, dangle underneath to grab key!)
        r(450.0, 370.0, 260.0, 34.0),
        // Tiny catch ledge below overhang for the cyan button
        r(520.0, 520.0, 120.0, 24.0),
        // High right exit cliff + ceiling over gate
        r(880.0, 380.0, 400.0, 340.0),
        r(920.0, 220.0, 332.0, 26.0),
        // Pit floor
        r(0.0, 692.0, 880.0, 28.0),
    ];
    lvl.spikes = vec![r(28.0, 668.0, 852.0, 24.0)];

    if n >= 2 {
        lvl.buttons = vec![ButtonDef {
            id: "abyss_gate".to_string(),
            x: 548.0,
            y: 506.0,
            w: 64.0,
            h: 14.0,
            color: "#22d3ee".to_string(),
            pressed: false,
        }];
        lvl.gates = vec![GateDef {
            button_id: "abyss_gate".to_string(),
            x: 930.0,
            y: 246.0,
            w: 26.0,
            h: 134.0,
            color: "#22d3ee".to_string(),
            is_bridge: false,
            solid: true,
        }];
    } else {
        lvl.solids.push(r(330.0, 480.0, 120.0, 24.0));
        lvl.solids.push(r(720.0, 460.0, 120.0, 24.0));
    }
    lvl
}
