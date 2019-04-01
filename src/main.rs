extern crate iron;

use iron::prelude::*;
use iron::status;

fn main()
{
	println!("Starting server on http://localhost:3000...");
	Iron::new(get_file).http("localhost:3000").unwrap();
}

fn get_file(_request: &mut Request) -> IronResult<Response>
{
	let mut response = Response::new();
	response.set_mut(status::NotFound);
	Ok(response)
}
