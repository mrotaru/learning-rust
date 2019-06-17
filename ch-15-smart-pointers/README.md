# Smart Pointers

- references borrow; but smart pointers can actually **own** the data they point to
- `String` and `Vec<T>` are smart pointers - ows some memory, and have extra meta-data and capabilities
- structs that implement `Deref` and `Drop` traits