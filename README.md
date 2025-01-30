# Rust Iterator Out-of-Bounds Panic

This repository demonstrates a common error in Rust: accessing elements beyond the bounds of an iterator. The `bug.rs` file contains code that panics due to this error.  The `bugSolution.rs` file provides a corrected version.

The bug arises from attempting to access elements from an iterator after it has reached the end.  This results in a runtime panic because the iterator has no more elements to return.

This example highlights the importance of checking the iterator's state (e.g., using `if let Some(x) = iter.next()` ) before accessing elements to prevent runtime panics and ensure robust code.