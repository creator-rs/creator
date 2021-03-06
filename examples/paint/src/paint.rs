use crate::line::*;
use bevy::prelude::*;

pub fn paint_setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let camera_entity = commands
        .spawn(Camera2dBundle::default())
        .current_entity()
        .unwrap();
    commands.insert_resource(LineDrawingState::new(camera_entity));
    commands.insert_resource(LineMaterial(
        materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
    ));
}

pub fn paint_system(
    commands: &mut Commands,
    mut state: ResMut<LineDrawingState>,
    line_material: Res<LineMaterial>,
    mouse_button_input: Res<Input<MouseButton>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    touch_input: Res<Touches>,
    touch_input_events: Res<Events<TouchInput>>,
    windows: Res<Windows>,
    transforms: Query<&Transform>,
) {
    let camera_transform = transforms.get(state.camera_entity).unwrap();
    if mouse_button_input.pressed(MouseButton::Left) {
        for event in state.cursor_event_reader.iter(&cursor_moved_events) {
            state.cursor_curve.push_front(screen_to_world(
                event.position,
                &camera_transform,
                &windows,
            ));
        }
    } else if touch_input.iter().count() > 0 {
        for event in state.touch_event_reader.iter(&touch_input_events) {
            state.cursor_curve.push_front(screen_to_world(
                event.position,
                &camera_transform,
                &windows,
            ));
        }
    } else {
        state.cursor_curve.clear();
    }
    let new_line_segments = state.pop_line_segments();
    for (p1, p2) in new_line_segments.into_iter() {
        spawn_line_segment(p1, p2, line_material.0.clone(), commands);
    }
}

fn spawn_line_segment(
    p1: Vec2,
    p2: Vec2,
    material: Handle<ColorMaterial>,
    commands: &mut Commands,
) {
    const LINE_THICKNESS: f32 = 10.0;

    let midpoint = (p1 + p2) / 2.0;
    let diff = p2 - p1;
    let length = diff.length() + 5.0;
    let angle = Vec2::new(1.0, 0.0).angle_between(diff);
    let x = midpoint.x;
    let y = midpoint.y;

    commands.spawn(SpriteBundle {
        material,
        sprite: Sprite {
            size: Vec2::new(length, LINE_THICKNESS),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(x, y, 0.0),
            rotation: Quat::from_rotation_z(angle),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn screen_to_world(pos: Vec2, camera_transform: &Transform, windows: &Windows) -> Vec2 {
    let w = windows.get_primary().unwrap();
    let resolution = Vec2::new(w.width() as f32, w.height() as f32);
    let p_ndc = pos - resolution / 2.0;
    let p_world = camera_transform.compute_matrix() * p_ndc.extend(0.0).extend(1.0);
    p_world.truncate().truncate()
}
