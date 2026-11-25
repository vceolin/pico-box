use super::{r, LevelData};

/// Fase 1: Torre Humana — Altura da plataforma escala com a quantidade de jogadores (até 5).
pub fn build(n: usize) -> LevelData {
    let stack = n.clamp(1, 5);
    let extra_layers = stack.saturating_sub(1) as f32;
    let key_ledge_y = (550.0 - 90.0 - extra_layers * 35.0).max(260.0);
    let hint = if stack == 1 {
        "Pegue a chave e vá até a porta! (Conecte mais celulares para o modo Torre Humana)".to_string()
    } else {
        format!("Façam uma torre de {stack} jogadores para alcançar a chave!")
    };

    let mut lvl = LevelData::base(
        0,
        "Fase 1: Torre Humana",
        hint,
        n,
        (70.0, 560.0),
        (680.0, key_ledge_y - 52.0),
        r(1120.0, 486.0, 48.0, 64.0),
    );
    lvl.solids = vec![
        r(0.0, 640.0, 1280.0, 80.0),
        r(0.0, 0.0, 28.0, 720.0),
        r(1252.0, 0.0, 28.0, 720.0),
        r(360.0, 550.0, 180.0, 90.0),
        r(580.0, key_ledge_y, 230.0, 32.0),
        r(940.0, 550.0, 312.0, 90.0),
    ];
    lvl
}
