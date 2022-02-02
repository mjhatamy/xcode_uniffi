use super::pbxproj_parser::*;
use super::CargoPackage;
use crate::xcodebind_gen::{CommandLineParser, SourceFileGenerator};
use colored::Colorize;
use crc::{Crc, CRC_64_ECMA_182};
use std::ffi::OsStr;
use std::fs::{create_dir, create_dir_all, File};
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;

// const APPLE_PRODUCT_TYPE_FRAMEWORK: &str = "com.apple.product-type.framework";
// const APPLE_PRODUCT_TYPE_STATIC_LIB: &str = "com.apple.product-type.library.static";
// const APPLE_PRODUCT_TYPE_EXECUTABLE: &str = "com.apple.product-type.tool";

pub(crate) struct XcodeProject {
    crc: Crc<u64>,
    id_base: u64,
    package: CargoPackage,
}

impl XcodeProject {
    pub(crate) fn new(package: CargoPackage) -> Self {
        let crc = Crc::<u64>::new(&CRC_64_ECMA_182);
        let id_base = crc.checksum(package.target_name.as_bytes());

        Self {
            crc,
            id_base,
            package,
        }
    }
    fn check_for_existing_project(&self) -> bool {
        if self.package.xcode_framework_path.exists() {
            // check for project name
            if !self.package.xcode_framework_path.is_dir() {
                eprintln!(
                    "Specified Xcode project Path is a file and not a directory: {:?}\n{}\n\n",
                    self.package.xcode_framework_path,
                    "Xcode framework path must be directory path and not a file path."
                        .red()
                        .bold()
                        .underline()
                );
                exit(1);
            }

            let dir_entry = match std::fs::read_dir(&self.package.xcode_framework_path) {
                Ok(paths) => {
                    paths.filter(|f| f.is_ok())
                        .map(|f| f.expect("Failed to convert Directory Entry to path").path() ).map(|f| {
                        match (f.file_name(), f.extension()) {
                            (Some(file_name), Some(ext)) => {
                                if ext == OsStr::new("xcodeproj") {
                                    if file_name == OsStr::new(format!("{}.xcodeproj", self.package.xcode_framework_name).as_str()) {
                                        Some(f)
                                    } else {
                                        eprintln!("An other xcode framework project exist in the same directory with different name.");
                                        eprintln!("{}\n\n", file_name.to_str().unwrap_or("").red());
                                        exit(1);
                                    }
                                } else {
                                    None
                                }
                            },
                            _ => None
                        }
                    })
                },
                Err(e) => {
                    eprintln!("Unable to open existing Xcode Project Directory at: {:?}\nError: {:?}",
                    self.package.xcode_framework_path, e);
                    exit(1);
                }
            }.filter(|f| f.is_some());

            // Check for existing project
            return dir_entry.count() > 0;
        }
        false
    }

    pub(crate) fn create(&self) -> &Self {
        if self.check_for_existing_project() {
            // Update project instead of create
            println!(
                "\n\n{}: {:?}\n",
                "Existing project found in:".bright_blue(),
                self.package.xcode_framework_path
            );
            if CommandLineParser::get_user_yes_or_no_input("hello") {
                println!("Updating project");
                return self.update();
            } else {
                exit(0);
            }
        }
        let items = self.generate_pbx_project();
        self.create_project(items);
        self
    }

    pub(crate) fn update(&self) -> &Self {
        self
    }

    // fn open_existing_project() {}

    // fn update_pbxproj_for_udl_files() {}

    fn make_id(&self, kind: &str, name: &str) -> String {
        let mut crc = self.crc.digest();
        crc.update(&self.id_base.to_ne_bytes());
        crc.update(kind.as_bytes());
        let kind = crc.finalize();

        let name = self.crc.checksum(name.as_bytes());
        let mut out = format!("CA60{:08X}{:012X}", kind as u32, name);
        out.truncate(24);
        out
    }

