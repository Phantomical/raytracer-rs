
use std::rc::Rc;
use std::vec::Vec;
use std::cell::{RefCell,Cell};
use std::collections::VecDeque;

use std::ops::*;

thread_local! {
	static TASK : RefCell<Option<CurrentTask>> = RefCell::new(None);
}

trait Coroutine {
	fn next(&mut self) -> Option<()>;
}

struct CoroutinePtr {
	ptr : RefCell<Box<Coroutine>>
}

#[derive(Default)]
struct CurrentTask {
	pub newdeps : Vec<CoroutinePtr>
}

pub struct Scheduler {
	queue : VecDeque<CoroutinePtr>
}

impl CoroutinePtr {
	pub fn next(&mut self) -> Option<()> {
		return self.ptr.get_mut().next();
	}
}


struct GenCoroutine<T, G>
	where G: Generator<Yield = (), Return = T>
{
	pub gen : G,
	pub fut : Rc<Cell<Option<T>>>
}

impl<T, G> GenCoroutine<T, G>
	where G: Generator<Yield = (), Return = T>
{
	fn new(gen : G) -> GenCoroutine<G::Return, G> {
		return Self {
			gen: gen,
			fut: Rc::new(Cell::new(None))
		};
	}
}

impl<T, G> Coroutine for GenCoroutine<T, G> 
	where G: Generator<Yield = (), Return = T>
{
	fn next(&mut self) -> Option<()> {
		match self.gen.resume() {
			GeneratorState::Yielded(_)  => Some(()),
			GeneratorState::Complete(v) => {
				self.fut.set(Some(v));
				None
			}
		}
	}
}

pub struct Future<T> {
	ptr : Rc<Cell<Option<T>>>
}

impl<T: Copy> Future<T> {
	pub fn new(val : Rc<Cell<Option<T>>>) -> Self {
		Future { ptr: val }
	}

	pub fn value(&self) -> Option<T> {
		 self.ptr.deref().get()
	}
}

pub fn add_dependency<T: Copy + 'static, G: 'static>(gen : G) -> Future<T>
	where G: Generator<Yield = (), Return = T>
{
	let mut fut = Future::new(Rc::new(Cell::new(None)));
	TASK.with(|x| {
		let coro = GenCoroutine::new(gen);

		fut = Future::new(coro.fut.clone());
		
		let mut opt = (*x).borrow_mut();

		match opt.deref_mut() {
			&mut Some(ref mut v) => { 
				v.newdeps.push(CoroutinePtr{ 
					ptr: RefCell::new(Box::new(coro))
				})
			}
			&mut None    => panic!()
		};
	});

	return fut;
}


impl Scheduler {
	fn run_task(&mut self, task : &mut CoroutinePtr) {
		TASK.with(|x| (*x).replace(Some(CurrentTask::default())));
	
		if task.next() != None {
			TASK.with(|x| {
				let old = (*x).replace(None);
				let mut newdeps = VecDeque::from(
					old.unwrap().newdeps);
				self.queue.append(&mut newdeps);
			});
		}
	}

	pub fn run_all(&mut self) {
		while let Some(mut task) = self.queue.pop_front() {
			self.run_task(&mut task);
		}
	}

	pub fn add_task<T: Copy + 'static, G: 'static>(&mut self, gen : G) 
		where G: Generator<Yield = (), Return = T>
	{
		let mut in_task : bool = false;

		TASK.with(|x| {
			if let Some(_) = *(*x).borrow() {
				in_task = true;
			}
		});

		if in_task {
			add_dependency(gen);
		}
		else {
			self.queue.push_back(task);
		}
	}
}
