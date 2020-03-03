# Unsafe - Pointers

- no need for `unsafe` if just _creating_ raw pointers - only when dereferencing
- pointers can be dereferenced only inside `unsafe` block
- also only inside `unsafe` can an unsafe fn or method be invoked
- docs on raw pointers: https://doc.rust-lang.org/std/primitive.pointer.html