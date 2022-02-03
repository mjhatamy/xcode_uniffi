# Xcode Framework Generator/Updater for Rust UniFFI

>**ATTENTION: Setup your project for Rust UniFFI. This code only works if you have \*.UDL file**
> 
>**UDL files are Mozilla UniFFI models to use your rust library in swift or kotlin or python.**
>***
>**To learn more: [Runi Uniffi]**
>
>**[UniFFI Github](https://mozilla.github.io/uniffi-rs/)**
>
>**[Rust UniFFI Learning Center](https://mozilla.github.io/uniffi-rs/)**
***

This tool generates a Xcode Framework using your rust code, using UniFFI
This solution, runs smoothly on iOS and iOS Simulator and MacOS (Both Intel and M1 Processors/SoCs).

>**This application will generate Xcode Framework, based on your Rust UniFFI Models and Cargo.toml files**

Future versions will support creating Applications as well.

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

## Installation

Simply type in terminal:

```bash
cargo install xcode_uniffi
```
Thats it 😂

---
### **Update Library Cargo.toml**

In order to support iOS/MacOS, static compilation is required.
Be sure following line is available in your rust library Cargo.toml file.

```toml
[lib]
crate-type = ["staticlib", "cdylib"]
```

## How to use

I recommend you to run this code inside the root of your rust project.

Simply type: 

```bash
xcode_uniffi create
```

By default, it will create Xcode Framework for iOS + iOS simulator + MacOS in:

**< rust project root directory >/xcode/< Cargo Library name >**

You can pass command line arguments to change the default values.
Please keep the Xcode project as a subdirectory of your rust project, so, Xcode can compile it everytime for different architectures.

> **BITCODE support is not available on RUST compiler yet and it is recommended, to disable BITCODE on both framework and Application. Check the following Tutorial, [how to disable BITCODE]()**


## Command line arguments

SUBCOMMANDS:
* create
 - --package-name/-k

   Specify cargo package name, only if you have more than one package inside your _Cargo.toml_ file.

 - --lib-name/-l

   Specify cargo library name, if you have more than one library in your _Cargo.toml file.
   By default, Application will try to use a package with **_crate_type_** that has _"staticlib"_ feature.

 - --name/-n

   Override the Name of the Xcode framework. (Automatically formatted to Pascal format)

   By default, the name of the Xcode framework will be the name of the Rust library in Pascal string format.

   >Example:
   >
   >Cargo package name : **my_fast_algo**
   >
   >Xcode Framework name: **MyFastAlgo**

 - --path/-p

   Path to generate Xcode Framework.
   By default, Xcode framework will be created inside your rust directory in:

   ./xcode/**< Cargo Lib name >**

   It is recommended to create Xcode framework as a subfolder of rust project. This way, your xcode will be able to compile and generate code, when ever you update your rust project or ...

 - --cargo/-r

   Path to the Rust Project directory or Cargo.toml file.

   It is recommended to run this application in command line inside Rust project directory, but you can pass this argument to specify a different path.

   **Xcode project will be created under this directory, unless you specify a different path**


* Update
> Still in progress


Author: [Jacob Hatami](mjhatamy@gmail.com)