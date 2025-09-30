// WebSocket处理模块占位符
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Extension,
    },
    response::Response,
};
use tracing::{error, info};

use crate::AppState;

/// WebSocket处理器
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    Extension(_app_state): Extension<AppState>,
) -> Response {
    ws.on_upgrade(handle_socket)
}

/// 处理WebSocket连接
async fn handle_socket(mut socket: WebSocket) {
    info!("新的WebSocket连接建立");

    // 发送欢迎消息
    if socket
        .send(Message::Text("欢迎连接到PolyVisor实时数据流".to_string()))
        .await
        .is_err()
    {
        error!("发送欢迎消息失败");
        return;
    }

    // 处理消息循环
    while let Some(msg) = socket.recv().await {
        let msg = match msg {
            Ok(msg) => msg,
            Err(e) => {
                error!("WebSocket消息接收错误: {}", e);
                break;
            }
        };

        match msg {
            Message::Text(text) => {
                info!("收到文本消息: {}", text);
                // 回显消息
                if socket
                    .send(Message::Text(format!("回声: {}", text)))
                    .await
                    .is_err()
                {
                    error!("发送回声消息失败");
                    break;
                }
            }
            Message::Binary(_) => {
                info!("收到二进制消息");
            }
            Message::Close(_) => {
                info!("WebSocket连接关闭");
                break;
            }
            _ => {}
        }
    }

    info!("WebSocket连接结束");
}