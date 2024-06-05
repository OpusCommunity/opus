/**--------------------------------------------------------------------------------------------------
 *   Copyright (c) OpusCommunity. All rights reserved.
 *   Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------------*/
mod commands;

use tauri::plugin::{Builder as PluginBuilder, TauriPlugin};
use tauri::Runtime;

#[derive(Default)]
pub struct Builder {}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build<R: Runtime>(self) -> TauriPlugin<R> {
        PluginBuilder::new("state")
            .invoke_handler(tauri::generate_handler![commands::greet,])
            .build()
    }
}
