use hex::encode;
use rand::Rng;

pub fn uuid8() -> String {
    let bytes = (
        rand::thread_rng().gen::<[u8; 2]>(),
        rand::thread_rng().gen::<[u8; 1]>(),
        rand::thread_rng().gen::<[u8; 1]>(),
        rand::thread_rng().gen::<[u8; 1]>(),
        rand::thread_rng().gen::<[u8; 3]>(),
    );

    format!(
        "{}-{}-{}-{}-{}",
        encode(bytes.0),
        encode(bytes.1),
        encode(bytes.2),
        encode(bytes.3),
        encode(bytes.4),
    )
}

pub fn uuid16() -> String {
    let bytes = (
        rand::thread_rng().gen::<[u8; 4]>(),
        rand::thread_rng().gen::<[u8; 2]>(),
        rand::thread_rng().gen::<[u8; 2]>(),
        rand::thread_rng().gen::<[u8; 2]>(),
        rand::thread_rng().gen::<[u8; 6]>(),
    );

    format!(
        "{}-{}-{}-{}-{}",
        encode(bytes.0),
        encode(bytes.1),
        encode(bytes.2),
        encode(bytes.3),
        encode(bytes.4),
    )
}

pub fn uuid32() -> String {
    let bytes = (
        rand::thread_rng().gen::<[u8; 8]>(),
        rand::thread_rng().gen::<[u8; 4]>(),
        rand::thread_rng().gen::<[u8; 4]>(),
        rand::thread_rng().gen::<[u8; 4]>(),
        rand::thread_rng().gen::<[u8; 12]>(),
    );

    format!(
        "{}-{}-{}-{}-{}",
        encode(bytes.0),
        encode(bytes.1),
        encode(bytes.2),
        encode(bytes.3),
        encode(bytes.4),
    )
}

#[cfg(test)]
#[test]
fn test_uuid8() {
    let id: String = uuid8();

    // 8 * 2 bytes + 4 dashes
    assert_eq!(id.len(), 20);
}

#[test]
fn test_uuid16() {
    let id: String = uuid16();

    // 16 * 2 bytes + 4 dashes
    assert_eq!(id.len(), 36);
}

#[test]
fn test_uuid32() {
    let id: String = uuid32();

    // 32 * 2 bytes + 4 dashes
    assert_eq!(id.len(), 68);
}