/// Takes metric height (centimeters) as an input and converts it to imperial system, returning tuple: (feets, inches)
pub fn metric_height_to_imperial(value: i32) -> (i32, i32) {
    // xdddddddddddddddd wtf is that system
    let total_feet = value as f32 / 30.48;
    let feet = total_feet.trunc();
    let decimal = total_feet.fract();
    let inches = (decimal * 12.0).round();

    (feet as i32, inches as i32)
}
