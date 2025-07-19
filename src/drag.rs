use bevy::prelude::*;

/// 拖拽状态，记录起点和终点
#[derive(Resource, Default)]
pub struct DragState {
    pub start: Option<(usize, usize)>,
    pub end: Option<(usize, usize)>,
    pub is_dragging: bool,
}