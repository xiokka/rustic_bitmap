use crate::constants::*;

pub trait Get {
	fn get_pixel_array_offset(&self) -> usize;
	fn get_height(&self) -> u32;
	fn get_width(&self) -> u32;
	fn get_bits_per_pixel(&self) -> u8;
	fn get_file_size(&self) -> u32;
	fn get_size_of_info_header(&self) -> u32;
	fn get_planes(&self) -> u16;
	fn get_compression(&self) -> u32;
	fn get_image_size(&self) -> u32;
	fn get_colors_used(&self) -> u32;
	fn get_important_colors(&self) -> u32;
}

impl Get for Vec<u8> {
	fn get_pixel_array_offset(&self) -> usize {
		return self[HEADER_PIXEL_ARRAY_OFFSET] as usize; 
	}
	fn get_width(&self) -> u32 {
		let byte_slice = &self[HEADER_WIDTH_OFFSET..HEADER_WIDTH_OFFSET+4];
		u32::from_le_bytes([byte_slice[0], byte_slice[1], byte_slice[2], byte_slice[3]])
	}
        fn get_height(&self) -> u32 {
		let byte_slice = &self[HEADER_HEIGHT_OFFSET..HEADER_HEIGHT_OFFSET+4];
		u32::from_le_bytes([byte_slice[0], byte_slice[1], byte_slice[2], byte_slice[3]])
        }
	fn get_bits_per_pixel(&self) -> u8 {
		return self[HEADER_BITS_PER_PIXEL];
	}

	fn get_file_size(&self) -> u32 {
		let byte_slice = &self[HEADER_FILE_SIZE..HEADER_FILE_SIZE+4];
		u32::from_le_bytes([byte_slice[0], byte_slice[1], byte_slice[2], byte_slice[3]])
	}

	fn get_size_of_info_header(&self) -> u32 {
		let byte_slice = &self[INFOHEADER_SIZE_OFFSET..INFOHEADER_SIZE_OFFSET+4];
		u32::from_le_bytes([byte_slice[0], byte_slice[1], byte_slice[2], byte_slice[3]])
	}
	
        fn get_planes(&self) -> u16 {
		let byte_slice = &self[HEADER_PLANES_OFFSET..HEADER_PLANES_OFFSET+2];
		u16::from_le_bytes([byte_slice[0], byte_slice[1]])
	}

        fn get_compression(&self) -> u32 {
		let byte_slice = &self[HEADER_COMPRESSION_OFFSET..HEADER_COMPRESSION_OFFSET+4];
		u32::from_le_bytes([byte_slice[0], byte_slice[1], byte_slice[2], byte_slice[3]])
	}

        fn get_image_size(&self) -> u32 {
		let byte_slice = &self[HEADER_IMAGE_SIZE..HEADER_IMAGE_SIZE+4];
		u32::from_le_bytes([byte_slice[0], byte_slice[1], byte_slice[2], byte_slice[3]])
	}

        fn get_colors_used(&self) -> u32 {
		let byte_slice = &self[HEADER_COLORS_USED..HEADER_COLORS_USED+4];
		u32::from_le_bytes([byte_slice[0], byte_slice[1], byte_slice[2], byte_slice[3]])
	}

        fn get_important_colors(&self) -> u32 {
		let byte_slice = &self[HEADER_IMPORTANT_COLORS..HEADER_IMPORTANT_COLORS+4];
		u32::from_le_bytes([byte_slice[0], byte_slice[1], byte_slice[2], byte_slice[3]])
	}
}
