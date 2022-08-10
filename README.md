# kernel_constants

A collection of structs and constants that the kernel uses,
but the stdlib (TODO: link) also needs.
This crate is their common source of truth.

For example, this project defines the `syscall` numbers.
Both the kernel and the stdlib use those numbers, so that
there is no need to maintain two separate lists.