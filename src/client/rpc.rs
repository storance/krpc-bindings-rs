use super::{ConnectionRequest,
            ConnectionRequest_Type,
            ConnectionResponse,
            ConnectionResponse_Status,
            Request,
            ProcedureCall,
            Argument,
            Response,
            RpcError,
            ConnectionNegotiateError,
            recv_msg,
            send_msg};

use std::sync::Arc;
use std::net::TcpStream;
use std::cell::RefCell;

use failure::Error;
use protobuf::RepeatedField;

pub struct Rpc {
    id: Vec<u8>,
    socket: RefCell<TcpStream>
}

impl Rpc {
    pub(super) fn id(&self) -> &Vec<u8> {
        &self.id
    }

    pub(super) fn connect(name: &str, host: &str, port: u16) -> Result<Arc<Rpc>, Error> {
        let mut rpc_socket = TcpStream::connect((host, port))?;

        let mut request = ConnectionRequest::new();
        request.set_field_type(ConnectionRequest_Type::RPC);
        request.set_client_name(name.to_owned());

        send_msg(&mut rpc_socket, &request)?;
        let response: ConnectionResponse = recv_msg(&mut rpc_socket)?;

        let id  = match response.status {
            ConnectionResponse_Status::OK =>
                Vec::from(response.get_client_identifier()),
            ConnectionResponse_Status::MALFORMED_MESSAGE =>
                Err(ConnectionNegotiateError::MalformedMessage(response.message))?,
            ConnectionResponse_Status::TIMEOUT =>
                Err(ConnectionNegotiateError::Timeout(response.message))?,
            ConnectionResponse_Status::WRONG_TYPE =>
                Err(ConnectionNegotiateError::WrongType(response.message))?
        };

        Ok(Arc::new(Rpc{
            id,
            socket: RefCell::new(rpc_socket)
        }))
    }

    pub(super) fn invoke(&self, service: &str, procedure: &str, args: &Vec<Vec<u8>>) -> Result<Vec<u8>, Error> {
        let request = Self::create_request(service, procedure, args);

        send_msg(&mut self.socket.borrow_mut(), &request)?;
        let response: Response = recv_msg(&mut self.socket.borrow_mut())?;

        if response.has_error() {
            Err(RpcError::from(response.get_error()))?
        } else {
            let results = response.get_results();
            if results.len() == 0 {
                Err(RpcError::NoResult)?
            } else if results[0].has_error() {
                Err(RpcError::from(response.get_error()))?
            } else {
                Ok(Vec::from(results[0].get_value()))
            }
        }
    }

    fn create_request(service: &str, procedure: &str, args: &Vec<Vec<u8>>) -> Request {
        let mut rpc_args= RepeatedField::new();

        for (i, arg) in args.iter().enumerate() {
            let mut rpc_arg = Argument::new();
            rpc_arg.set_position(i as u32);
            rpc_arg.set_value(arg.clone());

            rpc_args.push(rpc_arg);
        }

        let mut procedure_call = ProcedureCall::new();
        procedure_call.set_service(service.to_owned());
        procedure_call.set_procedure(procedure.to_owned());
        procedure_call.set_arguments(rpc_args);

        let mut request = Request::new();
        request.set_calls(RepeatedField::from_vec(vec![procedure_call]));

        request
    }
}