extern mod rsfml;

mod cpu;

#[test]
fn skip_if_Vx_is_NN() {
	// 0x3xNN
	let mut test = ::cpu::Cpu::new();
	let rom = [0x31, 0x20];
	test.V[0x1] = 0x20;
	test.load_vec(rom);
	test.cycle();
	assert!(test.pc == 0x204);

	test = ::cpu::Cpu::new();
	test.V[0x1] = 0x21;
	test.load_vec(rom);
	test.cycle();
	assert!(test.pc == 0x202);
}

#[test]
fn set_V_to_NN() {
	//0x6xNN
	let mut test = ::cpu::Cpu::new();
	let rom = [0x61, 0x11];
	test.load_vec(rom);
	test.cycle();
	assert!(test.V[0x1] == 0x11);
}

#[test]
fn skip_if_Vx_isnt_Vy() {
	// 0x9xy0
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

#[test]
fn set_I_to_NNN() {
	// 0xANNN
	let mut test = ::cpu::Cpu::new();
	let rom = [0xA1, 0x23];
	test.load_vec(rom);
	test.cycle();
	assert!(test.I == 0x123);
}

#[test]
fn fill_V0_to_Vx_from_memory() {
	// 0xFx65
	let mut test = ::cpu::Cpu::new();
	let rom = [0xF3, 0x65, 0x12, 0x34, 0x56, 0x78];
	test.load_vec(rom);
	test.I = 0x202;
	test.cycle();
	assert!(test.V[0] == 0x12);
	assert!(test.V[1] == 0x34);
	assert!(test.V[2] == 0x56);
	assert!(test.V[3] == 0x78);
}

#[test]
fn skip_if_key_is_pressed() {
	// 0xEx9E
	let mut test = ::cpu::Cpu::new();
	let rom = [0xE1, 0x9E];
	test.V[1] = 0xA;
	test.keys[0xA] = 1;
	test.load_vec(rom);
	test.cycle();
	assert!(test.pc == 0x204);

	test = ::cpu::Cpu::new();
	test.V[1] = 0xA;
	test.keys[0xA] = 0;
	test.load_vec(rom);
	test.cycle();
	assert!(test.pc == 0x202);
}

#[test]
fn skip_if_key_isnt_pressed() {
	// 0xExA1
	let mut test = ::cpu::Cpu::new();
	let rom = [0xE1, 0xA1];
	test.V[1] = 0xA;
	test.keys[0xA] = 0;
	test.load_vec(rom);
	test.cycle();
	assert!(test.pc == 0x204);

	test = ::cpu::Cpu::new();
	test.V[1] = 0xA;
	test.keys[0xA] = 1;
	test.load_vec(rom);
	test.cycle();
	assert!(test.pc == 0x202);
}