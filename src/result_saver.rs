use crate::communication::result_channel::Answer;
use serde::Serialize;
use std::fs::File;
use std::io::Write;

pub struct JsonResultSaver {}

impl JsonResultSaver {
    pub fn save_results(
        results: &Vec<Answer>,
        out_file_path: &String,
    ) {
        let mut json_results: Vec<JsonResponseInfo> =
            Vec::with_capacity(results.len());
        for answer in results.iter() {
            json_results.push(JsonResponseInfo {
                path: answer.url.path().to_string(),
                status: answer.status,
            })
        }

        let json_str = serde_json::to_string(&json_results).unwrap();

        let mut file = File::create(&out_file_path).expect(&format!(
            "JsonResultSaver: error creating file {}",
            out_file_path
        ));
        file.write_all(json_str.as_bytes()).expect(&format!(
            "JsonResultSaver: error writing in {}",
            out_file_path
        ));
    }
}

#[derive(Serialize)]
struct JsonResponseInfo {
    pub path: String,
    pub status: u16,
}
