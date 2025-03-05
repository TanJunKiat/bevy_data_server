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

#[derive(Resource)]
pub struct WriterResource<T> {
    pub writer: csv::Writer<std::fs::File>,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Default for WriterResource<T>
where
    T: RecordOps<T>,
{
    fn default() -> Self {
        let mut writer_resource = WriterResource {
            writer: csv::Writer::from_path(T::get_file_path()).unwrap(),
            _marker: std::marker::PhantomData,
        };

        writer_resource.writer.write_record(T::get_header()).unwrap();

        return writer_resource;
    }
}
