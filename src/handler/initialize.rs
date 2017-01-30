#[deny( private_in_public )]

use ::{
    parent_process
};
use super::{
    action,
    MessageHandlerWrapper
};

use futures::{
    Future
};
use ls_service::service::{
    ServiceHandle,
    ResponseOutput
};
use lsp_rs::{
    InitializeResult,
    ResponseError,
    ServerCapabilities,
    ServerNotification,
    ServerResponse,
    ServerRequest,
    TextDocumentSyncKind,

    SERVER_NOT_INITIALIZED
};
use tokio_core::reactor::{
    Handle
};

pub struct InitializeHandler {
    core_handle : Handle
}

impl InitializeHandler {

    pub fn new( core_handle : Handle ) -> Self {
        InitializeHandler {
            core_handle : core_handle
        }
    }

    pub fn handle_request( self, service : ServiceHandle, request : ServerRequest, output : ResponseOutput ) -> MessageHandlerWrapper {
        if let ServerRequest::Initialize( params ) = request {
            debug!( "Initializing with params {:?}", params );
            if let Some( parent_id ) = params.process_id {
                debug!( "Params has parent id {}, registering shutdown future.", parent_id );
                let cloned_service = service.clone( );
                let parent_future = parent_process::open_parent_process( parent_id ).then( move | _ | {
                    debug!( "Parent process terminated. Shutting down service." );
                    cloned_service.shutdown( );

                    Ok( ( ) )
                } );

                self.core_handle.spawn( parent_future );
            }

            let result = InitializeResult {
                capabilities : ServerCapabilities {
                    text_document_sync                   : Some( TextDocumentSyncKind::Full ),
                    hover_provider                       : Some( false ),
                    completion_provider                  : None,
                    signature_help_provider              : None,
                    definition_provider                  : Some( false ),
                    references_provider                  : Some( false ),
                    document_highlight_provider          : Some( false ),
                    document_symbol_provider             : Some( false ),
                    workspace_symbol_provider            : Some( false ),
                    code_action_provider                 : Some( false ),
                    code_lens_provider                   : None,
                    document_formatting_provider         : Some( false ),
                    document_range_formatting_provider   : Some( false ),
                    document_on_type_formatting_provider : None,
                    rename_provider                      : Some( false )
                }
            };
            output.send_result( ServerResponse::Init( result ) );

            MessageHandlerWrapper::Action( action::ActionHandler::new( ) )
        }
        else {
            output.send_error( ResponseError {
                code    : SERVER_NOT_INITIALIZED,
                message : "Server is not initialized.".to_string( )
            } );

            MessageHandlerWrapper::Initialize( self )
        }
    }

    pub fn handle_notification( self, service : ServiceHandle, notification : ServerNotification ) -> MessageHandlerWrapper {
        info!( "Received notification {:?} in initialization handler.", notification );

        MessageHandlerWrapper::Initialize( self )
    }

}