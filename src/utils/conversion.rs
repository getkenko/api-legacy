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

pub fn kg_to_st_lb(kg: f32) -> (f32, f32) {
    let total_st = kg / 6.35029318;
    let st = total_st.trunc();
    let decimal = total_st.fract();
    let lb = decimal * 14.0;
    (st, lb)
}

// imperial to metric
pub fn ft_in_to_cm(feets: i32, inches: i32) -> i32 {
    let cm = feets as f32 * 30.48 + inches as f32 * 2.54;
    cm.round() as _
}

pub fn lb_to_kg(lb: f32) -> f32 {
    lb * 0.45359237
}

pub fn st_lb_to_kg(st: f32, lb: f32) -> f32 {
    let total_lb = st * 14.0 + lb;
    total_lb * 45359237.0
}