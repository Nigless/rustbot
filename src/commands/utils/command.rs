﻿use crate::response::Response;
use std::str::Split;

pub trait Command {
	fn execute(&self, arguments: Split<&str>) -> Response;
	fn get_key(&self) -> String;
}
