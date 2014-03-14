extern crate rsfml;
extern crate test;

use test::BenchHarness;

mod cpu;

fn load_vec(cpu: &mut ::cpu::Cpu, data: &[u8]) {
	let mut i = 0x200;
	for b in data.iter() {
		cpu.memory[i] = *b;
		i += 1;
	}
}

#[test]
fn clear_screen() {
	// 0x00E0
	let mut test = ::cpu::Cpu::new();
	let rom = [0x00, 0xE0];
	test.graphics = [1, ..64 * 32];
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.draw_flag == true);
	assert!(test.graphics == [0, ..64 * 32]);
}

#[test]
fn return_from_subroutine() {
	// 0x00EE
	let mut test = ::cpu::Cpu::new();
	let rom = [0x00, 0xEE];
	test.sp = 1;
	test.stack[0] = 0x234;
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.pc == 0x234);
}

#[test]
fn jump_to_NNN() {
	// 0x1NNN
	let mut test = ::cpu::Cpu::new();
	let rom = [0x12, 0x05, 0x11, 0x22, 0x33, 0x44, 0x55];
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.pc == 0x205);
}

#[test]
fn call_subroutine_at_NNN() {
	// 0x2NNN
	let mut test = ::cpu::Cpu::new();
	let rom = [0x22, 0x34];
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.sp == 1);
	assert!(test.stack[0] == 0x202);
	assert!(test.pc == 0x234);
}

#[test]
fn skip_if_Vx_is_NN() {
	// 0x3xNN
	let mut test = ::cpu::Cpu::new();
	let rom = [0x31, 0x20];
	test.v[0x1] = 0x20;
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.pc == 0x204);

	test = ::cpu::Cpu::new();
	test.v[0x1] = 0x21;
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.pc == 0x202);
}

#[test]
fn skip_if_Vx_isnt_NN() {
	// 0x4xNN
	let mut test = ::cpu::Cpu::new();
	let rom = [0x41, 0x20];
	test.v[0x1] = 0x20;
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.pc == 0x202);

	test = ::cpu::Cpu::new();
	test.v[0x1] = 0x21;
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.pc == 0x204);
}

#[test]
fn skip_if_Vx_is_Vy() {
	// 0x5xy0
	let mut test = ::cpu::Cpu::new();
	let rom = [0x51, 0x20];
	test.v[0x1] = 1;
	test.v[0x2] = 2;
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.pc == 0x202);

	test = ::cpu::Cpu::new();
	test.v[0x1] = 1;
	test.v[0x2] = 1;
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.pc == 0x204);	
}

#[test]
fn set_V_to_NN() {
	// 0x6xNN
	let mut test = ::cpu::Cpu::new();
	let rom = [0x61, 0x11];
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.v[0x1] == 0x11);
}

#[test]
fn add_NN_to_Vx() {
	// 0x7xNN
	let mut test = ::cpu::Cpu::new();
	let rom = [0x71, 0x11];
	test.v[0x1] = 0x22;
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.v[0x1] == 0x33);	
}

#[test]
fn set_Vx_to_Vy() {
	// 0x8xy0
	let a = 5;
	let b = 6;
	let mut test = ::cpu::Cpu::new();
	let rom = [0x81, 0x20];
	test.v[1] = a;
	test.v[2] = b;
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.v[1] == b);	
}

#[test]
fn set_Vx_to_Vx_OR_Vy() {
	// 0x8xy1
	let a = 5;
	let b = 6;
	let mut test = ::cpu::Cpu::new();
	let rom = [0x81, 0x21];
	test.v[1] = a;
	test.v[2] = b;
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.v[1] == a | b);	
}

#[test]
fn set_Vx_to_Vx_AND_Vy() {
	// 0x8xy2
	let a = 5;
	let b = 6;
	let mut test = ::cpu::Cpu::new();
	let rom = [0x81, 0x22];
	test.v[1] = a;
	test.v[2] = b;
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.v[1] == a & b);	
}

#[test]
fn set_Vx_to_Vx_XOR_Vy() {
	// 0x8xy3
	let a = 5;
	let b = 6;
	let mut test = ::cpu::Cpu::new();
	let rom = [0x81, 0x23];
	test.v[1] = a;
	test.v[2] = b;
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.v[1] == a ^ b);	
}

