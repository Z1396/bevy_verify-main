//! Act 6: 输入 Input — Res<ButtonInput<KeyCode>> + WASD 移动。
//!
//! 演示键盘输入的两种读取方式：
//! - `pressed(key)`：按住期间每帧返回 true，适合移动等连续控制；
//! - `just_pressed(key)`：仅在按下那一帧返回 true，适合射击等单次触发。
//!
//! 运行方式：`cargo run --example act6_input`

use bevy::prelude::*;

/// 玩家标记组件。
#[derive(Component)]
struct Player;

/// 玩家移动系统：WASD 四方向移动，方向可叠加（斜向速度更快）。
///
/// # 参数
/// - `input`：只读键盘输入资源；
/// - `time`：帧间隔，保证移动速度与帧率无关；
/// - `query`：可变查询所有玩家的 `Transform`。
fn move_player(
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

/// 空格触发演示：`just_pressed` 只在按下瞬间触发一次，之后按住不再触发。
fn on_space(input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        println!("Space pressed!");
    }
}

/// 场景初始化：创建相机与玩家实体。
///
/// # 参数
/// - `commands`：命令队列，生成的实体在帧末统一应用。
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_color(Color::srgb(0.2, 0.6, 1.0), Vec2::new(50.0, 50.0)),
        Player,
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, on_space))
        .run();
}
