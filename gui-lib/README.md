## Object Safety

A trait has method definitions; a trait is object-safe if _all_ methods:
- have a return type that is different from `Self`
- have no generic type parameters

`Sized` - Types with a constant size known at compile time. (https://doc.rust-lang.org/std/marker/trait.Sized.html)

---

https://www.reddit.com/r/rust/comments/4llp84/thoughts_on_object_safety/
> Writing `trait Foo: Sized` states "anything that implements Foo must be Sized",
> not "the trait Foo itself is Sized". Traits, like slices, are unsized. So we
> have a problem. Given trait `Foo: Sized`, any `impl Foo for X` requires `X: Sized`.
> But `Foo` is not `Sized` therefore `X = Foo` is illegal.

> Sized is auto-generated for things like structs when it knows the size of
> every field. **A trait has no such fields**. It's not a thing that you allocate.
> A trait stands in for any type which satisfies the trait's interface, and
> those types could have any size at all.

> By definition "object safety" means "callable in the form `x.m(...)` where `x` is
> a trait object". That form is not valid for a static method.

> You don't generally know all implementations of Foo. Maybe you know
> everything in a small application but you certainly don't if you're writing a
> library. Someone else could take your code and write an implementation for
> your public traits.

> The way it deals with regular generic methods is to monomorphize - it creates a special version of the function for every type that is actually used. N.B.: this doesn't apply to traits, as you don't know all implementations.

---

- https://stackoverflow.com/questions/44096235/understanding-traits-and-object-safety
- http://huonw.github.io/blog/2015/01/peeking-inside-trait-objects/