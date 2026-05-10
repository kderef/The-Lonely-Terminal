fn main() {
    if std::env::var("PROFILE").unwrap() == "release" {
        static_vcruntime::metabuild();
    }
}
