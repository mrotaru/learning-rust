# Object-Oriented Code - Blog Workflow Example

The example consists of a blog publishing workflow - a blog post can transition between different states, from "draft" initially to "pending", and finally to "published". Until it is published, a blog post doesn't have content.

## With trait objects

`Post` struct has two private members: `state`, which is a smart pointer to a `State` trait object, and a `content` string. A few methods are implemented for it:
- `content()` and `add_text()` - get/set `content`
- `request_review()` and `approve()` - transition internal `state`


The `State` trait object is implemented for three structs, `Draft`, `PendingReview` and `Approved`. Transition means `state` is assigned a new structure, corresponding to the new state.

```
       request_review()            approve()
Draft ­­------------------→ Pending -----------→ Published
```

Disadvantages:
- the `State` methods have to be implemented for all structs, even if they don't need it
- non-trivial run-time performance cost (dynamic dispatch, as opposed to monomorphyzation) (https://medium.com/digitalfrontiers/rust-dynamic-dispatching-deep-dive-236a5896e49b)

## With Types

As an alternative, the book suggests encoding the state directly in the type:
- have `Post::new()` return a `DraftPost`
- `DraftPost` has an `request_review()` method returning a `PendingReviewPost`
- `PendingReviewPost` has an `approve()` method returning a `Post`
- only `Post` has a `content()` method