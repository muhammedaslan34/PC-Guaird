use crate::pairing::{CredentialStore, DeviceCredential, InMemoryCredentialStore, PairingCode};
use std::time::{Duration, SystemTime};

#[test]
fn pairing_code_generate_returns_six_char_alphanumeric_string() {
    let code = PairingCode::generate();
    assert_eq!(code.code.len(), 6);
    assert!(code.code.chars().all(|c: char| c.is_ascii_alphanumeric()));
}

#[test]
fn pairing_code_generate_returns_different_codes() {
    let a = PairingCode::generate();
    let b = PairingCode::generate();
    assert_ne!(a.code, b.code);
}

#[test]
fn pairing_code_is_not_expired_when_just_generated() {
    let code = PairingCode::generate();
    assert!(!code.is_expired());
}

#[test]
fn pairing_code_is_expired_when_expires_at_is_in_the_past() {
    let code = PairingCode {
        code: "ABCD23".to_string(),
        expires_at: SystemTime::now() - Duration::from_secs(1),
    };
    assert!(code.is_expired());
}

#[test]
fn credential_store_saves_and_loads_device_credential() {
    let mut store = InMemoryCredentialStore::default();
    let cred = DeviceCredential {
        device_uuid: "device-uuid-1234".to_string(),
        token: "backend-issued-token-xyz".to_string(),
    };
    store.save(cred).unwrap();
    let loaded = store.load().unwrap();
    assert_eq!(loaded.device_uuid, "device-uuid-1234");
    assert_eq!(loaded.token, "backend-issued-token-xyz");
}

#[test]
fn credential_store_is_not_paired_when_empty() {
    let store = InMemoryCredentialStore::default();
    assert!(!store.is_paired());
}

#[test]
fn credential_store_is_paired_after_save() {
    let mut store = InMemoryCredentialStore::default();
    store
        .save(DeviceCredential {
            device_uuid: "uuid".to_string(),
            token: "tok".to_string(),
        })
        .unwrap();
    assert!(store.is_paired());
}

#[test]
fn credential_store_clear_removes_stored_credential() {
    let mut store = InMemoryCredentialStore::default();
    store
        .save(DeviceCredential {
            device_uuid: "uuid".to_string(),
            token: "tok".to_string(),
        })
        .unwrap();
    store.clear();
    assert!(!store.is_paired());
    assert!(store.load().is_none());
}
