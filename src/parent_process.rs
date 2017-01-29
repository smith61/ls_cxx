
use futures::{
    Async,
    Future,
    Poll
};
use futures::sync::{
    oneshot
};
use std::{
    io,
    time,
    thread
};
use tokio_core::reactor::{
    Core,
    Handle,
    Timeout
};

#[cfg( windows )]
mod os {
    use kernel32::{
        CloseHandle,
        GetExitCodeProcess,
        OpenProcess
    };
    use std::{
        io
    };
    use winapi::minwindef::{
        DWORD,
        FALSE
    };
    use winapi::winnt::{
        HANDLE,
        PROCESS_QUERY_LIMITED_INFORMATION
    };

    #[derive( Debug )]
    pub struct Process {
        proc_handle : HANDLE
    }

    impl Process {

        pub fn new( proc_id : u64 ) -> io::Result< Process > {
            let proc_handle = unsafe {
                OpenProcess( PROCESS_QUERY_LIMITED_INFORMATION, FALSE, proc_id as DWORD )
            };

            if !proc_handle.is_null( ) {
                Ok( Process {
                    proc_handle : proc_handle
                } )
            }
            else {
                Err( io::Error::last_os_error( ) )
            }
        }

        pub fn poll_exit( &self ) -> io::Result< bool > {
            let mut exit_code : DWORD = 0;
            let result = unsafe {
                GetExitCodeProcess( self.proc_handle, &mut exit_code as *mut _ )
            };

            if result == FALSE {
                Err( io::Error::last_os_error( ) )
            }
            else {
                // 259 == STILL_ACTIVE
                Ok( exit_code != 259 )
            }
        }

    }

    impl Drop for Process {

        fn drop( &mut self ) {
            unsafe { CloseHandle( self.proc_handle ); }
        }

    }

}

pub type ParentProcessFuture = oneshot::Receiver< io::Result< ( ) > >;

struct ProcessFuture {
    process_handle  : os::Process,
    core_handle     : Handle,

    pending_timeout : Option< Timeout >
}

pub fn open_parent_process( proc_id : u64 ) -> ParentProcessFuture {

    let ( send, read ) = oneshot::channel( );
    thread::spawn( move | | {
        let result = ProcessFuture::run( proc_id );
        trace!( "ProcessFuture for id {:?} exited with result {:?}", proc_id, result );

        send.complete( result );
    } );

    read
}

impl ProcessFuture {

    fn run( proc_id : u64 ) -> io::Result< ( ) > {
        let mut core = Core::new( )?;
        let handle = core.handle( );

        let process_handle = os::Process::new( proc_id )?;
        trace!( "Allocated process handle {:?} for id {:?}.", process_handle, proc_id );

        let result = core.run( ProcessFuture {
            process_handle  : process_handle,
            core_handle     : handle,

            pending_timeout : None
        } );

        result
    }

}

impl Future for ProcessFuture {

    type Item  = ( );
    type Error = io::Error;

    fn poll( &mut self ) -> Poll< Self::Item, Self::Error > {
        trace!( "Polling for process handle {:?}", self.process_handle );
        match self.process_handle.poll_exit( ) {
            Ok( true ) => Ok( Async::Ready( ( ) ) ),
            Ok( false ) => {
                let mut timeout = Timeout::new( time::Duration::new( 5, 0 ), &self.core_handle ).unwrap( );
                timeout.poll( )?;

                // Need to keep timeout alive to prevent cancelling it
                self.pending_timeout = Some( timeout );
                Ok( Async::NotReady )
            },
            Err( error ) => Err( error )
        }
    }

}