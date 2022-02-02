extern crate core;
extern crate nom;

use std::env;
use std::fs;
use std::process::exit;

use crate::xcodebind_gen::{CargoPackage, XcodeProject};
use clap::{App, AppSettings, Arg};
use colored::Colorize;

mod xcodebind_gen;

fn main() {
    let current_directory = &env::current_dir().unwrap();
    // println!(
    //     "current_directory: {:?} - {:?}",
    //     current_directory,
    //     fs::canonicalize("/Users/mjhatamy/workspace/mjhatamy/xcode_uniffi_gen/../.")
    // );
    let matches = clap::App::new(clap::crate_name!())
        .about(clap::crate_description!())
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("create")
                .about("Create a new Xcode Framework project\nBy default project will be created at:\n\
                <Rust Project SRC DIR>/xcode/<Rust Package Name>\nIf you want to use a different path, specify the path using --path/-p subcommand\n\
                It is highly recommended to use a path inside the rust project folder.(Check documentation)")
                .arg(
                    Arg::new("path")
                        .long("path")
                        .short('p')
                        .value_name("path")
                        .help("Default Path is \"<current directory/xcode/\"\nIt is recommended to create xcode project as a sub-folder of rust project.")
                ).arg(
                Arg::new("xcode_framework_name")
                    .long("name")
                    .short('n')
                    .value_name("xcode_framework_name")
                    .help("Name of the Xcode framework. By default the name of the Framework is equal the Cargo package name\n\
                    If your cargo project, has multiple packages, then you have to specify the package name using -k/--package-name subcommand")
            )
                .arg(Arg::new("cargo")
                    .long("cargo")
                    .short('r')
                    .value_name("cargo")
                    .help("Path of the rust project. Default: current directory"))
                .arg(
                    Arg::new("package-name")
                        .long("package-name")
                        .short('k')
                        .value_name("package-name")
                        .help("Cargo package name. Used if your Cargo.toml file has multiple packages.")
                ).arg(
                Arg::new("lib-name")
                    .long("lib-name")
                    .short('l')
                    .value_name("lib-name")
                    .help("Name of the library [lib] in the cargo Package.\n\
                    If your Cargo.toml file has multiple packages, you have to specify corresponding package name as well.")
            )
        ).subcommand(
        App::new("update")
            .about("Updates existing Xcode framework project")
            .setting(AppSettings::ArgRequiredElseHelp)
    );

    match matches.get_matches().subcommand() {
        Some(("create", sub_matches)) => {
            let cargo_manifest_path = sub_matches.value_of("cargo").map_or_else(
                || current_directory.join("Cargo.toml").as_path().to_owned(),
                |f| {
                    let path = fs::canonicalize(f)
                        .expect("Unable to get path from path command");
                    if path.ends_with("Cargo.toml") {
                        path
                    } else {
                        path.join("Cargo.toml")
                    }
                },
            );

            let xcode_framework_path = match sub_matches.value_of("path") {
                Some(path) => fs::canonicalize(path)
                    .expect("Unable to get path from path command"),
                None => {
                    if cargo_manifest_path.ends_with("Cargo.toml") {
                        match cargo_manifest_path.parent() {
                            Some(parent_path) => parent_path.join("xcode"),
                            None => {
                                eprintln!(
                                    "Unable to detect Xcode framework path.\n\
                                It may be due to an invalid Rust(Cargo) project/Cargo.toml Path"
                                );
                                exit(1);
                            }
                        }
                    } else {
                        cargo_manifest_path.join("xcode").as_path().to_path_buf()
                    }
                }
            };

            let package_name = sub_matches.value_of("package-name").map(str::to_string);
            let lib_name = sub_matches.value_of("lib-name").map(str::to_string);
            let xcode_framework_name = sub_matches
                .value_of("xcode_framework_name")
                .map(str::to_string);

            if !cargo_manifest_path.exists() {
                eprintln!(
                    "{} {:?}",
                    "No Cargo.toml file found at:".red(),
                    cargo_manifest_path
                );
                exit(1);
            }
            let cargo = CargoPackage::new(
                &cargo_manifest_path,
                package_name,
                lib_name,
                xcode_framework_name,
                &xcode_framework_path,
            );
            XcodeProject::new(cargo).create();
            //println!("sub command create detected {:?}", cargo_manifest_path);
        }
        _ => unreachable!(),
    }
}
