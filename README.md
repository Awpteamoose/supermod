inspired by [automod](https://crates.io/crates/automod)

```rust
supermod::supermod!("src/modname")

// →→→→→
// →→→→→
// →→→→→

mod modname {
	pub mod submod1;
	pub mod submod2;
	pub mod submod3;
	...
}
pub use modname::*;
```
