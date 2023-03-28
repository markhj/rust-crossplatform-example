# Rust Cross-Platform Example
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.68+-lightgray.svg)](https://github.com/markhj/rust-crossplatform-example)
![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)

This is a demonstration of a Rust program which can build on multiple machines, using abstraction to separate OS-specific
logic from the general application logic.

This technique is suitable when you need to interface with OS features on several platforms, and would like a clean way of doing it.

The solution is based on static polymorphism and largely inspired by the strategy/policy pattern:
Everything starts with the _App_ trait, which is implemented for every operating system.
It specifies the required functions which each OS-scope must implement.
The OS-specific code is chosen as compile-time, and is chosed according to which OS it is built on.

Then we have the actual logic of the application, which only concerns itself with calling methods (per the trait).
In other words: The application logic isn't even aware which OS it's operating on.

You (or whoever writes the business logic) can now concentrate on higher business logic without messy _if-this-then-that_'esque OS-specific clutter.

# Compiling

Rust will compile the executable according to the operating system it's built on.

````
cargo build --release
````

## Dockerfile

A ``Dockerfile`` is bundled to test the Linux build from Windows.

You might have to adjust the following command to the name of your project.

````
CMD ["./target/release/<project-name>"]
````

When you load up the Docker image, it should simply say: ``Greeting: Hello from Unix!``

# What now?

Fork this repo, and start bulding your new cross-platform Rust application :slightly_smiling_face:
