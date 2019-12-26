# Naromat: Narou format converter

[![Coverage Status](https://coveralls.io/repos/github/Hitomaru/naromat/badge.svg?branch=feature/add-coverage-to-CI)](https://coveralls.io/github/Hitomaru/naromat?branch=feature/add-coverage-to-CI)

## What's Naromat?

Naromat is a library crate that converts text file from specific typesetting format to Shosetsuka ni Naro(https://syosetu.com/) format.

## Getting started

1. Add Naromat to your cargo.toml
2. Use like this:

```rust
use naromat::entities::chapter::Chapter;

let chapter = Chapter::new("
我が輩は猫である。名前はまだない。
どこで[生まれた:.]のかとんと[見当:けんとう]がつかぬ。
");

let formatted_string = chapter.get();
assert_eq!(formatted_string, "
　我が輩は猫である。名前はまだない。
　どこで|生まれた《・・・・》のかとんと|見当《けんとう》がつかぬ。");
```

## Contributing

* If You find any issue, please notice us by open GitHub issue.
* If You want to change code, please send pull request.
