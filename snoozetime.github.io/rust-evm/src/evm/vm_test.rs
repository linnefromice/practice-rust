use super::vm::Vm;

#[test]
fn addition() {
    let binary: Vec<u8> = vec![0x60, 0x0f, 0x60, 0x01, 0x01, 0x00];
    let mut vm = Vm::new(binary);

    // execute three instructions.
    // push 0x0f
    vm.interpret();
    // push 0x01
    vm.interpret();
    // add
    vm.interpret();
    // halt
    vm.interpret();

    // Now make sure the stack size is 1 and contains 16.
    assert_eq!(1, vm.stack.len());
    assert_eq!(16, vm.stack[0].as_u32()); // this is panicking in case of overflow.
}