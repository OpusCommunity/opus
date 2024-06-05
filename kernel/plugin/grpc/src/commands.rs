/**--------------------------------------------------------------------------------------------------
 *   Copyright (c) OpusCommunity. All rights reserved.
 *   Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------------*/
use log::info;
use tauri::command;

struct GrpcConfig {
    host: String,
    port: String,
}

impl GrpcConfig {
    pub fn new(host: String, port: String) -> Self {
        Self { host, port }
    }
}

#[command]
pub fn init_connection(addr: &str, port: String) -> () {
    info!("{}:{}", addr, port);
    GrpcConfig::new(addr.to_string(), port.to_string());
}
