fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let x = 5;

    println!("The value of x is: {}", x);
    println!("Hello, world!");

    println!("{a}, {b}", a = "Hello2", b = "world2!");

    let mut sum = 0;

    for i in 0..5 {
        sum += i;
        let text = if i % 2 == 0 { "even" } else { "odd" };

        println!("Hello, world! {} {}", i, text);
    }

    println!("sum is {}", sum);

    let a: i32 = add(20, 20);
    let b: i32 = add(20, 30);

    println!("a is {}", a);
    println!("b is {}", b);

    let i = 10;

    let i_ref = &i;
    println!("i ref is {}", i_ref);
    println!("i ref derrefed is {}", *i_ref);

    // References
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;
    let first = slice.get(0);
    let last = ints.get(5).unwrap_or(&-2);

    println!("first1 {:?}", slice.first());
    println!("first2 {:?}", ints.first());

    println!("last is {:?}", last);
}
