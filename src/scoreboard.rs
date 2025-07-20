use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Scoreboard {
    pub total_removed: usize, // 已消除宝石数
    pub score: usize,         // 当前得分
}