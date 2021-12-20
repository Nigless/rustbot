﻿use super::about;
use super::channel::channel;
use super::hello;
use super::utils::router;
use super::utils::router::Router;
use crate::response::Response;
use crate::request::Request;

pub struct Root {
	router: Router,
}

impl Root {
	pub fn execute(&self, request:Request) -> Response {
		match self.router.dispatch(request) {
			Some(response) => return response,
			None => return Err("ERROR".to_owned()),
		}
	}
	
	pub fn new() -> Self {
		let mut router = router::new();
	
		router.register(Box::new(hello::new()));
		router.register(Box::new(about::new()));
		router.register(Box::new(channel::new()));
	
		return Self {
			router,
		};
	}
}