    fn make_id_inst(id_base: u64, kind: &str, name: &str) -> String {
        let m_crc = Crc::<u64>::new(&CRC_64_ECMA_182);
        let mut crc = m_crc.digest();
        crc.update(&id_base.to_ne_bytes());
        crc.update(kind.as_bytes());
        let kind = crc.finalize();

        let name = m_crc.checksum(name.as_bytes());
        let mut out = format!("CA60{:08X}{:012X}", kind as u32, name);
        out.truncate(24);
        out
    }

    fn generate_pbx_project(&self) -> Vec<PlistItem> {
        let mut items: Vec<PlistItem> = vec![];
        let root_object_id = self.make_id("rootObject", self.package.xcode_framework_name.as_str());
        let root_object = PlistKeyValueItem::create_value_type(
            "rootObject",
            None,
            &root_object_id,
            Some("Project object"),
        );
        let archive_version =
            PlistKeyValueItem::create_value_type("archiveVersion", None, "1", None);
        let classes = PlistKeyValueItem::create_array_type("classes", None, vec![]);
        let object_version =
            PlistKeyValueItem::create_value_type("objectVersion", None, "55", None);
        items.push(archive_version);
        items.push(classes);
        items.push(object_version);

        // PBXFileReference section
        let (pbx_file_reference_section_ids, pbx_file_reference_section) =
            PlistKeyValueItem::create_pbx_file_reference_section(
                self.package.xcode_framework_name.as_str(),
                &self.package.udl_relative_files_path,
                self.id_base,
                XcodeProject::make_id_inst,
            );
        let (pbx_build_file_section_ids, pbx_build_file_section) =
            PlistKeyValueItem::create_pbx_build_file_section(
                &pbx_file_reference_section_ids,
                self.package.xcode_framework_name.as_str(),
                &self.package.udl_relative_files_path,
                self.id_base,
                XcodeProject::make_id_inst,
            );
        let (pbx_headers_build_section_ids, pbx_headers_build_section) =
            PlistKeyValueItem::create_pbx_headers_build_section(
                &pbx_build_file_section_ids,
                self.package.xcode_framework_name.as_str(),
                self.id_base,
                XcodeProject::make_id_inst,
            );
        let (pbx_sources_build_section_ids, pbx_sources_build_section) =
            PlistKeyValueItem::create_pbx_sources_build_section(
                &pbx_build_file_section_ids,
                self.package.xcode_framework_name.as_str(),
                self.id_base,
                XcodeProject::make_id_inst,
            );
        let (pbx_frameworks_build_section_ids, pbx_frameworks_build_section) =
            PlistKeyValueItem::create_pbx_frameworks_build_section(
                &pbx_build_file_section_ids,
                self.package.xcode_framework_name.as_str(),
                self.id_base,
                XcodeProject::make_id_inst,
            );
        let (pbx_build_rule_section_ids, pbx_build_rule_section) =
            PlistKeyValueItem::create_pbx_build_rule_section(
                self.package.xcode_framework_name.as_str(),
                self.id_base,
                XcodeProject::make_id_inst,
            );
        let (pbx_resource_build_section_ids, pbx_resource_build_section) =
            PlistKeyValueItem::create_pbx_resource_build_phase_section(
                self.package.xcode_framework_name.as_str(),
                self.id_base,
                XcodeProject::make_id_inst,
            );
        let (pbx_group_section_ids, pbx_group_section) =
            PlistKeyValueItem::create_pbx_group_section(
                &pbx_file_reference_section_ids,
                self.package.xcode_framework_name.as_str(),
                self.id_base,
                XcodeProject::make_id_inst,
            );

        let (pbx_shell_script_build_ids, pbx_shell_script_build_section) =
            PlistKeyValueItem::create_pbx_shell_script_build_phase_section(
                self.package
                    .cargo_relative_path_to_xcode_project
                    .to_path_buf()
                    .to_str()
                    .unwrap(),
                &self.package.package_name,
                self.package.target_name.as_str(),
                self.package.xcode_framework_name.as_str(),
                self.id_base,
                XcodeProject::make_id_inst,
            );

        let (pbx_xc_build_config_ids, pbx_xc_build_config_item) =
            PlistKeyValueItem::create_pbx_xc_build_configuration_section(
                &self.package.base_bundle_identifier,
                self.package
                    .cargo_relative_path_to_xcode_project
                    .to_path_buf()
                    .to_str()
                    .unwrap(),
                self.package.xcode_framework_name.as_str(),
                self.id_base,
                XcodeProject::make_id_inst,
            );

        let (pbx_xc_config_list_ids, pbx_xc_config_list_item) =
            PlistKeyValueItem::create_pbx_xc_configuration_list_section(
                &pbx_xc_build_config_ids,
                self.package.xcode_framework_name.as_str(),
                self.id_base,
                XcodeProject::make_id_inst,
            );
        let (pbx_target_dependency_ids, pbx_target_dependency_item) =
            PlistKeyValueItem::create_pbx_target_dependency_section(
                self.package.xcode_framework_name.as_str(),
                self.id_base,
                XcodeProject::make_id_inst,
            );

        let (_ /* pbx_container_item_proxy_ids */, pbx_container_item_proxy_item) =
            PlistKeyValueItem::create_pbx_container_item_proxy_section(
                &root_object_id,
                &pbx_target_dependency_ids,
                self.package.xcode_framework_name.as_str(),
                self.id_base,
                XcodeProject::make_id_inst,
            );

        let (pbx_native_target_ids, pbx_native_target_item) =
            PlistKeyValueItem::create_pbx_native_target_section(
                &pbx_target_dependency_ids,
                &pbx_xc_config_list_ids,
                &pbx_file_reference_section_ids,
                &pbx_shell_script_build_ids,
                &pbx_sources_build_section_ids,
                &pbx_frameworks_build_section_ids,
                &pbx_headers_build_section_ids,
                &pbx_resource_build_section_ids,
                &pbx_build_rule_section_ids,
                self.package.xcode_framework_name.as_str(),
                self.id_base,
                XcodeProject::make_id_inst,
            );

        let pbx_project_item = PlistKeyValueItem::create_pbx_project_section(&root_object_id, &pbx_group_section_ids, &pbx_native_target_ids,
        &pbx_xc_config_list_ids, self.package.xcode_framework_name.as_str());
        // create_pbx_xc_configuration_list_section
        let objects = PlistKeyValueItem::create_array_type(
            "objects",
            None,
            vec![
                pbx_build_file_section,
                pbx_build_rule_section,
                pbx_container_item_proxy_item,
                pbx_file_reference_section,
                pbx_frameworks_build_section,
                pbx_group_section,
                pbx_headers_build_section,
                pbx_native_target_item,
                pbx_project_item,
                pbx_resource_build_section,
                pbx_shell_script_build_section,
                pbx_sources_build_section,
                pbx_target_dependency_item,
                pbx_xc_build_config_item,
                pbx_xc_config_list_item,
            ],
        );
        items.push(objects);

        items.push(root_object);

        items
    }

