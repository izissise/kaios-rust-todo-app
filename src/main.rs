extern crate failure;

use failure::Fallible;
use stdweb::{_js_impl, js, Value};
use stdweb::web::{document, IParentNode};
use ::log::{info};

mod log;
use crate::log::init_logger;

fn main() -> Fallible<()> {

    // Initialize the logger
    init_logger()?;
    info!("Hello");

    // Create a new app
//     let _root = document()
//         .query_selector("#wasmapp")
//         .expect("can't get body node for rendering")
//         .expect("can't unwrap body node");

    let message = "Hello, 世界!";
    let _result = js! {
        alert( @{message} );
        return 2 + 2 * 2;
    };
    Ok(())
}
