fn main() {
 cc::Build::new()
    .cpp(true)
    .files([
        "../../core/src/node.cpp",
        "../../core/src/flow.cpp",
        "../../core/src/runner.cpp",
        "../../core/src/api.cpp",
    ])
    .include("../../core/src")
    .flag_if_supported("-std=c++20")
    .compile("loopdynamo");

    println!("cargo:rerun-if-changed=../../core/src");
}