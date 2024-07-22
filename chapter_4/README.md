So, what's the difference here? Why can `String` be mutated but literals cannot? The difference is in how these two types deal with memory.

## Memory Allocation
- In the case of a string literal, we know the contents at compile time, so the text is hard coded directly into the final executable. This is why string literals are fast and efficient. But these properties only come from the string literal's immutability.
- Unfortunately, we can't put a blob of memory into the binary for each piece of text whose size is unknown at compile time and whose size might change while running the program.
- With the `String` type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents.
- This means:
  - The memory must be requested from the memory allocator at runtime. It is done by us, when we call `String::from`, its implementation requests the memory it needs.
  - We need a way of returning this memory to the allocator when we're done with our `String`. Rust does not use Garbage Collector or manual deallocation, it takes different path: the memory is automatically returned once the variable that owns it goes out of the scope.
  - When a var goes out of scope, Rust calls a special function called `drop`, and it's where the author of `String` can put the code to return the memory. Rust calls `drop` automatically at the closing curly bracket.
