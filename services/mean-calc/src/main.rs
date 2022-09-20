/*
 * Copyright 2021 Fluence Labs Limited
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
 */

use marine_rs_sdk::{marine, module_manifest, WasmLoggerBuilder};
use std::convert::TryInto;

module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new().build().unwrap();
}

#[marine]
fn mean_iter_f64(data: Vec<Vec<u8>>) -> f64 {
    let sum: f64 = data
        .iter()
        .map(|v| v[..].try_into().unwrap())
        .collect::<Vec<[u8; std::mem::size_of::<f64>()]>>()
        .iter()
        .map(|b| f64::from_le_bytes(*b))
        .collect::<Vec<f64>>()
        .into_iter()
        .sum();
    println!("F64 ITER SUM: {:?}", sum);
    sum / data.len() as f64
}

#[marine]
fn mean_iter_i64(data: Vec<Vec<u8>>) -> f64 {
    let sum: i64 = data
        .iter()
        .map(|v| v[..].try_into().unwrap())
        .collect::<Vec<[u8; std::mem::size_of::<i64>()]>>()
        .iter()
        .map(|b| i64::from_le_bytes(*b))
        .collect::<Vec<i64>>()
        .into_iter()
        .sum();
    sum as f64 / data.len() as f64
}

#[marine]
fn f64s_to_bytes(data: Vec<f64>) -> Vec<Vec<u8>> {
    data.iter()
        .map(|v| v.to_le_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>()
}

#[marine]
fn i64s_to_bytes(data: Vec<i64>) -> Vec<Vec<u8>> {
    data.iter()
        .map(|v| v.to_le_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use marine_rs_sdk_test::marine_test;

    #[test]
    fn no_marine_i64_bytes() {
        let res = i64s_to_bytes(vec![1_i64]);
        assert_eq!(res, vec![[1, 0, 0, 0, 0, 0, 0, 0]]);
    }

    #[test]
    fn no_marine_f64_bytes() {
        let res = f64s_to_bytes(vec![1_f64]);
        assert_eq!(res, vec![[0, 0, 0, 0, 0, 0, 240, 63]]);
    }

    #[marine_test(config_path = "../configs/Config.toml", modules_dir = "../artifacts")]
    fn i64_bytes(mean_service: marine_test_env::mean_calc::ModuleInterface) {
        let res = mean_service.i64s_to_bytes(vec![1_i64]);
        assert_eq!(res, vec![[1, 0, 0, 0, 0, 0, 0, 0]]);
    }

    #[marine_test(config_path = "../configs/Config.toml", modules_dir = "../artifacts")]
    fn f64_bytes(mean_service: marine_test_env::mean_calc::ModuleInterface) {
        let res = mean_service.f64s_to_bytes(vec![1_f64]);
        assert_eq!(res, vec![[0, 0, 0, 0, 0, 0, 240, 63]]);
    }

    #[marine_test(config_path = "../configs/Config.toml", modules_dir = "../artifacts")]
    fn i64_mean(mean_service: marine_test_env::mean_calc::ModuleInterface) {
        let raw_data = vec![(-1) as i64, 1_i64, 6_i64];
        let data_bytes = mean_service.i64s_to_bytes(raw_data.clone());
        let res = mean_service.mean_iter_i64(data_bytes);
        assert_eq!(res, 2.0_f64);
    }

    #[marine_test(config_path = "../configs/Config.toml", modules_dir = "../artifacts")]
    fn f64_mean(mean_service: marine_test_env::mean_calc::ModuleInterface) {
        let raw_data = vec![(-1) as f64, 1_f64, 6_f64];
        let data_bytes = mean_service.f64s_to_bytes(raw_data.clone());
        let res = mean_service.mean_iter_f64(data_bytes);
        assert_eq!(res, 2.0_f64);
    }
}
