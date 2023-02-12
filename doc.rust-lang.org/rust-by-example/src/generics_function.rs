struct A; 
struct SingleGen<T>(T);

fn gen_spec_t(_s: SingleGen<A>) {}
fn gen_spec_i32(_s: SingleGen<i32>) {}
fn generic_func<T>(_s: SingleGen<T>) {}

pub fn execute() {
    gen_spec_t(SingleGen(A));
    gen_spec_i32(SingleGen(8));
    generic_func::<i32>(SingleGen(8));
    generic_func(SingleGen(8));
}
