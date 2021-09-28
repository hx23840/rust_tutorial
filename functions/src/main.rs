fn main() {
    println!("{}", another_function(5, 'h'));

    let x = 5;

    let y = {
        let x = x + 5;
        x + 1//;
    };

    println!("{}", y);
}

fn another_function(x: u32, unit_label: char) -> u32 {
    println!("The value of the x is {} label is {}", x, unit_label);
    5
}
