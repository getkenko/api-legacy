use crate::models::{database::enums::{HeightUnit, WeightUnit}, dto::users::UpdateUserDetailsRequest, errors::{AppResult, ValidationError}};

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

pub fn weight_from_unit(unit: &WeightUnit, details: &UpdateUserDetailsRequest) -> AppResult<f32> {
    let weight = match unit {
        WeightUnit::Kg => details.weight_kg.ok_or(ValidationError::MissingKgWeight)?,
        WeightUnit::Lb => {
            let lb = details.weight_lb.ok_or(ValidationError::MissingLbWeight)?;
            lb_to_kg(lb)
        }
    };

    if weight <= 0.0 || weight >= 10000.0 {
        return Err(ValidationError::InvalidWeight)?;
    }

    Ok(weight)
}

pub fn height_from_unit(unit: &HeightUnit, details: &UpdateUserDetailsRequest) -> AppResult<i32> {
    let height = match unit {
        HeightUnit::Cm => details.height_cm.ok_or(ValidationError::MissingCmHeight)?,
        HeightUnit::FtIn => {
            let ft = details.height_ft.ok_or(ValidationError::MissingFtInHeight)?;
            let inches = details.height_in.ok_or(ValidationError::MissingFtInHeight)?;
            ft_in_to_cm(ft, inches)
        }
    };

    if height <= 0 || height >= 300 {
        return Err(ValidationError::InvalidHeight)?;
    }

    Ok(height)
}
