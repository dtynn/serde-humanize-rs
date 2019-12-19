use serde_json;
use std::time::{Duration, SystemTime};

#[test]
fn test_parse_duration() {
    #[derive(Deserialize)]
    struct Foo {
        #[serde(with = "super")]
        time: Duration,
    }

    let json = r#"{"time": "15s"}"#;
    let foo = serde_json::from_str::<Foo>(json).unwrap();
    assert_eq!(foo.time, Duration::from_secs(15));
}

#[test]
fn test_parse_option_duration() {
    #[derive(Deserialize)]
    struct Foo {
        #[serde(with = "super")]
        time: Option<Duration>,
    }

    let json = r#"{"time": "15s"}"#;
    let foo = serde_json::from_str::<Foo>(json).unwrap();
    assert_eq!(foo.time, Some(Duration::from_secs(15)));

    let json2 = r#"{"time": null}"#;
    let foo2 = serde_json::from_str::<Foo>(json2).unwrap();
    assert_eq!(foo2.time, None);
}

#[test]
fn test_parse_byte_size() {
    #[derive(Deserialize)]
    struct Foo {
        #[serde(with = "super")]
        size: usize,
    }

    let json = r#"{"size": "1 M"}"#;
    let foo = serde_json::from_str::<Foo>(json).unwrap();
    assert_eq!(foo.size, 1_000_000);
}

#[test]
fn test_parse_option_byte_size() {
    #[derive(Deserialize)]
    struct Foo {
        #[serde(with = "super")]
        size: Option<usize>,
    }

    let json = r#"{"size": "1 M"}"#;
    let foo = serde_json::from_str::<Foo>(json).unwrap();
    assert_eq!(foo.size, Some(1_000_000));

    let json2 = r#"{"size": null}"#;
    let foo2 = serde_json::from_str::<Foo>(json2).unwrap();
    assert_eq!(foo2.size, None);
}

#[test]
fn test_parse_byte_size_small() {
    #[derive(Deserialize)]
    struct Foo {
        #[serde(with = "super")]
        size: u16,
    }

    let json_ok = r#"{"size": "1 KiB"}"#;
    let foo_ok = serde_json::from_str::<Foo>(json_ok).unwrap();
    assert_eq!(foo_ok.size, 1_024);

    let json_err = r#"{"size": "1 M"}"#;
    assert!(serde_json::from_str::<Foo>(json_err).is_err());
}

#[test]
fn test_parse_datetime() {
    #[derive(Deserialize)]
    struct Foo {
        #[serde(with = "super")]
        close_at: SystemTime,
    }

    let json = r#"{"close_at": "2105-03-01T10:23:57.000013579+08:00"}"#;
    let foo = serde_json::from_str::<Foo>(json).unwrap();
    assert_eq!(
        foo.close_at.duration_since(SystemTime::UNIX_EPOCH).unwrap(),
        Duration::new(4265317437, 000013579)
    );
}

#[test]
fn test_parse_option_datetime() {
    #[derive(Deserialize)]
    struct Foo {
        #[serde(with = "super")]
        close_at: Option<SystemTime>,
    }

    let json = r#"{"close_at": "2105-03-01T10:23:57.000013579+08:00"}"#;
    let foo = serde_json::from_str::<Foo>(json).unwrap();
    assert_eq!(
        foo.close_at
            .unwrap()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap(),
        Duration::new(4265317437, 000013579)
    );

    let json2 = r#"{"close_at": null}"#;
    let foo2 = serde_json::from_str::<Foo>(json2).unwrap();
    assert_eq!(foo2.close_at, None);
}
