use std::mem::size_of;
use std::fs::File;
use std::io::{Seek, SeekFrom, Read};

#[repr(C, packed)]
#[derive(serde::Deserialize)]
pub struct BitmapHeader {
	pub format: u16,
	pub size: u32,
	pub unused: u32,
	pub offset: u32
}

#[repr(C, packed)]
#[derive(serde::Deserialize)]
pub struct BitmapInfoHeader {
	pub size: u32,
	pub width: u32,
	pub height: u32,
	pub planes: u16,
	pub bits: u16,
	pub compression: u32,
	pub image_size: u32,
	pub xppm: u32,
	pub yppm: u32,
	pub colors_used: u32,
	pub colors_important: u32
}

#[repr(C, packed)]
#[derive(serde::Deserialize)]
pub struct BitmapPaletteColor {
	blue: u8,
	green: u8,
	red: u8,
	reserved: u8
}

#[allow(safe_packed_borrows)]
impl BitmapHeader {
	pub fn read(file: &mut File) -> BitmapHeader {
		let mut buffer = [0u8; size_of::<BitmapHeader>()];

		file.seek(SeekFrom::Start(0)).unwrap();
		file.read(&mut buffer).unwrap();

		return bincode::config()
			.little_endian()
			.deserialize::<BitmapHeader>(&buffer)
			.unwrap();
	}

	pub fn log(&self) {
		println!("[HEADER] Struct | 0x{0:x}", size_of::<BitmapHeader>());
		println!("[HEADER] Format | 0x{0:x}", self.format);
		println!("[HEADER] Size   | 0x{0:x}", self.size);
		println!("[HEADER] Unused | 0x{0:x}", self.unused);
		println!("[HEADER] Offset | 0x{0:x}", self.offset);
	}
}

#[allow(safe_packed_borrows)]
impl BitmapInfoHeader {
	pub fn read(file: &mut File) -> BitmapInfoHeader {
		let mut buffer = [0u8; size_of::<BitmapInfoHeader>()];

		file.seek(SeekFrom::Start(size_of::<BitmapHeader>() as u64)).unwrap();
		file.read(&mut buffer).unwrap();

		return bincode::config()
			.little_endian()
			.deserialize::<BitmapInfoHeader>(&buffer)
			.unwrap();
	}

	pub fn log(&self) {
		println!("[INFOHEADER] Struct size     | 0x{0:x}", size_of::<BitmapInfoHeader>());
		println!("[INFOHEADER] Size            | 0x{0:x}", self.size);
		println!("[INFOHEADER] Width           | 0x{0:x}", self.width);
		println!("[INFOHEADER] Height          | 0x{0:x}", self.height);
		println!("[INFOHEADER] Planes          | 0x{0:x}", self.planes);
		println!("[INFOHEADER] Bits            | 0x{0:x}", self.bits);
		println!("[INFOHEADER] Compression     | 0x{0:x}", self.compression);
		println!("[INFOHEADER] ImageSize       | 0x{0:x}", self.image_size);
		println!("[INFOHEADER] XPPM            | 0x{0:x}", self.xppm);
		println!("[INFOHEADER] YPPM            | 0x{0:x}", self.yppm);
		println!("[INFOHEADER] UsedColors      | 0x{0:x}", self.colors_used);
		println!("[INFOHEADER] ImportantColors | 0x{0:x}", self.colors_important);
	}
}

#[allow(safe_packed_borrows)]
impl BitmapPaletteColor {
	pub fn read(file: &mut File, index: u32) -> BitmapPaletteColor {
		let mut buffer = [0u8; size_of::<BitmapPaletteColor>()];

		let offset_to_color =
			size_of::<BitmapHeader>() as u32 +
			size_of::<BitmapInfoHeader>() as u32 +
			index;

		file.seek(SeekFrom::Start(offset_to_color as u64)).unwrap();
		file.read(&mut buffer).unwrap();

		return bincode::config()
			.little_endian()
			.deserialize::<BitmapPaletteColor>(&buffer)
			.unwrap();
	}

	pub fn log(&self) {
		println!("[PALETTECOLOR] #{0:02x}{1:02x}{2:02x}", self.red, self.green, self.blue);
	}
}