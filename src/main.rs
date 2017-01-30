extern crate clang;
extern crate clang_sys;
extern crate futures;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate lsp_rs;
extern crate ls_service;
extern crate tokio_core;
extern crate tokio_stdio;

#[cfg( windows )]
extern crate kernel32;
#[cfg( windows )]
extern crate winapi;

mod parent_process;
mod handler;

use ls_service::{
    service
};
use tokio_core::reactor::{
    Core
};
use tokio_stdio::stdio::{
    Stdio
};

fn main( ) {
    log4rs::init_file( "log4rs.yml", Default::default( ) ).unwrap( );

    let mut core = Core::new( ).unwrap( );

    let message_handler = handler::DynamicMessageHandler::new( core.handle( ) );
    let io = Stdio::new( 1024, 1024 );

    info!( "Starting service on stdin/stdout." );
    let service_handle = service::start_service( core.handle( ), message_handler, io );
    match core.run( service_handle.get_shutdown_future( ).clone( ) ) {
        Ok( _ ) => {
            info!( "Service terminated cleanly." );
        },
        Err( error ) => {
            info!( "Service terminated with error {:?}", error );
        }
    }
}
