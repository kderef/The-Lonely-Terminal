fn profile() -> String {
    std::env::var("PROFILE").expect("cargo build profile in env")
}

fn main() {
    let profile = profile();

    if profile == "release" {
        static_vcruntime::metabuild();
    }
}
