use super::schema::{
    Argument, ConnectionRequest, ConnectionRequest_Type, ConnectionResponse,
    ConnectionResponse_Status, ProcedureCall, Request, Response,
};
use super::{
    convert_procedure_result, recv_msg, send_msg, ConnectionError, ResponseError, RpcError,
};

use std::cell::RefCell;
use std::net::TcpStream;

#[derive(Debug)]
pub struct Rpc {
    id: Vec<u8>,
    socket: RefCell<TcpStream>,
}

impl Rpc {
    pub(super) fn id(&self) -> &[u8] {
        self.id.as_slice()
    }

    pub(super) fn connect(name: &str, host: &str, port: u16) -> Result<Rpc, ConnectionError> {
        let mut rpc_socket = TcpStream::connect((host, port))?;

        let mut request = ConnectionRequest::new();
        request.set_field_type(ConnectionRequest_Type::RPC);
        request.set_client_name(name.to_owned());

        send_msg(&mut rpc_socket, &request)?;
        let response: ConnectionResponse = recv_msg(&mut rpc_socket)?;

        if response.status == ConnectionResponse_Status::OK {
            Ok(Rpc {
                id: Vec::from(response.get_client_identifier()),
                socket: RefCell::new(rpc_socket),
            })
        } else {
            Err(ConnectionError::ConnectionError(response.message))
        }
    }

    pub(super) fn invoke(
        &self,
        service: &str,
        procedure: &str,
        args: &Vec<Vec<u8>>,
    ) -> Result<Vec<u8>, RpcError> {
        let request = Self::create_request(service, procedure, args);

        send_msg(&mut self.socket.borrow_mut(), &request)?;
        let response: Response = recv_msg(&mut self.socket.borrow_mut())?;

        if response.has_error() {
            Err(RpcError::ResponseError(ResponseError::from(
                response.get_error(),
            )))
        } else {
            let results = response.get_results();
            if results.len() == 0 {
                Err(RpcError::ResponseError(ResponseError::MissingResult))
            } else {
                Ok(convert_procedure_result(&results[0])?)
            }
        }
    }

    pub(super) fn create_request(service: &str, procedure: &str, args: &Vec<Vec<u8>>) -> Request {
        let mut request = Request::new();
        request
            .calls
            .push(Self::create_procedure_call(service, procedure, args));

        request
    }

    pub(super) fn create_procedure_call(
        service: &str,
        procedure: &str,
        args: &Vec<Vec<u8>>,
    ) -> ProcedureCall {
        let mut procedure_call = ProcedureCall::new();
        procedure_call.set_service(service.to_owned());
        procedure_call.set_procedure(procedure.to_owned());

        for (i, arg) in args.iter().enumerate() {
            let mut rpc_arg = Argument::new();
            rpc_arg.set_position(i as u32);
            rpc_arg.set_value(arg.clone());

            procedure_call.arguments.push(rpc_arg);
        }

        procedure_call
    }
}
