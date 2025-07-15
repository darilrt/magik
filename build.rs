fn main() {
    // Tell Cargo to rerun this build script if any file in the pages directory changes
    // The `pages/` directory contains your template files but you can adjust the path as needed
    println!("cargo:rerun-if-changed=pages/");

    // You can also specify individual files if preferred
    // println!("cargo:rerun-if-changed=pages/main.tmp");
    // println!("cargo:rerun-if-changed=pages/greeting.tmp");

    // TODO: Recursively watch all .tmp files in the pages directory
}
