# bitarray
The simple and naive rust implementation of a bit array.

## Install

```yaml
[dependencies]
...
bitarray-naive = { git = "https://github.com/obakhmach/bitarray", branch = "<desired-branch>", version = "<desired-version>"}
```

For example for dev branch with version 0.1.0 the dependecy will look the following.

```yaml
[dependencies]
...
bitarray-naive = { git = "https://github.com/obakhmach/bitarray", branch = "dev", version = "0.1.0"}
```

Or as a registered create

```yaml
[dependencies]
...
bitarray-naive = "0.1.0"
```

## Usage

```rust
use bitarray_naive::BitArray;

let bitarray_size: i64 = 9999;

let mut bitarray: BitArray = BitArray::new(bitarray_size);

bitarray.set(12, true).unwrap();

let bitarray_value: bool = bitarray.get(12).unwrap();
```
