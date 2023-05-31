// create aliases for crates.
use walkdir::WalkDir;
use std::{io, path::PathBuf};

fn main(){
	// guys! I am lazy to use win api. Look at this sh*tty piece of filtered drive paths ahahahahaha!
	let paths=('c'..='z').map(|c|{let tp_owned=format!("{}:\\", c); let tp=tp_owned.as_str(); let mut pb=PathBuf::new(); pb.push(tp); pb}).filter(|e| e.exists());
	let stdin = io::stdin();  // stdin handler.
	let mut search=String::new();
	println!("Введите фразу для поиска");
	stdin.read_line(&mut search).expect("oops! у вас тут какая-то ошиба в терминале.");
	let s=search.trim();
	println!("Идёт поиск по всем файлам. Это может занять некоторое время в зависимости от количества файлов на компьютере.");
	for j in paths{
		println!("поиск на диске {}", j.display());
		for (_, entry) in WalkDir::new(j).into_iter().filter_map(|e| e.ok()).enumerate() {
			if entry.file_name().to_str().unwrap().contains(&s){
				println!("Найден файл по пути {}", entry.path().display())
			}
	}
}

	println!("Поиск завершён! Нажмите энтер, чтобы выйти.");
	stdin.read_line(&mut search).unwrap();
}