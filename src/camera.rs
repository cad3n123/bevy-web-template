use bevy::{
    app::{self, App, Startup, Update},
    camera::{Camera, Camera2d, OrthographicProjection, Projection, ScalingMode, Viewport},
    color::Color,
    ecs::system::{ResMut, Single},
    math::Vec2,
    scene::{Scene, SceneComponent, SpawnSystem, bsn},
    ui::UiScale,
    window::Window,
};

pub(super) const GAME_RESOLUTION_X: f32 = 1280.0;
pub(super) const GAME_RESOLUTION_Y: f32 = 720.0;
const TARGET_ASPECT: f32 = GAME_RESOLUTION_X / GAME_RESOLUTION_Y;

pub(super) struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, MainCamera::scene.spawn())
            .add_systems(Update, MainCamera::fit_viewport);
    }
}

#[derive(SceneComponent, Clone, Default)]
struct MainCamera;

impl MainCamera {
    fn scene() -> impl Scene {
        bsn! {
            Camera {
                clear_color: Color::srgb(0.1, 0.1, 0.1)
            }
            Camera2d
            Projection::from(OrthographicProjection {
                scaling_mode: ScalingMode::Fixed {
                    width: GAME_RESOLUTION_X,
                    height: GAME_RESOLUTION_Y,
                },
                ..OrthographicProjection::default_2d()
            })
        }
    }

    fn fit_viewport(
        mut ui_scale: ResMut<UiScale>,
        window: Single<&Window>,
        mut camera: Single<&mut Camera>,
    ) {
        let window_size = Vec2::new(
            window.physical_width() as f32,
            window.physical_height() as f32,
        );
        if window_size.x <= 0.0 || window_size.y <= 0.0 {
            return; // minimized
        }

        let window_aspect = window_size.x / window_size.y;

        let viewport_size = if window_aspect > TARGET_ASPECT {
            // Window wider than 16:9 -> bars on left & right
            Vec2::new(window_size.y * TARGET_ASPECT, window_size.y)
        } else {
            // Window taller than 16:9 -> bars on top & bottom
            Vec2::new(window_size.x, window_size.x / TARGET_ASPECT)
        };
        let viewport_position = ((window_size - viewport_size) / 2.0).max(Vec2::ZERO);

        camera.viewport = Some(Viewport {
            physical_position: viewport_position.as_uvec2(),
            physical_size: viewport_size.as_uvec2(),
            ..Default::default()
        });

        // Also fixes it for ui
        ui_scale.0 = viewport_size.y / GAME_RESOLUTION_Y / window.scale_factor();
    }
}
