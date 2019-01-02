#[macro_export]
macro_rules! rpc_method {
    // RPC Class Method with pass self to rpc method and map decoded value
    (
        $(#[$meta:meta])*
        fn $func_name: ident (&self $(, $arg_name : ident : $arg_type : ty)* ) -> $ret : ty {
            $service : tt.$( $method : tt),+ ( self $(, $method_arg : expr )* )
                .map(|$value: ident : $value_type : ty| $success_expr: expr)
                .ok_or($fail_expr: expr)
        }
    ) => {
        rpc_method!(
            $(#[$meta])*
            fn $func_name(&self $(, $arg_name : $arg_type)* ) -> $ret {
                if let Some($value) = $service.$( $method ),+( self $(, $method_arg )* ) as $value_type {
                    $success_expr
                } else {
                    return Err($fail_expr)
                }
            }
        );
    };

    // RPC Class Method with map decoded value
    (
        $(#[$meta:meta])*
        fn $func_name: ident (&self $(, $arg_name : ident : $arg_type : ty)* ) -> $ret : ty {
            $service : tt.$( $method : tt),+ ( self $(, $method_arg : expr )* )
                .map(|$value: ident : $value_type : ty| $success_expr: expr)
                .ok_or($fail_expr: expr)
        }
    ) => {
        rpc_method!(
            $(#[$meta])*
            fn $func_name(&self $(, $arg_name : $arg_type)* ) -> $ret {
                if let Some($value) = $service.$( $method ),+( $( $method_arg ),* ) as $value_type {
                    success_expr
                } else {
                    return Err($fail_expr)
                }
            }
        );
    };

    // RPC Class Method with pass self to rpc method and return decoded value as is
    (
        $(#[$meta:meta])*
        fn $func_name: ident (&self $(, $arg_name : ident : $arg_type : ty)* ) -> $ret : ty {
            $service : tt.$( $method : tt),+ ( self $(, $method_arg : expr )* )
                .ok_or($fail_expr: expr)
        }
    ) => {
        rpc_method!(
            $(#[$meta])*
            fn $func_name(&self $(, $arg_name : $arg_type)* ) -> $ret {
                if let Some(value) = $service.$( $method ),+( self $(, $method_arg )* ) as $ret {
                    value
                } else {
                    return Err($fail_expr)
                }
            }
        );
    };

    // RPC Class Method with return decoded value as is
    (
        $(#[$meta:meta])*
        fn $func_name: ident (&self $(, $arg_name : ident : $arg_type : ty)* ) -> $ret : ty {
            $service : tt.$( $method : tt),+ ( self $(, $method_arg : expr )* )
                .ok_or($fail_expr: expr)
        }
    ) => {
        rpc_method!(
            $(#[$meta])*
            fn $func_name(&self $(, $arg_name : $arg_type)* ) -> $ret {
                if let Some(value) = $service.$( $method ),+( $( $method_arg ),* ) as $ret {
                    value
                } else {
                    return Err($fail_expr)
                }
            }
        );
    };

    // RPC Class Method with pass self to rpc method
    (
        $(#[$meta:meta])*
        fn $func_name: ident (&self $(, $arg_name : ident : $arg_type : ty)* ) -> $ret : ty {
            if let Some($value: ident) = $service : tt.$( $method : tt),+ ( self $(, $method_arg : expr )* ) as $value_type: ty {
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

            Ok(if let Some(response) = rpc.invoke(stringify!($service).to_owned(), concat!( $( stringify!($method) ),+).to_owned(), args)? {
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
            if let Some($value: ident) = $service : tt.$( $method : tt),+ ( $($method_arg : expr ),* ) as $value_type: ty {
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

            Ok(if let Some(response) = rpc.invoke(stringify!($service).to_owned(), concat!( $( stringify!($method) ),+).to_owned(), args)? {
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
            $service : tt.$( $method : tt),+( self $(, $method_arg : expr )* )
        }
    ) => {
        $(#[$meta])*
        pub fn $func_name(&self $(, $arg_name : $arg_type)*) -> KrpcResult<()> {
            let rpc = &mut (*self.client).borrow_mut().rpc;
            let args: Vec<Vec<u8>> = vec![self.encode()? $(, $method_arg.encode()?)*];

            rpc.invoke(stringify!($service).to_owned(), concat!( $( stringify!($method) ),+).to_owned(), args)?;
            Ok(())
        }
    };

    // RPC Class Method with Void return
    (
        $(#[$meta:meta])*
        fn $func_name: ident (&self $(, $arg_name : ident : $arg_type : ty)* ) {
            $service : tt.$( $method : tt),+ ( $($method_arg : expr ),* )
        }
    ) => {
        $(#[$meta])*
        pub fn $func_name(&self $(, $arg_name : $arg_type)*) -> KrpcResult<()> {
            let rpc = &mut (*self.client).borrow_mut().rpc;
            let args: Vec<Vec<u8>> = vec![$($method_arg.encode()?),*];

            rpc.invoke(stringify!($service).to_owned(), concat!( $( stringify!($method) ),+).to_owned(), args)?;
            Ok(())
        }
    };


    // RPC Static Method with return decoded value as is
    (
        $(#[$meta:meta])*
        fn $func_name: ident ($client: ident : &KrpcClient $(, $arg_name : ident : $arg_type : ty)* ) -> $ret : ty {
            $service : tt.$method : tt ( $( $method_arg : expr ),* )
                .ok_or($fail_expr: expr)
        }
    ) => {
        rpc_method!(
            $(#[$meta])*
            fn $func_name($client: &KrpcClient  $(, $arg_name : $arg_type)* ) -> $ret {
                if let Some(value) = $service.$method( $( $method_arg ),* ) as $ret {
                    value
                } else {
                    return Err($fail_expr)
                }
            }
        );
    };

    // RPC Static Method with map decoded value
    (
        $(#[$meta:meta])*
        fn $func_name: ident ($client: ident : &KrpcClient $(, $arg_name : ident : $arg_type : ty)* ) -> $ret : ty {
            $service : tt.$method : tt ( $( $method_arg : expr ),* )
                .map(|$value: ident : $value_type: ident| $success_expr: expr)
                .ok_or($fail_expr: expr)
        }
    ) => {
        rpc_method!(
            $(#[$meta])*
            fn $func_name($client: &KrpcClient  $(, $arg_name : $arg_type)* ) -> $ret {
                if let Some($value) = $service.$method( $( $method_arg ),* ) as $value_type {
                    $success_expr
                } else {
                    return Err($fail_expr)
                }
            }
        );
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

#[macro_export]
macro_rules! rpc_property {
    // Class level props
    (
        $prop_name: ident : $prop_type: ty {
            service: $service: tt,
            class: $class: tt,
            $(#[$getter_meta:meta])*
            $getter_name: ident,
            $(#[$setter_meta:meta])*
            $setter_name: ident($arg_name: ident)
        }
    ) => {
        rpc_property!(
            $prop_name: $prop_type {
                service: $service,
                class: $class,
                $(#[$getter_meta])*
                $getter_name => |value: $prop_type| value,
                $(#[$setter_meta])*
                $setter_name($arg_name) => $arg_name
            }
        );
    };

    (
        $prop_name: ident : $prop_type: ty {
            service: $service: tt,
            class: $class: tt,
            $(#[$getter_meta:meta])*
            $getter_name: ident => |$value: ident : $value_type: ty| $getter_expr: expr,
            $(#[$setter_meta:meta])*
            $setter_name: ident($arg_name: ident) => $setter_expr: expr
        }
    ) => {
        rpc_method!(
        $( #[$getter_meta] )*
        fn $getter_name(&self) -> $prop_type {
            $service.$class,_get_,$prop_name(self)
                .map(|$value: $value_type| $getter_expr)
                .ok_or(KrpcError::NullResponseValue)
        });

        rpc_method!(
        $(#[$setter_meta])*
        fn $setter_name(&self, $arg_name: $prop_type) {
            $service.$class,_set_,$prop_name(self, $setter_expr)
        });
    };

    (
        $prop_name: ident : $prop_type: ty {
            service: $service: tt,
            class: $class: tt,
            $(#[$getter_meta:meta])*
            $getter_name: ident
        }
    ) => {
        rpc_property!(
            $prop_name: $prop_type {
                service: $service,
                class: $class,
                $(#[$getter_meta])*
                $getter_name => |value: $prop_type| value
            }
        );
    };

    (
        $prop_name: ident : $prop_type: ty {
            service: $service: tt,
            class: $class: tt,
            $(#[$getter_meta:meta])*
            $getter_name: ident => |$value: ident : $value_type: ty| $getter_expr: expr
        }
    ) => {
        rpc_method!(
        $( #[$getter_meta] )*
        fn $getter_name(&self) -> $prop_type {
            $service.$class,_get_,$prop_name(self)
                .map(|$value: $value_type| $getter_expr)
                .ok_or(KrpcError::NullResponseValue)
        });
    };

    // Service level props
    (
        $prop_name: ident : $prop_type: ty {
            service: $service: tt,
            $(#[$getter_meta:meta])*
            $getter_name: ident,
            $(#[$setter_meta:meta])*
            $setter_name: ident($arg_name: ident)
        }
    ) => {
        rpc_property!(
            $prop_name: $prop_type {
                service: $service,
                $(#[$getter_meta])*
                $getter_name => |value: $prop_type| value,
                $(#[$setter_meta])*
                $setter_name($arg_name) => $arg_name
            }
        );
    };

    (
        $prop_name: ident : $prop_type: ty {
            service: $service: tt,
            $(#[$getter_meta:meta])*
            $getter_name: ident => |$value: ident : $value_type: ty| $getter_expr: expr,
            $(#[$setter_meta:meta])*
            $setter_name: ident($arg_name: ident) => $setter_expr: expr
        }
    ) => {
        rpc_method!(
        $( #[$getter_meta] )*
        fn $getter_name(&self) -> $prop_type {
            $service.get_,$prop_name()
                .map(|$value: $value_type| $getter_expr)
                .ok_or(KrpcError::NullResponseValue)
        });

        rpc_method!(
        $(#[$setter_meta])*
        fn $setter_name(&self, $arg_name: $prop_type) {
            $service._set_,$prop_name($setter_expr)
        });
    };

    (
        $prop_name: ident : $prop_type: ty {
            service: $service: tt,
            $(#[$getter_meta:meta])*
            $getter_name: ident
        }
    ) => {
        rpc_property!(
            $prop_name: $prop_type {
                service: $service,
                $(#[$getter_meta])*
                $getter_name => |value: $prop_type| value
            }
        );
    };

    (
        $prop_name: ident : $prop_type: ty {
            service: $service: tt,
            $(#[$getter_meta:meta])*
            $getter_name: ident => |$value: ident : $value_type: ty| $getter_expr: expr
        }
    ) => {
        rpc_method!(
        $( #[$getter_meta] )*
        fn $getter_name(&self) -> $prop_type {
            $service.get_,$prop_name()
                .map(|$value: $value_type| $getter_expr)
                .ok_or(KrpcError::NullResponseValue)
        });
    };

}

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
                let id: u64 = decode(bytes, client)?;
                if id == 0 {
                    Err(CodecError::NullValue)
                } else {
                    Ok($object_name::new(Rc::clone(client), id))
                }
            }
        }

        impl Decode for Option<$object_name> {
            fn decode(bytes: &Vec<u8>, client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError> {
                let id: u64 = decode(bytes, client)?;
                if id == 0 {
                    Ok(None)
                } else {
                    Ok(Some($object_name::new(Rc::clone(client), id)))
                }
            }
        }

        impl Encode for $object_name {
            fn encode(&self) -> Result<Vec<u8>, CodecError> {
                self.id().encode()
            }
        }

        impl Encode for Option<$object_name> {
            fn encode(&self) -> Result<Vec<u8>, CodecError> {
                match self {
                    None => (0 as u64).encode(),
                    Some(obj) => obj.id().encode()
                }
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
            fn decode(bytes: &Vec<u8>, client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError> {
                let value: i64 = decode(bytes, client)?;
                $enum_name::from_value(value)
                    .ok_or(CodecError::InvalidEnumValue(value))
            }
        }

        impl Encode for $enum_name {
            fn encode(&self) -> Result<Vec<u8>, CodecError> {
                self.value().encode()
            }
        }
    }
}