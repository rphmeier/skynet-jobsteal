extern crate jobsteal;
extern crate num_cpus;
extern crate time;

use std::sync::mpsc::{channel, Sender};

use jobsteal::{make_pool, Spawner};

const NUM_CHILDREN: u64 = 10;

fn skynet<'a, 'b>(spawner: Spawner<'a, 'b>, dst: Sender<u64>, num: u64, size: u64) {
	if size <= 1 {
		dst.send(num).unwrap();

		return;
	}

	let (tx, rx) = channel();
	let new_size = size / NUM_CHILDREN as u64;

	spawner.scope(|scope| {
		for i in 0..NUM_CHILDREN {
			let new_num = num + i*new_size;
			let tx      = tx.clone();

			scope.recurse(move |spawner| {
				skynet(spawner, tx, new_num, new_size);
			});
		}

	});

	dst.send((0..NUM_CHILDREN).map(|_| rx.recv().unwrap()).fold(0, std::ops::Add::add)).unwrap();
}

fn main() {
	let start = time::precise_time_ns();
	let mut pool = make_pool(num_cpus::get()).unwrap();
	let (tx, rx) = channel();

	skynet(pool.spawner(), tx, 0, 1000000);

	println!("result = {}, time elapsed = {} nanoseconds", rx.recv().unwrap(), time::precise_time_ns() - start);
}