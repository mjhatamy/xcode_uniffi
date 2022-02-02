# Xcode Framework Generator/Updater for Rust UniFFI

This tool generates a Xcode Framework using your rust code, using UniFFI

Future versions will support creating Applications as well.

## Installation

Simply type in terminal:

```bash
cargo install xcode_uniffi
```

## How to use

I recommend you to run this code inside the root of your rust project.
Simply type: 

```bash
xcode_uniffi create
```

By default it will create Xcode Framework for iOS + iOS simulator + MacOS in
<rust project root directory>/xcode/<Cargo Library name>

You can pass command line arguments to change the default values.
Please keep the Xcode project as a subdirectory of your rust project, so, Xcode can compile it everytime for different architectures.

