pub fn execute_conditional_branch() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            break 'outer;
        }

        println!("This point will never be reached");
    }
    println!("Exited the outer loop");
}

fn execute_simple_match(num: usize, boolean: bool) {
    println!("Tell me about {}", num);
    match num {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);
}

fn execute_match_destruct_tuple(triple: (i32, i32, i32)) {
    println!("Tell me about {:?}", triple);

    match triple {
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..) => println!("First is `1`, and the rest doesn't matter"),
        (2, ..) => println!("First is `2`, and the rest doesn't matter"),
        (3, .., 4) => println!("First is `3`, last is `4` and the rest doesn't matter"),
        _ => println!("It doesn't matter what they are"),
    }
}

fn execute_match_destruct_pointer() {
    let reference = &4;

    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let value = 5;
    // ref を使用 -> reference を作成
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    let mut mut_value = 6;
    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10, `mut_value`: {:?}", m);
        }
    }
}

pub fn execute_conditional_branch_match() {
    execute_simple_match(13, true);
    execute_match_destruct_tuple((0, -2, 3));
    execute_match_destruct_pointer();
}

enum Level {
    Some(u32),
    High,
    Middle,
    Row
}

pub fn execute_conditional_branch_guard() {
    let level = Level::Some(0);

    match level {
        Level::Some(n) if n > 0 => println!("Some n>0"),
        Level::Some(n) if n == 0 => println!("Some n==0"),
        Level::Some(_) => println!("Some"), // need for compile error
        Level::High => println!("High"),
        Level::Middle => println!("Middle"),
        Level::Row => println!("Row"),
    }
}