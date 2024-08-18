/* @file

    Login service of RUCI.
    SPDX-License-Identifier: WTFPL

*/

use crate::provider;
use crate::provider::password;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn login(username: &str, password: &str) -> Option<(String, u128)> {
    if password::check_password(username, password) {
        let token: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();
        let expiry_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            + 1000 * 3600 * 8; // 8 Hours is enough!

        let mut settings = provider::settings::initialize_settings();
        settings
            .set("ruci.root.sessionkey", &token)
            .expect("Cannot set root token");
        settings
            .set("ruci.root.sessionexpirytime", &*format!("{}", expiry_time))
            .expect("Cannot set root token expiry time");
        settings.apply("ruci");
        Some((token.parse().unwrap(), expiry_time))
    } else {
        None
    }
}
