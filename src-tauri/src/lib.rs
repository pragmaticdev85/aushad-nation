// main.rs or lib.rs
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ApiResponse {
    // Define structure matching the API response
    some_field: String,
}

#[tauri::command]
async fn fetch_data_from_rust(url: &str) -> Result<String, String> {
    
    let response = reqwest::get(url).await.map_err(|e| e.to_string())?;
    let status = response.status();
    let text = response.text().await.map_err(|e| e.to_string())?;

    println!("{}", text);
    if status.is_success() {
        Ok(format!("Status: {}, Body: {}", status, text))
        // Or deserialize into a struct and return that
        // let data: ApiResponse = serde_json::from_str(&text).map_err(|e| e.to_string())?;
        // Ok(format!("Field value: {}", data.some_field))
    } else {
        println!("Hello, error!");
        Ok(format!("API error: Status {}, Body: {}", status, text))
    }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn search(name: &str) -> String {
    format!("Searched for <b>{}</b>; Returning results:", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![search])
        .invoke_handler(tauri::generate_handler![fetch_data_from_rust])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
