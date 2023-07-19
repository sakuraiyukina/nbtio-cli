use std::string::String;
use rusty_leveldb::{DB, LdbIterator};

pub fn print_db(db_path: &str) {
	// 創建一個只存在於內存中的數據庫
	// let options = rusty_leveldb::in_memory();
	// 嘗試打開指定路徑的文件
	let db_result = DB::open(db_path, Default::default());
	// 使用match表達式來檢查打開文件的結果
	let mut db = match db_result {
		// 如果成功，返回數據庫對象
		Ok(db) => db,
		// 如果失敗，打印錯誤信息並退出程序
		Err(error) => {
			eprintln!("Failed to open the database: {:?}", error);
			std::process::exit(1);
		}
	};
	// 嘗試創建一個數據庫迭代器
	let iter_result = db.new_iter();
	// 使用match表達式來檢查創建迭代器的結果
	let mut iter = match iter_result {
		// 如果成功，返回迭代器對象
		Ok(iter) => iter,
		// 如果失敗，打印錯誤信息並退出程序
		Err(error) => {
			eprintln!("Failed to create the iterator: {:?}", error);
			std::process::exit(1);
		}
	};
	// 遍歷數據庫中的所有鍵值對
	while let Some((key, value)) = iter.next() {
		// 將鍵值對轉換為字符串並打印
		let key_str = String::from_utf8_lossy(&*key);
		let value_str = String::from_utf8_lossy(&*value);
		println!("{}: {}", key_str, value_str);
	}
	// 關閉數據庫
	let _ = db.close();
}