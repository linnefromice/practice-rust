macro_rules! calculate {
    (eval $e:expr) => {
        {
            let val: usize = $e;
            println!("{} = {}", stringify!{$e}, val);
        }
    };
}

// macro_rules! calculate_second {
//     (eval $e:expr) => {
//         {
//             let val: usize = $e; // Force types to be integers
//             println!("{} = {}", stringify!{$e}, val);
//         }
//     };
//     (eval $e:expr, $(eval $es:expr),+) => {{
//         calculate! { eval $e }
//         calculate! { $(eval $es),+ }
//     }};
// }

pub fn execute() {
    calculate! {
        eval 1 + 2
    }

    calculate! {
        eval (1 + 2) * (3 / 4)
    }

    // calculate_second! {
    //     eval 1 + 2,
    //     eval 3 + 4,
    //     eval (2 * 3) + 1
    // }
}