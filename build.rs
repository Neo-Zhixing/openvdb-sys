extern crate bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.hpp");

    // Build components: https://www.openvdb.org/documentation/doxygen/build.html#buildComponents
    let openvdb_path = PathBuf::from(cmake::Config::new("openvdb")
        .define("OPENVDB_BUILD_CORE", "ON")
        .define("OPENVDB_BUILD_BINARIES", "OFF")
        .define("OPENVDB_BUILD_VDB_PRINT", "OFF")
        .define("OPENVDB_BUILD_VDB_RENDER", "OFF")
        .define("OPENVDB_BUILD_VDB_VIEW", "OFF")
        .define("OPENVDB_BUILD_VDB_LOD", "OFF")
        .define("OPENVDB_BUILD_PYTHON_MODULE", "OFF")
        .define("OPENVDB_BUILD_UNITTESTS", "OFF")
        .define("OPENVDB_BUILD_HOUDINI_PLUGIN", "OFF")
        .define("OPENVDB_BUILD_MAYA_PLUGIN", "OFF")
        .define("OPENVDB_BUILD_DOCS", "OFF")
        .build());


    let bindings = bindgen::Builder::default()
        .header("wrapper.hpp")
        .clang_args(&["-x", "c++"]) // Force c++ on .h header files
        .clang_arg("-std=c++17")
        .clang_arg(format!("-I/{}", openvdb_path.join("include").display())) // Include OpenVDB header files
        .whitelist_recursively(false)
        .whitelist_type("openvdb::.*::tree::.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate OpenVDB bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write OpenVDB bindings!");
}
