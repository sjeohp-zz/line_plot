extern crate piston_window;
extern crate graphics as piston_graphics;

use piston_window::*;
use piston_graphics::*;

pub fn plot(
	x: &[f64], 
	y: &[f64], 
	min_x: f64, 
	min_y: f64, 
	max_x: f64, 
	max_y: f64) 
{
	assert!(min_x < max_x && min_y < max_y);

    let (window_width, window_height) = (600, 600);

    let mut window: PistonWindow = WindowSettings::new("2d_line_plot", [window_width, window_height]).exit_on_esc(true).build().unwrap();
    while let Some(e) = window.next() 
    {
        window.draw_2d(&e, |context, graphics| {
            clear([1.0, 1.0, 1.0, 1.0], graphics);

            let line = Line::new([0.0, 0.0, 0.0, 1.0], 1.0);

            let mut prev: Option<(f64, f64)> = None;
            for (x, y) in x.into_iter().zip(y.into_iter())
            {
            	match prev
            	{
            		Some((px, py)) => {
            			let coords = [
            				(px - min_x) / (max_x - min_x) * (window_width as f64), 
            				(py - min_y) / (max_y - min_y) * (window_height as f64), 
            				(x - min_x) / (max_x - min_x) * (window_width as f64), 
            				(y - min_y) / (max_y - min_y) * (window_height as f64)];
            			
            			line.draw(
            			    coords, 
            			    &context.draw_state, 
            			    context.transform,
            			    graphics);
            		}
            		None => {}
            	}
            	prev = Some((*x, *y));
            }
        });
    }
}

// fn main()
// {
// 	plot(
//         &[0.0, 300.0, 300.0, 600.0],
//         &[0.0, 0.0, 600.0, 600.0],
// 		0.0, 
// 		0.0, 
// 		1200.0,
// 		1200.0);
// }

#[cfg(test)]
mod tests 
{
	use super::*;

    #[test]
    fn it_works() 
    {

    }
}
