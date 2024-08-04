/* @file

    Password provider of RUCI.
    SPDX-License-Identifier: WTFPL

*/

pub fn check_password(username: &str, password: &str) -> bool {
    let hash = shadow::Shadow::from_name(username).unwrap();
    pwhash::unix::verify(password, &hash.password)
}
