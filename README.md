![maintenance: actively developed](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# `iter-identify_first_last`

A helper iterator, flagging first and last elements.

```rust
use iter_identify_first_last::IteratorIdentifyFirstLastExt;
use std::fmt::Write;

fn main() {
    let list = [1, 2, 3, 4, 5, 6];
    let mut string = String::new();
    for (is_first, item) in list.iter().identify_first() {
        if !is_first {
            string.push(' ');
        }
        write!(string, "{item}").unwrap();
    }
    assert_eq!(string, "1 2 3 4 5 6");
}
```