#[test]
fn add_Vy_to_Vx() {
	// 0x8xy4
	let a: u8 = 5;
	let b: u8 = 6;
	let mut test = ::cpu::Cpu::new();
	let rom = [0x81, 0x24];
	test.v[1] = a;
	test.v[2] = b;
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.v[0xF] == 0);
	assert!(test.v[1] == a + b);

	let c: u8 = 0xFF;
	let d: u8 = 0xFF;
	test = ::cpu::Cpu::new();
	test.v[1] = c;
	test.v[2] = d;
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.v[0xF] == 1);
	assert!(test.v[1] == c + d);
}

#[test]
fn subtract_Vy_from_Vx() {
	// 0x8xy5
	let a: u8 = 5;
	let b: u8 = 6;
	let mut test = ::cpu::Cpu::new();
	let rom = [0x81, 0x25];
	test.v[1] = b;
	test.v[2] = a;
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.v[0xF] == 1);
	assert!(test.v[1] == b - a);

	test = ::cpu::Cpu::new();
	test.v[1] = a;
	test.v[2] = b;
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.v[0xF] == 0);
	assert!(test.v[1] == a - b);
}

#[test]
fn shift_Vx_right() {
	// 0x8xN6
	let mut test = ::cpu::Cpu::new();
	let rom = [0x81, 0x06];
	let a = 1;
	test.v[0x1] = a;
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.v[0xF] == 1);
	assert!(test.v[0x1] == a >> 1);
}

#[test]
fn set_Vx_to_Vy_minus_Vx() {
	// 0x8xN7
	let mut test = ::cpu::Cpu::new();
	let rom = [0x81, 0x27];
	let a = 1;
	let b = 2;
	test.v[0x1] = a;
	test.v[0x2] = b;
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.v[0xF] == 1);
	assert!(test.v[0x1] == b - a);

	test = ::cpu::Cpu::new();
	test.v[0x1] = b;
	test.v[0x2] = a;
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.v[0xF] == 0);
	assert!(test.v[0x1] == a - b);
}

#[test]
fn shift_Vx_left() {
	// 0x8xNE
	let mut test = ::cpu::Cpu::new();
	let rom = [0x81, 0x0E];
	let a = 1;
	test.v[0x1] = a;
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.v[0xF] == 0);
	assert!(test.v[0x1] == a << 1);	
}

#[test]
fn skip_if_Vx_isnt_Vy() {
	// 0x9xy0
	let mut test = ::cpu::Cpu::new();
	let rom = [0x91, 0x20];
	test.v[0x1] = 1;
	test.v[0x2] = 2;
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.pc == 0x204);

	test = ::cpu::Cpu::new();
	test.v[0x1] = 1;
	test.v[0x2] = 1;
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.pc == 0x202);
}

#[test]
fn set_I_to_NNN() {
	// 0xANNN
	let mut test = ::cpu::Cpu::new();
	let rom = [0xA1, 0x23];
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.index == 0x123);
}

#[test]
fn jump_to_NNN_plus_V0() {
	// 0xBNNN
	let mut test = ::cpu::Cpu::new();
	let rom = [0xB1, 0x23];
	test.v[0] = 2;
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.pc == 0x125);	
}

#[test]
fn set_Vx_to_rand_AND_NN() {
	// 0xCNNN
	let mut test = ::cpu::Cpu::new();
	let rom = [0xC1, 0x23];
	test.v[1] = 2;
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.v[1] < 0xFF);
}

