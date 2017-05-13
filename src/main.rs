extern crate line_plot;

use line_plot::*;

use std::thread;
use std::sync::mpsc;

fn main()
{
	// feature::plot(
 //        &[0.0, 300.0, 300.0, 600.0],
 //        &[0.0, 0.0, 600.0, 600.0],
	// 	0.0, 
	// 	1200.0,
	// 	0.0, 
	// 	1200.0);

	let (tx, rx) = mpsc::channel();

	let handle = thread::spawn(move || {
		init(100, 100, 500, 500, 5, rx);
	});
	
	thread::sleep(std::time::Duration::from_millis(100));
	
	tx.send(PlotData {
		axis_x: vec![0.0, 1.0],
		axis_y: vec![-1.0, 1.0],
		values_x: vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5],//, 300.0, 400.0],
		values_y: vec![0.0, -0.1, 0.5, -0.2, 0.0, 0.5]//, 400.0, 400.0]
	}).unwrap();

	thread::sleep(std::time::Duration::from_millis(10000));
}