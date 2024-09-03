use askama::Result;
use std::fmt::Display;

pub fn id<T: Display>(id: T) -> Result<String> {
    Ok(format!("#{id:03}"))
}
