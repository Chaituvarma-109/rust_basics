#[derive(Debug)]
#[allow(dead_code)]
enum CarColor {
    Red,
    Blue,
    Green,
    Yellow,
}

#[derive(Debug)]
#[allow(dead_code)]
enum GivenResult<T, E> {
    Ok(T),
    Err(E),
}

#[derive(Debug)]
#[allow(dead_code)]
enum GivenOption<T> {
    None,
    Some(T),
}

#[allow(dead_code)]
fn create_car_color_blue() -> CarColor {
    let my_car_color: CarColor = CarColor::Blue;
    my_car_color
}

#[allow(dead_code)]
fn check_under_five(num_check: u8) -> GivenResult<u8, String> {
    if num_check < 5 {
        GivenResult::Ok(num_check)
    } else {
        GivenResult::Err("Number is too big ".to_string())
    }
}

#[allow(dead_code)]
fn check_under_five_built_in(num_check: u8) -> Result<u8, String> {
    if num_check < 5 {
        Ok(num_check)
    } else {
        Err("Number is too big ".to_string())
    }
}

#[allow(dead_code)]
fn remainder_zero(num_check: f32) -> GivenOption<f32> {
    let remainder: f32 = num_check % 10.0;

    if remainder != 0.0 {
        GivenOption::Some(remainder)
    } else {
        GivenOption::None
    }
}

#[allow(dead_code)]
fn remainder_zero_built_in(num_check: f32) -> Option<f32> {
    let remainder: f32 = num_check % 10.0;

    if remainder != 0.0 {
        Option::Some(remainder)
    } else {
        Option::None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[allow(dead_code)]
    fn test_enums() {
        let car_color: CarColor = create_car_color_blue();
        dbg!(car_color);

        let under_five_res: GivenResult<u8, String> = check_under_five(4);
        dbg!(under_five_res);

        let under_five_res: GivenResult<u8, String> = check_under_five(7);
        dbg!(under_five_res);

        let under_five_res_built_in: Result<u8, String> = check_under_five_built_in(4);
        println!("{:?}", under_five_res_built_in);

        let under_five_res_built_in: Result<u8, String> = check_under_five_built_in(7);
        println!("{:?}", under_five_res_built_in);

        let remainder_zero_res: GivenOption<f32> = remainder_zero(10.0);
        dbg!(remainder_zero_res);

        let remainder_zero_res: GivenOption<f32> = remainder_zero(11.45);
        dbg!(remainder_zero_res);

        let remainder_zero_res_built_in: Option<f32> = remainder_zero_built_in(10.0);
        dbg!(remainder_zero_res_built_in);

        let remainder_zero_res_built_in: Option<f32> = remainder_zero_built_in(11.45);
        dbg!(remainder_zero_res_built_in);
    }
}
