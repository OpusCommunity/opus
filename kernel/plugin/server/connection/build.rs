/**--------------------------------------------------------------------------------------------------
 *    Copyright (c) OpusCommunity. All rights reserved.
 *   Licensed under the GPL v3.0 License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------------*/
const COMMANDS: &[&str] = &["greet", "call"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
