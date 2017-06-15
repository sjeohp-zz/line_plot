extern crate line_plot;

use line_plot::*;

use std::thread;
use std::sync::mpsc;

fn main()
{
	let (tx, rx) = mpsc::channel();

	let handle = thread::spawn(move || 
	{
		thread::sleep(std::time::Duration::from_millis(100));
		
		tx.send(PlotData {
			axis_x: vec![0.0, 1.0],
			axis_y: vec![-1.0, 1.0],
			values_x: vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5],
			values_y: vec![0.0, -0.1, 0.5, 0.0, 0.0, 0.5]
		}).unwrap();

		thread::sleep(std::time::Duration::from_millis(10000));
	});

	let mut render_state = init(100, 100, 500, 500, 0.2, 6);
	let mut quit = false;

	while !quit 
	{
		let data = match rx.try_recv() {
			Ok(data) => Some(data),
			Err(_) => None
		};
		quit = render(data, & mut render_state);
	}
}