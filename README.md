# Xcode Framework Generator/Updater for Rust UniFFI

This tool generates a Xcode Framework using your rust code, using UniFFI

Future versions will support creating Applications as well.

## Installation

Simply type in terminal:

```bash
cargo install xcode_uniffi
```

## Prepare to use the

You have to install rust targets for MacOS, iOS, and iOS simulator.
[Build Targets](https://doc.rust-lang.org/nightly/rustc/platform-support.html)**

It is important to add following targets to rust compiler. No matter what kind of CPU you use, you have to add both ARM64 libraries and X86_64 libraries to generate libraries for Apple store. (MacOs)
The size of following libraries are too small and would not hurt you and will save a lot of problems later.
Also, compiler scripts, strictly needs following targets to be available.

 | platform | Details |
 | -------- | ------- |
 | x86_64-apple-darwin | MacOS support on Intel platform |
 | aarch64-apple-darwin | MacOS support on M1 (arm64) platform |
 | aarch64-apple-ios | iOS arm64 support |
 | aarch64-apple-ios-sim | iOS simulator on M1 Mac (arm64) |
 | x86_64-apple-ios | iOS simulator on Intel Mac (arm64) |

 
 ```bash
 rustup target add x86_64-apple-darwin
 rustup target add aarch64-apple-darwin
 rustup target add x86_64-apple-ios
 rustup target add aarch64-apple-ios
 rustup target add aarch64-apple-ios-sim
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

