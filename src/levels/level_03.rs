use super::{r, LevelData, PushBlockDef};

/// Fase 3: Empurra Junto! — Peso do bloco exige todos os jogadores empurrando (até 5).
pub fn build(n: usize) -> LevelData {
    let req = n.clamp(1, 5);
    let block_w = (84.0 + (req.saturating_sub(2) as f32) * 14.0).min(130.0);
    let stack = n.clamp(1, 4);
    let key_y = (460.0 - 92.0 - (stack.saturating_sub(1) as f32) * 34.0).max(220.0);

    let mut lvl = LevelData::base(
        2,
        "Fase 3: Empurra Junto!",
        format!("Precisa de {req} jogador(es) empurrando o bloco azul juntos!"),
        n,
        (55.0, 560.0),
        (1035.0, key_y - 52.0),
        r(1150.0, 396.0, 48.0, 64.0),
    );
    lvl.solids = vec![
        r(0.0, 640.0, 1280.0, 80.0),
        r(0.0, 0.0, 28.0, 720.0),
        r(1252.0, 0.0, 28.0, 720.0),
        r(840.0, 460.0, 440.0, 180.0),
        r(940.0, key_y, 220.0, 26.0),
    ];
    lvl.push_blocks = vec![PushBlockDef {
        x: 360.0,
        y: 556.0,
        w: block_w,
        h: 84.0,
        vx: 0.0,
        vy: 0.0,
        required: req,
        current_pushers: 0,
    }];
    lvl
}
