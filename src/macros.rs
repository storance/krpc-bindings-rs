#[macro_export]
macro_rules! rpc_method {
    // RPC Class Method with pass self to rpc method
    (
        $(#[$meta:meta])*
        fn $func_name: ident (&self $(, $arg_name : ident : $arg_type : ty)* ) -> $ret : ty {
            if let Some($value: ident) = $service : tt.$method : tt ( self $(, $method_arg : expr )* ) as $value_type: ty {
                $success_expr: expr
            } else {
                $fail_expr: expr
            }
        }
    ) => {
        $(#[$meta])*
        pub fn $func_name(&self $(, $arg_name : $arg_type)*) -> KrpcResult<$ret> {
            let rpc = &mut (*self.client).borrow_mut().rpc;
            let args: Vec<Vec<u8>> = vec![self.encode()? $(, $method_arg.encode()?)*];

            Ok(if let Some(response) = rpc.invoke(stringify!($service).to_owned(), stringify!($method).to_owned(), args)? {
                let $value : $value_type = decode(&response, &self.client)?;
                $success_expr
            } else {
                $fail_expr
            })
        }
    };

    // RPC Class Method
    (
        $(#[$meta:meta])*
        fn $func_name: ident (&self $(, $arg_name : ident : $arg_type : ty)* ) -> $ret : ty {
            if let Some($value: ident) = $service : tt.$method : tt ( $($method_arg : expr ),* ) as $value_type: ty {
                $success_expr: expr
            } else {
                $fail_expr: expr
            }
        }
    ) => {
        $(#[$meta])*
        pub fn $func_name(&self $(, $arg_name : $arg_type)*) -> KrpcResult<$ret> {
            let rpc = &mut (*self.client).borrow_mut().rpc;
            let args: Vec<Vec<u8>> = vec![$($method_arg.encode()?),*];

            Ok(if let Some(response) = rpc.invoke(stringify!($service).to_owned(), stringify!($method).to_owned(), args)? {
                let $value : $value_type = decode(&response, &self.client)?;
                $success_expr
            } else {
                $fail_expr
            })
        }
    };

    // RPC Class Method with pass self to rpc method and Void return
    (
        $(#[$meta:meta])*
        fn $func_name: ident (&self $(, $arg_name : ident : $arg_type : ty)* ) {
            $service : tt.$method : tt ( self $(, $method_arg : expr )* )
        }
    ) => {
        $(#[$meta])*
        pub fn $func_name(&self $(, $arg_name : $arg_type)*) -> KrpcResult<()> {
            let rpc = &mut (*self.client).borrow_mut().rpc;
            let args: Vec<Vec<u8>> = vec![self.encode()? $(, $method_arg.encode()?)*];

            rpc.invoke(stringify!($service).to_owned(), stringify!($method).to_owned(), args)?;
            Ok(())
        }
    };

    // RPC Class Method with Void return
    (
        $(#[$meta:meta])*
        fn $func_name: ident (&self $(, $arg_name : ident : $arg_type : ty)* ) {
            $service : tt.$method : tt ( $($method_arg : expr ),* )
        }
    ) => {
        $(#[$meta])*
        pub fn $func_name(&self $(, $arg_name : $arg_type)*) -> KrpcResult<()> {
            let rpc = &mut (*self.client).borrow_mut().rpc;
            let args: Vec<Vec<u8>> = vec![$($method_arg.encode()?),*];

            rpc.invoke(stringify!($service).to_owned(), stringify!($method).to_owned(), args)?;
            Ok(())
        }
    };

    // RPC Static Method
    (
        $(#[$meta:meta])*
        fn $func_name: ident ($client: ident : &KrpcClient $(, $arg_name : ident : $arg_type : ty)* ) -> $ret : ty {
            if let Some($value: ident) = $service : tt.$method : tt ( $($method_arg : expr ),* ) as $value_type: ty {
                $success_expr: expr
            } else {
                $fail_expr: expr
            }
        }
    ) => {
        $(#[$meta])*
        pub fn $func_name($client : &Rc<RefCell<KrpcClient>> $(, $arg_name : $arg_type)*) -> KrpcResult<$ret> {
            let rpc = &mut (*$client).borrow_mut().rpc;
            let args: Vec<Vec<u8>> = vec![$($method_arg.encode()?),*];

            Ok(if let Some(response) = rpc.invoke(stringify!($service).to_owned(), stringify!($method).to_owned(), args)? {
                let $value : $value_type = decode(&response, $client)?;
                $success_expr
            } else {
                $fail_expr
            })
        }
    };

    // RPC Static Method with Void return
    (
        $(#[$meta:meta])*
        fn $func_name: ident ($client: ident : &KrpcClient $(, $arg_name : ident : $arg_type : ty)* ) {
            $service : tt.$method : tt ( $($method_arg : expr ),* )
        }
    ) => {
        $(#[$meta])*
        pub fn $func_name($client : &Rc<RefCell<KrpcClient>> $(, $arg_name : $arg_type)*) -> KrpcResult<()> {
            let rpc = &mut (*$client).borrow_mut().rpc;
            let args: Vec<Vec<u8>> = vec![$($method_arg.encode()?),*];

            rpc.invoke(stringify!($service).to_owned(), stringify!($method).to_owned(), args)?;
            Ok(())
        }
    };

}

/*macro_rules! rpc_method {
    // RPC Method with an additional transform on the optional response value
    ( $(#[$meta:meta])*
    fn $func_name: ident ($self_alias: ident $(, $arg_name : ident : $arg_type : ty)* ) -> Option<$ret : ty> {
        $service : tt.$method : tt ( $( $method_arg : expr ),* )
            .map(|$value: ident $(: $value_type: ty)*| $some_result: expr)
    }) => {
        rpc_method_impl ! (class_method
            meta = ( $(#[$meta])* ),
            func_name = $func_name,
            self_alias = $self_alias,
            args = ( $( $arg_name: $arg_type), * ),
            return_type = Option<$ret>,
            service = $service,
            method = $method,
            method_args = ( $( $method_arg ),* ),
            value_name = $value,
            value_type = ( $($value_type),* ),
            some_result = $some_result,
            none_result = None);
    };

    // RPC Method with an additional transform on the response value
    ( $(#[$meta:meta])*
    fn $func_name: ident ($self_alias: ident $(, $arg_name : ident : $arg_type : ty)* ) -> $ret : ty {
        $service : tt.$method : tt ( $($method_arg : expr ),* )
            .map(|$value: ident $(: $value_type: ty)*| $some_result: expr)
    }) => {
        rpc_method_impl ! (class_method
            meta = ( $(#[$meta])* ),
            func_name = $func_name,
            self_alias = $self_alias,
            args = ( $( $arg_name: $arg_type), * ),
            return_type = $ret,
            service = $service,
            method = $method,
            method_args = ( $( $method_arg ),* ),
            value_name = $value,
            value_type = ( $($value_type),* ),
            some_result = $some_result,
            none_result = return Err(KrpcError::NullResponseValue));
    };

    // RPC Method with no response value
    ( $(#[$meta:meta])*
    fn $func_name: ident ($self_alias: ident $(, $arg_name : ident : $arg_type : ty)* ) {
        $service : tt.$method : tt ( $($method_arg : expr ),* )
    }) => {
        rpc_method_impl ! (class_method
            meta = ( $(#[$meta])* ),
            func_name = $func_name,
            self_alias = $self_alias,
            args = ( $( $arg_name: $arg_type), * ),
            no_return,
            service = $service,
            method = $method,
            method_args = ( $( $method_arg ),* ));
    };

    // RPC Method with an optional response value and no additional transform
    ( $(#[$meta:meta])*
    fn $func_name: ident ($self_alias: ident $(, $arg_name : ident : $arg_type : ty)* ) -> Option<$ret : ty> {
        $service : tt.$method : tt ( $( $method_arg : expr ),* )
    }) => {
        rpc_method_impl ! (class_method
            meta = ( $(#[$meta])* ),
            func_name = $func_name,
            self_alias = $self_alias,
            args = ( $( $arg_name: $arg_type), * ),
            return_type = Option<$ret>,
            service = $service,
            method = $method,
            method_args = ( $( $method_arg ),* ),
            value_name = value,
            value_type = ( ),
            some_result = value,
            none_result = None);
    };

    // RPC Method with a response value and no additional transform
    ( $(#[$meta:meta])*
    fn $func_name: ident ($self_alias: ident $(, $arg_name : ident : $arg_type : ty)* ) -> $ret : ty {
        $service : tt.$method : tt ( $( $method_arg : expr ),* )
    }) => {
        rpc_method_impl ! (class_method
            meta = ( $(#[$meta])* ),
            func_name = $func_name,
            self_alias = $self_alias,
            args = ( $( $arg_name: $arg_type),* ),
            return_type = $ret,
            service = $service,
            method = $method,
            method_args = ( $( $method_arg ),* ),
            value_name = value,
            value_type = ( ),
            some_result = value,
            none_result = return Err(KrpcError::NullResponseValue));
    };

    // Static RPC Method with an additional transform on the optional response value
    ( $(#[$meta:meta])*
    fn $func_name: ident (&client, $(, $arg_name : ident : $arg_type : ty)* ) -> Option<$ret : ty> {
        $service : tt.$method : tt ( $( $method_arg : expr ),* )
            .map(|$value: ident $(: $value_type: ty)*| $some_result: expr)
    }) => {
        rpc_method_impl ! (static_method
            meta = ( $(#[$meta])* ),
            func_name = $func_name,
            args = ( $( $arg_name: $arg_type),* ),
            return_type = Option<$ret>,
            service = $service,
            method = $method,
            method_args = ( $( $method_arg ),* ),
            value_name = $value,
            value_type = ( $($value_type),* ),
            some_result = $some_result,
            none_result = None);
    };

    // Static RPC Method with an additional transform on the response value
    ( $(#[$meta:meta])*
    fn $func_name: ident (&client, $(, $arg_name : ident : $arg_type : ty)* ) -> $ret : ty {
        $service : tt.$method : tt ( $( $method_arg : expr ),* )
            .map(|$value: ident $(: $value_type: ty)*| $some_result: expr)
    }) => {
        rpc_method_impl ! (static_method
            meta = ( $(#[$meta])* ),
            func_name = $func_name,
            args = ( $( $arg_name: $arg_type), * ),
            return_type = Option<$ret>,
            service = $service,
            method = $method,
            method_args = ( $( $method_arg ), * ),
            value_name = $value,
            value_type = ( $($value_type),* ),
            some_result = $some_result,
            none_result = return Err(KrpcError::NullResponseValue));
    };

    // Static RPC Method with no response value
    ( $(#[$meta:meta])*
    fn $func_name: ident (&client $(, $arg_name : ident : $arg_type : ty)* ) {
        $service : tt.$method : tt ( $( $method_arg : expr ),* )
    }) => {
        rpc_method_impl ! (static_method
            meta = ( $(#[$meta])* ),
            func_name = $func_name,
            args = ( $( $arg_name: $arg_type), * ),
            no_return,
            service = $service,
            method = $method,
            method_args = ( $( $method_arg ), * ));
    };

    // Static RPC Method with an optional response value and no additional transform
    ( $(#[$meta:meta])*
    fn $func_name: ident (&client $(, $arg_name : ident : $arg_type : ty)* ) -> Option<$ret : ty> {
        $service : tt.$method : tt ( $( $method_arg : expr ),* )
    }) => {
        rpc_method_impl ! (static_method
            meta = ( $(#[$meta])* ),
            func_name = $func_name,
            args = ( $( $arg_name: $arg_type), * ),
            return_type = Option<$ret>,
            service = $service,
            method = $method,
            method_args = ( $( $method_arg ), * ),
            value_name = value,
            value_type = ( ),
            some_result = value,
            none_result = None);
    };

    // Static RPC Method with a response value and no additional transform
    ( $(#[$meta:meta])*
    fn $func_name: ident (&client $(, $arg_name : ident : $arg_type : ty)* ) -> $ret : ty {
        $service : tt.$method : tt ( $( $method_arg : expr ),* )
    }) => {
        rpc_method_impl ! (static_method
            meta = ( $(#[$meta])* ),
            func_name = $func_name,
            args = ( $( $arg_name: $arg_type), * ),
            return_type = $ret,
            service = $service,
            method = $method,
            method_args = ( $( $method_arg ), * ),
            value_name = value,
            value_type = ( ),
            some_result = value,
            none_result = return Err(KrpcError::NullResponseValue));
    };
}*/

/*
#[macro_export]
macro_rules! rpc_method_impl {
    (
        class_method
        meta=( $( #[$meta:meta])* ),
        func_name=$func_name: ident,
        self_alias=$self_alias: ident,
        args=( $($arg_name: ident : $arg_type: ty),* ),
        return_type=$ret: ty,
        service=$service: tt,
        method=$method: tt,
        method_args=($( $method_arg : expr ),*),
        value_name=$value: ident,
        value_type=( $($value_type: ty),* ),
        some_result=$some_result: expr,
        none_result=$none_result: expr
    ) => {
        $(#[$meta])*
        pub fn $func_name(&self $(, $arg_name : $arg_type)*) -> KrpcResult<$ret> {
            let rpc = &mut (*self.client).borrow_mut().rpc;
            let $self_alias = self;
            let args: Vec<Vec<u8>> = vec![ $($method_arg.encode()?),* ];

            Ok(match rpc.invoke(stringify!($service).to_owned(), stringify!($method).to_owned(), args)? {
                Some(response) => {
                    let $value $(: $value_type)* = decode(&response, &self.client)?;
                    $some_result
                },
                None => $none_result
            })
        }
    };

    (
        class_method
        meta=( $( #[$meta:meta])* ),
        func_name=$func_name: ident,
        self_alias=$self_alias: ident,
        args=( $($arg_name : ident : $arg_type : ty),* ),
        no_return,
        service=$service : tt,
        method=$method : tt,
        method_args=($( $method_arg : expr ),*)
    ) => {
        $(#[$meta])*
        pub fn $func_name(&self $(, $arg_name : $arg_type)*) -> KrpcResult<()> {
            let rpc = &mut (*self.client).borrow_mut().rpc;
            let $self_alias = self;
            let args: Vec<Vec<u8>> = vec![ $($method_arg.encode()?),* ];

            rpc.invoke(stringify!($service).to_owned(), stringify!($method).to_owned(), args)?;
            Ok(())
        }
    };

    (
        static_method
        meta=( $( #[$meta:meta])* ),
        func_name=$func_name: ident,
        args=( $($arg_name : ident : $arg_type : ty),* ),
        return_type=$ret : ty,
        service=$service : tt,
        method=$method : tt,
        method_args=($( $method_arg : expr ),*),
        value_name=$value:ident,
        value_type=( $($value_type: ty),* ),
        some_result=$some_result: expr,
        none_result=$none_result: expr
    ) => {
        $(#[$meta])*
        pub fn $func_name(client: &Rc<RefCell<KrpcClient>> $(, $arg_name : $arg_type)*) -> KrpcResult<$ret> {
            let rpc = &mut (*client).borrow_mut().rpc;
            let mut _args = vec!();
            $(
                _args.push($method_arg.encode()?);
            )*

            Ok(match rpc.invoke(stringify!($service).to_owned(), stringify!($method).to_owned(), _args)? {
                Some(response) => {
                    let $value $(: $value_type)* = decode(&response, &client)?;
                    $some_result
                },
                None => $none_result
            })
        }
    };

    (
        static_method
        meta=( $( #[$meta:meta])* ),
        func_name=$func_name: ident,
        args=( $($arg_name : ident : $arg_type : ty),* ),
        no_return,
        service=$service : tt,
        method=$method : tt,
        method_args=($( $method_arg : expr ),*)
    ) => {
        $(#[$meta])*
        pub fn $func_name(client: &Rc<RefCell<KrpcClient>> $(, $arg_name : $arg_type)*) -> KrpcResult<()> {
            let rpc = &mut (*client).borrow_mut().rpc;
            let mut _args = vec!();
            $(
                _args.push($method_arg.encode()?);
            )*

            rpc.invoke(stringify!($service).to_owned(), stringify!($method).to_owned(), _args)?;
            Ok(())
        }
    };
}*/

#[macro_export]
macro_rules! remote_type {
    ( $(#[$meta:meta])*
      object $object_name: ident {}) => {
        $(#[$meta])*
        pub struct $object_name {
            #[allow(dead_code)]
            client : Rc<RefCell<KrpcClient>>,
            id: u64
        }

        impl RemoteObject for $object_name {
            fn new(client: Rc<RefCell<KrpcClient>>, id: u64) -> $object_name {
                $object_name{client, id}
            }

            fn id(&self) -> u64 { self.id }
        }

        impl Decode for $object_name {
            fn decode(bytes: &Vec<u8>, client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError> {
                decode_remote_obj(bytes, client)
            }
        }

        impl Decode for Option<$object_name> {
            fn decode(bytes: &Vec<u8>, client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError> {
                decode_remote_obj_opt(bytes, client)
            }
        }

        impl Encode for $object_name {
            fn encode(&self) -> Result<Vec<u8>, CodecError> {
                encode_remote_obj(self)
            }
        }

        impl Encode for Option<$object_name> {
            fn encode(&self) -> Result<Vec<u8>, CodecError> {
                encode_remote_obj_opt(self)
            }
        }
    };

    ( $(#[$enum_meta:meta])*
    enum $enum_name: ident {
        $( $(#[$variant_meta:meta])* $value_name: ident => $value_int : expr),+
    }) => {
        $(#[$enum_meta])*
        #[derive(Debug)]
        pub enum $enum_name {
            $(
                $(#[$variant_meta])*
                $value_name
            ),+
        }

        impl RemoteEnum for $enum_name {
            fn from_value(value: i64) -> Option<Self> {
                match value {
                    $( $value_int => Some($enum_name::$value_name)),+,
                    _ => None
                }
            }

            fn value(&self) -> i64 {
                match self {
                    $( $enum_name::$value_name => $value_int),+
                }
            }
        }

        impl Decode for $enum_name {
            fn decode(bytes: &Vec<u8>, _client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError> {
                decode_remote_enum(bytes)
            }
        }

        impl Encode for $enum_name {
            fn encode(&self) -> Result<Vec<u8>, CodecError> {
                encode_remote_enum(self)
            }
        }
    }
}