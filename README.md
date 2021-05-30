# mona: Transform nested container types in Rust

## Usage

The crate provides the `.transpose()` and `.flatten()` on nested container types.

`.transpose()` exchanges the inner and outer containers types. For example,
- `Vec<Vec<T>>` -> `Option<Vec<Vec<T>>>`
- `Vec<HashMap<K, V>>` -> `Option<HashMap<K, Vec<V>>>`
- `HashMap<K, Vec<V>>` -> `HashMap<K, Vec<V>>`
- `Result<Result<T, E>, F>` -> `Result<Result<T, F>, E>`
- `Option<Option<T>>` -> `Option<Option<T>>`

`.flatten()` merges the inner and outer container types. For example,
- `Vec<Vec<T>>` -> `Vec<T>`
- `HashMap<K, HashMap<L, V>>` -> `HashMap<(K, L), V>`
- `HashMap<K, Vec<V>>` -> `HashMap<(K, usize), V>`
- `Vec<HashMap<K, V>>` -> `HashMap<(usize, K), V>`

This is an example usage of `.transpose` and `.flatten()`.

```rust
use mona::prelude::*;

let vec_of_vec = vec![vec![1, 2, 3], vec![4, 5, 6]];

assert_eq!(
    vec_of_vec.clone().transpose(),
    Some(vec![vec![1, 4], vec![2, 5], vec![3, 6]])
);

assert_eq!(vec_of_vec.flatten(), vec![1, 2, 3, 4, 5, 6]);
```

## License

MIT license. See [LICENSE.txt](LICENSE.txt).
