/* @file

    "provider" mod configuration of RUCI.
    SPDX-License-Identifier: WTFPL

*/

#[cfg(feature = "settings_provider_uci")]
use rust_uci::Uci;

/// key / value provider.
/// To be implemented by undergoing providers.
/// fn get: get value from provider.
/// fn set: set value to provider.
/// fn apply: finally apply changes to provider.
/// WARN: Any 'set' without 'apply' WILL BE DISCARDED!
pub trait SettingsProvider {
    fn get   (&mut self, name:&str) -> Result<String,String>;
    fn set   (&mut self, name:&str, value:&str) -> Result<i32,String>;
    fn apply (&mut self, section:&str);
}

struct DummySettingsProvider {

}
impl SettingsProvider for DummySettingsProvider {
    fn get(&mut self, name: &str) -> Result<String, String> {
        Ok(name.to_owned() +":Stub!")
    }

    fn set(&mut self, _: &str, _: &str) -> Result<i32, String> {
        Err(String::from("Not implemented"))
    }

    fn apply(&mut self, _: &str) {

    }
}
#[cfg(feature = "settings_provider_uci")]
struct UciSettingsProvider {
    instance:Uci
}

#[cfg(feature = "settings_provider_uci")]
impl SettingsProvider for UciSettingsProvider {
    fn get(&mut self, name: &str) -> Result<String, String> {
        match self.instance.get(name) {
            Ok(result) => Ok(result),
            Err(err) => Err(format!("{:}",err))
        }
    }

    fn set(&mut self, name: &str, value: &str) -> Result<i32, String> {
        match self.instance.set(name, value) {
            Ok(_) => Ok(0),
            Err(err) => Err(format!("{:}",err)),
        }
    }

    fn apply(&mut self, section:&str) {
        self.instance.commit(section).unwrap()
    }
}

pub fn initialize_settings() -> &'static mut dyn SettingsProvider {
    if cfg!(settings_provider_uci) {
        match {Uci::new()} {
            Ok(ins) => {
                Box::leak(
                    Box::new (
                        UciSettingsProvider {
                            instance: ins,
                        }
                    )
                )
            }
            Err(_) => {panic!("Error!")}
        }
    }
    else {
        // TODO: Other implementations
        Box::leak(
            Box::new (
                DummySettingsProvider {
                }
            )
        )
    }
}