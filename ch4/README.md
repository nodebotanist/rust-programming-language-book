# Ch 4 -- Understanding Ownership

The ownership system in Rust allows it to operate efficiently without a garbage collector.

Ownership rules are checked at compile time

## Stack vs Heap memory

* Stack is LIFO	
	* Fixed-size items only
* Heap is a, well, heap of memory
	* When you allocate to the heap, it finds a spot big enough and returns a pointer
	* You can store heap pointers on the stack because they're a fixed size
* Pushing to the stack is faster than allocating to the heap because you don't have to follow a pointer to the stack
* Problems that ownership addresses
	* Keeping track of what's on the heap
	* minimizing duplicate data on the heap
	* cleaning up unused heap data

## Ownership Rules

1. Each value has a variable that is its owner
1. There can only ever be one owner at a time
1. When the owner goes out of scope, the value is dropped

## Variable Scope

A variable is *valid* when it comes into scope, and remains valid until that scope ends

When a variable goes out of scope, Rust calls `drop`, where a user can define how to return the memory (this is done for us for built-in types).

When you reassign a reference, the original reference becomes invalid

```rust
let s1 = String::from("Hello");
let s2 = s1; // s2 now holds the pointer to "hello", s1 is now invalid

println!("{}", s1); // this will error as s1 is invalid
```

Instead of a shallow copy, where the pointer would be *copied* into `s2`, Rust **moves** the pointer into `s2`, which is why s1 becomes invalid

**Rust will never automatically deep copy heap data**. This is to prevent duplicate heap data as much as possible.

If you want to deep copy, use `clone`.

### The copy annotation

Some types stored on the stack have the `Copy` annotation, which allows them to be deep copied automatically:

```rust
let x = 5;
let y = x; // because integers have the Copy trait, x is still valid

println!("x is {} and y is {}", x, y) // this will work
```

Some types that contain `Copy`:

* All integer types
* `bool`
* `char`
* All floating-point types
* Tuples, but *only if they contain types that all have the `Copy` trait

### Ownership and Functions

Passing a value to a function is similar to assigning it to a variable -- the value will be moved or copied just as it would for assignment.

### Return Values and Scope

Returning values can change ownership.

## References and Borrowing

You can pass references to a value with the `&`, allowing you to reference a value without taking ownership of it

References are immutable by default: you can create a mutable reference with `&mut`.

One big rule: **You can only have one mutable reference to any one piece of data within a scope.**

This prevents race conditions at runtime

You can use curly braces to create scopes to allow mutable refs in different scopes in the same function

**You cannot have a mutable reference when you already have an immutable one.**

Guarantees no hanging references

### Summary: the Rules of References

1. You can either have **one** mutable reference or multiple immutable references in one scope.
1. References must always be valid within their scope.

