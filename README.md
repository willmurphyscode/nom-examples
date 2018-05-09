# nom examples

These are some example parsers that I wrote for a talk on Rust
I gave at Rust DC on May 10, 2018. The 
[slides are also on GitHub](https://github.com/willmurphyscode/nom-and-scheme).

Feel free to use these examples, though I would appreciate credit if you do
use them, either by providing a link to this repo, or to
[my blog](https://willmurphyscode.net/).

I can sometimes be instructive to view the expanded macros, which
can be done with this command:
```
cargo rustc -- -Z unstable-options --pretty=expanded > expanded.rs
```

I have gitignored `expanded.rs` to keep from accidentally tracking an outdated
version of the expansion.