extern crate line_plot;

use line_plot::*;

fn main()
{
	plot(
        &[0.0, 300.0, 300.0, 600.0],
        &[0.0, 0.0, 600.0, 600.0],
		0.0, 
		1200.0,
		0.0, 
		1200.0);
}