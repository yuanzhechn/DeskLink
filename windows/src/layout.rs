use desklink_config::TopologyConfig;
use desklink_protocol::ScreenEdge;
use desklink_topology::{coordinate_ratio, links_between, ratio_coordinate, EdgeLink, ScreenRect};
use serde::Serialize;
use windows_sys::Win32::{
    Foundation::{BOOL, LPARAM, RECT},
    Graphics::Gdi::{EnumDisplayMonitors, HDC, HMONITOR},
};

#[derive(Debug, Clone, Serialize)]
pub struct LayoutSnapshot {
    pub windows: Vec<ScreenRect>,
    pub remote: ScreenRect,
    pub links: Vec<EdgeLink>,
    pub signature: String,
}

unsafe extern "system" fn monitor_callback(
    _monitor: HMONITOR,
    _hdc: HDC,
    rect: *mut RECT,
    data: LPARAM,
) -> BOOL {
    if rect.is_null() || data == 0 {
        return 0;
    }
    let rectangles = &mut *(data as *mut Vec<RECT>);
    rectangles.push(*rect);
    1
}

pub fn discover_windows_screens() -> Vec<ScreenRect> {
    let mut rectangles = Vec::<RECT>::new();
    unsafe {
        EnumDisplayMonitors(
            0,
            std::ptr::null(),
            Some(monitor_callback),
            &mut rectangles as *mut _ as LPARAM,
        );
    }
    rectangles.sort_by_key(|rect| (rect.top, rect.left, rect.bottom, rect.right));
    rectangles
        .into_iter()
        .enumerate()
        .map(|(index, rect)| ScreenRect {
            id: format!("win-{index}"),
            x: rect.left,
            y: rect.top,
            width: (rect.right - rect.left).max(1) as u32,
            height: (rect.bottom - rect.top).max(1) as u32,
        })
        .collect()
}

pub fn layout_signature(screens: &[ScreenRect]) -> String {
    screens
        .iter()
        .map(|screen| {
            format!(
                "{}:{}:{}:{}",
                screen.x, screen.y, screen.width, screen.height
            )
        })
        .collect::<Vec<_>>()
        .join("|")
}

impl LayoutSnapshot {
    pub fn from_config(config: &mut TopologyConfig) -> (Self, bool) {
        let windows = discover_windows_screens();
        let signature = layout_signature(&windows);
        let changed = config
            .windows_layout_signature
            .as_deref()
            .is_some_and(|old| old != signature);
        if changed {
            config.remote_x = None;
            config.remote_y = None;
        }
        config.windows_layout_signature = Some(signature.clone());
        let (default_x, default_y) = default_remote_position(&windows, config);
        let remote = ScreenRect {
            id: "linux-main".to_owned(),
            x: config.remote_x.unwrap_or(default_x),
            y: config.remote_y.unwrap_or(default_y),
            width: config.remote_width.max(1),
            height: config.remote_height.max(1),
        };
        config.remote_x = Some(remote.x);
        config.remote_y = Some(remote.y);
        let mut snapshot = Self {
            windows,
            remote,
            links: Vec::new(),
            signature,
        };
        snapshot.rebuild_links();
        (snapshot, changed)
    }

    pub fn update_remote(&mut self, x: i32, y: i32, width: u32, height: u32) {
        let width = width.max(1);
        let height = height.max(1);
        let x_candidates = self
            .windows
            .iter()
            .flat_map(|screen| [screen.x - width as i32, screen.right()]);
        let y_candidates = self
            .windows
            .iter()
            .flat_map(|screen| [screen.y - height as i32, screen.bottom()]);
        self.remote.x = nearest_snap(x, x_candidates, 80);
        self.remote.y = nearest_snap(y, y_candidates, 80);
        self.remote.width = width;
        self.remote.height = height;
        self.rebuild_links();
    }

    pub fn entry_at(&self, x: i32, y: i32, margin: i32) -> Option<(ScreenEdge, f32)> {
        for link in &self.links {
            let screen = self
                .windows
                .iter()
                .find(|screen| screen.id == link.from_id)?;
            let coordinate = match link.from_edge {
                ScreenEdge::Left | ScreenEdge::Right => y,
                ScreenEdge::Top | ScreenEdge::Bottom => x,
            };
            if coordinate < link.overlap_start || coordinate >= link.overlap_end {
                continue;
            }
            let margin = margin.max(0);
            let at_edge = match link.from_edge {
                ScreenEdge::Left => x <= screen.x + margin,
                ScreenEdge::Right => x >= screen.right() - 1 - margin,
                ScreenEdge::Top => y <= screen.y + margin,
                ScreenEdge::Bottom => y >= screen.bottom() - 1 - margin,
            };
            if at_edge {
                return Some((
                    link.to_edge,
                    coordinate_ratio(&self.remote, link.to_edge, coordinate),
                ));
            }
        }
        None
    }

    pub fn return_point(&self, remote_edge: ScreenEdge, ratio: f32) -> Option<(i32, i32)> {
        let coordinate = ratio_coordinate(&self.remote, remote_edge, ratio);
        let link = self.links.iter().find(|link| {
            link.to_edge == remote_edge
                && coordinate >= link.overlap_start
                && coordinate < link.overlap_end
        })?;
        let screen = self
            .windows
            .iter()
            .find(|screen| screen.id == link.from_id)?;
        Some(match link.from_edge {
            ScreenEdge::Left => (screen.x + 2, coordinate),
            ScreenEdge::Right => (screen.right() - 3, coordinate),
            ScreenEdge::Top => (coordinate, screen.y + 2),
            ScreenEdge::Bottom => (coordinate, screen.bottom() - 3),
        })
    }

    pub fn default_entry(&self) -> Option<(ScreenEdge, f32)> {
        let link = self.links.first()?;
        let coordinate = link.overlap_start + (link.overlap_end - link.overlap_start) / 2;
        Some((
            link.to_edge,
            coordinate_ratio(&self.remote, link.to_edge, coordinate),
        ))
    }

    fn rebuild_links(&mut self) {
        self.links = self
            .windows
            .iter()
            .flat_map(|screen| links_between(screen, &self.remote))
            .collect();
    }
}

fn nearest_snap(value: i32, candidates: impl Iterator<Item = i32>, threshold: i32) -> i32 {
    candidates
        .map(|candidate| (candidate, (candidate - value).abs()))
        .filter(|(_, distance)| *distance <= threshold)
        .min_by_key(|(_, distance)| *distance)
        .map(|(candidate, _)| candidate)
        .unwrap_or(value)
}

fn default_remote_position(screens: &[ScreenRect], config: &TopologyConfig) -> (i32, i32) {
    let left = screens.iter().map(|screen| screen.x).min().unwrap_or(0);
    let top = screens.iter().map(|screen| screen.y).min().unwrap_or(0);
    let right = screens.iter().map(ScreenRect::right).max().unwrap_or(1920);
    let bottom = screens.iter().map(ScreenRect::bottom).max().unwrap_or(1080);
    match config.edge.to_ascii_lowercase().as_str() {
        "left" => (left - config.remote_width as i32, top),
        "top" => (left, top - config.remote_height as i32),
        "bottom" => (left, bottom),
        _ => (right, top),
    }
}
