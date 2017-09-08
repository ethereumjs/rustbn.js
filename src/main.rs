extern crate bn;
extern crate rand;
extern crate num;
extern crate rustc_serialize;
extern crate hex_slice;

use std::os::raw::c_char;
use std::ffi::CStr;

use rustc_serialize::hex::FromHex;
use rustc_serialize::hex::ToHex;
use std::io::{self, Read};


#[derive(Debug)]
pub struct Error(pub &'static str);

impl From<&'static str> for Error {
	fn from(val: &'static str) -> Self {
		Error(val)
	}
}


fn read_fr(reader: &mut io::Chain<&[u8], io::Repeat>) -> Result<::bn::Fr, Error> {
	use bn::Fr;
	let mut buf = [0u8; 32];
	reader.read_exact(&mut buf[..]).expect("reading from zero-extended memory cannot fail; qed");
	Fr::from_slice(&buf[0..32]).map_err(|_| Error::from("Invalid field element"))
}


fn read_point(reader: &mut io::Chain<&[u8], io::Repeat>) -> Result<::bn::G1, Error> {
	use bn::{Fq, AffineG1, G1, Group};

	let mut buf = [0u8; 32];

	reader.read_exact(&mut buf[..]).expect("reading from zero-extended memory cannot fail; qed");
	let px = Fq::from_slice(&buf[0..32]).map_err(|_| Error::from("Invalid point x coordinate"))?;

	reader.read_exact(&mut buf[..]).expect("reading from zero-extended memory cannot fail; qed");
	let py = Fq::from_slice(&buf[0..32]).map_err(|_| Error::from("Invalid point y coordinate"))?;

	Ok(
		if px == Fq::zero() && py == Fq::zero() {
			G1::zero()
		} else {
			AffineG1::new(px, py).map_err(|_| Error::from("Invalid curve point"))?.into()
		}
	)
}



#[no_mangle]
pub fn ec_mul(input_hex_ptr: *const c_char) -> *const c_char {
	use bn::AffineG1;

	let input_hex = unsafe { CStr::from_ptr(input_hex_ptr) };
	let input_str: &str = input_hex.to_str().unwrap();
	let input_parsed = FromHex::from_hex(input_str).unwrap();
	let mut padded_input = input_parsed.chain(io::repeat(0));

	let p1 = read_point(&mut padded_input).unwrap();
	let fr = read_fr(&mut padded_input).unwrap();

	let mut ecmul_output_buf = [0u8; 64];
	if let Some(sum) = AffineG1::from_jacobian(p1 * fr) {
		// point not at infinity
		sum.x().to_big_endian(&mut ecmul_output_buf[0..32]).expect("Cannot fail since 0..32 is 32-byte length");
		sum.y().to_big_endian(&mut ecmul_output_buf[32..64]).expect("Cannot fail since 32..64 is 32-byte length");;
	}

	let mut ec_mul_output_str = ecmul_output_buf.to_hex();
	ec_mul_output_str.push_str("\0");
	return ec_mul_output_str.as_ptr()
}



#[no_mangle]
pub fn ec_add(input_hex_ptr: *const c_char) -> *const c_char {
	use bn::AffineG1;

	let input_hex = unsafe { CStr::from_ptr(input_hex_ptr) };
	let input_str: &str = input_hex.to_str().unwrap();
	let input_parsed = FromHex::from_hex(input_str).unwrap();
	let mut padded_input = input_parsed.chain(io::repeat(0));

	let mut padded_buf = [0u8; 128];
	padded_input.read_exact(&mut padded_buf[..]).expect("reading from zero-extended memory cannot fail; qed");

	let point1 = &padded_buf[0..64];
	let point2 = &padded_buf[64..128];

	let mut point1_padded = point1.chain(io::repeat(0));
	let mut point2_padded = point2.chain(io::repeat(0));

	let p1 = read_point(&mut point1_padded).unwrap();
	let p2 = read_point(&mut point2_padded).unwrap();

	let mut ecadd_output_buf = [0u8; 64];

	if let Some(sum) = AffineG1::from_jacobian(p1 + p2) {
		// point not at infinity
		sum.x().to_big_endian(&mut ecadd_output_buf[0..32]).expect("Cannot fail since 0..32 is 32-byte length");
		sum.y().to_big_endian(&mut ecadd_output_buf[32..64]).expect("Cannot fail since 32..64 is 32-byte length");;
	}

	let mut ec_add_output_str = ecadd_output_buf.to_hex();
	ec_add_output_str.push_str("\0");
	return ec_add_output_str.as_ptr()
}



