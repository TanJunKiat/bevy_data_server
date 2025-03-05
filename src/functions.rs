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
use super::*;

pub fn write_component_to_csv<T>(queries: &Query<(Option<&Parent>, Entity, &T)>, writer_resource: &mut WriterResource<T>)
where
    T: Component + RecordOps<T>,
{
    for (parent, entity, component) in queries.iter() {
        let mut data_array = component.get_data();
        data_array.insert(0, format!("{:?}", parent));
        data_array.insert(0, format!("{:?}", entity));
        data_array.insert(0, format!("{:?}", chrono::offset::Local::now()));
        writer_resource.writer.write_record(&data_array).unwrap();
    }
    let _ = writer_resource.writer.flush().unwrap();
}

pub fn write_component_with_transform_to_csv<T>(queries: &Query<(Option<&Parent>, Entity, &T, &GlobalTransform)>, writer_resource: &mut WriterResource<T>)
where
    T: Component + RecordOps<T>,
{
    for (parent, entity, component, transform) in queries.iter() {
        let mut data_array = component.get_data();
        data_array.insert(0, format!("{:?}", parent));
        data_array.insert(0, format!("{:?}", entity));
        data_array.insert(0, format!("{:?}", chrono::offset::Local::now()));
        data_array.append(&mut transform.to_string_vec());
        writer_resource.writer.write_record(&data_array).unwrap();
    }
    let _ = writer_resource.writer.flush().unwrap();
}
