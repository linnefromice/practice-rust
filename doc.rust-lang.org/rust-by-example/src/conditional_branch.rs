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

pub fn execute_conditional_branch_match() {
    execute_match_destruct_tuple((0, -2, 3));
}