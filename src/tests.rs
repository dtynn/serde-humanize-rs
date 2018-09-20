use serde_json;
use std::time::Duration;

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
