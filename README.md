## serde-humanize-rs
A Serde deserializer for duration and byte-size using the crate [humanize-rs](https://github.com/dtynn/humanize-rs).

### Usage
1. Add this lib as a dependency
```
[dependencies]
serde-humanize-rs = "0.1"
```

2. Add the crate reference
```
extern crate serde_humanize_rs;
```

### Example
```
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate serde_humanize_rs;

use std::time::Duration;

#[derive(Deserialize)]
struct Config {
    #[serde(with = "serde_humanize_rs")]
    size: usize,

    #[serde(with = "serde_humanize_rs")]
    interval: Duration,
}

let json = r#"{"size": "1 M", "interval": "1h30m"}"#;
let cfg = serde_json::from_str::<Config>(json).unwrap();
assert_eq!(cfg.size, 1_000_000);
assert_eq!(cfg.interval, Duration::from_secs(5400));
```
