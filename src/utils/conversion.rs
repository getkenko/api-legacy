// metric to imperial
pub fn cm_to_ft_in(value: i32) -> (i32, i32) {
    let total_feet = value as f32 / 30.48;
    let feet = total_feet.trunc();
    let decimal = total_feet.fract();
    let inches = (decimal * 12.0).round();
    (feet as _, inches as _)
}

pub fn kg_to_lb(kg: f32) -> f32 {
    kg / 0.45359237
}

// imperial to metric
pub fn ft_in_to_cm(feets: i32, inches: i32) -> i32 {
    let cm = feets as f32 * 30.48 + inches as f32 * 2.54;
    cm.round() as _
}

pub fn lb_to_kg(lb: f32) -> f32 {
    lb * 0.45359237
}