/**--------------------------------------------------------------------------------------------------
 *   Copyright (c) OpusCommunity. All rights reserved.
 *   Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------------*/
const COMMANDS: &[&str] = &["init_connection"];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/user.proto")?;
    tauri_plugin::Builder::new(COMMANDS).build();

    Ok(())
}