    fn create_project(&self, items: Vec<PlistItem>) {
        

        let project_dir = &self.package.xcode_framework_path;
        let project_name = &self.package.xcode_framework_name;

        let xcodeproj_dir = &project_dir.join(format!("{}.xcodeproj", project_name));
        let pbxproject_file = &project_dir.join(format!("{}.xcodeproj", project_name)).join("project.pbxproj");
        let xcworkspace_dir = &project_dir.join(format!("{}.xcodeproj", project_name)).join("project.xcworkspace");
        let xcuserdata_dir = &project_dir.join(format!("{}.xcodeproj", project_name)).join("xcuserdata");
        let xcuserdata_in_xcworkspace_dir = &xcworkspace_dir.join("xcuserdata");
        let xcshareddata_in_xcworkspace_dir = &xcworkspace_dir.join("xcshareddata");

        let ios_framework_directory = &project_dir.join(format!("{}_ios", project_name));
        let ios_framework_docc_directory = &project_dir.join(format!("{}_ios", project_name)).join(format!("{}_ios.docc", self.package.xcode_framework_name));
        let ios_framework_docc_file = ios_framework_docc_directory.join(format!("{}_ios.md", project_name));
        let ios_framework_objc_header_file = ios_framework_directory.join(format!("{}_ios.h", self.package.xcode_framework_name));
        let macos_framework_directory = &project_dir.join(format!("{}_macos", project_name));
        let macos_framework_docc_directory = &project_dir.join(format!("{}_macos", project_name)).join(format!("{}_macos.docc", self.package.xcode_framework_name));
        let macos_framework_docc_file = macos_framework_docc_directory.join(format!("{}_macos.md", project_name));
        let macos_framework_objc_header_file = macos_framework_directory.join(format!("{}_macos.h", self.package.xcode_framework_name));

        let framework_ios_tests_directory = &project_dir.join(format!("{}_iosTests", project_name));
        let framework_ios_tests_swift_file = framework_ios_tests_directory.join(format!("{}_iosTests.swift", self.package.xcode_framework_name));
        let framework_macos_tests_directory = &project_dir.join(format!("{}_macosTests", project_name));
        let framework_macos_tests_swift_file = framework_macos_tests_directory.join(format!("{}_macosTests.swift", self.package.xcode_framework_name));

        // Create Project Directory
        if !project_dir.exists() {
            create_dir_all(project_dir)
                .unwrap_or_else(|_| panic!("Failed to create xcode project directory at: {:?}", project_dir));
        }

        if !ios_framework_directory.exists() {
            create_dir(ios_framework_directory)
                .unwrap_or_else(|_| panic!("Failed to create framework inside xcode project directory at: {:?}", ios_framework_directory));
        }
        if !macos_framework_directory.exists() {
            create_dir(macos_framework_directory)
                .unwrap_or_else(|_| panic!("Failed to create framework inside xcode project directory at: {:?}", macos_framework_directory));
        }

        if !ios_framework_docc_directory.exists() {
            create_dir(ios_framework_docc_directory)
                .unwrap_or_else(|_| panic!("Failed to create framework inside xcode project directory at: {:?}", ios_framework_docc_directory));
        }
        if !macos_framework_docc_directory.exists() {
            create_dir(macos_framework_docc_directory)
                .unwrap_or_else(|_| panic!("Failed to create framework inside xcode project directory at: {:?}", macos_framework_docc_directory));
        }

        if !framework_ios_tests_directory.exists() {
            create_dir(framework_ios_tests_directory)
                .unwrap_or_else(|_| panic!("Failed to create framework tests inside xcode project directory at: {:?}", framework_ios_tests_directory));
        }
        if !framework_macos_tests_directory.exists() {
            create_dir(framework_macos_tests_directory)
                .unwrap_or_else(|_| panic!("Failed to create framework tests inside xcode project directory at: {:?}", framework_macos_tests_directory));
        }

        if !xcodeproj_dir.exists() {
            create_dir(xcodeproj_dir)
                .unwrap_or_else(|_| panic!("Failed to create xcodeproj (workspace) directory at: {:?}", xcodeproj_dir));
        }

        //xcworkspace_dir
        if !xcworkspace_dir.exists() {
            create_dir(xcworkspace_dir)
                .unwrap_or_else(|_| panic!("Failed to create project.xcworkspace directory inside xcode project directory at: {:?}", xcworkspace_dir));
        }

        if !xcuserdata_dir.exists() {
            create_dir(xcuserdata_dir)
                .unwrap_or_else(|_| panic!("Failed to create xcuserdata directory inside xcode project directory at: {:?}", xcuserdata_dir));
        }

        if !xcuserdata_in_xcworkspace_dir.exists() {
            create_dir(xcuserdata_in_xcworkspace_dir)
                .unwrap_or_else(|_| panic!("Failed to create xcuserdata directory inside xcode project.xcworkspace  directory at: {:?}", xcuserdata_in_xcworkspace_dir));
        }

        if !xcshareddata_in_xcworkspace_dir.exists() {
            create_dir(xcshareddata_in_xcworkspace_dir)
                .unwrap_or_else(|_| panic!("Failed to create xcshareddata directory inside xcode project.xcworkspace  directory at: {:?}", xcshareddata_in_xcworkspace_dir));
        }

        let mut file = File::create(pbxproject_file).unwrap_or_else(|_| panic!("Unable to create project.pbxproj file"));
        file.write_all(items.serialize().as_bytes()).unwrap_or_else(|_| panic!("Failed to write data to {:?} file", &pbxproject_file));

        let mut file = File::create(&ios_framework_objc_header_file).unwrap_or_else(|_| panic!("Unable to create {:?} file", &ios_framework_objc_header_file));
        file.write_all(SourceFileGenerator::create_objc_header(&self.package, false).as_bytes()).unwrap_or_else(|_| panic!("Failed to write data to {:?} file", &ios_framework_objc_header_file));

        let mut file = File::create(&macos_framework_objc_header_file).unwrap_or_else(|_| panic!("Unable to create {:?} file", &macos_framework_objc_header_file));
        file.write_all(SourceFileGenerator::create_objc_header(&self.package, true).as_bytes()).unwrap_or_else(|_| panic!("Failed to write data to {:?} file", &macos_framework_objc_header_file));


        let mut file = File::create(&ios_framework_docc_file).unwrap_or_else(|_| panic!("Unable to create {:?} file", &ios_framework_docc_file));
        file.write_all(SourceFileGenerator::create_docc_file(&self.package, false).as_bytes()).unwrap_or_else(|_| panic!("Failed to write data to {:?} file", &ios_framework_docc_file));

        let mut file = File::create(&macos_framework_docc_file).unwrap_or_else(|_| panic!("Unable to create {:?} file", &macos_framework_docc_file));
        file.write_all(SourceFileGenerator::create_docc_file(&self.package, true).as_bytes()).unwrap_or_else(|_| panic!("Failed to write data to {:?} file", &macos_framework_docc_file));

        // framework_ios_tests_swift_file
        let mut file = File::create(&framework_ios_tests_swift_file).unwrap_or_else(|_| panic!("Unable to create {:?} file", &framework_ios_tests_swift_file));
        file.write_all(SourceFileGenerator::create_swift_file(&self.package, false).as_bytes()).unwrap_or_else(|_| panic!("Failed to write data to {:?} file", &framework_ios_tests_swift_file));

        let mut file = File::create(&framework_macos_tests_swift_file).unwrap_or_else(|_| panic!("Unable to create {:?} file", &framework_macos_tests_swift_file));
        file.write_all(SourceFileGenerator::create_swift_file(&self.package, true).as_bytes()).unwrap_or_else(|_| panic!("Failed to write data to {:?} file", &framework_macos_tests_swift_file));

        let mut script_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        script_file = script_file.join("scripts").join("rust-xc-universal-binary.sh");
        let mut file = File::open(&script_file).unwrap_or_else(|_| panic!("Unable to open {:?} file", &script_file));
        let mut scripts_file_contents = String::new();
        file.read_to_string(&mut scripts_file_contents).unwrap_or_else(|_| panic!("Failed to read data from {:?} file", &script_file));

        let dst_dir = &self.package.cargo_base_dir.join("rust-xc-universal-binary.sh");
        let mut file = File::create(&dst_dir).unwrap_or_else(|_| panic!("Unable to create {:?} file", &dst_dir));
        file.write_all(scripts_file_contents.as_bytes()).unwrap_or_else(|_| panic!("Failed to write data to {:?} file", &dst_dir));
    }

}
