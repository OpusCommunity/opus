/**--------------------------------------------------------------------------------------------------
 *   Copyright (c) OpusCommunity. All rights reserved.
 *   Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------------*/
use log::info;
use tauri::command;

#[command]
pub fn greet(name: &str) -> Result<String, String> {
    info!("Hello, {}! You've been greeted from Rust!", name);
    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}