#[test]
fn draw_sprite() {
	// 0xDxyh
	let mut test = ::cpu::Cpu::new();
	let rom = [0xD1, 0x22, 0x12, 0x34, 0xA0, 0xC0];
	test.v[1] = 4;
	test.v[2] = 5;
	test.index = 0x204;
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.graphics[4 + 5 * 64] == 1);
	assert!(test.graphics[5 + 5 * 64] == 0);
	assert!(test.graphics[6 + 5 * 64] == 1);
	assert!(test.graphics[4 + 6 * 64] == 1);
	assert!(test.graphics[5 + 6 * 64] == 1);

	test.pc = 0x200;
	test.cycle();
	assert!(test.graphics[4 + 5 * 64] == 0);
	assert!(test.graphics[5 + 5 * 64] == 0);
	assert!(test.graphics[6 + 5 * 64] == 0);
	assert!(test.graphics[4 + 6 * 64] == 0);
	assert!(test.graphics[5 + 6 * 64] == 0);

	test = ::cpu::Cpu::new();
	test.v[1] = 63;
	test.v[2] = 31;
	test.index = 0x204;
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.graphics[63 + 31 * 64] == 1);
	assert!(test.graphics[0 + 31 * 64] == 0);
	assert!(test.graphics[1 + 31 * 64] == 1);
	assert!(test.graphics[63 + 0 * 64] == 1);
	assert!(test.graphics[0 + 0 * 64] == 1);
}

#[test]
fn skip_if_key_is_pressed() {
	// 0xEx9E
	let mut test = ::cpu::Cpu::new();
	let rom = [0xE1, 0x9E];
	test.v[1] = 0xA;
	test.keys[0xA] = 1;
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.pc == 0x204);

	test = ::cpu::Cpu::new();
	test.v[1] = 0xA;
	test.keys[0xA] = 0;
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.pc == 0x202);
}

#[test]
fn skip_if_key_isnt_pressed() {
	// 0xExA1
	let mut test = ::cpu::Cpu::new();
	let rom = [0xE1, 0xA1];
	test.v[1] = 0xA;
	test.keys[0xA] = 0;
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.pc == 0x204);

	test = ::cpu::Cpu::new();
	test.v[1] = 0xA;
	test.keys[0xA] = 1;
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.pc == 0x202);
}

#[test]
fn set_Vx_to_delay_timer() {
	// 0xFx07
	let mut test = ::cpu::Cpu::new();
	let rom = [0xF2, 0x07];
	test.delay_timer = 0xF1;
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.v[2] == 0xF1);
}

#[test]
fn wait_on_keypress(){
	// 0xFx0A
	let mut test = ::cpu::Cpu::new();
	let rom = [0xF2, 0x0A];
	load_vec(&mut test, rom);
	test.cycle();
	assert!(test.wait_register == 2);
	assert!(test.is_waiting());
}

#[test]
fn continue_after_wait(){
	// 0xFx0A
	let mut test = ::cpu::Cpu::new();
	test.wait_register = 2;
	test.set_wait_register(5);
	assert!(test.v[2] == 5);
	assert!(!test.is_waiting());		
}

#[test]
fn fill_V0_to_Vx_from_memory() {
	// 0xFx65
	let mut test = ::cpu::Cpu::new();
	let rom = [0xF3, 0x65, 0x12, 0x34, 0x56, 0x78];
	load_vec(&mut test, rom);
	test.index = 0x202;
	test.cycle();
	assert!(test.v[0] == 0x12);
	assert!(test.v[1] == 0x34);
	assert!(test.v[2] == 0x56);
	assert!(test.v[3] == 0x78);
}

#[bench]
fn loop_0_to_255(b: &mut BenchHarness) {
	let mut test = ::cpu::Cpu::new();
	let rom = [0x60, 0x00, // 200: set v0 to 0
			   0x70, 0x01, // 202: add 1 to v0
			   0x30, 0xFF, // 204: skip next instruction if v0 is FF
			   0x12, 0x02, // 206: jump to address 202
			   0x12, 0x08, // 208: jump to address 0x208
			  ];
	load_vec(&mut test, rom);
	b.iter(|| {
		test.pc = 0x200;
		while test.pc != 0x208 {
			test.cycle();
		}
	} );
}

#[bench]
fn draw_sprite_bench(b: &mut BenchHarness) {
	let mut test = ::cpu::Cpu::new();
	let rom = [0xD1, 0x22, 0x12, 0x34, 0xA0, 0xC0];
	test.v[1] = 4;
	test.v[2] = 5;
	test.index = 0x204;
	load_vec(&mut test, rom);
	b.iter(|| {
		test.pc = 0x200;
		test.cycle();
	});
}