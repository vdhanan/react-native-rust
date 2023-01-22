#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn rust_multiply(a: f64, b: f64) -> f64;

    }
}

fn rust_multiply(a: f64, b: f64) -> f64 {
    a * b
}
