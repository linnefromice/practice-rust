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

pub fn execute_conditional_branch_match() {
    execute_simple_match(13, true);
}