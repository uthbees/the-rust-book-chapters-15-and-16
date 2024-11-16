## Overview

**Project Title**: Learning Rust - Smart Pointers and Concurrency

**Project Description**: A bank simulation that uses threads and smart pointers (Arc\<T\> and Mutex\<T\>).

**Project Goals**: To give me a chance to practice using smart pointers and concurrency in Rust.

## Instructions for Build and Use

Steps to build and/or run the software:

1. Install Rust: https://rustup.rs/
2. Run the application with: `cargo run`

Instructions for using the software:

1. The application will automatically start simulating deposits and withdrawals.
2. To exit, press enter.

There's also some linked list code, but there's not really any sort of finished project that you can run there.

## Development Environment

To recreate the development environment, you need the following software and/or libraries with the specified versions:

* In addition to following the Instructions for Build and Use, you should note that the git hooks are stored in the `.githooks` directory. Run `git config --local core.hooksPath .githooks` when setting up a new environment.

## Useful Websites to Learn More

I found these websites useful in developing this software:

* [https://doc.rust-lang.org/book](https://doc.rust-lang.org/book)
* [https://doc.rust-lang.org/std](https://doc.rust-lang.org/std)
* [https://users.rust-lang.org](https://users.rust-lang.org)
* [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/index.html)

## Future Work

The following items I plan to fix, improve, and/or add to this project in the future:

* [ ] Improve the simulation - some ideas are: Allow users to affect the simulation; Use a normal distribution for transactions (or at least, something better than a linear distribution); Detect bankruptcy.
* [ ] Continue working through the linked lists tutorial - next up is: https://rust-unofficial.github.io/too-many-lists/second-into-iter.html
