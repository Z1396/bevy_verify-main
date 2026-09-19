//! 游戏系统集合 —— 场景搭建与全部玩法逻辑。
//!
//! 系统流水线：移动玩家 → 射击 → 生成子弹 → 移动子弹 →
//! 碰撞检测 → 清理越界子弹 → （UI 刷新见 score_ui.rs）。

use bevy::prelude::*;

use crate::components::{Bullet, Enemy, Player};
use crate::messages::BulletFired;
use crate::resources::Score;

/// 场景初始化：创建相机、玩家与敌人实体。
///
/// # 参数
/// - `commands`：命令队列，生成的实体在帧末统一应用。
pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    // 玩家：蓝色方块，初始位于屏幕下方 (0, -300)
    commands.spawn((
        Sprite::from_color(Color::srgb(0.2, 0.6, 1.0), Vec2::new(50.0, 50.0)),
        Transform::from_xyz(0.0, -300.0, 0.0),
        Player,
    ));
    // 敌人：红色方块，初始位于 (100, 200)
    commands.spawn((
        Sprite::from_color(Color::srgb(1.0, 0.3, 0.3), Vec2::new(40.0, 40.0)),
        Transform::from_xyz(100.0, 200.0, 0.0),
        Enemy,
    ));
}

/// 玩家移动系统（Act 6: Input + Act 5: Query）。
///
/// WASD 四方向移动，方向可叠加；速度与帧率无关。
///
/// # 参数
/// - `input`：只读键盘输入资源；
/// - `time`：帧间隔，乘出本帧位移；
/// - `query`：可变查询所有玩家的 `Transform`。
pub fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let speed = 300.0 * time.delta_secs();
    for mut tf in &mut query {
        if input.pressed(KeyCode::KeyW) {
            tf.translation.y += speed;
        }
        if input.pressed(KeyCode::KeyS) {
            tf.translation.y -= speed;
        }
        if input.pressed(KeyCode::KeyA) {
            tf.translation.x -= speed;
        }
        if input.pressed(KeyCode::KeyD) {
            tf.translation.x += speed;
        }
    }
}

/// 射击系统（Act 8: Message）：空格按下时广播 `BulletFired` 消息。
///
/// 本系统只负责"发出事件"，不直接生成实体 —— 生成逻辑由
/// `spawn_bullet` 消费消息完成，二者通过消息解耦。
///
/// # 参数
/// - `input`：只读键盘输入，`just_pressed` 保证一次按键只触发一发；
/// - `player`：只读查询玩家位置作为发射起点；
/// - `writer`：消息写入端。
pub fn shoot(
    input: Res<ButtonInput<KeyCode>>,
    player: Query<&Transform, With<Player>>,
    mut writer: MessageWriter<BulletFired>,
) {
    if input.just_pressed(KeyCode::Space) {
        // single() 返回 Result：无玩家实体时静默跳过，不 panic
        if let Ok(tf) = player.single() {
            writer.write(BulletFired {
                position: tf.translation.xy(),
            });
        }
    }
}

/// 子弹生成系统：消费 `BulletFired` 消息，在指定位置生成子弹实体。
///
/// # 参数
/// - `reader`：消息读取端，内置游标避免重复消费；
/// - `commands`：命令队列，用于生成子弹实体。
pub fn spawn_bullet(
    mut reader: MessageReader<BulletFired>,
    mut commands: Commands,
) {
    for bullet in reader.read() {
        commands.spawn((
            // 黄色竖长条 10×20，模拟弹体外观
            Sprite::from_color(Color::srgb(1.0, 0.9, 0.0), Vec2::new(10.0, 20.0)),
            // 出生点 = 消息携带的发射位置（玩家坐标）
            Transform::from_xyz(bullet.position.x, bullet.position.y, 0.0),
            Bullet { speed: 500.0 },
        ));
    }
}

/// 子弹移动系统：每帧沿 +Y 方向（向上）匀速飞行。
///
/// # 参数
/// - `query`：可变查询子弹实体的 `Transform` 与 `Bullet` 数据；
/// - `time`：帧间隔，保证飞行速度与帧率无关。
pub fn move_bullet(
    mut query: Query<(&mut Transform, &Bullet)>,
    time: Res<Time>,
) {
    for (mut tf, bullet) in &mut query {
        tf.translation.y += bullet.speed * time.delta_secs();
    }
}

/// 清理系统：销毁飞出屏幕上边界的子弹，防止实体无限堆积。
///
/// # 参数
/// - `bullets`：只读查询子弹实体与其位置；
/// - `commands`：命令队列，用于执行 despawn。
pub fn cleanup_bullets(
    bullets: Query<(Entity, &Transform), With<Bullet>>,
    mut commands: Commands,
) {
    // 阈值 500.0：屏幕上边界之外的判定线（视口高度约 720 时够用）
    for (entity, tf) in &bullets {
        if tf.translation.y > 500.0 {
            commands.entity(entity).despawn();
        }
    }
}

/// 碰撞检测系统（Act 7: Resource + Act 5: Query）。
///
/// 遍历所有"子弹 × 敌人"组合，以中心点距离小于 30.0 像素判定命中；
/// 命中后销毁双方并加 100 分。O(n×m) 暴力遍历，实体数量少时足够；
/// 规模扩大后应换用空间划分（如四叉树）或 Bevy 的碰撞插件。
///
/// # 参数
/// - `bullets`：子弹实体与位置（只读）；
/// - `enemies`：敌人实体与位置（只读）；
/// - `commands`：命令队列，用于销毁命中实体；
/// - `score`：可写访问全局分数资源。
pub fn check_collision(
    bullets: Query<(Entity, &Transform), With<Bullet>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut commands: Commands,
    mut score: ResMut<Score>,
) {
    for (b_entity, b_tf) in &bullets {
        for (e_entity, e_tf) in &enemies {
            // 命中阈值 30.0：子弹与敌人中心点距离（单位 像素），
            // 近似等于两者半径之和（子弹 ~5 + 敌人 ~20）
            let dist = b_tf.translation.distance(e_tf.translation);
            if dist < 30.0 {
                commands.entity(b_entity).despawn();
                commands.entity(e_entity).despawn();
                score.total += 100;
            }
        }
    }
}
