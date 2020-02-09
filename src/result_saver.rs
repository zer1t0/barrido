use std::io::Write;
use serde::Serialize;
use std::fs::File;
use crate::discoverer;


pub struct JsonResultSaver {
}

impl JsonResultSaver {

    pub fn save_results(
        results: &Vec<discoverer::response_info::ResponseInfo>,
        out_file_path: &String
    ) {
    
        let mut json_results : Vec<JsonResponseInfo> = Vec::with_capacity(results.len());
        for response_info in results.iter() {
            json_results.push(
                JsonResponseInfo {
                    path: response_info.url().path().to_string(),
                    status: response_info.status()
                }
            )
        }

        let json_str = serde_json::to_string(&json_results).unwrap();

        let mut file = File::create(&out_file_path)
            .expect(&format!("JsonResultSaver: error creating file {}", out_file_path));
        file.write_all(json_str.as_bytes())
            .expect(&format!("JsonResultSaver: error writing in {}", out_file_path));
    }

}

#[derive(Serialize)]
struct JsonResponseInfo {
    pub path: String,
    pub status: u16
}