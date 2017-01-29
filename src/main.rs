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

fn main( ) {
    log4rs::init_file( "log4rs.yml", Default::default( ) ).unwrap( );
}
