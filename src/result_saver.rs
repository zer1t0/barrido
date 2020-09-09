use crate::communication::result_channel::Answer;
use serde::Serialize;
use std::fs::File;
use std::io::Write;

use log::error;
use std::collections::HashMap;

pub struct JsonResultSaver {}

impl JsonResultSaver {
    pub fn save_results(results: &Vec<Answer>, filepath: &String) {
        let mut json_results: Vec<JsonResponseInfo> =
            Vec::with_capacity(results.len());
        for answer in results.iter() {
            let mut headers = HashMap::new();
            for (name, value) in answer.headers.iter() {
                let str_value = if let Ok(value) = value.to_str() {
                    value
                } else {
                    "---- No ASCII Header ----"
                };

                headers.insert(name.to_string(), str_value.to_string());
            }

            json_results.push(JsonResponseInfo {
                url: answer.url.to_string(),
                path: answer.url.path().to_string(),
                status: answer.status,
                headers,
            })
        }

        let json_str = serde_json::to_string(&json_results)
            .expect("Error serializing json");

        match File::create(&filepath) {
            Ok(mut file) => {
                if let Err(err) = file.write_all(json_str.as_bytes()) {
                    error!("Error writing {}: {}", filepath, err);
                }
            }
            Err(err) => {
                error!("Error opening {}: {}", filepath, err);
            }
        }
    }
}

#[derive(Serialize)]
struct JsonResponseInfo {
    pub url: String,
    pub path: String,
    pub status: u16,
    pub headers: HashMap<String, String>,
}
