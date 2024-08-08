# Bind exemple

```Rust
pub fn sample(argsv: Vec<String>) -> i32 {
    let nuke = DataSet::from_str("name", "some.thing", "command --foo bar");
    return bob(nuke, argsv.clone());
}
```
