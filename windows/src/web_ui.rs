use crate::layout::LayoutSnapshot;
use anyhow::Result;
use axum::{extract::State, http::StatusCode, response::Html, routing::get, Json, Router};
use desklink_config::DeskLinkConfig;
use serde::Deserialize;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::{net::TcpListener, sync::RwLock};

#[derive(Clone)]
pub struct UiState {
    pub layout: Arc<RwLock<LayoutSnapshot>>,
    pub config: Arc<RwLock<DeskLinkConfig>>,
    pub config_path: PathBuf,
}

#[derive(Debug, Deserialize)]
struct LayoutUpdate {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

pub async fn serve(bind: SocketAddr, state: UiState) -> Result<()> {
    let app = Router::new()
        .route("/", get(index))
        .route("/api/layout", get(get_layout).post(update_layout))
        .with_state(state);
    let listener = TcpListener::bind(bind).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn index() -> Html<&'static str> {
    Html(INDEX_HTML)
}

async fn get_layout(State(state): State<UiState>) -> Json<LayoutSnapshot> {
    Json(state.layout.read().await.clone())
}

async fn update_layout(
    State(state): State<UiState>,
    Json(update): Json<LayoutUpdate>,
) -> Result<Json<LayoutSnapshot>, (StatusCode, String)> {
    let snapshot = {
        let mut layout = state.layout.write().await;
        layout.update_remote(update.x, update.y, update.width, update.height);
        layout.clone()
    };
    {
        let mut config = state.config.write().await;
        config.topology.remote_x = Some(snapshot.remote.x);
        config.topology.remote_y = Some(snapshot.remote.y);
        config.topology.remote_width = snapshot.remote.width;
        config.topology.remote_height = snapshot.remote.height;
        config.topology.windows_layout_signature = Some(snapshot.signature.clone());
        config.save(&state.config_path).map_err(internal_error)?;
    }
    Ok(Json(snapshot))
}

fn internal_error(error: anyhow::Error) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
}

const INDEX_HTML: &str = r#"<!doctype html>
<html lang="zh-CN"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width">
<title>DeskLink 屏幕布局</title>
<style>
body{margin:0;background:#0b1020;color:#e5e7eb;font-family:system-ui;padding:24px}.bar{display:flex;gap:16px;align-items:center}.hint{color:#94a3b8}
#stage{position:relative;height:620px;margin-top:20px;border:1px solid #334155;border-radius:12px;background:#111827;overflow:hidden}
.screen{position:absolute;border:2px solid #60a5fa;border-radius:8px;background:#1e3a5f;display:flex;align-items:center;justify-content:center;box-sizing:border-box;user-select:none}
.linux{border-color:#34d399;background:#064e3b;cursor:move}.link{position:absolute;background:#fbbf24;pointer-events:none}.status{margin-left:auto;color:#34d399}
button{background:#2563eb;color:white;border:0;border-radius:6px;padding:8px 14px;cursor:pointer}</style></head>
<body><div class="bar"><h2>DeskLink 屏幕拼图</h2><span class="hint">拖动绿色 Linux 屏幕，使边缘贴住任意 Windows 屏幕</span><span id="status" class="status"></span><button onclick="load()">重新读取</button></div>
<div id="stage"></div>
<script>
let data, scale=1, originX=0, originY=0, drag=null;
async function load(){data=await (await fetch('/api/layout')).json();render()}
function render(){const s=document.querySelector('#stage');s.innerHTML='';const all=[...data.windows,data.remote];let minX=Math.min(...all.map(x=>x.x))-300,minY=Math.min(...all.map(x=>x.y))-300,maxX=Math.max(...all.map(x=>x.x+x.width))+300,maxY=Math.max(...all.map(x=>x.y+x.height))+300;scale=Math.min(s.clientWidth/(maxX-minX),s.clientHeight/(maxY-minY));originX=minX;originY=minY;data.windows.forEach(x=>screen(s,x,false));screen(s,data.remote,true)}
function screen(stage,x,isLinux){const e=document.createElement('div');e.className='screen '+(isLinux?'linux':'');e.textContent=isLinux?'Linux':x.id;e.style.left=(x.x-originX)*scale+'px';e.style.top=(x.y-originY)*scale+'px';e.style.width=Math.max(70,x.width*scale)+'px';e.style.height=Math.max(45,x.height*scale)+'px';stage.appendChild(e);if(isLinux)e.onpointerdown=ev=>{e.setPointerCapture(ev.pointerId);drag={sx:ev.clientX,sy:ev.clientY,x:x.x,y:x.y};e.onpointermove=m=>{if(!drag)return;e.style.left=(drag.x+(m.clientX-drag.sx)/scale-originX)*scale+'px';e.style.top=(drag.y+(m.clientY-drag.sy)/scale-originY)*scale+'px'};e.onpointerup=async m=>{const nx=Math.round(drag.x+(m.clientX-drag.sx)/scale),ny=Math.round(drag.y+(m.clientY-drag.sy)/scale);drag=null;document.querySelector('#status').textContent='保存中…';data=await (await fetch('/api/layout',{method:'POST',headers:{'content-type':'application/json'},body:JSON.stringify({x:nx,y:ny,width:x.width,height:x.height})})).json();document.querySelector('#status').textContent='已保存并生效';render()}}}
addEventListener('resize',()=>data&&render());load();
</script></body></html>"#;
