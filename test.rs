extern mod rsfml;

mod cpu;

#[test]
fn set_V_to_NN() {
	let mut test = ::cpu::Cpu::new();
	let rom = [0x61, 0x11];
	test.load_vec(rom);
	test.cycle();
	assert!(test.V[0x1] == 0x11);
}

#[test]
fn skip_next_instruction() {
	let mut test = ::cpu::Cpu::new();
	let rom = [0x91, 0x20];
	test.V[0x1] = 1;
	test.V[0x2] = 2;
	test.load_vec(rom);
	test.cycle();
	assert!(test.pc == 0x204);

	test = ::cpu::Cpu::new();
	test.V[0x1] = 1;
	test.V[0x2] = 1;
	test.load_vec(rom);
	test.cycle();
	assert!(test.pc == 0x202);
}