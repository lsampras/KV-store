#![allow(unused)]
use std::io;
use std::fs::{File, OpenOptions};
use bson::Document;
use std::io::{BufReader, BufWriter, prelude::*};
use serde::{Serialize, Deserialize};
use bson;
use bson::de;
use std::str;
use kvs::error::{KVError, KVResult};
use kvs::KvStore;
use kvs::command::LogRecord;
use kvs::traits::KvsEngine;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Debug)]
enum Direction {
	Up = 0,
	Left = 1,
	Down = 2,
	Right = 3
}
impl Direction {
    pub fn from_int(i: u32) -> Self {
        match i%4 {
            0 => Direction::Up,
            1 => Direction::Left,
            2 => Direction::Down,
            _ => Direction::Right,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Move {
	heading: Direction,
	length: u32
}

#[derive(Serialize, Deserialize, Debug)]
struct Moves {
	data: Vec<Move>
}

// #[derive(Debug)]
// enum testE {
// 	Data(u32),
// 	Double(u32, u32)
// }


fn main() -> KVResult<()>{
	let mut store =	KvStore::new().unwrap();
	store.set("key23".into(), "val23".into()).unwrap();
	store.set("key23".into(), "val24".into()).unwrap();
	store.set("key2".into(), "val3".into()).unwrap();
	store.set("key4".into(), "val".into()).unwrap();
	store.set("key10".into(), "notval".into()).unwrap();
	store.set("key69".into(), "ooooo".into()).unwrap();
	store.remove("key23".into()).unwrap();
	store.remove("key4".into()).unwrap();
	store.set("key23".into(), "val".into()).unwrap();
	println!("{}", store.get("key23".to_owned())?.unwrap());
	store.state_print();
	println!("\n\n\n{:?}\n\n", store.compaction());
	store.set("key23".into(), "va2l".into()).unwrap();
	store.state_print();
	// print
	// let mut iterator = reader.lock().unwrap();
	// println!("{:?}", &iterator);
	// for i in  iterator.map(|i| i.unwrap()) {
	// 	println!("{:?}", i);
	// }
	// store.set("key1".into(), "val1".into())?;
	// store.set("key2".into(), "val2".into())?;
	// store.set("key3".into(), "val2".into())?;
	// store.remove("key3".into())?;
	// Ok(())
	// if false {
	// 	let mut file = File::create("foo3.txt").unwrap();
	// 	let mut pointer = 0;
	// 	for i in 1..10 {
	// 		let move_1 = Move {
	// 			heading: Direction::from_int(i),
	// 			length: i
	// 		};
	// 	// let move_1 = LogRecord::Delete("key".to_owned());
	// 		let data = bson::to_vec(&move_1).unwrap();
	// 		println!("{:?}", &data);
	// 		pointer += file.write(data.as_slice()).unwrap();
	// 		// println!("{}", pointer);
	// 	}
	// } else {
	// 	let mut file = OpenOptions::new().read(true).write(true).open("foo3.txt").map_err(|err|  {
	// 		err
	// 	}).unwrap();
	// 	let mut reader = BufReader::new(file.try_clone().unwrap());
	// 	loop {
	// 		match Document::from_reader(&mut reader) {
	// 			Ok(doc) =>  println!("{:?}", bson::from_document::<Move>(doc).unwrap()),
	// 			Err(de::Error::Io(i)) => {
	// 				if i.kind() == io::ErrorKind::UnexpectedEof {
	// 					break;
	// 				} else {
	// 					return Err(KVError::Io(i));
	// 				}
	// 			},
	// 			Err(i) => {return Err(KVError::Deserialization(i));},
	// 		}
	// 	}
	// 	let move_2: Move = Move {
	// 		heading: Direction::from_int(2),
	// 		length: 324
	// 	};
	// 	let mut writer = BufWriter::new(file);

	// 	// println!("{:?}", move_2);
	// 	let data = bson::to_vec(&move_2).unwrap();
	// 	writer.write(data.as_slice()).unwrap();
	// }
	Ok(())
}