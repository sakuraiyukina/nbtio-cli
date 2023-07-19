use std::fs::File;
use std::io::{BufReader, Read};
use std::fmt::Write;

pub fn print_db(db_path: &str) {
	// 從傳入參數讀取文件路徑
	let file = File::open(db_path).expect("Failed to open file");
	// 構建一個緩衝區
	let mut reader = BufReader::new(file);
	// 建立一個u8陣列暫存數據
	let mut buffer = [0u8; 0x10];
	// 建立一個空字串存放最終結果
	let mut result = String::new();

	loop {
		// 嘗試讀取16位元到buffer
		match reader.read(&mut buffer) {
			Ok(bytes_read) => {
				if bytes_read == 0 {
					break;
				}
				// 遍歷buffer中的位元組
				for byte in &buffer[..bytes_read] {
					// 0x30 - 0x7e 爲可打印的ASCII字元
					if *byte >= 0x30 && *byte <= 0x7e {
						// 轉爲char，寫入到result變數
						write!(&mut result, "{}", std::char::from_u32(*byte as u32).unwrap()).unwrap();
					} else {
						// 不可見字元則打印空格
						result.push(' ');
					}
				}
			}
			Err(e) => {
				println!("Error reading file: {}", e);
				break;
			}
		}
	}
	// 打印最終結果
	println!("{}", result);
}