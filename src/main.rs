extern crate jobsteal;
extern crate num_cpus;
extern crate time;

use jobsteal::{make_pool, Spawner};

const NUM_CHILDREN: usize = 10;

fn skynet(spawner: Spawner, dst: &mut u64, num: u64, size: u64) {
	if size <= 1 {
		*dst = num;
		return;
	}

	let mut dst_arr = [0; NUM_CHILDREN];
	let new_size = size / NUM_CHILDREN as u64;
	spawner.scope(|scope| {
		for (i, new_dst) in dst_arr.iter_mut().enumerate() {
			let i = i as u64;
			let new_num = num + i*new_size;
			scope.recurse(move |spawner| {
				skynet(spawner, new_dst, new_num, new_size);
			});
		}
	});

	*dst = dst_arr.iter().fold(0, |acc, &x| acc + x);
}

fn main() {
	let start = time::precise_time_ns();
	let mut pool = jobsteal::make_pool(num_cpus::get()).unwrap();

	let mut result = 0;
	skynet(pool.spawner(), &mut result, 0, 1000000);
	println!("result = {} in {} ms.", result,
			(time::precise_time_ns() - start) / 1_000_000);
}