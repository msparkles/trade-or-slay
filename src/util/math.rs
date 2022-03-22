pub fn angle_difference(from: &f32, to: &f32) -> f32 {
    (to - from).sin().atan2((to - from).cos())
}
