pub fn execute_closure() {
    let outer_var = 42;
    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_infered = |j: i32|  j + outer_var ;

    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_infered: {}", closure_infered(1));

    let one = || 1;
    println!("closure returning one: {}", one());
}

// for function to get closure
fn apply<F>(f: F)
    where
        F: FnOnce() { f(); }
// for function to get function
fn call_me<F: Fn()>(f: F) {
    f();
}
fn call_function() {
    println!("I'm a function!")
}

pub fn execute_closure_2() {
    let x = 7;
    let print = || println!("{}", x);
    apply(print);

    let closure = || println!("I'm a closure!");
    call_me(closure);
    call_me(call_function);
}

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();
    move || println!("This is a: {}", text)
}
fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();
    move || println!("This is a: {}", text)
}
fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();
    move || println!("This is a: {}", text)
}

pub fn execute_closure_3() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}