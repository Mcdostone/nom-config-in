<div align="center">
  <br/>
  <h1>A Config.in parser written in rust.</h1>
</div>
<p align="center">
  <a href="https://www.rust-lang.org/">
    <img src="https://img.shields.io/badge/Rust-1.71.0-green.svg?logo=rust" alt="Rust version"/>
  </a>
 <a href="https://codecov.io/gh/Mcdostone/nom-config-in" > 
 <img src="https://codecov.io/gh/Mcdostone/nom-config-in/graph/badge.svg?token=PSMOBWY478"/> 
 </a>
 <a href="https://github.com/rust-bakery/nom#rust-version-requirements-msrv" > 
   <img src="https://img.shields.io/badge/MSRV-1.56.0+-lightgray.svg?logo=rust" alt="Minimum supported rust version: 1.56.0 or plus"/> 
 </a>
 <a href="https://crates.io/crates/nom-config-in" > 
   <img src="https://img.shields.io/crates/v/nom-config-in.svg?logo=crate" alt="crates.io Version"/> 
 </a>
</p>


**This parser is not actively maintained** since it has been replaced by [Kconfig](https://www.kernel.org/doc/html/next/kbuild/kconfig-language.html). This library has been tested from linux kernel [1.0](https://cdn.kernel.org/pub/linux/kernel/v1.0/) to [2.5.44](https://cdn.kernel.org/pub/linux/kernel/v2.5/) (542 versions).


A Config.in file looks like this:

```
# 2.5.0/arch/i386/config.in
mainmenu_name "Linux Kernel Configuration"

define_bool CONFIG_X86 y
define_bool CONFIG_ISA y
define_bool CONFIG_SBUS n

define_bool CONFIG_UID16 y

mainmenu_option next_comment
comment 'Code maturity level options'
bool 'Prompt for development and/or incomplete code/drivers' CONFIG_EXPERIMENTAL
endmenu
```


## Getting started

```bash
cargo add nom-config-in
``````
```rust
use nom_config_in::{parse_config_in, ConfigInFile, ConfigInInput};
use std::path::PathBuf;

// curl https://cdn.kernel.org/pub/linux/kernel/v2.5/linux-2.5.0.tar.xz | tar -xJ -C /tmp/
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_in_file = ConfigInFile::new(
        PathBuf::from("/tmp/linux/"),
        PathBuf::from("/tmp/linux/arch/i386/config.in"),
    );
    let input = config_in_file.read_to_string().unwrap();
    let config_in = parse_config_in(ConfigInInput::new_extra(&input, config_in_file));
    println!("{:?}", config_in);
    Ok(())
}
```

## Resources
- https://lwn.net/Articles/8117/
- https://os.inf.tu-dresden.de/l4env/doc/html/bid-spec/node36.html
- https://github.com/xfguo/menuconfig/blob/master/config-language.txt