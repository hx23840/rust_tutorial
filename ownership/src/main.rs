fn main() {
    let mut s = String::from("hello ");
    s.push_str("world");

    println!("{}", s);

    let s1 = String::from("hello ");
    let mut s2 = s1.clone();
    let mut s3 = s2.clone();

    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);

    let i = print(&mut s2);
    println!("{}", i);

    let s5: String = if 1 != 2 {
        println!("{}", s3);
        s3.push_str(" new s3");
        String::from("j")
    } else {
        s1
    };

    println!("{}", s3);

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用

    let r3 = &mut s; // 没问题
    println!("{}", r3);

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    //此位置之后 r3就不可以再使用了
    // println!("{}", r3);
}

fn print(s4: &mut String) -> usize {
    println!("{}", s4);
    println!("{}", s4);
    s4.push_str("new s4");
    s4.len()
}
