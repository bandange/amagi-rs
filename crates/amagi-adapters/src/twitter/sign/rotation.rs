pub(super) fn convert_rotation_to_matrix(rotation_degrees: f64) -> [f64; 4] {
    let radians = rotation_degrees * std::f64::consts::PI / 180.0;
    [radians.cos(), -radians.sin(), radians.sin(), radians.cos()]
}
