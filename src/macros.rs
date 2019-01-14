#[macro_export]
macro_rules! remote_type {
    //
    // Service
    //
    (
        $(#[$meta:meta])*
        service $service: ident {
            properties: {
                $( { $( $property: tt)+ } )*
            },
            methods: {
                $( { $( $method: tt)+ } )*
            }
        }
    ) => {
        $(#[$meta])*
        pub struct $service {
            connection: Rc<Connection>
        }

        impl $service {
            /// Creates a new service using the given `client`.
            pub fn new(connection: Rc<Connection>) -> Self {
                $service{connection}
            }

            // Properties
            $(
                remote_type!(@service_property(service=$service) $( $property )+ );
            )*

            // Methods
            $(
                remote_type!(@service_method(service=$service) $( $method )+ );
            )*
        }
    };

    //
    // Properties
    //
    (
        @service_property(service=$service:tt)
            $prop_name: ident : $prop_type: ty,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident
    ) => {
        remote_type!(@service_method(service=$service, prefix=get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> $prop_type {
                $prop_name()
            });

        remote_type!(@service_method(service=$service, prefix=set_)
            $( #[$setter_meta] )*
            fn $setter_name(value: $prop_type) {
                $prop_name(value)
            });
    };

    (
        @service_property(service=$service:tt)
            $prop_name: ident : $set_type: ty => $get_type: ty,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident
    ) => {
        remote_type!(@service_method(service=$service, prefix=get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> $get_type {
                $prop_name()
            });

        $(remote_type!(@service_method(service=$service, prefix=set_)
            $( #[$setter_meta] )*
            fn $setter_name(value: $set_type) {
                $prop_name(value)
            }))?;
    };

    (
        @service_property(service=$service:tt)
            $prop_name: ident : $prop_type: ty,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident
    ) => {
        remote_type!(@service_method(service=$service, prefix=get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> $prop_type {
                $prop_name()
            });
    };

    //
    // Methods
    //
    (
        @service_method(service=$service:tt $(, prefix=$prefix:tt)?)
        $(#[$meta:meta])*
        fn $method_name: ident ($( $arg_name: ident : $arg_type: ty), *) -> $return_type: ty {
            $rpc_name: tt($( $arg_expr: expr ),* )
        }
    ) => {
        $(#[$meta])*
        pub fn $method_name(&self $(, $arg_name : $arg_type)*) -> RpcResult<$return_type> {
            let args: Vec<Vec<u8>> = vec![$($arg_expr.encode()?),*];

            let response = self.connection.invoke(stringify!($service),
                concat!( $( stringify!($prefix), )? stringify!($rpc_name)),
                &args)?;
            Ok(decode(&response, self.connection.clone())?)
        }
    };

    (
        @service_method(service=$service:tt $(, prefix=$prefix:tt)?)
        $(#[$meta:meta])*
        fn $method_name: ident ($( $arg_name: ident : $arg_type: ty), *) {
            $rpc_name: tt($( $arg_expr: expr ),* )
        }
    ) => {
        $(#[$meta])*
        pub fn $method_name(&self $(, $arg_name : $arg_type)*) -> RpcResult<()> {
            let args: Vec<Vec<u8>> = vec![$($arg_expr.encode()?),*];

            self.connection.invoke(stringify!($service),
                concat!( $( stringify!($prefix), )? stringify!($rpc_name)),
                &args)?;
            Ok(())
        }
    };

    //
    // Remote Object
    //
    (
        $(#[$meta:meta])*
        object $service: tt.$object_name: ident {
            $(properties: {
                $( { $( $property: tt)+ } )*
            })?
            $(methods: {
                $( { $( $method: tt)+ } )*
            })?
            $(static_methods: {
                $( { $( $static_method: tt)+ } )*
            })?
        }
     ) => {
        $(#[$meta])*
        pub struct $object_name {
            #[allow(dead_code)]
            connection: Rc<Connection>,
            id: u64
        }

        impl RemoteObject for $object_name {
            fn new(connection: Rc<Connection>, id: u64) -> Self {
                $object_name{connection, id}
            }

            fn id(&self) -> u64 { self.id }
        }

        impl Decode for $object_name {
            fn decode(bytes: &Vec<u8>, connection: Rc<Connection>) -> Result<Self, CodecError> {
                let id: u64 = decode(bytes, connection.clone())?;
                if id == 0 {
                    Err(CodecError::NullValue)
                } else {
                    Ok($object_name::new(connection, id))
                }
            }
        }

        impl Decode for Option<$object_name> {
            fn decode(bytes: &Vec<u8>, connection: Rc<Connection>) -> Result<Self, CodecError> {
                let id: u64 = decode(bytes, connection.clone())?;
                if id == 0 {
                    Ok(None)
                } else {
                    Ok(Some($object_name::new(connection, id)))
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

        impl $object_name {
            // Properties
            $(
                $(
                    remote_type!(@object_property(service=$service, class=$object_name) $( $property )+ );
                )*
            )?

            // Methods
            $(
                $(
                    remote_type!(@object_method(service=$service, class=$object_name, separator=_) $( $method )+ );
                )*
            )?

            // Static Methods
            $(
                $(
                    remote_type!(@object_static_method(service=$service, class=$object_name) $( $static_method )+ );
                )*
            )?
        }
    };

    //
    // Properties
    //
    (
        @object_property(service=$service:tt, class=$class:tt)
            $prop_name: ident : $prop_type: ty,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident
    ) => {
        remote_type!(@object_method(service=$service, class=$class, separator=_get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> $prop_type {
                $prop_name()
            });

        remote_type!(@object_method(service=$service, class=$class, separator=_set_)
            $( #[$setter_meta] )*
            fn $setter_name(value: $prop_type) {
                $prop_name(value)
            });
    };

    (
        @object_property(service=$service:tt, class=$class:tt)
            $prop_name: ident : $set_type: ty => $get_type: ty,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident
    ) => {
        remote_type!(@object_method(service=$service, class=$class, separator=_get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> $get_type {
                $prop_name()
            });

        $(remote_type!(@object_method(service=$service, class=$class, separator=_set_)
            $( #[$setter_meta] )*
            fn $setter_name(value: $set_type) {
                $prop_name(value)
            }))?;
    };

    (
        @object_property(service=$service:tt, class=$class:tt)
            $prop_name: ident : $prop_type: ty,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident
    ) => {
        remote_type!(@object_method(service=$service, class=$class, separator=_get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> $prop_type {
                $prop_name()
            });
    };

    //
    // Methods
    //
    (
        @object_method(service=$service:tt, class=$class:tt, separator=$separator:tt)
        $(#[$meta:meta])*
        fn $method_name: ident ($( $arg_name: ident : $arg_type: ty), *) -> $return_type: ty {
            $rpc_name: tt($( $arg_expr: expr ),* )
        }
    ) => {
        $(#[$meta])*
        pub fn $method_name(&self $(, $arg_name : $arg_type)*) -> RpcResult<$return_type> {
            let args: Vec<Vec<u8>> = vec![self.encode()? $(, $arg_expr.encode()?)*];

            let response = self.connection.invoke(stringify!($service),
                concat!( stringify!($class), stringify!($separator), stringify!($rpc_name)),
                &args)?;
            Ok(decode(&response, self.connection.clone())?)
        }
    };

    (
        @object_method(service=$service:tt, class=$class:tt, separator=$separator:tt)
        $(#[$meta:meta])*
        fn $method_name: ident ($( $arg_name: ident : $arg_type: ty), *) {
            $rpc_name: tt($( $arg_expr: expr ),* )
        }
    ) => {
        $(#[$meta])*
        pub fn $method_name(&self $(, $arg_name : $arg_type)*) -> RpcResult<()> {
            let args: Vec<Vec<u8>> = vec![self.encode()? $(, $arg_expr.encode()?)*];

            self.connection.invoke(stringify!($service),
                concat!( stringify!($class), stringify!($separator), stringify!($rpc_name)),
                &args)?;
            Ok(())
        }
    };

    //
    // Static Methods
    //
    (
        @object_static_method(service=$service:tt, class=$class:tt)
        $(#[$meta:meta])*
        fn $method_name: ident ($( $arg_name: ident : $arg_type: ty), *) -> $return_type: ty {
            $rpc_name: tt($( $arg_expr: expr ),* )
        }
    ) => {
        $(#[$meta])*
        pub fn $method_name(connection: Rc<Connection> $(, $arg_name : $arg_type)*) -> RpcResult<$return_type> {
            let args: Vec<Vec<u8>> = vec![$($arg_expr.encode()?),*];

            let response = connection.invoke(stringify!($service),
                concat!( stringify!($class), "_static_", stringify!($rpc_name)),
                &args)?;
            Ok(decode(&response, connection)?)
        }
    };

    //
    // Remote Enum
    //

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
            fn decode(bytes: &Vec<u8>, connection: Rc<Connection>) -> Result<Self, CodecError> {
                let value: i64 = decode(bytes, connection)?;
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
