# Naromat: Narou format converter

[![Coverage Status](https://coveralls.io/repos/github/Hitomaru/naromat/badge.svg?branch=master&service=github)](https://coveralls.io/github/Hitomaru/naromat?branch=master)

## What's Naromat?

Naromat is a library crate that converts text file from specific typesetting format to Shosetsuka ni Naro(https://syosetu.com/) format.

## Getting started

### As a tool

1. `cargo install naromat`
2. `naromat -h`

### As a library

1. Add Naromat to your cargo.toml
2. Use like this:

```rust
use naromat::entities::chapter::Chapter;

let chapter = Chapter::new("
我が輩は猫[#犬も検討する]である。名前はまだない。
どこで[生まれた:.]のかとんと[見当:けんとう]がつかぬ。
// コメント行
");

let formatted_string = chapter.get();
assert_eq!(formatted_string, "
　我が輩は猫である。名前はまだない。
　どこで｜生まれた《・・・・》のかとんと｜見当《けんとう》がつかぬ。");

use naromat::entities::file::TextFile;

let text = TextFile::new("./path/to/source/file").unwrap();
text.format_and_save("./path/to/save.txt");

```

## Contributing

* If You find any issue, please notice us by open GitHub issue.
* If You want to change code, please send pull request.
