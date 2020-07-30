extern crate bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    // println!("cargo:rustc-link-lib=bz2");
    // println!("cargo:rustc-link-lib=static=foo");


    println!("cargo:rerun-if-changed=wrapper.hpp");

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
        .clang_args(&["-x", "c++"])
        .clang_arg("-std=c++14")
        .clang_arg(format!("-I/{}", openvdb_path.join("include").display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // /usr/bin/c++ -DBOOST_ALL_NO_LIB -DBOOST_IOSTREAMS_DYN_LINK -DBOOST_SYSTEM_DYN_LINK -DOPENVDB_ABI_VERSION_NUMBER=7 -DOPENVDB_DLL -DOPENVDB_PRIVATE -DOPENVDB_USE_BLOSC -Dopenvdb_shared_EXPORTS -I/Users/neo/Developer/openvdb-sys/openvdb/openvdb/.. -I/Users/neo/Developer/openvdb-sys/openvdb/openvdb/. -isystem /usr/local/include -isystem /usr/local/include/OpenEXR -ffunction-sections -fdata-sections -fPIC -m64 -arch x86_64 -g -isysroot /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX10.15.sdk -fPIC -std=c++14 -o CMakeFiles/openvdb_shared.dir/math/Maps.cc.o -c /Users/neo/Developer/openvdb-sys/openvdb/openvdb/math/Maps.cc
}
