fn main() {
    // Tell Cargo to rerun this build script if any file in the pages directory changes
    // The `templates/` directory contains your template files but you can adjust the path as needed
    println!("cargo:rerun-if-changed=templates/");

    // You can also specify individual files if preferred
    // println!("cargo:rerun-if-changed=templates/main.tmp");
    // println!("cargo:rerun-if-changed=templates/greeting.tmp");

    // Recursively watch all .tmp files in the templates directory
    println!("cargo:rerun-if-changed=templates/**/*.tmp");
}
