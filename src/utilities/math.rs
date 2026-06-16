pub fn map_range(
    value: u32,
    input_min: u32,
    input_max: u32,
    output_min: u32,
    output_max: u32,
) -> u32 {
    if input_max == input_min {
        return output_min;
    }

    (value - input_min) * (output_max - output_min) / (input_max - input_min) + output_min
}

pub fn constrain_f32(value: f32, min: f32, max: f32) -> f32 {
    value.clamp(min, max)
}

pub fn constrain_i32(value: i32, min: i32, max: i32) -> i32 {
    value.clamp(min, max)
}