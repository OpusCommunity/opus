/**--------------------------------------------------------------------------------------------------
 *   Copyright (c) OpusCommunity. All rights reserved.
 *   Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------------*/
const COMMANDS: &[&str] = &["greet"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
