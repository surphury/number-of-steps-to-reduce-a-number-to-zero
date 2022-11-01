mod test;

pub fn number_of_steps(num: i32) -> i32 {
    let mut number = num;
    let mut steps = 0;

    while number != 0 {
        steps += 1;

        let is_even = number % 2 == 0;

        if is_even {
            number = number / 2
        } else {
            number = number - 1
        };
    }

    steps
}

fn main() {
    number_of_steps(12);
}
