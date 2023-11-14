use bevy::{prelude::{Camera2dBundle, Commands}, ecs::{system::{Query, Res}, query::With}, transform::components::Transform, render::camera::{OrthographicProjection, Camera}, input::{Input, keyboard::KeyCode}};

pub fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

struct CameraSettings{
    pub camera_speed:f32,
    pub zoom_speed:f32,
    pub min_zoom:f32,
    pub max_zoom:f32,
    pub current_zoom:f32,
}

impl Default for CameraSettings{
    fn default() -> Self {
        CameraSettings { camera_speed: (1.), zoom_speed: (1.), min_zoom: (0.1), max_zoom: (10.), current_zoom: (1.) }
    }
}

pub fn camera_movement(
    mut camera_trans_query: Query<&mut Transform, With<Camera>>,
    mut camera_proj_query: Query<&mut OrthographicProjection, With<Camera>>,
    keys: Res<Input<KeyCode>>
)
{
    let mut camera_proj = camera_proj_query.single_mut();
    let mut camera_transform = camera_trans_query.single_mut();
    let mut camera_settings: CameraSettings = Default::default();

    if keys.pressed(KeyCode::Up) {
        camera_transform.translation.y += camera_settings.camera_speed * camera_proj.scale * 2.0;
    }

    if keys.pressed(KeyCode::Left) {
        camera_transform.translation.x -= camera_settings.camera_speed * camera_proj.scale * 2.0;
    }

    if keys.pressed(KeyCode::Down) {
        camera_transform.translation.y -= camera_settings.camera_speed * camera_proj.scale * 2.0;
    }

    if keys.pressed(KeyCode::Right) {
        camera_transform.translation.x += camera_settings.camera_speed * camera_proj.scale * 2.0;
    }


}