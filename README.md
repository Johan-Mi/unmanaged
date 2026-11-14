# Unmanaged

Experimental safe[^1], unmanaged data structures in Rust using lifetime brands.

Since Rust 1.100.0, standard containers such as `Box` support custom allocators.
The problem is that each `Box` then stores a reference to its allocator.
If you've got a lot of `Box`es then that's a lot of wasted memory.
This library lets you save that memory by instead passing the allocator to each method that uses it.
For this to be sound, each container must receieve the same allocator every time.
In Zig, where unmanaged containers are all the rage, this is done by being careful.
Rust programmers, however, expect their code to be safe.
To achieve this, every container is branded with a lifetime, and can only be passed the one allocator that has the same lifetime.

[^1]: Implemented with plenty of unsafe code, but hopefully safe to use.
