use std::mem::size_of;
use std::fs::File;
use std::io::{Seek, SeekFrom, Read};
use std::env;

mod bmp;
use crate::bmp::*;

#[allow(safe_packed_borrows)]
fn main() {

	let args: Vec<String> = env::args().collect();

	if args.len() != 2 {
		println!("No file presented! Run arduino-chargen.exe [PATH TO BITMAP]");
		return;
	}

	let mut bitmap = match File::open(&args[1]) {
		Ok(f) => f,
		Err(_) => {
			println!("Can't load bitmap image from path");
			return;
		},
	};

	let header = BitmapHeader::read(&mut bitmap);

	if header.format != 0x4d42 {
		println!("Only BM (Windows) format supported! Given {0:X}", header.format);
		return;
	}

	let info_header = BitmapInfoHeader::read(&mut bitmap);

	if info_header.width != 5 || info_header.height != 7 {
		println!("Only 5x7 images supported! Given {}x{}", info_header.width, info_header.height);
		return;
	}

	if info_header.bits != 4 {
		println!("Only 4 bits images supported! Given {}", info_header.bits);
		return;
	}

	convert_image_to_bits_array(&mut bitmap, &header);
}

fn convert_image_to_bits_array(bitmap: &mut File, header: &BitmapHeader)  {
	
	let image_offset =
		size_of::<BitmapHeader>() as u32+
		size_of::<BitmapInfoHeader>() as u32 +
		(size_of::<BitmapPaletteColor>() * 16) as u32;

	let image_size = (header.size - image_offset) as usize;

	let mut buffer = vec![0u8; image_size];

	bitmap.seek(SeekFrom::Start(image_offset as u64)).unwrap();
	bitmap.read(&mut buffer).unwrap();

	println!("byte symbol[8] = {{");

	/*
		The image in BMP is vertically inverted. See below:

		####                  #####
		#  #                  #   #
		#  #                  #   #
		#####                 #####
		#   #                 #  # 
		#   #                 #  # 
		#####                 ####

		Original image  --->  Stored in BMP
	*/

	for y in (0..7).rev()  {
		print!("\tB");

		/*
			The image is strictly 5x7, but raws are always rounded to four bytes.
			The palette is four-bit and one byte will be exactly two pixels.
			Read the first two bytes in their entirety and half of the last one.
		*/

		for x in 0..3 {
			let px1 = buffer[y * 4 + x] & 0xF0;
			let px2 = buffer[y * 4 + x] & 0x0F;

			if x != 2 {
				print!("{0:}{1:}", px_to_bit(px1), px_to_bit(px2));
			} else {
				print!("{:}", px_to_bit(px1));
			}
		}

		println!(",");
	}

	println!("}};");
}

fn px_to_bit(px: u8) -> u32 {
	if px == 0 { 1 } else { 0 }
}