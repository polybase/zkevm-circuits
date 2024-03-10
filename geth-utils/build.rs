use std::{
    env,
    io::{self, Write},
};

fn main() {
    if let Ok(target) = env::var("TARGET") {
        if target.contains("android") || target.contains("ios") || target.contains("wasm") {
            println!("cargo:warning=Building for Android or iOS");
            return;
        }
    } else {
        println!("cargo:warning=Building for non-mobile platforms");
    }

    return
}

fn fail(message: String) {
    let _ = writeln!(
        io::stderr(),
        "\n\nError while building geth-utils: {message}\n\n"
    );
    std::process::exit(1);
}
