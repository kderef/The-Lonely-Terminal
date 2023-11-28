// faster Vector3 initialization
#[macro_export]
macro_rules! vec3 {
    ($x:literal, $y:literal, $z:literal) => {
        Vector3{x: $x, y: $y, z: $z}
    }
}