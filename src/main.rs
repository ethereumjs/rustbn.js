//#![cfg(feature="enable-unstable")]
//#![feature(link_args)]


//#[link_args = "-s EXPORTED_FUNCTIONS=['_add']"]
//extern {}

extern crate bn;
extern crate rand;
extern crate num;
extern crate rustc_serialize;

extern crate hex_slice;

use hex_slice::AsHex;

//use num::bigint::BigInt;
use num::bigint::BigUint;

use bn::*;

use rustc_serialize::hex::FromHex;

use std::io::{self, Read};



#[derive(Debug)]
pub struct Error(pub &'static str);

impl From<&'static str> for Error {
	fn from(val: &'static str) -> Self {
		Error(val)
	}
}

fn typeid<T: std::any::Any>(_: &T) {
    println!("{:?}", std::any::TypeId::of::<T>());
}


fn read_fr(reader: &mut io::Chain<&[u8], io::Repeat>) -> Result<::bn::Fr, Error> {
	let mut buf = [0u8; 32];
	
	reader.read_exact(&mut buf[..]).expect("reading from zero-extended memory cannot fail; qed");
	::bn::Fr::from_slice(&buf[0..32]).map_err(|_| Error::from("Invalid field element"))
}


fn read_point(reader: &mut io::Chain<&[u8], io::Repeat>) -> Result<::bn::G1, Error> {
	use bn::{Fq, AffineG1, G1, Group};
	
	let mut buf = [0u8; 32];
	
	reader.read_exact(&mut buf[..]).expect("reading from zero-extended memory cannot fail; qed");
	let px = Fq::from_slice(&buf[0..32]).map_err(|_| Error::from("Invalid point x coordinate"))?;

	reader.read_exact(&mut buf[..]).expect("reading from zero-extended memory cannot fail; qed");
	let py = Fq::from_slice(&buf[0..32]).map_err(|_| Error::from("Invalid point x coordinate"))?;
	
	Ok(
		if px == Fq::zero() && py == Fq::zero() {
			G1::zero()
		} else {
			AffineG1::new(px, py).map_err(|_| Error::from("Invalid curve point"))?.into()
		}
	)
}



#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
    return a + b
}



#[no_mangle]
pub fn bn128(a: i32) -> i32 {

	println!("Hello bn128!");
	
	
	//println!("bigint! {:?}", "6851077925310461602867742977619883934042581405263014789956638244065803308498".parse::<BigInt>().unwrap());
	//let x_point = "6851077925310461602867742977619883934042581405263014789956638244065803308498".parse::<BigInt>().unwrap();
	
	
	/* *****
	test input copied from https://github.com/ethereum/cpp-ethereum/blob/8a68de66b84f9170797384909f0d90aad479d059/test/unittests/libdevcrypto/LibSnark.cpp#L128-L133
	*/
	
	let x_point = "6851077925310461602867742977619883934042581405263014789956638244065803308498".parse::<BigUint>().unwrap();
	let y_point = "10336382210592135525880811046708757754106524561907815205241508542912494488506".parse::<BigUint>().unwrap();
	

	/* *****
	do bn128 add
	https://github.com/paritytech/parity/blob/efe0f8449c6c47f6c7bf1c0f8e59df81ac886cc4/ethcore/src/builtin.rs#L360-L369
	*/

	let x_bytes =  [x_point.to_bytes_be(), y_point.to_bytes_be()].concat();

	println!("bigint! {:x}", x_point.to_bytes_be().as_hex());
	
	println!("x_bytes! {:x}", x_bytes.as_hex());
	
	let mut x_input = x_bytes.chain(io::repeat(0));


	let mut buff_add = [0u8; 64];
	
	match read_point(&mut x_input) {
		Ok(p1) => {
			//println!("{:?} bytes read", n);
			//println!("{:?}", padded_input);
			
			if let Some(sum) = AffineG1::from_jacobian(p1 + p1) {
				// point not at infinity
				sum.x().to_big_endian(&mut buff_add[0..32]).expect("Cannot fail since 0..32 is 32-byte length");
				sum.y().to_big_endian(&mut buff_add[32..64]).expect("Cannot fail since 32..64 is 32-byte length");;
			}
			
			//println!("affine G1: {:?}", &buff_add[..])
			println!("affine G1: {:x}", &buff_add[..].as_hex())
			// affine G1: [2d 97 a8 f9 a1 47 c3 cf 59 73 b6 1 23 9e 4f d7 3d b2 8d 3 3e d0 4a 5c c1 f9 58 8d e8 65 1d 1d 19 13 89 77 c0 e5 80 20 65 c 37 ed 22 4f 76 33 df c e6 c8 6e 7d 6a e9 35 7 44 df dc 10 da 54]

		}
		Err(error) => println!("error: {:?}", error),
	}
	
	
	/* *****
	now do bn128mul
	https://github.com/paritytech/parity/blob/efe0f8449c6c47f6c7bf1c0f8e59df81ac886cc4/ethcore/src/builtin.rs#L381-L390
	*/

	// see the test case in cpp-ethereum link above
	// x + x == x * 2
	let scalar = FromHex::from_hex("0000000000000000000000000000000000000000000000000000000000000002").unwrap();

	println!("scalar: {:?}", scalar);
	
	//let scalar = "2".parse::<BigUint>().unwrap();

	let mul_bytes =  [&x_bytes[..], &scalar[..]].concat();
	//let mul_bytes =  [&x_bytes[..], &scalar.to_bytes_be()[..]].concat();
	println!("mul_bytes: {:x}", mul_bytes.as_hex());

	//let mut mul_input = x_bytes.chain(io::repeat(0));
	let mut mul_input = mul_bytes.chain(io::repeat(0));

	let mut p_buf = [0u8; 32];
	let p = read_point(&mut mul_input).unwrap();
	p.x().to_big_endian(&mut p_buf[0..32]);
	println!("read point p.x: {:x}", p_buf.as_hex());


	let fr = read_fr(&mut mul_input).unwrap();
	
	let mut fr_buf = [0u8; 32];
	fr.to_big_endian(&mut fr_buf[0..32]);
	
	println!("read point fr: {:x}", fr_buf.as_hex());
	
	
	let mut mul_write_buf = [0u8; 64];
	if let Some(sum) = AffineG1::from_jacobian(p * fr) {
		// point not at infinity
		sum.x().to_big_endian(&mut mul_write_buf[0..32]).expect("Cannot fail since 0..32 is 32-byte length");
		sum.y().to_big_endian(&mut mul_write_buf[32..64]).expect("Cannot fail since 32..64 is 32-byte length");;
	}
	
	println!("mul_write_buf: {:x}", mul_write_buf.as_hex());
	// mul_write_buf: [2d 97 a8 f9 a1 47 c3 cf 59 73 b6 1 23 9e 4f d7 3d b2 8d 3 3e d0 4a 5c c1 f9 58 8d e8 65 1d 1d 19 13 89 77 c0 e5 80 20 65 c 37 ed 22 4f 76 33 df c e6 c8 6e 7d 6a e9 35 7 44 df dc 10 da 54]


	return 55;

}



// #[link_args = "-s NO_EXIT_RUNTIME=1"]
// extern {}

/*
extern {
	fn emscripten_exit_with_live_runtime();
}


fn main() {
	unsafe {
		emscripten_exit_with_live_runtime();
	}

}*/

fn main() {
    println!("Hello world!");
}

