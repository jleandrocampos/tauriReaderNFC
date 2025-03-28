// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use pcsc::*;

#[tauri::command]
fn read_nfc() -> Result<String, String> {
    // Sets the context
    let ctx = match Context::establish(Scope::User) {
        Ok(ctx) => ctx,
        Err(err) => return Err(format!("Error establishing context: {:?}", err)),
    };

    // Buffer for listing readers
    let mut buffer = [0u8; 1024];
    let readers = match ctx.list_readers(&mut buffer) {
        Ok(readers) => readers,
        Err(err) => return Err(format!("Error listing readers: {:?}", err)),
    };

    // Converts readers into a collection for verification
    let reader_iter = readers.into_iter();
    let reader_vec: Vec<_> = reader_iter.collect();

    // Checks for connected readers
    if reader_vec.is_empty() {
        return Err("No reader connected".to_string());
    }

    // Search for ACR122
    let reader_name = match reader_vec
        .iter()
        .find(|&&r| r.to_str().map(|s| s.contains("ACR122")).unwrap_or(false))
    {
        Some(&reader) => reader,
        None => {
            // List available readers on error
            let available_readers: String = reader_vec
                .iter()
                .map(|r| r.to_str().unwrap_or("Invalid name").to_string())
                .collect::<Vec<String>>()
                .join(", ");
            return Err(format!(
                "ACR122 reader not found. Available readers: {}",
                available_readers
            ));
        }
    };

    // Connects to the reader
    let card = match ctx.connect(reader_name, ShareMode::Shared, Protocols::ANY) {
        Ok(card) => card,
        Err(err) => return Err(format!("Error connecting to reader: {:?}", err)),
    };

    // Send APDU command to read UID
    let apdu = [0xFF, 0xCA, 0x00, 0x00, 0x00];
    let mut buffer = [0u8; 256];
    let response = match card.transmit(&apdu, &mut buffer) {
        Ok(response) => response,
        Err(err) => return Err(format!("Error reading tag: {:?}", err)),
    };

    // Format UUID as hexadecimal string
    let uid = response[..response.len() - 2]
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<String>();

    Ok(uid)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![read_nfc])
        .run(tauri::generate_context!())
        .expect("Erro ao executar aplicação Tauri");
}
