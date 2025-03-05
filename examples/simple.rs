// =============================================================================
/*
 * Copyright (C) 2024 Tan Jun Kiat
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
*/
// =============================================================================
use bevy::prelude::*;
use bevy_data_server::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, record_component_with_transform::<Box>)
        .insert_resource(Time::<Fixed>::from_hz(1.0))
        .run();
}

#[derive(Component)]
pub struct Box {
    name: String,
}

impl RecordOps<Box> for Box {
    fn get_file_path() -> String {
        "data/box.csv".to_string()
    }
    fn get_data_len() -> usize {
        return 8;
    }

    fn get_header() -> Vec<String> {
        return vec![
            "time".into(),
            "entity".into(),
            "parent".into(),
            "name".into(),
            "x".into(),
            "y".into(),
            "z".into(),
            "x".into(),
            "y".into(),
            "z".into(),
            "w".into(),
        ];
    }

    fn get_data(&self) -> Vec<String> {
        return vec![self.name.clone()];
    }
}

/// set up a simple 3D scene
fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // circular base
    commands.spawn((
        Box { name: "box_2".into() },
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // cube
    commands.spawn((
        Box { name: "box_1".into() },
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    // light
    commands.spawn((PointLight { shadows_enabled: true, ..default() }, Transform::from_xyz(4.0, 8.0, 4.0)));
    // camera
    commands.spawn((Camera3d::default(), Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y)));
}
