use crate::xcodebind_gen::helpers::*;
use std::collections::HashMap;
use std::fmt::format;
use std::path::PathBuf;

use super::*;
pub(crate) struct PBXFrameworksBuildPhaseIds {
    pub(crate) ios_framework: String,
    pub(crate) ios_tests_xctest: String,
    pub(crate) macos_framework: String,
    pub(crate) macos_tests_xctest: String,
}

pub(crate) struct PBXSourcesBuildPhaseIds {
    pub(crate) ios_docc: String,
    pub(crate) ios_tests_swift: String,
    pub(crate) macos_docc: String,
    pub(crate) macos_tests_swift: String,
}

pub(crate) struct PBXResourcesBuildPhaseIds {
    pub(crate) ios: String,
    pub(crate) ios_tests: String,
    pub(crate) macos: String,
    pub(crate) macos_tests: String,
}

#[allow (dead_code)]
pub(crate) struct PBXGroupIds {
    pub(crate) groups: String,
    pub(crate) products: String,
    pub(crate) ios: String,
    pub(crate) ios_tests: String,
    pub(crate) macos: String,
    pub(crate) macos_tests: String,
    pub(crate) frameworks: String,
}

pub(crate) struct PBXShellScriptBuildPhaseIds {
    pub(crate) ios: String,
    pub(crate) macos: String,
}

pub(crate) struct PBXBuildRuleIds {
    pub(crate) ios: String,
    pub(crate) macos: String,
}

pub(crate) struct PBXHeadersBuildPhaseIds {
    pub(crate) headers_ios: String,
    pub(crate) headers_macos: String,
}
pub(crate) struct PBXBuildFileIds {
    pub(crate) ios_framework: String,
    pub(crate) ios_h: String,
    pub(crate) ios_docc: String,
    pub(crate) ios_tests_swift: String,
    pub(crate) macos_framework: String,
    pub(crate) macos_h: String,
    pub(crate) macos_docc: String,
    pub(crate) macos_tests_swift: String,
    pub(crate) udl_files_hashmap: HashMap<String, (String, String)>,
}

pub(crate) struct PBXFileReferenceIds {
    pub(crate) ios_framework: String,
    pub(crate) ios_h: String,
    pub(crate) ios_docc: String,
    pub(crate) ios_tests_xc: String,
    pub(crate) ios_tests_swift: String,
    pub(crate) macos_framework: String,
    pub(crate) macos_h: String,
    pub(crate) macos_docc: String,
    pub(crate) macos_tests_xc: String,
    pub(crate) macos_tests_swift: String,
    pub(crate) udl_files_hashmap: HashMap<String, String>,
}

pub(crate) struct XCBuildConfigurationIds {
    pub(crate) shared_debug: String,
    pub(crate) shared_release: String,
    pub(crate) ios_debug: String,
    pub(crate) ios_release: String,
    pub(crate) ios_tests_debug: String,
    pub(crate) ios_tests_release: String,
    pub(crate) macos_debug: String,
    pub(crate) macos_release: String,
    pub(crate) macos_tests_debug: String,
    pub(crate) macos_tests_release: String,
}

pub(crate) struct XCConfigurationListIds {
    pub(crate) shared: String,
    pub(crate) ios: String,
    pub(crate) ios_tests: String,
    pub(crate) macos: String,
    pub(crate) macos_test: String,
}

pub(crate) struct PBXNativeTargetIds {
    pub(crate) ios: String,
    pub(crate) ios_tests: String,
    pub(crate) macos: String,
    pub(crate) macos_test: String,
}

#[allow (dead_code)]
pub(crate) struct PBXTargetDependencyIds {
    pub(crate) ios_id: String,
    pub(crate) macos_id: String,

    pub(crate) ios_native_target_id: String,
    pub(crate) macos_native_target_id: String,

    pub(crate) ios_container_item_proxy_id: String,
    pub(crate) macos_container_item_proxy_id: String,
}

#[allow (dead_code)]
pub(crate) struct PBXContainerItemProxyIds {
    pub(crate) ios: String,
    pub(crate) macos: String,
}

#[derive(Debug, std::cmp::Eq, std::cmp::PartialEq)]
pub struct PlistKeyValueItem {
    pub key: Box<PlistKey>,
    pub value: Box<PlistItem>,
}

impl InternalXcodePlistSerializer for PlistKeyValueItem {
    fn serialize(&self, number_of_tabs: &mut i32) -> String {
        format!(
            "{}{} = {}",
            String::get_tabs(*number_of_tabs),
            self.key.serialize(number_of_tabs),
            self.value.serialize(number_of_tabs)
        )
    }
}

impl PlistKeyValueItem {
    pub(crate) fn create_value_type_by_array(key_val_items: Vec<(&str, &str)>) -> Vec<PlistItem> {
        let mut items: Vec<PlistItem> = vec![];
        for (key, val) in key_val_items {
            items.push(PlistItem::Item(PlistKeyValueItem {
                key: Box::from(PlistKey {
                    name: key.to_string(),
                    comment: None,
                }),
                value: Box::from(PlistItem::Value(PlistValue {
                    value: val.to_string(),
                    comment: None,
                })),
            }))
        }
        items
    }

    pub(crate) fn create_value_type(
        key: &str,
        key_comment: Option<&str>,
        value: &str,
        value_comment: Option<&str>,
    ) -> PlistItem {
        PlistItem::Item(PlistKeyValueItem {
            key: Box::from(PlistKey {
                name: key.to_string(),
                comment: key_comment.map(|f| f.to_string()),
            }),
            value: Box::from(PlistItem::Value(PlistValue {
                value: value.to_string(),
                comment: value_comment.map(|f| f.to_string()),
            })),
        })
    }

    pub(crate) fn create_array_type(
        key: &str,
        key_comment: Option<&str>,
        value: Vec<PlistItem>,
    ) -> PlistItem {
        PlistItem::Item(PlistKeyValueItem {
            key: Box::from(PlistKey {
                name: key.to_string(),
                comment: key_comment.map(|f| f.to_string()),
            }),
            value: Box::from(PlistItem::Array(value)),
        })
    }

    pub(crate) fn create_pbx_file_reference_section(
        xc_fw_name: &str,
        udl_files_path: &Vec<(PathBuf, String)>,
        id_base: u64,
        id_maker: fn(u64, kind: &str, name: &str) -> String,
    ) -> (PBXFileReferenceIds, PlistItem) {
        let kind = "PBXFileReference";
        let ios_framework = id_maker(
            id_base,
            kind,
            format!("{}_ios.framework", xc_fw_name).as_str(),
        );
        let ios_h = id_maker(id_base, kind, format!("{}_ios.h", xc_fw_name).as_str());
        let ios_docc = id_maker(id_base, kind, format!("{}_ios.docc", xc_fw_name).as_str());
        let ios_tests_xc = id_maker(
            id_base,
            kind,
            format!("{}_iosTests.xctest", xc_fw_name).as_str(),
        );
        let ios_tests_swift = id_maker(
            id_base,
            kind,
            format!("{}_iosTests.swift", xc_fw_name).as_str(),
        );

        let macos_framework = id_maker(
            id_base,
            kind,
            format!("{}_macos.framework", xc_fw_name).as_str(),
        );
        let macos_h = id_maker(id_base, kind, format!("{}_macos.h", xc_fw_name).as_str());
        let macos_docc = id_maker(id_base, kind, format!("{}_macos.docc", xc_fw_name).as_str());
        let macos_tests_xc = id_maker(
            id_base,
            kind,
            format!("{}_macosTests.xctest", xc_fw_name).as_str(),
        );
        let macos_tests_swift = id_maker(
            id_base,
            kind,
            format!("{}_macosTests.swift", xc_fw_name).as_str(),
        );

        println!("name: {} - id_base: {}", xc_fw_name, id_base);
        let ios_framework_item = PlistKeyValueItem::create_array_type(
            ios_framework.as_str(),
            Some(format!("{}_ios.framework", xc_fw_name).as_str()),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type(
                    "explicitFileType",
                    None,
                    "wrapper.framework",
                    None,
                ),
                PlistKeyValueItem::create_value_type("includeInIndex", None, "0", None),
                PlistKeyValueItem::create_value_type(
                    "path",
                    None,
                    format!("{}_ios.framework", xc_fw_name).as_str(),
                    None,
                ),
                PlistKeyValueItem::create_value_type(
                    "sourceTree",
                    None,
                    "BUILT_PRODUCTS_DIR",
                    None,
                ),
            ],
        );