#[no_mangle]
pub fn ec_pairing(input_hex_ptr: *const c_char) -> *const c_char {
	use bn::{Fq, Fq2, G1, G2, Gt, AffineG1, AffineG2, Group, pairing};

	let input_hex = unsafe { CStr::from_ptr(input_hex_ptr) };
	let input_str: &str = input_hex.to_str().unwrap();
	let input = FromHex::from_hex(input_str).unwrap();
	//println!("input: {:?}", input);

	let elements = input.len() / 192;

	if input.len() % 192 != 0 {
		panic!("Invalid input length, must be multiple of 192 (3 * (32*2))");
	}

	let ret_val = if input.len() == 0 {
		bn::arith::U256::one()
	} else {
		let mut vals = Vec::new();
		
		for idx in 0..elements {
			let x_1 = Fq::from_slice(&input[idx*192..idx*192+32])
				.expect("Invalid a argument x coordinate");

			let y_1 = Fq::from_slice(&input[idx*192+32..idx*192+64])
				.expect("Invalid a argument y coordinate");

			let x2_i = Fq::from_slice(&input[idx*192+64..idx*192+96])
				.expect("Invalid b argument imaginary coeff x coordinate");

			let x2_r = Fq::from_slice(&input[idx*192+96..idx*192+128])
				.expect("Invalid b argument imaginary coeff y coordinate");

			let y2_i = Fq::from_slice(&input[idx*192+128..idx*192+160])
				.expect("Invalid b argument real coeff x coordinate");

			let y2_r = Fq::from_slice(&input[idx*192+160..idx*192+192])
				.expect("Invalid b argument real coeff y coordinate");

			//println!("creating g1_point with x1 and y1...");
			//println!("x1: {:?}  y1: {:?}", x_1, y_1);

			let g1_point;
			if x_1 == Fq::zero() && y_1 == Fq::zero() {
				g1_point = G1::zero();
			} else {
				g1_point = G1::from(AffineG1::new(x_1, y_1).expect("Invalid a argument - not on curve"));
			}

			/*
			let mut g1_point_x_buf = [0u8; 32];
			let mut g1_point_y_buf = [0u8; 32];
			g1_point.x().to_big_endian(&mut g1_point_x_buf[0..32]);
			println!("g1_point.x(): {:?}", g1_point_x_buf.to_hex());
			g1_point.y().to_big_endian(&mut g1_point_y_buf[0..32]);
			println!("g1_point.y(): {:?}", g1_point_y_buf.to_hex());
			*/

			let fq2_x = Fq2::new(x2_r, x2_i);
			let fq2_y = Fq2::new(y2_r, y2_i);

			let g2_point;
			if x2_r.is_zero() && x2_i.is_zero() && y2_r.is_zero() && y2_i.is_zero() {
				g2_point = G2::zero();
			} else {
				let g2_affine_point = AffineG2::new(fq2_x, fq2_y).expect("Invalid b argument - not on curve");
				g2_point = G2::from(g2_affine_point);
			}

			vals.push((g1_point, g2_point));
		};

		let mul = vals.into_iter().fold(Gt::one(), |s, (a, b)| s * pairing(a, b));
		if mul == Gt::one() {
			bn::arith::U256::one()
		} else {
			bn::arith::U256::zero()
		}
	};

	let mut ec_pairing_output_buf = [0u8; 32];
	ret_val.to_big_endian(&mut ec_pairing_output_buf);
	let mut ec_pairing_output_str = ec_pairing_output_buf.to_hex();
	//println!("ec_pairing_output_str: {:?}", ec_pairing_output_str);

	ec_pairing_output_str.push_str("\0");
	return ec_pairing_output_str.as_ptr()
}



extern {
	fn emscripten_exit_with_live_runtime();
}


fn main() {
	unsafe {
		emscripten_exit_with_live_runtime();
	}

}
