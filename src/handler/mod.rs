
use ls_service::service::{
    MessageHandler,
    ResponseOutput,
    ServiceHandle
};
use lsp_rs::{
    ServerNotification,
    ServerRequest
};
use std::cell::{
    RefCell
};
use tokio_core::reactor::{
    Handle
};

mod action;
mod initialize;

pub struct DynamicMessageHandler {
    current_handler : RefCell< Option< MessageHandlerWrapper > >
}

pub enum MessageHandlerWrapper {
    Initialize( initialize::InitializeHandler ),
    Action( action::ActionHandler )
}

impl DynamicMessageHandler {

    pub fn new( core_handle : Handle ) -> Self {
        let init_handler = initialize::InitializeHandler::new( core_handle );

        DynamicMessageHandler {
            current_handler : RefCell::new( Some( MessageHandlerWrapper::Initialize( init_handler ) ) )
        }
    }

}

impl MessageHandler for DynamicMessageHandler {

    fn handle_request( &self, service : ServiceHandle, request : ServerRequest, output : ResponseOutput ) {
        let mut cell = self.current_handler.borrow_mut( );
        let handler = cell.take( ).unwrap( );

        *cell = Some( handler.handle_request( service, request, output ) );
    }

    fn handle_notification( &self, service : ServiceHandle, notification : ServerNotification ) {
        let mut cell = self.current_handler.borrow_mut( );
        let handler = cell.take( ).unwrap( );

        *cell = Some( handler.handle_notification( service, notification ) );
    }

}

impl MessageHandlerWrapper {

    fn handle_request( self, service : ServiceHandle, request : ServerRequest, output : ResponseOutput ) -> MessageHandlerWrapper {
        match self {
            MessageHandlerWrapper::Initialize( initialize ) => initialize.handle_request( service, request, output ),
            MessageHandlerWrapper::Action( action ) => action.handle_request( service, request, output )
        }
    }

    fn handle_notification( self, service : ServiceHandle, notification : ServerNotification ) -> MessageHandlerWrapper {
        match self {
            MessageHandlerWrapper::Initialize( initialize ) => initialize.handle_notification( service, notification ),
            MessageHandlerWrapper::Action( action ) => action.handle_notification( service, notification )
        }
    }

}