extern crate bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.hpp");

    // Static Links
    println!("cargo:rustc-link-lib=static=openvdb");

    // Dynamic Links
    println!("cargo:rustc-link-lib=dylib=c++");   // stdlib
    println!("cargo:rustc-link-lib=dylib=blosc"); // Blosc
    println!("cargo:rustc-link-lib=dylib=tbb");   // TBB::tbb
    println!("cargo:rustc-link-lib=dylib=Half");  // ilmbase::Half
    println!("cargo:rustc-link-lib=dylib=z");     // ZLIB

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

    // Include the compiled libopenvdb static lib in the search path
    println!("cargo:rustc-link-search={}", openvdb_path.join("lib").display());

    let bindings = bindgen::Builder::default()
        .header("wrapper.hpp")
        .clang_args(&["-x", "c++"]) // Force c++ on .h header files
        .clang_arg("-std=c++17")
        .clang_arg(format!("-I/{}", openvdb_path.join("include").display())) // Include OpenVDB header files
        .whitelist_recursively(true)
        .whitelist_function("openvdb::.*::initialize")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate OpenVDB bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write OpenVDB bindings!");
}