        let ios_h_item = PlistKeyValueItem::create_array_type(
            ios_h.as_str(),
            Some(format!("{}_ios.h", xc_fw_name).as_str()),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type(
                    "lastKnownFileType",
                    None,
                    "sourcecode.c.h",
                    None,
                ),
                PlistKeyValueItem::create_value_type(
                    "path",
                    None,
                    format!("{}_ios.h", xc_fw_name).as_str(),
                    None,
                ),
                PlistKeyValueItem::create_value_type("sourceTree", None, "\"<group>\"", None),
            ],
        );

        let ios_docc_item = PlistKeyValueItem::create_array_type(
            ios_docc.as_str(),
            Some(format!("{}_ios.docc", xc_fw_name).as_str()),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type(
                    "lastKnownFileType",
                    None,
                    "folder.documentationcatalog",
                    None,
                ),
                PlistKeyValueItem::create_value_type(
                    "path",
                    None,
                    format!("{}_ios.docc", xc_fw_name).as_str(),
                    None,
                ),
                PlistKeyValueItem::create_value_type("sourceTree", None, "\"<group>\"", None),
            ],
        );

        let ios_tests_xc_item = PlistKeyValueItem::create_array_type(
            ios_tests_xc.as_str(),
            Some(format!("{}_iosTests.xctest", xc_fw_name).as_str()),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type(
                    "explicitFileType",
                    None,
                    "wrapper.cfbundle",
                    None,
                ),
                PlistKeyValueItem::create_value_type("includeInIndex", None, "0", None),
                PlistKeyValueItem::create_value_type(
                    "path",
                    None,
                    format!("{}_iosTests.xctest", xc_fw_name).as_str(),
                    None,
                ),
                PlistKeyValueItem::create_value_type(
                    "sourceTree",
                    None,
                    "BUILT_PRODUCTS_DIR",
                    None,
                ),
            ],
        );
        let ios_swift_item = PlistKeyValueItem::create_array_type(
            ios_tests_swift.as_str(),
            Some(format!("{}_iosTests.swift", xc_fw_name).as_str()),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type(
                    "lastKnownFileType",
                    None,
                    "sourcecode.swift",
                    None,
                ),
                PlistKeyValueItem::create_value_type(
                    "path",
                    None,
                    format!("{}_iosTests.swift", xc_fw_name).as_str(),
                    None,
                ),
                PlistKeyValueItem::create_value_type("sourceTree", None, "\"<group>\"", None),
            ],
        );

        // --------------- MacOS
        let macos_framework_item = PlistKeyValueItem::create_array_type(
            macos_framework.as_str(),
            Some(format!("{}_macos.framework", xc_fw_name).as_str()),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type(
                    "explicitFileType",
                    None,
                    "wrapper.framework",
                    None,
                ),
                PlistKeyValueItem::create_value_type("includeInIndex", None, "0", None),
                PlistKeyValueItem::create_value_type(
                    "path",
                    None,
                    format!("{}_macos.framework", xc_fw_name).as_str(),
                    None,
                ),
                PlistKeyValueItem::create_value_type(
                    "sourceTree",
                    None,
                    "BUILT_PRODUCTS_DIR",
                    None,
                ),
            ],
        );

        let macos_h_item = PlistKeyValueItem::create_array_type(
            macos_h.as_str(),
            Some(format!("{}_macos.h", xc_fw_name).as_str()),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type(
                    "lastKnownFileType",
                    None,
                    "sourcecode.c.h",
                    None,
                ),
                PlistKeyValueItem::create_value_type(
                    "path",
                    None,
                    format!("{}_macos.h", xc_fw_name).as_str(),
                    None,
                ),
                PlistKeyValueItem::create_value_type("sourceTree", None, "\"<group>\"", None),
            ],
        );

        let macos_docc_item = PlistKeyValueItem::create_array_type(
            macos_docc.as_str(),
            Some(format!("{}_macos.docc", xc_fw_name).as_str()),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type(
                    "lastKnownFileType",
                    None,
                    "folder.documentationcatalog",
                    None,
                ),
                PlistKeyValueItem::create_value_type(
                    "path",
                    None,
                    format!("{}_macos.docc", xc_fw_name).as_str(),
                    None,
                ),
                PlistKeyValueItem::create_value_type("sourceTree", None, "\"<group>\"", None),
            ],
        );

        let macos_tests_xc_item = PlistKeyValueItem::create_array_type(
            macos_tests_xc.as_str(),
            Some(format!("{}_macosTests.xctest", xc_fw_name).as_str()),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type(
                    "explicitFileType",
                    None,
                    "wrapper.cfbundle",
                    None,
                ),
                PlistKeyValueItem::create_value_type("includeInIndex", None, "0", None),
                PlistKeyValueItem::create_value_type(
                    "path",
                    None,
                    format!("{}_macosTests.xctest", xc_fw_name).as_str(),
                    None,
                ),
                PlistKeyValueItem::create_value_type(
                    "sourceTree",
                    None,
                    "BUILT_PRODUCTS_DIR",
                    None,
                ),
            ],
        );
        let macos_swift_item = PlistKeyValueItem::create_array_type(
            macos_tests_swift.as_str(),
            Some(format!("{}_macosTests.swift", xc_fw_name).as_str()),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type(
                    "lastKnownFileType",
                    None,
                    "sourcecode.swift",
                    None,
                ),
                PlistKeyValueItem::create_value_type(
                    "path",
                    None,
                    format!("{}_macosTests.swift", xc_fw_name).as_str(),
                    None,
                ),
                PlistKeyValueItem::create_value_type("sourceTree", None, "\"<group>\"", None),
            ],
        );

        let mut udl_files_hashmap: HashMap<String, String> = HashMap::new();
        let mut items = vec![
            ios_framework_item,
            ios_h_item,
            ios_docc_item,
            ios_tests_xc_item,
            ios_swift_item,
            macos_framework_item,
            macos_h_item,
            macos_docc_item,
            macos_tests_xc_item,
            macos_swift_item,
        ];
        for (path, udl_file_name) in udl_files_path {
            let udl_file_id = id_maker(id_base, kind, udl_file_name.as_str());
            let udl_file_item = PlistKeyValueItem::create_array_type(
                udl_file_id.as_str(),
                Some(udl_file_name),
                vec![
                    PlistKeyValueItem::create_value_type("isa", None, kind, None),
                    PlistKeyValueItem::create_value_type("lastKnownFileType", None, "text", None),
                    PlistKeyValueItem::create_value_type("name", None, udl_file_name, None),
                    PlistKeyValueItem::create_value_type(
                        "path",
                        None,
                        path.to_str()
                            .expect("Unable to convert from UDL file Path to Str"),
                        None,
                    ),
                    PlistKeyValueItem::create_value_type("sourceTree", None, "\"<group>\"", None),
                ],
            );
            udl_files_hashmap.insert(udl_file_name.to_string(), udl_file_id);
            items.push(udl_file_item);
        }

        let file_ref_ids = PBXFileReferenceIds {
            ios_framework,
            ios_h,
            ios_docc,
            ios_tests_xc,
            ios_tests_swift,
            macos_framework,
            macos_h,
            macos_docc,
            macos_tests_xc,
            macos_tests_swift,
            udl_files_hashmap,
        };

        (
            file_ref_ids,
            PlistItem::SectionItem(PlistSectionItem {
                name: kind.to_string(),
                items,
            }),
        )
    }

    pub(crate) fn create_pbx_build_file_section(
        pbx_file_ref_ids: &PBXFileReferenceIds,
        name: &str,
        udl_files_path: &Vec<(PathBuf, String)>,
        id_base: u64,
        id_maker: fn(u64, kind: &str, name: &str) -> String,
    ) -> (PBXBuildFileIds, PlistItem) {
        let kind = "PBXBuildFile";
        let ios_framework = id_maker(id_base, kind, format!("{}_ios.framework", name).as_str());
        let ios_h = id_maker(id_base, kind, format!("{}_ios.h", name).as_str());
        let ios_docc = id_maker(id_base, kind, format!("{}_ios.docc", name).as_str());
        let ios_tests_swift = id_maker(id_base, kind, format!("{}_iosTests.swift", name).as_str());

        let macos_framework = id_maker(id_base, kind, format!("{}_macos.framework", name).as_str());
        let macos_h = id_maker(id_base, kind, format!("{}_macos.h", name).as_str());
        let macos_docc = id_maker(id_base, kind, format!("{}_macos.docc", name).as_str());
        let macos_tests_swift =
            id_maker(id_base, kind, format!("{}_macosTests.swift", name).as_str());

        let ios_framework_item = PlistKeyValueItem::create_array_type(
            ios_framework.as_str(),
            Some(format!("{}_ios.framework in Frameworks", name).as_str()),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type(
                    "fileRef",
                    None,
                    pbx_file_ref_ids.ios_framework.as_str(),
                    Some(format!("{}_ios.framework", name).as_str()),
                ),
            ],
        );
        let ios_h_item = PlistKeyValueItem::create_array_type(
            ios_h.as_str(),
            Some(format!("{}_ios.h in Headers", name).as_str()),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type(
                    "fileRef",
                    None,
                    pbx_file_ref_ids.ios_h.as_str(),
                    Some(format!("{}_ios.h", name).as_str()),
                ),
                PlistKeyValueItem::create_array_type(
                    "settings",
                    None,
                    vec![PlistTupleItem::create("ATTRIBUTES", vec![("Public", None)])],
                ),
            ],
        );
        let ios_docc_item = PlistKeyValueItem::create_array_type(
            ios_docc.as_str(),
            Some(format!("{}_ios.docc in Sources", name).as_str()),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type(
                    "fileRef",
                    None,
                    pbx_file_ref_ids.ios_docc.as_str(),
                    Some(format!("{}_ios.docc", name).as_str()),
                ),
            ],
        );
        let ios_tests_swift_item = PlistKeyValueItem::create_array_type(
            ios_tests_swift.as_str(),
            Some(format!("{}_iosTests.swift in Sources", name).as_str()),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type(
                    "fileRef",
                    None,
                    pbx_file_ref_ids.ios_tests_swift.as_str(),
                    Some(format!("{}_iosTests.swift", name).as_str()),
                ),
            ],
        );

        // --------------- MacOS
        let macos_framework_item = PlistKeyValueItem::create_array_type(
            macos_framework.as_str(),
            Some(format!("{}_macos.framework in Frameworks", name).as_str()),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type(
                    "fileRef",
                    None,
                    pbx_file_ref_ids.macos_framework.as_str(),
                    Some(format!("{}_macos.framework", name).as_str()),
                ),
            ],
        );
        let macos_h_item = PlistKeyValueItem::create_array_type(
            macos_h.as_str(),
            Some(format!("{}_macos.h in Headers", name).as_str()),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type(
                    "fileRef",
                    None,
                    pbx_file_ref_ids.macos_h.as_str(),
                    Some(format!("{}_macos.h", name).as_str()),
                ),
                PlistKeyValueItem::create_array_type(
                    "settings",
                    None,
                    vec![PlistTupleItem::create("ATTRIBUTES", vec![("Public", None)])],
                ),
            ],
        );
        let macos_docc_item = PlistKeyValueItem::create_array_type(
            macos_docc.as_str(),
            Some(format!("{}_macos.docc in Sources", name).as_str()),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type(
                    "fileRef",
                    None,
                    pbx_file_ref_ids.macos_docc.as_str(),
                    Some(format!("{}_macos.docc", name).as_str()),
                ),
            ],
        );
        let macos_tests_swift_item = PlistKeyValueItem::create_array_type(
            macos_tests_swift.as_str(),
            Some(format!("{}_macosTests.swift in Sources", name).as_str()),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type(
                    "fileRef",
                    None,
                    pbx_file_ref_ids.macos_tests_swift.as_str(),
                    Some(format!("{}_macosTests.swift", name).as_str()),
                ),
            ],
        );

        let mut udl_files_hashmap: HashMap<String, (String, String)> = HashMap::new();
        let mut items = vec![
            ios_framework_item,
            ios_h_item,
            ios_docc_item,
            ios_tests_swift_item,
            macos_framework_item,
            macos_h_item,
            macos_docc_item,
            macos_tests_swift_item,
        ];
        for (_, udl_file_name) in udl_files_path {
            let udl_file_ref_id = pbx_file_ref_ids
                .udl_files_hashmap
                .get(udl_file_name)
                .unwrap();
            let udl_file_id = id_maker(id_base, kind, format!("{}_ios", udl_file_name).as_str());
            let udl_file_item = PlistKeyValueItem::create_array_type(
                udl_file_id.as_str(),
                Some(format!("{} in Sources", udl_file_name).as_str()),
                vec![
                    PlistKeyValueItem::create_value_type("isa", None, kind, None),
                    PlistKeyValueItem::create_value_type("fileRef", None, udl_file_ref_id, None),
                ],
            );
            //udl_files_hashmap.insert(udl_file_id, udl_file_name.to_string());
            items.push(udl_file_item);
            udl_files_hashmap.insert(
                format!("{}_ios", udl_file_name),
                (udl_file_id, udl_file_name.to_string()),
            );

            let udl_file_id = id_maker(id_base, kind, format!("{}_macos", udl_file_name).as_str());
            let udl_file_item = PlistKeyValueItem::create_array_type(
                udl_file_id.as_str(),
                Some(format!("{} in Sources", udl_file_name).as_str()),
                vec![
                    PlistKeyValueItem::create_value_type("isa", None, kind, None),
                    PlistKeyValueItem::create_value_type("fileRef", None, udl_file_ref_id, None),
                ],
            );
            udl_files_hashmap.insert(
                format!("{}_macos", udl_file_name),
                (udl_file_id, udl_file_name.to_string()),
            );
            items.push(udl_file_item);
        }

        let file_ref_ids = PBXBuildFileIds {
            ios_framework,
            ios_h,
            ios_docc,
            ios_tests_swift,
            macos_framework,
            macos_h,
            macos_docc,
            macos_tests_swift,
            udl_files_hashmap,
        };
        (
            file_ref_ids,
            PlistItem::SectionItem(PlistSectionItem {
                name: kind.to_string(),
                items,
            }),
        )
    }

    pub(crate) fn create_pbx_headers_build_section(
        pbx_file_ref_ids: &PBXBuildFileIds,
        name: &str,
        id_base: u64,
        id_maker: fn(u64, kind: &str, name: &str) -> String,
    ) -> (PBXHeadersBuildPhaseIds, PlistItem) {
        let kind = "PBXHeadersBuildPhase";
        let headers_ios = id_maker(id_base, kind, format!("{}_ios.headers", name).as_str());
        let headers_macos = id_maker(id_base, kind, format!("{}_macos.headers", name).as_str());

        let ios_headers = PlistKeyValueItem::create_array_type(
            headers_ios.as_str(),
            Some("Headers"),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type("buildActionMask", None, "2147483647", None),
                PlistTupleItem::create(
                    "files",
                    vec![(
                        pbx_file_ref_ids.ios_h.as_str(),
                        Some(format!("{}_ios.h in Headers", name)),
                    )],
                ),
                PlistKeyValueItem::create_value_type(
                    "runOnlyForDeploymentPostprocessing",
                    None,
                    "0",
                    None,
                ),
            ],
        );
        let macos_headers = PlistKeyValueItem::create_array_type(
            headers_macos.as_str(),
            Some("Headers"),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type("buildActionMask", None, "2147483647", None),
                PlistTupleItem::create(
                    "files",
                    vec![(
                        pbx_file_ref_ids.macos_h.as_str(),
                        Some(format!("{}_macos.h in Headers", name)),
                    )],
                ),
                PlistKeyValueItem::create_value_type(
                    "runOnlyForDeploymentPostprocessing",
                    None,
                    "0",
                    None,
                ),
            ],
        );
        let file_ref_ids = PBXHeadersBuildPhaseIds {
            headers_ios,
            headers_macos,
        };
        (
            file_ref_ids,
            PlistItem::SectionItem(PlistSectionItem {
                name: kind.to_string(),
                items: vec![ios_headers, macos_headers],
            }),
        )
    }

    pub(crate) fn create_pbx_sources_build_section(
        pbx_file_ref_ids: &PBXBuildFileIds,
        name: &str,
        id_base: u64,
        id_maker: fn(u64, kind: &str, name: &str) -> String,
    ) -> (PBXSourcesBuildPhaseIds, PlistItem) {
        let kind = "PBXSourcesBuildPhase";
        let ios_docc = id_maker(id_base, kind, format!("{}_ios.docc", name).as_str());
        let ios_tests_swift = id_maker(id_base, kind, format!("{}_iosTests.swift", name).as_str());
        let macos_docc = id_maker(id_base, kind, format!("{}_macos.docc", name).as_str());
        let macos_tests_swift =
            id_maker(id_base, kind, format!("{}_macosTests.swift", name).as_str());

        let mut macos_udls_ios = vec![];
        pbx_file_ref_ids
            .udl_files_hashmap
            .iter()
            .for_each(|(key, (id, name))| {
                if key.ends_with("ios") {
                    macos_udls_ios.push((id.as_str(), Some(format!("{} in Sources", name))));
                }
            });
        macos_udls_ios.push((
            pbx_file_ref_ids.ios_docc.as_str(),
            Some(format!("{}_ios.docc in Sources", name)),
        ));
        let ios_docc_item = PlistKeyValueItem::create_array_type(
            ios_docc.as_str(),
            Some("Sources"),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type("buildActionMask", None, "2147483647", None),
                PlistTupleItem::create("files", macos_udls_ios),
                PlistKeyValueItem::create_value_type(
                    "runOnlyForDeploymentPostprocessing",
                    None,
                    "0",
                    None,
                ),
            ],
        );
        let ios_swift_item = PlistKeyValueItem::create_array_type(
            ios_tests_swift.as_str(),
            Some("Sources"),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type("buildActionMask", None, "2147483647", None),
                PlistTupleItem::create(
                    "files",
                    vec![(
                        pbx_file_ref_ids.ios_tests_swift.as_str(),
                        Some(format!("{}_iosTests.swift in Sources", name)),
                    )],
                ),
                PlistKeyValueItem::create_value_type(
                    "runOnlyForDeploymentPostprocessing",
                    None,
                    "0",
                    None,
                ),
            ],
        );

        // --------- MacOS
        let mut macos_udls_macos = vec![];
        pbx_file_ref_ids
            .udl_files_hashmap
            .iter()
            .for_each(|(key, (id, name))| {
                if key.ends_with("macos") {
                    macos_udls_macos.push((id.as_str(), Some(format!("{} in Sources", name))));
                }
            });
        macos_udls_macos.push((
            pbx_file_ref_ids.macos_docc.as_str(),
            Some(format!("{}_macos.docc in Sources", name)),
        ));
        let macos_docc_item = PlistKeyValueItem::create_array_type(
            macos_docc.as_str(),
            Some("Sources"),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type("buildActionMask", None, "2147483647", None),
                PlistTupleItem::create("files", macos_udls_macos),
                PlistKeyValueItem::create_value_type(
                    "runOnlyForDeploymentPostprocessing",
                    None,
                    "0",
                    None,
                ),
            ],
        );
        let macos_swift_item = PlistKeyValueItem::create_array_type(
            macos_tests_swift.as_str(),
            Some("Sources"),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type("buildActionMask", None, "2147483647", None),
                PlistTupleItem::create(
                    "files",
                    vec![(
                        pbx_file_ref_ids.macos_tests_swift.as_str(),
                        Some(format!("{}_macosTests.swift in Sources", name)),
                    )],
                ),
                PlistKeyValueItem::create_value_type(
                    "runOnlyForDeploymentPostprocessing",
                    None,
                    "0",
                    None,
                ),
            ],
        );
        let file_ref_ids = PBXSourcesBuildPhaseIds {
            ios_docc,
            ios_tests_swift,
            macos_docc,
            macos_tests_swift,
        };
        (
            file_ref_ids,
            PlistItem::SectionItem(PlistSectionItem {
                name: kind.to_string(),
                items: vec![
                    ios_docc_item,
                    ios_swift_item,
                    macos_docc_item,
                    macos_swift_item,
                ],
            }),
        )
    }

    pub(crate) fn create_pbx_frameworks_build_section(
        pbx_file_ref_ids: &PBXBuildFileIds,
        name: &str,
        id_base: u64,
        id_maker: fn(u64, kind: &str, name: &str) -> String,
    ) -> (PBXFrameworksBuildPhaseIds, PlistItem) {
        let kind = "PBXFrameworksBuildPhase";
        let ios_framework = id_maker(id_base, kind, format!("{}_ios.framework", name).as_str());
        let ios_tests_xctest =
            id_maker(id_base, kind, format!("{}_iosTests.xctest", name).as_str());
        let macos_framework = id_maker(id_base, kind, format!("{}_macos.framework", name).as_str());
        let macos_tests_xctest = id_maker(
            id_base,
            kind,
            format!("{}_macosTests.xctest", name).as_str(),
        );

        let ios_framework_item = PlistKeyValueItem::create_array_type(
            ios_framework.as_str(),
            Some("Frameworks"),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type("buildActionMask", None, "2147483647", None),
                PlistTupleItem::create(
                    "files",
                    vec![],
                ),
                PlistKeyValueItem::create_value_type(
                    "runOnlyForDeploymentPostprocessing",
                    None,
                    "0",
                    None,
                ),
            ],
        );
        let ios_tests_xctest_item = PlistKeyValueItem::create_array_type(
            ios_tests_xctest.as_str(),
            Some("Frameworks"),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type("buildActionMask", None, "2147483647", None),
                PlistTupleItem::create("files", vec![(
                    pbx_file_ref_ids.ios_framework.as_str(),
                    Some(format!("{}_ios.framework in Sources", name)),
                )]),
                PlistKeyValueItem::create_value_type(
                    "runOnlyForDeploymentPostprocessing",
                    None,
                    "0",
                    None,
                ),
            ],
        );

        // --------- MacOS
        let macos_framework_item = PlistKeyValueItem::create_array_type(
            macos_framework.as_str(),
            Some("Frameworks"),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type("buildActionMask", None, "2147483647", None),
                PlistTupleItem::create(
                    "files",
                    vec![],
                ),
                PlistKeyValueItem::create_value_type(
                    "runOnlyForDeploymentPostprocessing",
                    None,
                    "0",
                    None,
                ),
            ],
        );
        let macos_tests_xctest_item = PlistKeyValueItem::create_array_type(
            macos_tests_xctest.as_str(),
            Some("Frameworks"),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type("buildActionMask", None, "2147483647", None),
                PlistTupleItem::create("files", vec![(
                    pbx_file_ref_ids.macos_framework.as_str(),
                    Some(format!("{}_macos.framework in Frameworks", name)),
                )]),
                PlistKeyValueItem::create_value_type(
                    "runOnlyForDeploymentPostprocessing",
                    None,
                    "0",
                    None,
                ),
            ],
        );
        let file_ref_ids = PBXFrameworksBuildPhaseIds {
            ios_framework,
            ios_tests_xctest,
            macos_framework,
            macos_tests_xctest,
        };
        (
            file_ref_ids,
            PlistItem::SectionItem(PlistSectionItem {
                name: kind.to_string(),
                items: vec![
                    ios_framework_item,
                    ios_tests_xctest_item,
                    macos_framework_item,
                    macos_tests_xctest_item,
                ],
            }),
        )
    }

    pub(crate) fn create_pbx_build_rule_section(
        name: &str,
        id_base: u64,
        id_maker: fn(u64, kind: &str, name: &str) -> String,
    ) -> (PBXBuildRuleIds, PlistItem) {
        let kind = "PBXBuildRule";
        let ios_build_rule_id = id_maker(id_base, kind, format!("{}_ios_uniffi", name).as_str());
        let macos_build_rule_id =
            id_maker(id_base, kind, format!("{}_macos_uniffi", name).as_str());

        let ios_build_rule_item = PlistKeyValueItem::create_array_type(ios_build_rule_id.as_str(), Some(kind),
                                                                 vec![
                                                                     PlistKeyValueItem::create_value_type("isa", None, kind, None),
                                                                     PlistKeyValueItem::create_value_type("compilerSpec", None, "com.apple.compilers.proxy.script", None),
                                                                     PlistKeyValueItem::create_value_type("filePatterns", None, "\"*.udl\"", None),
                                                                     PlistKeyValueItem::create_value_type("fileType", None, "pattern.proxy", None),
                                                                     PlistTupleItem::create("inputFiles", vec![]),
                                                                     PlistKeyValueItem::create_value_type("isEditable", None, "1", None),
                                                                     PlistKeyValueItem::create_value_type("name", None, format!("\"{} - UniFFI Build rule for '*.udl' files - ios\"", name).as_str(), None),
                                                                     PlistTupleItem::create("outputFiles", vec![
                                                                         (format!("\"$(SRCROOT)/rust_libs/{}/$(INPUT_FILE_BASE).swift\"", name).as_str(), None),
                                                                         (format!("\"$(SRCROOT)/rust_libs/{}/$(INPUT_FILE_BASE)FFI.h\"", name).as_str(), None),
                                                                     ]),
                                                                     PlistKeyValueItem::create_value_type("runOncePerArchitecture", None, "0", None),
                                                                     PlistKeyValueItem::create_value_type("script", None,
                                                                                                          format!(r#"" # Generate swift bindings for the todolist rust library.\nset -e\necho \"Generating files for $INPUT_FILE_PATH\"\nexport PATH=\"$PATH:/opt/homebrew/bin/\"\n$HOME/.cargo/bin/uniffi-bindgen generate \"$INPUT_FILE_PATH\" --language swift --out-dir \"$SRCROOT/rust_libs/{0}\"\nmv \"$SRCROOT/rust_libs/{0}/\"$INPUT_FILE_BASE\"FFI.modulemap\" \"$SRCROOT/rust_libs/{0}/module.modulemap\"\necho \"Generated files for $INPUT_FILE_BASE in $SRCROOT/rust_libs/{0}\"\n""#,
                                                                                                                  name).as_str(), None),
                                                                 ]);
        let macos_build_rule_item = PlistKeyValueItem::create_array_type(macos_build_rule_id.as_str(), Some(kind),
                                                                       vec![
                                                                           PlistKeyValueItem::create_value_type("isa", None, kind, None),
                                                                           PlistKeyValueItem::create_value_type("compilerSpec", None, "com.apple.compilers.proxy.script", None),
                                                                           PlistKeyValueItem::create_value_type("filePatterns", None, "\"*.udl\"", None),
                                                                           PlistKeyValueItem::create_value_type("fileType", None, "pattern.proxy", None),
                                                                           PlistTupleItem::create("inputFiles", vec![]),
                                                                           PlistKeyValueItem::create_value_type("isEditable", None, "1", None),
                                                                           PlistKeyValueItem::create_value_type("name", None, format!("\"{} - UniFFI Build rule for '*.udl' files - macos\"", name).as_str(), None),
                                                                           PlistTupleItem::create("outputFiles", vec![
                                                                               (format!("\"$(SRCROOT)/rust_libs/{}/$(INPUT_FILE_BASE).swift\"", name).as_str(), None),
                                                                               (format!("\"$(SRCROOT)/rust_libs/{}/$(INPUT_FILE_BASE)FFI.h\"", name).as_str(), None),
                                                                           ]),
                                                                           PlistKeyValueItem::create_value_type("runOncePerArchitecture", None, "0", None),
                                                                           PlistKeyValueItem::create_value_type("script", None,
                                                                                                                format!(r#"" # Generate swift bindings for the todolist rust library.\nset -e\necho \"Generating files for $INPUT_FILE_PATH\"\nexport PATH=\"$PATH:/opt/homebrew/bin/\"\n$HOME/.cargo/bin/uniffi-bindgen generate \"$INPUT_FILE_PATH\" --language swift --out-dir \"$SRCROOT/rust_libs/{0}\"\nmv \"$SRCROOT/rust_libs/{0}/\"$INPUT_FILE_BASE\"FFI.modulemap\" \"$SRCROOT/rust_libs/{0}/module.modulemap\"\necho \"Generated files for $INPUT_FILE_BASE in $SRCROOT/rust_libs/{0}\"\n""#,
                                                                                                                        name).as_str(), None),
                                                                       ]);
        let file_ref_ids = PBXBuildRuleIds {
            ios: ios_build_rule_id,
            macos: macos_build_rule_id,
        };
        (
            file_ref_ids,
            PlistItem::SectionItem(PlistSectionItem {
                name: kind.to_string(),
                items: vec![ios_build_rule_item, macos_build_rule_item],
            }),
        )
    }

    pub(crate) fn create_pbx_resource_build_phase_section(
        name: &str,
        id_base: u64,
        id_maker: fn(u64, kind: &str, name: &str) -> String,
    ) -> (PBXResourcesBuildPhaseIds, PlistItem) {
        let kind = "PBXResourcesBuildPhase";
        let ios_resource_build_id = id_maker(id_base, kind, format!("{}_ios", name).as_str());
        let ios_resource_build_tests_id =
            id_maker(id_base, kind, format!("{}_iosTests", name).as_str());
        let macos_resource_build_id = id_maker(id_base, kind, format!("{}_macos", name).as_str());
        let macos_resource_build_tests_id =
            id_maker(id_base, kind, format!("{}_macosTests", name).as_str());

        let ios_resource_build_item = PlistKeyValueItem::create_array_type(
            ios_resource_build_id.as_str(),
            Some("Resources"),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type("buildActionMask", None, "2147483647", None),
                PlistTupleItem::create("files", vec![]),
                PlistKeyValueItem::create_value_type(
                    "runOnlyForDeploymentPostprocessing",
                    None,
                    "0",
                    None,
                ),
            ],
        );
        let ios_resource_build_tests_item = PlistKeyValueItem::create_array_type(
            ios_resource_build_tests_id.as_str(),
            Some("Resources"),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type("buildActionMask", None, "2147483647", None),
                PlistTupleItem::create("files", vec![]),
                PlistKeyValueItem::create_value_type(
                    "runOnlyForDeploymentPostprocessing",
                    None,
                    "0",
                    None,
                ),
            ],
        );
        let macos_resource_build_item = PlistKeyValueItem::create_array_type(
            macos_resource_build_id.as_str(),
            Some("Resources"),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type("buildActionMask", None, "2147483647", None),
                PlistTupleItem::create("files", vec![]),
                PlistKeyValueItem::create_value_type(
                    "runOnlyForDeploymentPostprocessing",
                    None,
                    "0",
                    None,
                ),
            ],
        );
        let macos_resource_build_tests_item = PlistKeyValueItem::create_array_type(
            macos_resource_build_tests_id.as_str(),
            Some("Resources"),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type("buildActionMask", None, "2147483647", None),
                PlistTupleItem::create("files", vec![]),
                PlistKeyValueItem::create_value_type(
                    "runOnlyForDeploymentPostprocessing",
                    None,
                    "0",
                    None,
                ),
            ],
        );

        let file_ref_ids = PBXResourcesBuildPhaseIds {
            ios: ios_resource_build_id,
            ios_tests: ios_resource_build_tests_id,
            macos: macos_resource_build_id,
            macos_tests: macos_resource_build_tests_id,
        };
        (
            file_ref_ids,
            PlistItem::SectionItem(PlistSectionItem {
                name: kind.to_string(),
                items: vec![
                    ios_resource_build_item,
                    ios_resource_build_tests_item,
                    macos_resource_build_item,
                    macos_resource_build_tests_item,
                ],
            }),
        )
    }

    pub(crate) fn create_pbx_group_section(
        pbx_file_ref: &PBXFileReferenceIds,
        xfw_name: &str,
        id_base: u64,
        id_maker: fn(u64, kind: &str, name: &str) -> String,
    ) -> (PBXGroupIds, PlistItem) {
        let kind = "PBXGroup";
        let products_id = id_maker(id_base, kind, format!("{}_products", xfw_name).as_str());
        let ios_id = id_maker(
            id_base,
            kind,
            format!("{}_ios.framework", xfw_name).as_str(),
        );
        let ios_tests_id = id_maker(id_base, kind, format!("{}_iosTests", xfw_name).as_str());
        let macos_id = id_maker(id_base, kind, format!("{}_macos", xfw_name).as_str());
        let macos_tests_id = id_maker(id_base, kind, format!("{}_macosTests", xfw_name).as_str());
        let frameworks_id = id_maker(id_base, kind, format!("{}_Frameworks", xfw_name).as_str());

        let products_item = PlistKeyValueItem::create_array_type(
            products_id.as_str(),
            Some("Products"),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistTupleItem::create(
                    "children",
                    vec![
                        (
                            pbx_file_ref.ios_framework.as_str(),
                            Some(format!("{}_ios.framework", xfw_name)),
                        ),
                        (
                            pbx_file_ref.ios_tests_xc.as_str(),
                            Some(format!("{}_iosTests.xctest", xfw_name)),
                        ),
                        (
                            pbx_file_ref.macos_framework.as_str(),
                            Some(format!("{}_macos.framework", xfw_name)),
                        ),
                        (
                            pbx_file_ref.macos_tests_xc.as_str(),
                            Some(format!("{}_macosTests.xctest", xfw_name)),
                        ),
                    ],
                ),
                PlistKeyValueItem::create_value_type("name", None, "Products", None),
                PlistKeyValueItem::create_value_type("sourceTree", None, "\"<group>\"", None),
            ],
        );

        let ios_item = PlistKeyValueItem::create_array_type(
            ios_id.as_str(),
            Some(format!("{}_ios", xfw_name).as_str()),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistTupleItem::create(
                    "children",
                    vec![
                        (
                            pbx_file_ref.ios_h.as_str(),
                            Some(format!("{}_ios.h", xfw_name)),
                        ),
                        (
                            pbx_file_ref.ios_docc.as_str(),
                            Some(format!("{}_ios.docc", xfw_name)),
                        ),
                    ],
                ),
                PlistKeyValueItem::create_value_type(
                    "path",
                    None,
                    format!("\"{}_ios\"", xfw_name).as_str(),
                    None,
                ),
                PlistKeyValueItem::create_value_type("sourceTree", None, "\"<group>\"", None),
            ],
        );

        let ios_tests_item = PlistKeyValueItem::create_array_type(
            ios_tests_id.as_str(),
            Some(format!("{}_iosTests", xfw_name).as_str()),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistTupleItem::create(
                    "children",
                    vec![(
                        pbx_file_ref.ios_tests_swift.as_str(),
                        Some(format!("{}_iosTests.swift", xfw_name)),
                    )],
                ),
                PlistKeyValueItem::create_value_type(
                    "path",
                    None,
                    format!("\"{}_iosTests\"", xfw_name).as_str(),
                    None,
                ),
                PlistKeyValueItem::create_value_type("sourceTree", None, "\"<group>\"", None),
            ],
        );

        let macos_item = PlistKeyValueItem::create_array_type(
            macos_id.as_str(),
            Some(format!("{}_macos", xfw_name).as_str()),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistTupleItem::create(
                    "children",
                    vec![
                        (
                            pbx_file_ref.macos_h.as_str(),
                            Some(format!("{}_macos.h", xfw_name)),
                        ),
                        (
                            pbx_file_ref.macos_docc.as_str(),
                            Some(format!("{}_macos.docc", xfw_name)),
                        ),
                    ],
                ),
                PlistKeyValueItem::create_value_type(
                    "path",
                    None,
                    format!("\"{}_macos\"", xfw_name).as_str(),
                    None,
                ),
                PlistKeyValueItem::create_value_type("sourceTree", None, "\"<group>\"", None),
            ],
        );
        let macos_tests_item = PlistKeyValueItem::create_array_type(
            macos_tests_id.as_str(),
            Some(format!("{}_macosTests", xfw_name).as_str()),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistTupleItem::create(
                    "children",
                    vec![(
                        pbx_file_ref.macos_tests_swift.as_str(),
                        Some(format!("{}_macosTests.swift", xfw_name)),
                    )],
                ),
                PlistKeyValueItem::create_value_type(
                    "path",
                    None,
                    format!("\"{}_macosTests\"", xfw_name).as_str(),
                    None,
                ),
                PlistKeyValueItem::create_value_type("sourceTree", None, "\"<group>\"", None),
            ],
        );

        let frameworks_item = PlistKeyValueItem::create_array_type(
            frameworks_id.as_str(),
            Some("Frameworks"),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistTupleItem::create("children", vec![]),
                PlistKeyValueItem::create_value_type("name", None, "Frameworks", None),
                PlistKeyValueItem::create_value_type("sourceTree", None, "\"<group>\"", None),
            ],
        );

        let groups_id = id_maker(id_base, kind, format!("{}_Groups", xfw_name).as_str());
        let mut items = vec![
            (ios_id.as_str(), Some(format!("{}_ios", xfw_name))),
            (
                ios_tests_id.as_str(),
                Some(format!("{}_iosTests", xfw_name)),
            ),
            (macos_id.as_str(), Some(format!("{}_macos", xfw_name))),
            (
                macos_tests_id.as_str(),
                Some(format!("{}_macosTests", xfw_name)),
            ),
            (products_id.as_str(), Some("Products".to_string())),
            (frameworks_id.as_str(), Some("Frameworks".to_string())),
        ];
        pbx_file_ref
            .udl_files_hashmap
            .iter()
            .for_each(|(key, val)| items.push((val.as_str(), Some(key.to_string()))));

        let groups_item = PlistKeyValueItem::create_array_type(
            groups_id.as_str(),
            None,
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistTupleItem::create("children", items),
                PlistKeyValueItem::create_value_type("sourceTree", None, "\"<group>\"", None),
            ],
        );

        let file_ref_ids = PBXGroupIds {
            groups: groups_id,
            products: products_id,
            ios: ios_id,
            ios_tests: ios_tests_id,
            macos: macos_id,
            macos_tests: macos_tests_id,
            frameworks: frameworks_id,
        };
        (
            file_ref_ids,
            PlistItem::SectionItem(PlistSectionItem {
                name: kind.to_string(),
                items: vec![
                    groups_item,
                    products_item,
                    ios_item,
                    ios_tests_item,
                    macos_item,
                    macos_tests_item,
                    frameworks_item,
                ],
            }),
        )
    }

    pub(crate) fn create_pbx_shell_script_build_phase_section(
        cargo_relative_path_to_xcode_project: &str,
        cargo_package_name: &str,
        target_name: &str,
        xfw_name: &str,
        id_base: u64,
        id_maker: fn(u64, kind: &str, name: &str) -> String,
    ) -> (PBXShellScriptBuildPhaseIds, PlistItem) {
        let kind = "PBXShellScriptBuildPhase";
        let ios_id = id_maker(id_base, kind, format!("{}_ios", xfw_name).as_str());
        let macos_id = id_maker(id_base, kind, format!("{}_macos", xfw_name).as_str());

        let ios_item = PlistKeyValueItem::create_array_type(ios_id.as_str(), Some("ShellScript"),
                                                                 vec![
                                                                     PlistKeyValueItem::create_value_type("isa", None, kind, None),
                                                                     PlistKeyValueItem::create_value_type("buildActionMask", None, "2147483647", None),
                                                                     PlistTupleItem::create("files", vec![]),
                                                                     PlistTupleItem::create("inputFileListPaths", vec![]),
                                                                     PlistTupleItem::create("inputPaths", vec![]),
                                                                     PlistTupleItem::create("outputFileListPaths", vec![]),
                                                                     PlistTupleItem::create("outputPaths", vec![]),
                                                                     PlistKeyValueItem::create_value_type("runOnlyForDeploymentPostprocessing", None, "0", None),
                                                                     PlistKeyValueItem::create_value_type("shellPath", None, "/bin/sh", None),
                                                                     PlistKeyValueItem::create_value_type("shellScript", None,
                                                                                                          format!(r#"" bash $SRCROOT/{0}/rust-xc-universal-binary.sh lib{1}.a {2} \"$SRCROOT/{0}\" ""#,
                                                                                                                  cargo_relative_path_to_xcode_project, target_name, cargo_package_name).as_str(), None),
                                                                 ]);
        let macos_item = PlistKeyValueItem::create_array_type(macos_id.as_str(), Some("ShellScript"),
                                                            vec![
                                                                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                                                                PlistKeyValueItem::create_value_type("buildActionMask", None, "2147483647", None),
                                                                PlistTupleItem::create("files", vec![]),
                                                                PlistTupleItem::create("inputFileListPaths", vec![]),
                                                                PlistTupleItem::create("inputPaths", vec![]),
                                                                PlistTupleItem::create("outputFileListPaths", vec![]),
                                                                PlistTupleItem::create("outputPaths", vec![]),
                                                                PlistKeyValueItem::create_value_type("runOnlyForDeploymentPostprocessing", None, "0", None),
                                                                PlistKeyValueItem::create_value_type("shellPath", None, "/bin/sh", None),
                                                                PlistKeyValueItem::create_value_type("shellScript", None,
                                                                                                     format!(r#"" bash $SRCROOT/{0}/rust-xc-universal-binary.sh lib{1}.a {2} \"$SRCROOT/{0}\" ""#,
                                                                                                             cargo_relative_path_to_xcode_project, target_name, cargo_package_name).as_str(), None),
                                                            ]);

        let file_ref_ids = PBXShellScriptBuildPhaseIds {
            ios: ios_id,
            macos: macos_id,
        };
        (
            file_ref_ids,
            PlistItem::SectionItem(PlistSectionItem {
                name: kind.to_string(),
                items: vec![ios_item, macos_item],
            }),
        )
    }

    pub(crate) fn create_pbx_xc_build_configuration_section(
        base_bundle_identifier: &str,
        cargo_relative_path_to_xcode_project: &str,
        cargo_lib_name: &str,
        xfw_name: &str,
        id_base: u64,
        id_maker: fn(u64, kind: &str, name: &str) -> String,
    ) -> (XCBuildConfigurationIds, PlistItem) {
        let kind = "XCBuildConfiguration";

        let mut shared_debug_build_settings = PlistKeyValueItem::create_value_type_by_array(vec![
            ("ALWAYS_SEARCH_USER_PATHS", "NO"),
            ("CLANG_ANALYZER_NONNULL", "YES"),
            ("CLANG_ANALYZER_NUMBER_OBJECT_CONVERSION", "YES_AGGRESSIVE"),
            ("CLANG_CXX_LANGUAGE_STANDARD", "\"gnu++17\""),
            ("CLANG_CXX_LIBRARY", "\"libc++\""),
            ("CLANG_ENABLE_MODULES", "YES"),
            ("CLANG_ENABLE_OBJC_ARC", "YES"),
            ("CLANG_ENABLE_OBJC_WEAK", "YES"),
            ("CLANG_WARN_BLOCK_CAPTURE_AUTORELEASING", "YES"),
            ("CLANG_WARN_BOOL_CONVERSION", "YES"),
            ("CLANG_WARN_COMMA", "YES"),
            ("CLANG_WARN_CONSTANT_CONVERSION", "YES"),
            ("CLANG_WARN_DEPRECATED_OBJC_IMPLEMENTATIONS", "YES"),
            ("CLANG_WARN_DIRECT_OBJC_ISA_USAGE", "YES_ERROR"),
            ("CLANG_WARN_DOCUMENTATION_COMMENTS", "YES"),
            ("CLANG_WARN_EMPTY_BODY", "YES"),
            ("CLANG_WARN_ENUM_CONVERSION", "YES"),
            ("CLANG_WARN_INFINITE_RECURSION", "YES"),
            ("CLANG_WARN_INT_CONVERSION", "YES"),
            ("CLANG_WARN_NON_LITERAL_NULL_CONVERSION", "YES"),
            ("CLANG_WARN_OBJC_IMPLICIT_RETAIN_SELF", "YES"),
            ("CLANG_WARN_OBJC_LITERAL_CONVERSION", "YES"),
            ("CLANG_WARN_OBJC_ROOT_CLASS", "YES_ERROR"),
            ("CLANG_WARN_QUOTED_INCLUDE_IN_FRAMEWORK_HEADER", "YES"),
            ("CLANG_WARN_RANGE_LOOP_ANALYSIS", "YES"),
            ("CLANG_WARN_STRICT_PROTOTYPES", "YES"),
            ("CLANG_WARN_SUSPICIOUS_MOVE", "YES"),
            ("CLANG_WARN_UNGUARDED_AVAILABILITY", "YES_AGGRESSIVE"),
            ("CLANG_WARN_UNREACHABLE_CODE", "YES"),
            ("CLANG_WARN__DUPLICATE_METHOD_MATCH", "YES"),
            ("COPY_PHASE_STRIP", "NO"),
            ("CURRENT_PROJECT_VERSION", "1"),
            ("DEBUG_INFORMATION_FORMAT", "dwarf"),
            ("ENABLE_BITCODE", "NO"),
            ("ENABLE_STRICT_OBJC_MSGSEND", "YES"),
            ("ENABLE_TESTABILITY", "YES"),
            ("GCC_C_LANGUAGE_STANDARD", "gnu11"),
            ("GCC_DYNAMIC_NO_PIC", "NO"),
            ("GCC_NO_COMMON_BLOCKS", "YES"),
            ("GCC_OPTIMIZATION_LEVEL", "0"),
            ("GCC_WARN_64_TO_32_BIT_CONVERSION", "YES"),
            ("GCC_WARN_ABOUT_RETURN_TYPE", "YES_ERROR"),
            ("GCC_WARN_UNDECLARED_SELECTOR", "YES"),
            ("GCC_WARN_UNINITIALIZED_AUTOS", "YES_AGGRESSIVE"),
            ("GCC_WARN_UNUSED_FUNCTION", "YES"),
            ("GCC_WARN_UNUSED_VARIABLE", "YES"),
            ("OTHER_LDFLAGS", format!("\"-l{}\"", cargo_lib_name).as_str()),
            (
                "HEADER_SEARCH_PATHS",
                format!("\"$(SRCROOT)/rust_libs/{}\"", xfw_name).as_str(),
            ),
            ("IPHONEOS_DEPLOYMENT_TARGET", "15.2"),
            (
                "\"LIBRARY_SEARCH_PATHS[sdk=iphoneos*]\"",
                format!(
                    "\"$(SRCROOT)/{}/target/universal/debug/ios\"",
                    cargo_relative_path_to_xcode_project
                )
                .as_str(),
            ),
            (
                "\"LIBRARY_SEARCH_PATHS[sdk=iphonesimulator*]\"",
                format!(
                    "\"$(SRCROOT)/{}/target/universal/debug/ios_sim\"",
                    cargo_relative_path_to_xcode_project
                )
                .as_str(),
            ),
            (
                "\"LIBRARY_SEARCH_PATHS[sdk=macosx*]\"",
                format!(
                    "\"$(SRCROOT)/{}/target/universal/debug/osx\"",
                    cargo_relative_path_to_xcode_project
                )
                .as_str(),
            ),
            ("MTL_ENABLE_DEBUG_INFO", "INCLUDE_SOURCE"),
            ("MTL_FAST_MATH", "YES"),
            ("ONLY_ACTIVE_ARCH", "YES"),
            ("SDKROOT", "iphoneos"),
            ("SWIFT_ACTIVE_COMPILATION_CONDITIONS", "DEBUG"),
            (
                "SWIFT_INCLUDE_PATHS",
                format!("\"$(SRCROOT)/rust_libs/{}\"", xfw_name).as_str(),
            ),
            ("SWIFT_OPTIMIZATION_LEVEL", "\"-Onone\""),
            ("VERSIONING_SYSTEM", "\"apple-generic\""),
            ("VERSION_INFO_PREFIX", "\"\""),
        ]);
        shared_debug_build_settings.push(PlistTupleItem::create(
            "GCC_PREPROCESSOR_DEFINITIONS",
            vec![("\"DEBUG=1\"", None), ("\"$(inherited)\"", None)],
        ));
        let shared_debug_id =
            id_maker(id_base, kind, format!("{}-shared-Debug", xfw_name).as_str());
        let shared_debug_item = PlistKeyValueItem::create_array_type(
            shared_debug_id.as_str(),
            Some("Debug"),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_array_type(
                    "buildSettings",
                    None,
                    shared_debug_build_settings,
                ),
                PlistKeyValueItem::create_value_type("name", None, "Debug", None),
            ],
        );

        let shared_release_build_settings = PlistKeyValueItem::create_value_type_by_array(vec![
            ("ALWAYS_SEARCH_USER_PATHS", "NO"),
            ("CLANG_ANALYZER_NONNULL", "YES"),
            ("CLANG_ANALYZER_NUMBER_OBJECT_CONVERSION", "YES_AGGRESSIVE"),
            ("CLANG_CXX_LANGUAGE_STANDARD", "\"gnu++17\""),
            ("CLANG_CXX_LIBRARY", "\"libc++\""),
            ("CLANG_ENABLE_MODULES", "YES"),
            ("CLANG_ENABLE_OBJC_ARC", "YES"),
            ("CLANG_ENABLE_OBJC_WEAK", "YES"),
            ("CLANG_WARN_BLOCK_CAPTURE_AUTORELEASING", "YES"),
            ("CLANG_WARN_BOOL_CONVERSION", "YES"),
            ("CLANG_WARN_COMMA", "YES"),
            ("CLANG_WARN_CONSTANT_CONVERSION", "YES"),
            ("CLANG_WARN_DEPRECATED_OBJC_IMPLEMENTATIONS", "YES"),
            ("CLANG_WARN_DIRECT_OBJC_ISA_USAGE", "YES_ERROR"),
            ("CLANG_WARN_DOCUMENTATION_COMMENTS", "YES"),
            ("CLANG_WARN_EMPTY_BODY", "YES"),
            ("CLANG_WARN_ENUM_CONVERSION", "YES"),
            ("CLANG_WARN_INFINITE_RECURSION", "YES"),
            ("CLANG_WARN_INT_CONVERSION", "YES"),
            ("CLANG_WARN_NON_LITERAL_NULL_CONVERSION", "YES"),
            ("CLANG_WARN_OBJC_IMPLICIT_RETAIN_SELF", "YES"),
            ("CLANG_WARN_OBJC_LITERAL_CONVERSION", "YES"),
            ("CLANG_WARN_OBJC_ROOT_CLASS", "YES_ERROR"),
            ("CLANG_WARN_QUOTED_INCLUDE_IN_FRAMEWORK_HEADER", "YES"),
            ("CLANG_WARN_RANGE_LOOP_ANALYSIS", "YES"),
            ("CLANG_WARN_STRICT_PROTOTYPES", "YES"),
            ("CLANG_WARN_SUSPICIOUS_MOVE", "YES"),
            ("CLANG_WARN_UNGUARDED_AVAILABILITY", "YES_AGGRESSIVE"),
            ("CLANG_WARN_UNREACHABLE_CODE", "YES"),
            ("CLANG_WARN__DUPLICATE_METHOD_MATCH", "YES"),
            ("COPY_PHASE_STRIP", "NO"),
            ("CURRENT_PROJECT_VERSION", "1"),
            ("DEBUG_INFORMATION_FORMAT", "\"dwarf-with-dsym\""),
            ("ENABLE_BITCODE", "NO"),
            ("ENABLE_NS_ASSERTIONS", "NO"),
            ("ENABLE_STRICT_OBJC_MSGSEND", "YES"),
            ("GCC_C_LANGUAGE_STANDARD", "gnu11"),
            ("GCC_NO_COMMON_BLOCKS", "YES"),
            ("GCC_WARN_64_TO_32_BIT_CONVERSION", "YES"),
            ("GCC_WARN_ABOUT_RETURN_TYPE", "YES_ERROR"),
            ("GCC_WARN_UNDECLARED_SELECTOR", "YES"),
            ("GCC_WARN_UNINITIALIZED_AUTOS", "YES_AGGRESSIVE"),
            ("GCC_WARN_UNUSED_FUNCTION", "YES"),
            ("GCC_WARN_UNUSED_VARIABLE", "YES"),
            ("OTHER_LDFLAGS", format!("\"-l{}\"", cargo_lib_name).as_str()),
            (
                "HEADER_SEARCH_PATHS",
                format!("\"$(SRCROOT)/rust_libs/{}\"", xfw_name).as_str(),
            ),
            ("IPHONEOS_DEPLOYMENT_TARGET", "15.2"),
            (
                "\"LIBRARY_SEARCH_PATHS[sdk=iphoneos*]\"",
                format!(
                    "\"$(SRCROOT)/{}/target/universal/release/ios\"",
                    cargo_relative_path_to_xcode_project
                )
                .as_str(),
            ),
            (
                "\"LIBRARY_SEARCH_PATHS[sdk=iphonesimulator*]\"",
                format!(
                    "\"$(SRCROOT)/{}/target/universal/release/ios_sim\"",
                    cargo_relative_path_to_xcode_project
                )
                .as_str(),
            ),
            (
                "\"LIBRARY_SEARCH_PATHS[sdk=macosx*]\"",
                format!(
                    "\"$(SRCROOT)/{}/target/universal/release/osx\"",
                    cargo_relative_path_to_xcode_project
                )
                .as_str(),
            ),
            ("MTL_ENABLE_DEBUG_INFO", "NO"),
            ("MTL_FAST_MATH", "YES"),
            ("SDKROOT", "iphoneos"),
            ("SWIFT_COMPILATION_MODE", "wholemodule"),
            (
                "SWIFT_INCLUDE_PATHS",
                format!("\"$(SRCROOT)/rust_libs/{}\"", xfw_name).as_str(),
            ),
            ("SWIFT_OPTIMIZATION_LEVEL", "\"-O\""),
            ("VALIDATE_PRODUCT", "YES"),
            ("VERSIONING_SYSTEM", "\"apple-generic\""),
            ("VERSION_INFO_PREFIX", "\"\""),
        ]);

        let shared_release_id = id_maker(
            id_base,
            kind,
            format!("{}-shared-Release", xfw_name).as_str(),
        );
        let shared_release_item = PlistKeyValueItem::create_array_type(
            shared_release_id.as_str(),
            Some("Release"),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_array_type(
                    "buildSettings",
                    None,
                    shared_release_build_settings,
                ),
                PlistKeyValueItem::create_value_type("name", None, "Release", None),
            ],
        );

        let get_ios_config = || {
            let mut ios_debug_build_settings = PlistKeyValueItem::create_value_type_by_array(vec![
                ("CODE_SIGN_STYLE", "Automatic"),
                ("CURRENT_PROJECT_VERSION", "1"),
                ("DEFINES_MODULE", "YES"),
                ("DEVELOPMENT_TEAM", "\"\""),
                ("DYLIB_COMPATIBILITY_VERSION", "1"),
                ("DYLIB_CURRENT_VERSION", "1"),
                ("DYLIB_INSTALL_NAME_BASE", "\"@rpath\""),
                ("GENERATE_INFOPLIST_FILE", "YES"),
                ("INFOPLIST_KEY_NSHumanReadableCopyright", "\"\""),
                ("INSTALL_PATH", "\"$(LOCAL_LIBRARY_DIR)/Frameworks\""),
                ("MARKETING_VERSION", "1.0"),
                (
                    "PRODUCT_BUNDLE_IDENTIFIER",
                    format!("\"{}.{}-ios\"", base_bundle_identifier, xfw_name).as_str(),
                ),
                ("PRODUCT_NAME", format!("{}", xfw_name).as_str() ), //"\"$(TARGET_NAME:c99extidentifier)\""),
                ("SKIP_INSTALL", "YES"),
                ("SWIFT_EMIT_LOC_STRINGS", "YES"),
                ("SWIFT_VERSION", "5.0"),
                ("TARGETED_DEVICE_FAMILY", "\"1,2\""),
            ]);
            ios_debug_build_settings.push(PlistTupleItem::create(
                "LD_RUNPATH_SEARCH_PATHS",
                vec![
                    ("\"$(inherited)\"", None),
                    ("\"@executable_path/Frameworks\"", None),
                    ("\"@loader_path/Frameworks\"", None),
                ],
            ));
            ios_debug_build_settings
        };

        let ios_debug_id = id_maker(id_base, kind, format!("{}_ios-Debug", xfw_name).as_str());
        let ios_debug_item = PlistKeyValueItem::create_array_type(
            ios_debug_id.as_str(),
            Some("Debug"),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_array_type("buildSettings", None, get_ios_config()),
                PlistKeyValueItem::create_value_type("name", None, "Debug", None),
            ],
        );

        let ios_release_id = id_maker(id_base, kind, format!("{}_ios-Release", xfw_name).as_str());
        let ios_release_item = PlistKeyValueItem::create_array_type(
            ios_release_id.as_str(),
            Some("Release"),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_array_type("buildSettings", None, get_ios_config()),
                PlistKeyValueItem::create_value_type("name", None, "Release", None),
            ],
        );
        // ----- IOS Debug Tests
        let get_ios_tests_build_settings = || {
            PlistKeyValueItem::create_value_type_by_array(vec![
                ("CODE_SIGN_STYLE", "Automatic"),
                ("CURRENT_PROJECT_VERSION", "1"),
                ("DEVELOPMENT_TEAM", "\"\""),
                ("GENERATE_INFOPLIST_FILE", "YES"),
                ("MARKETING_VERSION", "1.0"),
                (
                    "PRODUCT_BUNDLE_IDENTIFIER",
                    format!("\"{}.{}-iosTest\"", base_bundle_identifier, xfw_name).as_str(),
                ),
                ("PRODUCT_NAME", format!("{}", xfw_name).as_str() ),
                ("SWIFT_EMIT_LOC_STRINGS", "NO"),
                ("SWIFT_VERSION", "5.0"),
                ("TARGETED_DEVICE_FAMILY", "\"1,2\""),
            ])
        };
        let ios_tests_debug_id = id_maker(
            id_base,
            kind,
            format!("{}_iosTests-Debug", xfw_name).as_str(),
        );
        let ios_tests_debug_item = PlistKeyValueItem::create_array_type(
            ios_tests_debug_id.as_str(),
            Some("Debug"),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_array_type(
                    "buildSettings",
                    None,
                    get_ios_tests_build_settings(),
                ),
                PlistKeyValueItem::create_value_type("name", None, "Debug", None),
            ],
        );

        let ios_tests_release_id = id_maker(
            id_base,
            kind,
            format!("{}_iosTests-Release", xfw_name).as_str(),
        );
        let ios_tests_release_item = PlistKeyValueItem::create_array_type(
            ios_tests_release_id.as_str(),
            Some("Release"),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_array_type(
                    "buildSettings",
                    None,
                    get_ios_tests_build_settings(),
                ),
                PlistKeyValueItem::create_value_type("name", None, "Release", None),
            ],
        );

        let get_macos_build_settings = || {
            let mut items = PlistKeyValueItem::create_value_type_by_array(vec![
                ("CODE_SIGN_STYLE", "Automatic"),
                ("COMBINE_HIDPI_IMAGES", "YES"),
                ("CURRENT_PROJECT_VERSION", "1"),
                ("DEFINES_MODULE", "YES"),
                ("DEVELOPMENT_TEAM", "\"\""),
                ("DYLIB_COMPATIBILITY_VERSION", "1"),
                ("DYLIB_CURRENT_VERSION", "1"),
                ("DYLIB_INSTALL_NAME_BASE", "\"@rpath\""),
                ("GENERATE_INFOPLIST_FILE", "YES"),
                ("INFOPLIST_KEY_NSHumanReadableCopyright", "\"\""),
                ("INSTALL_PATH", "\"$(LOCAL_LIBRARY_DIR)/Frameworks\""),
                ("MACOSX_DEPLOYMENT_TARGET", "12.1"),
                ("MARKETING_VERSION", "1.0"),
                ("MODULEMAP_FILE", "\"\""),
                (
                    "PRODUCT_BUNDLE_IDENTIFIER",
                    format!("\"{}.{}-macos\"", base_bundle_identifier, xfw_name).as_str(),
                ),
                ("PRODUCT_NAME", "\"$(TARGET_NAME:c99extidentifier)\""),
                ("SDKROOT", "macosx"),
                ("SKIP_INSTALL", "YES"),
                ("SWIFT_EMIT_LOC_STRINGS", "YES"),
                ("SWIFT_VERSION", "5.0"),
            ]);
            items.push(PlistTupleItem::create(
                "LD_RUNPATH_SEARCH_PATHS",
                vec![
                    ("\"$(inherited)\"", None),
                    ("\"@executable_path/../Frameworks\"", None),
                    ("\"@loader_path/Frameworks\"", None),
                ],
            ));
            items
        };

        let macos_debug_id = id_maker(id_base, kind, format!("{}_macos-Debug", xfw_name).as_str());
        let macos_debug_item = PlistKeyValueItem::create_array_type(
            macos_debug_id.as_str(),
            Some("Debug"),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_array_type(
                    "buildSettings",
                    None,
                    get_macos_build_settings(),
                ),
                PlistKeyValueItem::create_value_type("name", None, "Debug", None),
            ],
        );

        let macos_release_id =
            id_maker(id_base, kind, format!("{}_macos-Release", xfw_name).as_str());
        let macos_release_item = PlistKeyValueItem::create_array_type(
            macos_release_id.as_str(),
            Some("Release"),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_array_type(
                    "buildSettings",
                    None,
                    get_macos_build_settings(),
                ),
                PlistKeyValueItem::create_value_type("name", None, "Release", None),
            ],
        );

        let get_macos_tests_build_settings = || {
            PlistKeyValueItem::create_value_type_by_array(vec![
                ("CODE_SIGN_STYLE", "Automatic"),
                ("CURRENT_PROJECT_VERSION", "1"),
                ("DEVELOPMENT_TEAM", "\"\""),
                ("GENERATE_INFOPLIST_FILE", "YES"),
                ("MACOSX_DEPLOYMENT_TARGET", "12.1"),
                ("MARKETING_VERSION", "1.0"),
                (
                    "PRODUCT_BUNDLE_IDENTIFIER",
                    format!("\"{}.{}-macosTests\"", base_bundle_identifier, xfw_name).as_str(),
                ),
                ("PRODUCT_NAME", "\"$(TARGET_NAME)\""),
                ("SDKROOT", "macosx"),
                ("SWIFT_EMIT_LOC_STRINGS", "NO"),
                ("SWIFT_VERSION", "5.0"),
            ])
        };

        let macos_tests_debug_id = id_maker(
            id_base,
            kind,
            format!("{}_macosTests-Debug", xfw_name).as_str(),
        );
        let macos_tests_debug_item = PlistKeyValueItem::create_array_type(
            macos_tests_debug_id.as_str(),
            Some("Debug"),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_array_type(
                    "buildSettings",
                    None,
                    get_macos_tests_build_settings(),
                ),
                PlistKeyValueItem::create_value_type("name", None, "Debug", None),
            ],
        );

        let macos_tests_release_id = id_maker(
            id_base,
            kind,
            format!("{}_macosTests-Release", xfw_name).as_str(),
        );
        let macos_tests_release_item = PlistKeyValueItem::create_array_type(
            macos_tests_release_id.as_str(),
            Some("Release"),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_array_type(
                    "buildSettings",
                    None,
                    get_macos_tests_build_settings(),
                ),
                PlistKeyValueItem::create_value_type("name", None, "Release", None),
            ],
        );

        let ids = XCBuildConfigurationIds {
            shared_debug: shared_debug_id,
            shared_release: shared_release_id,
            ios_debug: ios_debug_id,
            ios_release: ios_release_id,
            ios_tests_debug: ios_tests_debug_id,
            ios_tests_release: ios_tests_release_id,
            macos_debug: macos_debug_id,
            macos_release: macos_release_id,
            macos_tests_debug: macos_tests_debug_id,
            macos_tests_release: macos_tests_release_id,
        };

        (
            ids,
            PlistItem::SectionItem(PlistSectionItem {
                name: kind.to_string(),
                items: vec![
                    shared_debug_item,
                    shared_release_item,
                    ios_debug_item,
                    ios_release_item,
                    ios_tests_debug_item,
                    ios_tests_release_item,
                    macos_debug_item,
                    macos_release_item,
                    macos_tests_debug_item,
                    macos_tests_release_item,
                ],
            }),
        )
    }

    pub(crate) fn create_pbx_xc_configuration_list_section(
        pbx_build_config_ref_ids: &XCBuildConfigurationIds,
        xfw_name: &str,
        id_base: u64,
        id_maker: fn(u64, kind: &str, name: &str) -> String,
    ) -> (XCConfigurationListIds, PlistItem) {
        let kind = "XCConfigurationList";

        let shared_build_id = id_maker(
            id_base,
            kind,
            format!("{}-shared-config", xfw_name).as_str(),
        );
        let shared_build_item = PlistKeyValueItem::create_array_type(
            shared_build_id.as_str(),
            Some(format!("Build configuration list for PBXProject \"{}\"", xfw_name).as_str()),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistTupleItem::create(
                    "buildConfigurations",
                    vec![
                        (
                            pbx_build_config_ref_ids.shared_debug.as_str(),
                            Some("Debug".to_string()),
                        ),
                        (
                            pbx_build_config_ref_ids.shared_release.as_str(),
                            Some("Release".to_string()),
                        ),
                    ],
                ),
                PlistKeyValueItem::create_value_type(
                    "defaultConfigurationIsVisible",
                    None,
                    "0",
                    None,
                ),
                PlistKeyValueItem::create_value_type(
                    "defaultConfigurationName",
                    None,
                    "Release",
                    None,
                ),
            ],
        );

        let ios_build_id = id_maker(id_base, kind, format!("{}_ios-config", xfw_name).as_str());
        let ios_build_item = PlistKeyValueItem::create_array_type(
            ios_build_id.as_str(),
            Some(
                format!(
                    "Build configuration list for PBXNativeTarget \"{}_ios\"",
                    xfw_name
                )
                .as_str(),
            ),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistTupleItem::create(
                    "buildConfigurations",
                    vec![
                        (
                            pbx_build_config_ref_ids.ios_debug.as_str(),
                            Some("Debug".to_string()),
                        ),
                        (
                            pbx_build_config_ref_ids.ios_release.as_str(),
                            Some("Release".to_string()),
                        ),
                    ],
                ),
                PlistKeyValueItem::create_value_type(
                    "defaultConfigurationIsVisible",
                    None,
                    "0",
                    None,
                ),
                PlistKeyValueItem::create_value_type(
                    "defaultConfigurationName",
                    None,
                    "Release",
                    None,
                ),
            ],
        );

        let ios_tests_build_id =
            id_maker(id_base, kind, format!("{}_iosTests-config", xfw_name).as_str());
        let ios_tests_build_item = PlistKeyValueItem::create_array_type(
            ios_tests_build_id.as_str(),
            Some(
                format!(
                    "Build configuration list for PBXNativeTarget \"{}_iosTest\"",
                    xfw_name
                )
                .as_str(),
            ),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistTupleItem::create(
                    "buildConfigurations",
                    vec![
                        (
                            pbx_build_config_ref_ids.ios_tests_debug.as_str(),
                            Some("Debug".to_string()),
                        ),
                        (
                            pbx_build_config_ref_ids.ios_tests_release.as_str(),
                            Some("Release".to_string()),
                        ),
                    ],
                ),
                PlistKeyValueItem::create_value_type(
                    "defaultConfigurationIsVisible",
                    None,
                    "0",
                    None,
                ),
                PlistKeyValueItem::create_value_type(
                    "defaultConfigurationName",
                    None,
                    "Release",
                    None,
                ),
            ],
        );

        let macos_build_id = id_maker(id_base, kind, format!("{}_macos-config", xfw_name).as_str());
        let macos_build_item = PlistKeyValueItem::create_array_type(
            macos_build_id.as_str(),
            Some(
                format!(
                    "Build configuration list for PBXNativeTarget \"{}_macos\"",
                    xfw_name
                )
                .as_str(),
            ),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistTupleItem::create(
                    "buildConfigurations",
                    vec![
                        (
                            pbx_build_config_ref_ids.macos_debug.as_str(),
                            Some("Debug".to_string()),
                        ),
                        (
                            pbx_build_config_ref_ids.macos_release.as_str(),
                            Some("Release".to_string()),
                        ),
                    ],
                ),
                PlistKeyValueItem::create_value_type(
                    "defaultConfigurationIsVisible",
                    None,
                    "0",
                    None,
                ),
                PlistKeyValueItem::create_value_type(
                    "defaultConfigurationName",
                    None,
                    "Release",
                    None,
                ),
            ],
        );

        let macos_test_build_id =
            id_maker(id_base, kind, format!("{}_macosTests-config", xfw_name).as_str());
        let macos_test_build_item = PlistKeyValueItem::create_array_type(
            macos_test_build_id.as_str(),
            Some(
                format!(
                    "Build configuration list for PBXNativeTarget \"{}_macosTests\"",
                    xfw_name
                )
                .as_str(),
            ),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistTupleItem::create(
                    "buildConfigurations",
                    vec![
                        (
                            pbx_build_config_ref_ids.macos_tests_debug.as_str(),
                            Some("Debug".to_string()),
                        ),
                        (
                            pbx_build_config_ref_ids.macos_tests_release.as_str(),
                            Some("Release".to_string()),
                        ),
                    ],
                ),
                PlistKeyValueItem::create_value_type(
                    "defaultConfigurationIsVisible",
                    None,
                    "0",
                    None,
                ),
                PlistKeyValueItem::create_value_type(
                    "defaultConfigurationName",
                    None,
                    "Release",
                    None,
                ),
            ],
        );

        let ids = XCConfigurationListIds {
            shared: shared_build_id,
            ios: ios_build_id,
            ios_tests: ios_tests_build_id,
            macos: macos_build_id,
            macos_test: macos_test_build_id,
        };
        (
            ids,
            PlistItem::SectionItem(PlistSectionItem {
                name: kind.to_string(),
                items: vec![
                    shared_build_item,
                    ios_build_item,
                    ios_tests_build_item,
                    macos_build_item,
                    macos_test_build_item,
                ],
            }),
        )
    }

    pub(crate) fn create_pbx_target_dependency_section(
        xfw_name: &str,
        id_base: u64,
        id_maker: fn(u64, kind: &str, name: &str) -> String,
    ) -> (PBXTargetDependencyIds, PlistItem) {
        let kind = "PBXTargetDependency";

        let native_target_kind = "PBXNativeTarget";
        let ios_native_target_id = id_maker(
            id_base,
            native_target_kind,
            format!("{}_ios", xfw_name).as_str(),
        );
        let macos_native_target_id = id_maker(
            id_base,
            native_target_kind,
            format!("{}_macos", xfw_name).as_str(),
        );

        let container_item_proxy_kind = "PBXContainerItemProxy";
        let ios_container_item_proxy_id = id_maker(
            id_base,
            container_item_proxy_kind,
            format!("{}_ios", xfw_name).as_str(),
        );
        let macos_container_item_proxy_id = id_maker(
            id_base,
            container_item_proxy_kind,
            format!("{}_macos", xfw_name).as_str(),
        );

        let ios_id = id_maker(id_base, kind, format!("{}_ios", xfw_name).as_str());
        let macos_id = id_maker(id_base, kind, format!("{}_macos", xfw_name).as_str());
        let ios_item = PlistKeyValueItem::create_array_type(
            ios_id.as_str(),
            Some(kind),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type(
                    "target",
                    None,
                    ios_native_target_id.as_str(),
                    Some(format!("{}_ios", xfw_name).as_str()),
                ),
                PlistKeyValueItem::create_value_type(
                    "targetProxy",
                    None,
                    ios_container_item_proxy_id.as_str(),
                    Some(container_item_proxy_kind),
                ),
            ],
        );

        let macos_item = PlistKeyValueItem::create_array_type(
            macos_id.as_str(),
            Some(kind),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type(
                    "target",
                    None,
                    macos_native_target_id.as_str(),
                    Some(format!("{}_macos", xfw_name).as_str()),
                ),
                PlistKeyValueItem::create_value_type(
                    "targetProxy",
                    None,
                    macos_container_item_proxy_id.as_str(),
                    Some(container_item_proxy_kind),
                ),
            ],
        );

        let ids = PBXTargetDependencyIds {
            ios_id,
            macos_id,
            ios_native_target_id,
            macos_native_target_id,
            ios_container_item_proxy_id,
            macos_container_item_proxy_id,
        };
        (
            ids,
            PlistItem::SectionItem(PlistSectionItem {
                name: kind.to_string(),
                items: vec![ios_item, macos_item],
            }),
        )
    }

    pub(crate) fn create_pbx_container_item_proxy_section(
        project_object_id: &str,
        pbx_target_dependency_ids: &PBXTargetDependencyIds,
        xfw_name: &str,
        id_base: u64,
        id_maker: fn(u64, kind: &str, name: &str) -> String,
    ) -> (PBXContainerItemProxyIds, PlistItem) {
        let kind = "PBXContainerItemProxy";

        let ios_id = id_maker(id_base, kind, format!("{}_ios", xfw_name).as_str());
        let ios_item = PlistKeyValueItem::create_array_type(
            ios_id.as_str(),
            Some(kind),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type("containerPortal", None, project_object_id, Some("Project object")),
                PlistKeyValueItem::create_value_type(
                    "proxyType",
                    None,
                    "1",
                    None,
                ),
                PlistKeyValueItem::create_value_type(
                    "remoteGlobalIDString",
                    None,
                    pbx_target_dependency_ids.ios_native_target_id.as_str(),
                    None,
                ),
                PlistKeyValueItem::create_value_type(
                    "remoteInfo",
                    None,
                    format!("\"{}_ios\"", xfw_name).as_str(),
                    None,
                ),
            ],
        );

        let macos_id = id_maker(id_base, kind, format!("{}_macos", xfw_name).as_str());
        let macos_item = PlistKeyValueItem::create_array_type(
            macos_id.as_str(),
            Some(kind),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type("containerPortal", None, project_object_id, Some("Project object")),
                PlistKeyValueItem::create_value_type(
                    "proxyType",
                    None,
                    "1",
                    None,
                ),
                PlistKeyValueItem::create_value_type(
                    "remoteGlobalIDString",
                    None,
                    pbx_target_dependency_ids.macos_native_target_id.as_str(),
                    None,
                ),
                PlistKeyValueItem::create_value_type(
                    "remoteInfo",
                    None,
                    format!("\"{}_macos\"", xfw_name).as_str(),
                    None,
                ),
            ],
        );

        let ids = PBXContainerItemProxyIds {
            ios: ios_id,
            macos: macos_id,
        };
        (
            ids,
            PlistItem::SectionItem(PlistSectionItem {
                name: kind.to_string(),
                items: vec![ios_item, macos_item],
            }),
        )
    }

    pub(crate) fn create_pbx_native_target_section(
        pbx_target_dependency_ids: &PBXTargetDependencyIds,
        pbx_config_list: &XCConfigurationListIds,
        pbx_file_ref_ids: &PBXFileReferenceIds,
        pbx_shell_script_ids_ref: &PBXShellScriptBuildPhaseIds,
        pbx_sources_build_ids_ref: &PBXSourcesBuildPhaseIds,
        pbx_framework_build_ids_ref: &PBXFrameworksBuildPhaseIds,
        pbx_headers_build_ids_ref: &PBXHeadersBuildPhaseIds,
        pbx_resources_build_ids_ref: &PBXResourcesBuildPhaseIds,
        pbx_build_rule_ids_ref: &PBXBuildRuleIds,
        xfw_name: &str,
        id_base: u64,
        id_maker: fn(u64, kind: &str, name: &str) -> String,
    ) -> (PBXNativeTargetIds, PlistItem) {
        let kind = "PBXNativeTarget";

        let ios_id = id_maker(id_base, kind, format!("{}_ios", xfw_name).as_str());
        let ios_item = PlistKeyValueItem::create_array_type(
            ios_id.as_str(),
            Some(format!("{}_ios", xfw_name).as_str()),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type(
                    "buildConfigurationList",
                    None,
                    pbx_config_list.ios.as_str(),
                    Some(
                        format!(
                            "Build configuration list for PBXNativeTarget \"{}_ios\"",
                            xfw_name
                        )
                        .as_str(),
                    ),
                ),
                PlistTupleItem::create(
                    "buildPhases",
                    vec![
                        (
                            pbx_shell_script_ids_ref.ios.as_str(),
                            Some("ShellScript".to_string()),
                        ),
                        (
                            pbx_headers_build_ids_ref.headers_ios.as_str(),
                            Some("Headers".to_string()),
                        ),
                        (
                            pbx_sources_build_ids_ref.ios_docc.as_str(),
                            Some("Sources".to_string()),
                        ),
                        (
                            pbx_framework_build_ids_ref.ios_framework.as_str(),
                            Some("Frameworks".to_string()),
                        ),
                        (
                            pbx_resources_build_ids_ref.ios.as_str(),
                            Some("Resources".to_string()),
                        ),
                    ],
                ),
                PlistTupleItem::create(
                    "buildRules",
                    vec![(
                        pbx_build_rule_ids_ref.ios.as_str(),
                        Some("PBXBuildRule".to_string()),
                    )],
                ),
                PlistKeyValueItem::create_value_type(
                    "name",
                    None,
                    format!("\"{}_ios\"", xfw_name).as_str(),
                    None,
                ),
                PlistKeyValueItem::create_value_type(
                    "productName",
                    None,
                    format!("\"{}_ios\"", xfw_name).as_str(),
                    None,
                ),
                PlistKeyValueItem::create_value_type(
                    "productReference",
                    None,
                    pbx_file_ref_ids.ios_framework.as_str(),
                    Some(format!("\"{}_ios.framework\"", xfw_name).as_str()),
                ),
                PlistKeyValueItem::create_value_type(
                    "productType",
                    None,
                    "\"com.apple.product-type.framework\"",
                    None,
                ),
            ],
        );

        let ios_tests_id = id_maker(id_base, kind, format!("{}_iosTests", xfw_name).as_str());
        let ios_tests_item = PlistKeyValueItem::create_array_type(
            ios_tests_id.as_str(),
            Some(format!("{}_iosTests", xfw_name).as_str()),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type(
                    "buildConfigurationList",
                    None,
                    pbx_config_list.ios_tests.as_str(),
                    Some(
                        format!(
                            "Build configuration list for PBXNativeTarget \"{}_iosTests\"",
                            xfw_name
                        )
                        .as_str(),
                    ),
                ),
                PlistTupleItem::create(
                    "buildPhases",
                    vec![
                        (
                            pbx_sources_build_ids_ref.ios_tests_swift.as_str(),
                            Some("Sources".to_string()),
                        ),
                        (
                            pbx_framework_build_ids_ref.ios_tests_xctest.as_str(),
                            Some("Frameworks".to_string()),
                        ),
                        (
                            pbx_resources_build_ids_ref.ios_tests.as_str(),
                            Some("Resources".to_string()),
                        ),
                    ],
                ),
                PlistTupleItem::create("buildRules", vec![]),
                PlistTupleItem::create(
                    "dependencies",
                    vec![(
                        pbx_target_dependency_ids.ios_id.as_str(),
                        Some("PBXTargetDependency".to_string()),
                    )],
                ),
                PlistKeyValueItem::create_value_type(
                    "name",
                    None,
                    format!("\"{}_iosTests\"", xfw_name).as_str(),
                    None,
                ),
                PlistKeyValueItem::create_value_type(
                    "productName",
                    None,
                    format!("\"{}_iosTests\"", xfw_name).as_str(),
                    None,
                ),
                PlistKeyValueItem::create_value_type(
                    "productReference",
                    None,
                    pbx_file_ref_ids.ios_tests_xc.as_str(),
                    Some(format!("\"{}_iosTests.xctest\"", xfw_name).as_str()),
                ),
                PlistKeyValueItem::create_value_type(
                    "productType",
                    None,
                    "\"com.apple.product-type.bundle.unit-test\"",
                    None,
                ),
            ],
        );

        let macos_id = id_maker(id_base, kind, format!("{}_macos", xfw_name).as_str());
        let macos_item = PlistKeyValueItem::create_array_type(
            macos_id.as_str(),
            Some(format!("{}_macos", xfw_name).as_str()),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type(
                    "buildConfigurationList",
                    None,
                    pbx_config_list.macos.as_str(),
                    Some(
                        format!(
                            "Build configuration list for PBXNativeTarget \"{}_ios\"",
                            xfw_name
                        )
                        .as_str(),
                    ),
                ),
                PlistTupleItem::create(
                    "buildPhases",
                    vec![
                        (
                            pbx_shell_script_ids_ref.macos.as_str(),
                            Some("ShellScript".to_string()),
                        ),
                        (
                            pbx_headers_build_ids_ref.headers_macos.as_str(),
                            Some("Headers".to_string()),
                        ),
                        (
                            pbx_sources_build_ids_ref.macos_docc.as_str(),
                            Some("Sources".to_string()),
                        ),
                        (
                            pbx_framework_build_ids_ref.macos_framework.as_str(),
                            Some("Frameworks".to_string()),
                        ),
                        (
                            pbx_resources_build_ids_ref.macos.as_str(),
                            Some("Resources".to_string()),
                        ),
                    ],
                ),
                PlistTupleItem::create(
                    "buildRules",
                    vec![(
                        pbx_build_rule_ids_ref.macos.as_str(),
                        Some("PBXBuildRule".to_string()),
                    )],
                ),
                PlistKeyValueItem::create_value_type(
                    "name",
                    None,
                    format!("\"{}_macos\"", xfw_name).as_str(),
                    None,
                ),
                PlistKeyValueItem::create_value_type(
                    "productName",
                    None,
                    format!("\"{}_macos\"", xfw_name).as_str(),
                    None,
                ),
                PlistKeyValueItem::create_value_type(
                    "productReference",
                    None,
                    pbx_file_ref_ids.macos_framework.as_str(),
                    Some(format!("\"{}_macos.framework\"", xfw_name).as_str()),
                ),
                PlistKeyValueItem::create_value_type(
                    "productType",
                    None,
                    "\"com.apple.product-type.framework\"",
                    None,
                ),
            ],
        );

        let macos_tests_id = id_maker(id_base, kind, format!("{}_macosTests", xfw_name).as_str());
        let macos_tests_item = PlistKeyValueItem::create_array_type(
            macos_tests_id.as_str(),
            Some(format!("{}_macosTests", xfw_name).as_str()),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_value_type(
                    "buildConfigurationList",
                    None,
                    pbx_config_list.macos_test.as_str(),
                    Some(
                        format!(
                            "Build configuration list for PBXNativeTarget \"{}_macosTests\"",
                            xfw_name
                        )
                        .as_str(),
                    ),
                ),
                PlistTupleItem::create(
                    "buildPhases",
                    vec![
                        (
                            pbx_sources_build_ids_ref.macos_tests_swift.as_str(),
                            Some("Sources".to_string()),
                        ),
                        (
                            pbx_framework_build_ids_ref.macos_tests_xctest.as_str(),
                            Some("Frameworks".to_string()),
                        ),
                        (
                            pbx_resources_build_ids_ref.macos_tests.as_str(),
                            Some("Resources".to_string()),
                        ),
                    ],
                ),
                PlistTupleItem::create("buildRules", vec![]),
                PlistTupleItem::create(
                    "dependencies",
                    vec![(
                        pbx_target_dependency_ids.macos_id.as_str(),
                        Some("PBXTargetDependency".to_string()),
                    )],
                ),
                PlistKeyValueItem::create_value_type(
                    "name",
                    None,
                    format!("\"{}_macosTests\"", xfw_name).as_str(),
                    None,
                ),
                PlistKeyValueItem::create_value_type(
                    "productName",
                    None,
                    format!("\"{}_macosTests\"", xfw_name).as_str(),
                    None,
                ),
                PlistKeyValueItem::create_value_type(
                    "productReference",
                    None,
                    pbx_file_ref_ids.macos_tests_xc.as_str(),
                    Some(format!("\"{}_macosTests.xctest\"", xfw_name).as_str()),
                ),
                PlistKeyValueItem::create_value_type(
                    "productType",
                    None,
                    "\"com.apple.product-type.bundle.unit-test\"",
                    None,
                ),
            ],
        );

        let ids = PBXNativeTargetIds {
            ios: ios_id,
            ios_tests: ios_tests_id,
            macos: macos_id,
            macos_test: macos_tests_id,
        };
        (
            ids,
            PlistItem::SectionItem(PlistSectionItem {
                name: kind.to_string(),
                items: vec![ios_item, ios_tests_item, macos_item, macos_tests_item],
            }),
        )
    }

    pub(crate) fn create_pbx_project_section(
        project_object_id: &str,
        pbx_group_ids: &PBXGroupIds,
        pbx_native_target_ids: &PBXNativeTargetIds,
        pbx_xc_config_list_ids: &XCConfigurationListIds,
        xfw_name: &str,
    ) -> PlistItem {
        let kind = "PBXProject";
        let project_item = PlistKeyValueItem::create_array_type(
            project_object_id,
            Some("Project object"),
            vec![
                PlistKeyValueItem::create_value_type("isa", None, kind, None),
                PlistKeyValueItem::create_array_type( "attributes", None, vec![
                    PlistKeyValueItem::create_value_type("BuildIndependentTargetsInParallel", None, "1", None),
                    PlistKeyValueItem::create_value_type("LastSwiftUpdateCheck", None, "1320", None),
                    PlistKeyValueItem::create_value_type("LastUpgradeCheck", None, "1320", None),
                    PlistKeyValueItem::create_array_type( "TargetAttributes", None, vec![
                        //PlistKeyValueItem::create_array_type(""
                        PlistKeyValueItem::create_array_type(pbx_native_target_ids.ios.as_str(), None, vec![PlistKeyValueItem::create_value_type("CreatedOnToolsVersion", None, "13.2.1", None),]),
                        PlistKeyValueItem::create_array_type(pbx_native_target_ids.ios_tests.as_str(), None, vec![PlistKeyValueItem::create_value_type("CreatedOnToolsVersion", None, "13.2.1", None),]),
                        PlistKeyValueItem::create_array_type(pbx_native_target_ids.macos.as_str(), None, vec![PlistKeyValueItem::create_value_type("CreatedOnToolsVersion", None, "13.2.1", None),]),
                        PlistKeyValueItem::create_array_type(pbx_native_target_ids.macos_test.as_str(), None, vec![PlistKeyValueItem::create_value_type("CreatedOnToolsVersion", None, "13.2.1", None),]),
                    ]),
                ]),
                PlistKeyValueItem::create_value_type("buildConfigurationList", None, pbx_xc_config_list_ids.shared.as_str(), Some(format!("Build configuration list for PBXProject \"{}\"", xfw_name).as_str())),
                PlistKeyValueItem::create_value_type("compatibilityVersion", None, "\"Xcode 13.0\"", None),
                PlistKeyValueItem::create_value_type("developmentRegion", None, "en", None),
                PlistKeyValueItem::create_value_type("hasScannedForEncodings", None, "0", None),
                PlistTupleItem::create("knownRegions", vec![ ( "en", None, ), ( "Base", None, ), ], ),
                PlistKeyValueItem::create_value_type("mainGroup", None, pbx_group_ids.groups.as_str(), None),
                PlistKeyValueItem::create_value_type("productRefGroup", None, pbx_group_ids.products.as_str(), Some("Products")),
                PlistKeyValueItem::create_value_type("projectDirPath", None, "\"\"", None),
                PlistKeyValueItem::create_value_type("projectRoot", None, "\"\"", None),
                PlistTupleItem::create("targets", vec![
                    ( pbx_native_target_ids.ios.as_str(), Some(format!("{}_ios", xfw_name)) ),
                    ( pbx_native_target_ids.ios_tests.as_str(), Some(format!("{}_iosTests", xfw_name)) ),
                    ( pbx_native_target_ids.macos.as_str(), Some(format!("{}_macos", xfw_name)) ),
                    ( pbx_native_target_ids.macos_test.as_str(), Some(format!("{}_macosTests", xfw_name)) )
                ], ),
            ]);

        PlistItem::SectionItem(PlistSectionItem {
            name: kind.to_string(),
            items: vec![project_item],
        })
    }
}


