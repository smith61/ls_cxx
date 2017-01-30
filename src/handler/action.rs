
use super::{
    MessageHandlerWrapper
};
use ls_service::service::{
    ResponseOutput,
    ServiceHandle
};
use lsp_rs::{
    ServerNotification,
    ServerRequest
};

pub struct ActionHandler {

}

impl ActionHandler {

    pub fn new( ) -> Self {
        ActionHandler {

        }
    }

    pub fn handle_request( self, service : ServiceHandle, request : ServerRequest, output : ResponseOutput ) -> MessageHandlerWrapper {
        info!( "Received request: {:?}", request );

        MessageHandlerWrapper::Action( self )
    }

    pub fn handle_notification( self, service : ServiceHandle, notification : ServerNotification ) -> MessageHandlerWrapper {
        info!( "Received notification: {:?}", notification );

        MessageHandlerWrapper::Action( self )
    }

}