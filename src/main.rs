mod levels;
mod physics;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path as AxumPath, State,
    },
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use futures_util::{SinkExt, StreamExt};
use physics::GameWorld;
use qrcode::{render::svg, QrCode};
use serde::Deserialize;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{broadcast, Mutex};

#[derive(Clone)]
struct AppState {
    world: Arc<Mutex<GameWorld>>,
    frame_tx: broadcast::Sender<String>,
    public_dir: PathBuf,
    port: u16,
    join_url: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ClientMsg {
    WatchTv,
    Join {
        player_id: String,
        name: String,
    },
    Input {
        player_id: Option<String>,
        left: bool,
        right: bool,
        jump: bool,
    },
    Restart,
    SetLevel {
        index: usize,
    },
}

fn find_public_dir() -> PathBuf {
    if let Ok(cwd) = std::env::current_dir() {
        let p = cwd.join("public");
        if p.exists() {
            return p;
        }
        let p2 = cwd.join("apps").join("pico-box").join("public");
        if p2.exists() {
            return p2;
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(root) = exe.parent().and_then(|p| p.parent()).and_then(|p| p.parent()) {
            let p = root.join("apps").join("pico-box").join("public");
            if p.exists() {
                return p;
            }
        }
    }
    PathBuf::from("public")
}

fn detect_lan_ip() -> String {
    if let Ok(ip) = local_ip_address::local_ip() {
        if ip.is_ipv4() && !ip.is_loopback() {
            return ip.to_string();
        }
    }
    "localhost".to_string()
}

fn content_type_for(path: &std::path::Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()).unwrap_or("") {
        "html" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" => "application/javascript; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        _ => "application/octet-stream",
    }
}

async fn serve_file(state: &AppState, name: &str) -> Response {
    let clean = name.trim_start_matches('/');
    if clean.contains("..") {
        return (StatusCode::FORBIDDEN, "Forbidden").into_response();
    }
    let full = state.public_dir.join(clean);
    match tokio::fs::read(&full).await {
        Ok(bytes) => (
            StatusCode::OK,
            [
                (header::CONTENT_TYPE, content_type_for(&full)),
                (header::CACHE_CONTROL, "no-cache"),
            ],
            bytes,
        )
            .into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Not found").into_response(),
    }
}

async fn serve_play(State(state): State<AppState>) -> Response {
    serve_file(&state, "index.html").await
}

async fn serve_tv(State(state): State<AppState>) -> Response {
    serve_file(&state, "index.html").await
}

async fn serve_asset(State(state): State<AppState>, AxumPath(path): AxumPath<String>) -> Response {
    let full = state.public_dir.join(path.trim_start_matches('/'));
    if full.exists() && full.is_file() {
        serve_file(&state, &path).await
    } else {
        serve_file(&state, "index.html").await
    }
}

async fn serve_qr(State(state): State<AppState>) -> Response {
    let code = QrCode::new(state.join_url.as_bytes()).expect("QR generation failed");
    let svg_str = code
        .render::<svg::Color>()
        .min_dimensions(240, 240)
        .dark_color(svg::Color("#1e293b"))
        .light_color(svg::Color("#ffffff"))
        .quiet_zone(true)
        .build();
    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "image/svg+xml"),
            (header::CACHE_CONTROL, "no-cache"),
        ],
        svg_str,
    )
        .into_response()
}

async fn api_info(State(state): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "joinUrl": state.join_url,
        "port": state.port,
    }))
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    let mut frame_rx = state.frame_tx.subscribe();
    let mut is_tv = false;
    let mut bound_player_id: Option<String> = None;

    loop {
        tokio::select! {
            frame = frame_rx.recv(), if is_tv => {
                match frame {
                    Ok(json) => {
                        if sender.send(Message::Text(json.into())).await.is_err() {
                            break;
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(_) => break,
                }
            }
            maybe_msg = receiver.next() => {
                let Some(Ok(msg)) = maybe_msg else { break };
                let Message::Text(text) = msg else { continue };
                let Ok(parsed) = serde_json::from_str::<ClientMsg>(&text) else { continue };

                match parsed {
                    ClientMsg::WatchTv => {
                        is_tv = true;
                        let snap = state.world.lock().await.snapshot().to_string();
                        let _ = sender.send(Message::Text(snap.into())).await;
                    }
                    ClientMsg::Join { player_id, name } => {
                        let pid = player_id.trim().to_string();
                        if !pid.is_empty() {
                            let player = state.world.lock().await.join_player(pid.clone(), name);
                            if !is_tv {
                                bound_player_id = Some(pid);
                            }
                            let ack = serde_json::json!({
                                "type": "joined",
                                "player": player,
                            }).to_string();
                            let _ = sender.send(Message::Text(ack.into())).await;
                        }
                    }
                    ClientMsg::Input { player_id, left, right, jump } => {
                        let target = player_id.or_else(|| bound_player_id.clone());
                        if let Some(pid) = target {
                            state.world.lock().await.set_input(&pid, left, right, jump);
                        }
                    }
                    ClientMsg::Restart => {
                        state.world.lock().await.reset_level(Some("↻ Fase reiniciada!"));
                    }
                    ClientMsg::SetLevel { index } => {
                        state.world.lock().await.load_level(index);
                    }
                }
            }
        }
    }

    if let Some(pid) = bound_player_id {
        state.world.lock().await.set_connected(&pid, false);
    }
}

#[tokio::main]
async fn main() {
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(3103);

    let lan_ip = detect_lan_ip();
    let join_url = format!("http://{lan_ip}:{port}/");
    let public_dir = find_public_dir();

    let world = Arc::new(Mutex::new(GameWorld::new()));
    let (frame_tx, _) = broadcast::channel::<String>(16);

    // 60 Hz physics loop!
    {
        let w = Arc::clone(&world);
        let tx = frame_tx.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_millis(16));
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
            loop {
                interval.tick().await;
                let snap = {
                    let mut guard = w.lock().await;
                    guard.step(1.0 / 60.0);
                    if tx.receiver_count() > 0 {
                        Some(guard.snapshot().to_string())
                    } else {
                        None
                    }
                };
                if let Some(json) = snap {
                    let _ = tx.send(json);
                }
            }
        });
    }

    let state = AppState {
        world,
        frame_tx,
        public_dir,
        port,
        join_url: join_url.clone(),
    };

    let app = Router::new()
        .route("/", get(serve_play))
        .route("/tv", get(serve_tv))
        .route("/qr.svg", get(serve_qr))
        .route("/api/info", get(api_info))
        .route("/ws", get(ws_handler))
        .route("/{*path}", get(serve_asset))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind PicoBox port");

    println!("PicoBox (Rust 60 FPS) running!");
    println!("  TV Screen:  http://localhost:{port}/tv");
    println!("  Phones LAN: {join_url}");

    axum::serve(listener, app).await.expect("PicoBox error");
}
