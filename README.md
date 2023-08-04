<div align="center">
  <br/>
  <h1>A Config.in parser written in rust.</h1>
</div>
<p align="center">
  <a href="https://www.rust-lang.org/">
    <img src="https://img.shields.io/badge/Rust-1.71.0-green.svg?logo=rust" alt="Rust version"/>
  </a>
<a href="https://codecov.io/gh/Mcdostone/nom-config-in" > 
 <img src="https://codecov.io/gh/Mcdostone/nom-config-in/branch/main/graph/badge.svg?token=PSMOBWY478"/> 
 </a>

</p>

Parsing relies on the [nom library](https://github.com/rust-bakery/nom).
## Getting started

```bash
cargo add nom-config-in
``````
```rust
use std::path::PathBuf;
use nom_config_in::{kconfig::{parse_config_in}, ConfigInInput, ConfigInFile};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let kconfig_file = ConfigInFile::new(PathBuf::from("/path/to/kernel-dir"), PathBuf::from("Kconfig"));
    let input = kconfig_file.read_to_string().unwrap();
    let kconfig = parse_config_in(ConfigInInput::new_extra(&input, kconfig_file));
    Ok(())
}
```

## Resources
- https://lwn.net/Articles/8117/
- https://os.inf.tu-dresden.de/l4env/doc/html/bid-spec/node36.html
- https://github.com/xfguo/menuconfig/blob/master/config-language.txt