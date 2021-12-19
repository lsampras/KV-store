use std::sync::{
	mpsc::{channel, Sender, Receiver},
	Arc, Mutex, RwLock
};
use std::{thread::{self, JoinHandle}};


pub struct NaiveThreadPool {
	handles: Vec<JoinHandle<()>>,
	count: u8,
	sender: Sender<Box<dyn FnOnce() + Send + 'static>>,
	receiver: Arc<Mutex<Receiver<Box<dyn FnOnce() + Send + 'static>>>>,
	handler: Option<Box<dyn Fn()>>,
	shutdown: Arc<RwLock<bool>>
}

pub trait Task<F>: FnOnce() -> F + Send + 'static {}


impl NaiveThreadPool {

	pub fn new(count: u8) -> Self {
		let (tx, rx) = channel::<Box<dyn FnOnce() + Send + 'static>>();
		NaiveThreadPool {
			handles: vec![],
			count: count,
			sender: tx,
			receiver: Arc::new(Mutex::new(rx)),
			handler: None,
			shutdown: Arc::new(RwLock::new(false))
		}
	}
	pub fn initialize_pool(&mut self) {
		for i in 0..self.count {
			let receiver = self.receiver.clone();
			let trigger = self.shutdown.clone();
			self.handles.push(thread::spawn(move || {
				loop {
					if let Ok(shutdown) = trigger.read() {
						if *shutdown == true {
							break;
						}
					}
					let mut task: Option<Box<dyn FnOnce() + Send + 'static>> = None;
					if let Ok(receiver) = receiver.lock() {
						if let Ok(message) = receiver.recv() {
							task = Some(message);
						}
					}
					match task {
						Some(func) => {
							println!("Executing in thread {}", i);
							func();
						},
						None => {}
					};
				}
			}));
		}
	}
	pub fn set_handler(&mut self, handler: Box<dyn Fn()>) {
		self.handler = Some(handler);
	}
	pub fn spawn<F>(&mut self, task: F) 
	where F: FnOnce() -> () + Send + 'static {
		self.sender.send(Box::new(task)).unwrap();
	}
}

impl Drop for NaiveThreadPool {
	fn drop(&mut self) {
		if let Ok(mut trigger) = self.shutdown.write() {
			*trigger = true;
		} else {
			println!("could not drop properly");
			return;
		}
		for i in self.handles.drain(..) {
			i.join().unwrap();
		}
	}
}