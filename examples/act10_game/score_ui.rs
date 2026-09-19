//! 计分 UI —— 创建并实时刷新屏幕左上角的分数文本。

use bevy::prelude::*;

use crate::resources::Score;

/// 分数文本标记组件，用于在 Query 中筛选分数显示实体。
#[derive(Component)]
pub struct ScoreDisplay;

/// 创建分数显示实体：绝对定位在窗口左上角的文本。
///
/// # 参数
/// - `commands`：命令队列，生成的实体在帧末统一应用。
pub fn setup_score_display(mut commands: Commands) {
    commands.spawn((
        Text::new("Score: 0"),                      // 初始文本，之后每帧被刷新覆盖
        TextFont {
            font_size: FontSize::Px(32.0),          // 字号：32 像素
            ..Default::default()
        },
        TextColor(Color::srgb(1.0, 1.0, 1.0)),      // 颜色：白色
        Node {
            position_type: PositionType::Absolute,  // 绝对定位：脱离布局流，随窗口定位
            top: Val::Px(10.0),                     // 距窗口顶部 10 像素
            left: Val::Px(10.0),                    // 距窗口左侧 10 像素
            ..Default::default()
        },
        ScoreDisplay,
    ));
}

/// UI 刷新系统：每帧把资源中的最新分数同步到文本。
///
/// # 参数
/// - `score`：只读访问全局分数资源；
/// - `query`：可变查询分数文本实体（`.0` 为字符串内容）。
pub fn update_score_display(
    score: Res<Score>,
    mut query: Query<&mut Text, With<ScoreDisplay>>,
) {
    for mut text in &mut query {
        text.0 = format!("Score: {}", score.total);
    }
}
