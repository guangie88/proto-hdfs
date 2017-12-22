extern crate protobuf;

pub mod hadoop_common;
pub mod hadoop_hdfs;

pub mod client;
pub mod error;

pub use client::*;
pub use error::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}