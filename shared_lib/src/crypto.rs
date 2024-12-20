/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// This example uses `uniffi::` macros to describe the interface.


use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;



#[uniffi::export]
pub fn create_ed25519() -> String {
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();
    return hex::encode(verifying_key.to_bytes());

}

