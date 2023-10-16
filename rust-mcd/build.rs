use std::{env, fs::File, io::Write, path::PathBuf};

use bindgen::{callbacks::{ParseCallbacks, TypeKind}, CargoCallbacks, EnumVariation};

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_file = out_path.join("bindings.rs");

    if std::env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() != "windows" {
        // Include the pregenerated file, to ease development on linux
        // Note that this library will currently not run on linux since it requires
        // mcdxdas.dll to be installed
        let pregenerated = include_bytes!("pregenerated.rs");
        let mut bindings = File::create(bindings_file).unwrap();
        bindings.write_all(pregenerated).unwrap();
    } else {
        // If we build on linux, we need to specify import where
        // to find the header files. We assume the xwin project is in use to provide
        // the required header files
        #[cfg(target_os = "linux")]
        const CLANG_ARGS: [&str; 4] = [
            "-I/xwin/crt/include",
            "-I/xwin/sdk/include/ucrt",
            "-I/xwin/sdk/include/um",
            "-I/xwin/sdk/include/shared",
        ];

        // When building on windows, all the header file should be in the path
        // already
        #[cfg(target_os = "windows")]
        const CLANG_ARGS: [&str; 0] = [];

        println!("cargo:rerun-if-changed=mcd_demo_basic_120412/src/mcd_api.h");

        // The bindgen::Builder is the main entry point
        // to bindgen, and lets you build up options for
        // the resulting bindings.
        let bindings = bindgen::Builder::default()
            // The input header we would like to generate
            // bindings for.
            .header("mcd_demo_basic_120412/src/mcd_api.h")
            .clang_args(CLANG_ARGS)
            .dynamic_library_name("DynamicMCDxDAS")
            .derive_default(true)
            .default_enum_style(EnumVariation::Rust {
                non_exhaustive: false,
            })
            // Enums that were identified to be non-exclusive
            // TODO: What about mcd_trace_type_et and mcd_trace_mode_et?
            .bitfield_enum("enum_mcd_error_event_et")
            .bitfield_enum("enum_mcd_mem_type_et")
            .bitfield_enum("enum_mcd_trig_type_et")
            .bitfield_enum("enum_mcd_trig_opt_et")
            .bitfield_enum("enum_mcd_trig_action_et")
            .bitfield_enum("enum_mcd_tx_access_opt_et")
            .bitfield_enum("enum_mcd_core_event_et")
            .bitfield_enum("enum_mcd_chl_attributes_et")
            .bitfield_enum("enum_mcd_trace_marker_et")
            // Finish the builder and generate the bindings.
            .generate()
            // Unwrap the Result and panic on failure.
            .expect("Unable to generate bindings");

        // Write the bindings to the $OUT_DIR/bindings.rs file.
        bindings
            .write_to_file(bindings_file)
            .expect("Couldn't write bindings!");
    }
}

#[derive(Debug)]
pub struct McdCallbacks(CargoCallbacks);

impl ParseCallbacks for McdCallbacks {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        self.0.will_parse_macro(name)
    }

    fn generated_name_override(
        &self,
        item_info: bindgen::callbacks::ItemInfo<'_>,
    ) -> Option<String> {
        self.0.generated_name_override(item_info)
    }

    fn int_macro(&self, name: &str, value: i64) -> Option<bindgen::callbacks::IntKind> {
        self.0.int_macro(name, value)
    }

    fn str_macro(&self, name: &str, value: &[u8]) {
        self.0.str_macro(name, value)
    }

    fn func_macro(&self, name: &str, value: &[&[u8]]) {
        self.0.func_macro(name, value)
    }

    fn enum_variant_behavior(
        &self,
        enum_name: Option<&str>,
        original_variant_name: &str,
        variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<bindgen::callbacks::EnumVariantCustomBehavior> {
        self.0
            .enum_variant_behavior(enum_name, original_variant_name, variant_value)
    }

    fn enum_variant_name(
        &self,
        enum_name: Option<&str>,
        original_variant_name: &str,
        variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<String> {
        self.0
            .enum_variant_name(enum_name, original_variant_name, variant_value)
    }

    fn item_name(&self, original_item_name: &str) -> Option<String> {
        self.0.item_name(original_item_name)
    }

    fn include_file(&self, filename: &str) {
        self.0.include_file(filename)
    }

    fn blocklisted_type_implements_trait(
        &self,
        name: &str,
        derive_trait: bindgen::callbacks::DeriveTrait,
    ) -> Option<bindgen::callbacks::ImplementsTrait> {
        self.0.blocklisted_type_implements_trait(name, derive_trait)
    }

    fn add_derives(&self, info: &bindgen::callbacks::DeriveInfo<'_>) -> Vec<String> {
        let mut original_derives = Vec::new();

        if info.kind == TypeKind::Enum {
            original_derives.push("num_enum::TryFromPrimitive".to_owned());
        }

        original_derives
    }

    fn process_comment(&self, comment: &str) -> Option<String> {
        self.0.process_comment(comment)
    }
}
