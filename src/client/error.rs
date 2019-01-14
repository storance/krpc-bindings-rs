use super::schema;

#[derive(Debug, Fail)]
pub enum ConnectionNegotiateError {
    #[fail(display = "Malformed message: {}", _0)]
    MalformedMessage(String),
    #[fail(display = "Wrong connection type: {}", _0)]
    WrongType(String),
    #[fail(display = "Timeout: {}", _0)]
    Timeout(String)
}

#[derive(Debug, Fail)]
pub enum StreamError {
    #[fail(display = "Stream not started yet")]
    NotStarted,
    #[fail(display = "Value for stream not received yet")]
    MissingValue
}

#[derive(Debug, Fail)]
pub enum RpcError {
    #[fail(display = "{}: {}", description, stack_trace)]
    ResponseHasError {
        service: String,
        name: String,
        description: String,
        stack_trace: String
    },
    #[fail(display = "No procedure result for the rpc call")]
    NoResult
}

impl From<schema::Error> for RpcError {
    fn from(err: schema::Error) -> Self {
       RpcError::ResponseHasError{
           service: err.get_service().to_owned(),
           name: err.get_name().to_owned(),
           description: err.get_description().to_owned(),
           stack_trace: err.get_description().to_owned()
       }
    }
}
impl From<&schema::Error> for RpcError {
    fn from(err: &schema::Error) -> Self {
        RpcError::ResponseHasError{
            service: err.get_service().to_owned(),
            name: err.get_name().to_owned(),
            description: err.get_description().to_owned(),
            stack_trace: err.get_description().to_owned()
        }
    }
}