#[cfg(feature = "reload")]
use hot_lib::*;
#[cfg(not(feature = "reload"))]
use lib::*;

#[cfg(feature = "reload")]
#[hot_lib_reloader::hot_module(dylib = "lib")]
mod hot_lib {
    hot_functions_from_file!("lib/src/lib.rs");
}

fn main() {
    loop {
        println!("{}", get_str());
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
