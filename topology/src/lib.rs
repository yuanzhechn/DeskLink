use desklink_protocol::ScreenEdge;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScreenRect {
    pub id: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl ScreenRect {
    pub fn right(&self) -> i32 {
        self.x.saturating_add(self.width as i32)
    }
    pub fn bottom(&self) -> i32 {
        self.y.saturating_add(self.height as i32)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EdgeLink {
    pub from_id: String,
    pub to_id: String,
    pub from_edge: ScreenEdge,
    pub to_edge: ScreenEdge,
    pub overlap_start: i32,
    pub overlap_end: i32,
}

pub fn links_between(from: &ScreenRect, to: &ScreenRect) -> Vec<EdgeLink> {
    let mut links = Vec::new();
    if from.right() == to.x {
        push_vertical(&mut links, from, to, ScreenEdge::Right, ScreenEdge::Left);
    }
    if from.x == to.right() {
        push_vertical(&mut links, from, to, ScreenEdge::Left, ScreenEdge::Right);
    }
    if from.bottom() == to.y {
        push_horizontal(&mut links, from, to, ScreenEdge::Bottom, ScreenEdge::Top);
    }
    if from.y == to.bottom() {
        push_horizontal(&mut links, from, to, ScreenEdge::Top, ScreenEdge::Bottom);
    }
    links
}

fn push_vertical(
    links: &mut Vec<EdgeLink>,
    from: &ScreenRect,
    to: &ScreenRect,
    from_edge: ScreenEdge,
    to_edge: ScreenEdge,
) {
    let start = from.y.max(to.y);
    let end = from.bottom().min(to.bottom());
    if start < end {
        links.push(link(from, to, from_edge, to_edge, start, end));
    }
}

fn push_horizontal(
    links: &mut Vec<EdgeLink>,
    from: &ScreenRect,
    to: &ScreenRect,
    from_edge: ScreenEdge,
    to_edge: ScreenEdge,
) {
    let start = from.x.max(to.x);
    let end = from.right().min(to.right());
    if start < end {
        links.push(link(from, to, from_edge, to_edge, start, end));
    }
}

fn link(
    from: &ScreenRect,
    to: &ScreenRect,
    from_edge: ScreenEdge,
    to_edge: ScreenEdge,
    overlap_start: i32,
    overlap_end: i32,
) -> EdgeLink {
    EdgeLink {
        from_id: from.id.clone(),
        to_id: to.id.clone(),
        from_edge,
        to_edge,
        overlap_start,
        overlap_end,
    }
}

pub fn coordinate_ratio(screen: &ScreenRect, edge: ScreenEdge, coordinate: i32) -> f32 {
    let (origin, length) = match edge {
        ScreenEdge::Left | ScreenEdge::Right => (screen.y, screen.height.max(1)),
        ScreenEdge::Top | ScreenEdge::Bottom => (screen.x, screen.width.max(1)),
    };
    ((coordinate - origin) as f32 / length as f32).clamp(0.0, 1.0)
}

pub fn ratio_coordinate(screen: &ScreenRect, edge: ScreenEdge, ratio: f32) -> i32 {
    let (origin, length) = match edge {
        ScreenEdge::Left | ScreenEdge::Right => (screen.y, screen.height.max(1)),
        ScreenEdge::Top | ScreenEdge::Bottom => (screen.x, screen.width.max(1)),
    };
    origin + (length as f32 * ratio.clamp(0.0, 1.0)) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_only_the_real_overlapping_edge() {
        let windows = ScreenRect {
            id: "win".into(),
            x: 0,
            y: 0,
            width: 1920,
            height: 1080,
        };
        let linux = ScreenRect {
            id: "linux".into(),
            x: 1920,
            y: 300,
            width: 2560,
            height: 1440,
        };
        let links = links_between(&windows, &linux);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].from_edge, ScreenEdge::Right);
        assert_eq!((links[0].overlap_start, links[0].overlap_end), (300, 1080));
    }

    #[test]
    fn negative_coordinates_are_supported() {
        let windows = ScreenRect {
            id: "win".into(),
            x: -1920,
            y: 0,
            width: 1920,
            height: 1080,
        };
        let linux = ScreenRect {
            id: "linux".into(),
            x: 0,
            y: 200,
            width: 1920,
            height: 1080,
        };
        assert_eq!(links_between(&windows, &linux)[0].overlap_start, 200);
    }
}
