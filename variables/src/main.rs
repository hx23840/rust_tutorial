fn main() {
    let x = 5;
    println!("The value of the x is {}", x);

    let x = x + 1;

    {
        let x = x + 2;
        println!("The value of the x is {}", x);
    }

    println!("The value of the x is {}", x);

    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);

    let tup = (500, 6.4, 1);
    println!("{} {} {}", tup.0, tup.1, tup.2);

    let arr = [3; 5];
    println!("{}", arr[4]);
}
