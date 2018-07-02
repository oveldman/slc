extern crate chrono;
extern crate curl;
extern crate serde;
extern crate serde_json;
extern crate gnuplot;
#[macro_use]
extern crate serde_derive;
extern crate postgres;

pub mod database;
pub mod gulden;
pub mod request;
pub mod price_updater;
pub mod reader;
pub mod graph;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
