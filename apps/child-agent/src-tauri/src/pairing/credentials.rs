#![allow(dead_code)]

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeviceCredential {
    pub device_uuid: String,
    pub token: String,
}

pub trait CredentialStore {
    fn save(&mut self, credential: DeviceCredential) -> Result<(), String>;
    fn load(&self) -> Option<&DeviceCredential>;
    fn clear(&mut self);

    fn is_paired(&self) -> bool {
        self.load().is_some()
    }
}

#[derive(Default)]
pub struct InMemoryCredentialStore {
    stored: Option<DeviceCredential>,
}

impl CredentialStore for InMemoryCredentialStore {
    fn save(&mut self, credential: DeviceCredential) -> Result<(), String> {
        self.stored = Some(credential);
        Ok(())
    }

    fn load(&self) -> Option<&DeviceCredential> {
        self.stored.as_ref()
    }

    fn clear(&mut self) {
        self.stored = None;
    }
}
