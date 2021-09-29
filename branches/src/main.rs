fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = if number < 5 {
        5
    } else {
        10
    };

    println!("The value of the number is {}", number);

    let mut count = 0;

    'continue_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'continue_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End of count = {}", count);

    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2;
        }
    };

    println!("The result is {}", result);

    while count != 0 {
        println!("{}", count);

        count -= 1;
    }

    println!("LIFTOFF!!!");

    let mut a = [10, 20, 30, 40, 50];
    a.reverse();

    for element in a.iter() {
        println!("The value of the element is {}", element);
    };

    for number in (1..10).rev() {
        println!("{}", number);
    }
}
