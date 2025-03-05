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
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_hierarchy::prelude::*;
use bevy_transform::prelude::*;
use chrono;

mod functions;
use functions::*;

mod traits;
pub use traits::*;

mod resources;
use resources::*;

pub struct DataServerPlugin;

impl Plugin for DataServerPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Resource)]
pub struct DataServerResource {
    pub folder_location: String,
}

pub fn record_component<T>(mut queries: Query<(Option<&Parent>, Entity, &T), With<T>>, mut writer_resource: Local<WriterResource<T>>)
where
    T: Component + RecordOps<T>,
{
    let mut lens = queries.transmute_lens::<(Option<&Parent>, Entity, &T)>();
    let queries = lens.query();
    write_component_to_csv::<T>(&queries, &mut writer_resource);
}

pub fn record_component_on_change<T>(mut queries: Query<(Option<&Parent>, Entity, &T), Changed<T>>, mut writer_resource: Local<WriterResource<T>>)
where
    T: Component + RecordOps<T>,
{
    let mut lens = queries.transmute_lens::<(Option<&Parent>, Entity, &T)>();
    let queries = lens.query();
    write_component_to_csv::<T>(&queries, &mut writer_resource);
}

pub fn record_component_on_add<T>(mut queries: Query<(Option<&Parent>, Entity, &T), Added<T>>, mut writer_resource: Local<WriterResource<T>>)
where
    T: Component + RecordOps<T>,
{
    let mut lens = queries.transmute_lens::<(Option<&Parent>, Entity, &T)>();
    let queries = lens.query();
    write_component_to_csv::<T>(&queries, &mut writer_resource);
}

pub fn record_component_on_delete<T>(mut writer_resource: Local<WriterResource<T>>, mut removed: RemovedComponents<T>)
where
    T: Component + RecordOps<T>,
{
    let mut data_array = vec!["".to_string(); T::get_data_len()];
    for entity in removed.read() {
        data_array.insert(0, "".to_string());
        data_array.insert(0, format!("{:?}", entity));
        data_array.insert(0, format!("{:?}", chrono::offset::Local::now()));
        writer_resource.writer.write_record(&data_array).unwrap();
    }
    let _ = writer_resource.writer.flush().unwrap();
}

pub fn record_component_with_transform<T>(mut queries: Query<(Option<&Parent>, Entity, &T, &GlobalTransform), (With<T>, With<GlobalTransform>)>, mut writer_resource: Local<WriterResource<T>>)
where
    T: Component + RecordOps<T>,
{
    let mut lens = queries.transmute_lens::<(Option<&Parent>, Entity, &T, &GlobalTransform)>();
    let queries = lens.query();
    write_component_with_transform_to_csv::<T>(&queries, &mut writer_resource);
}

pub fn record_component_with_transform_change<T>(mut queries: Query<(Option<&Parent>, Entity, &T, &GlobalTransform), (With<T>, Changed<GlobalTransform>)>, mut writer_resource: Local<WriterResource<T>>)
where
    T: Component + RecordOps<T>,
{
    let mut lens = queries.transmute_lens::<(Option<&Parent>, Entity, &T, &GlobalTransform)>();
    let queries = lens.query();
    write_component_with_transform_to_csv::<T>(&queries, &mut writer_resource);
}

pub fn record_component_on_add_with_transform<T>(mut queries: Query<(Option<&Parent>, Entity, &T, &GlobalTransform), (Added<T>, With<GlobalTransform>)>, mut writer_resource: Local<WriterResource<T>>)
where
    T: Component + RecordOps<T>,
{
    let mut lens = queries.transmute_lens::<(Option<&Parent>, Entity, &T, &GlobalTransform)>();
    let queries = lens.query();
    write_component_with_transform_to_csv::<T>(&queries, &mut writer_resource);
}

pub fn record_component_on_change_with_transform<T>(mut queries: Query<(Option<&Parent>, Entity, &T, &GlobalTransform), (Changed<T>, With<GlobalTransform>)>, mut writer_resource: Local<WriterResource<T>>)
where
    T: Component + RecordOps<T>,
{
    let mut lens = queries.transmute_lens::<(Option<&Parent>, Entity, &T, &GlobalTransform)>();
    let queries = lens.query();
    write_component_with_transform_to_csv::<T>(&queries, &mut writer_resource);
}

pub fn record_component_on_change_with_optional_transform<T>(queries: Query<(Option<&Parent>, Entity, &T, Option<&GlobalTransform>), Changed<T>>, mut writer_resource: Local<WriterResource<T>>)
where
    T: Component + RecordOps<T>,
{
    for (parent, entity, component, transform) in queries.iter() {
        let mut data_array = component.get_data();
        data_array.insert(0, format!("{:?}", parent));
        data_array.insert(0, format!("{:?}", entity));
        data_array.insert(0, format!("{:?}", chrono::offset::Local::now()));
        match transform {
            Some(transform) => data_array.append(&mut transform.to_string_vec()),
            None => data_array.extend(vec!["".to_string(); 7]),
        }
        writer_resource.writer.write_record(&data_array).unwrap();
    }
    let _ = writer_resource.writer.flush().unwrap();
}

