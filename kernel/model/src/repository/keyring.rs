/**--------------------------------------------------------------------------------------------------
 *   Copyright (c) OpusCommunity. All rights reserved.
 *   Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------------*/
use crate::entity::keyring::KeyringCredential;
use async_trait::async_trait;
use error::AnyError;

#[async_trait]
pub trait KeyringStorageRepository: Send + Sync {
    async fn read(&mut self) -> Result<Option<KeyringCredential>, AnyError>;
    async fn store(&mut self, value: KeyringCredential)
        -> Result<(), AnyError>;
    async fn clear(&mut self) -> Result<(), AnyError>;
}
