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

pub trait RecordOps<T> {
    fn get_file_path() -> String;
    fn get_header() -> Vec<String>;
    fn get_data_len() -> usize;
    fn get_data(&self) -> Vec<String>;
}

pub trait CsvOps {
    fn to_string_vec(&self) -> Vec<String>;
}

impl CsvOps for GlobalTransform {
    fn to_string_vec(&self) -> Vec<String> {
        let position = self.translation();
        let rotation = self.rotation();
        return vec![
            format!("{}", position.x),
            format!("{}", position.y),
            format!("{}", position.z),
            format!("{}", rotation.x),
            format!("{}", rotation.y),
            format!("{}", rotation.z),
            format!("{}", rotation.w),
        ];
    }
}
