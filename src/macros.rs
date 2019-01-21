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
            }
            methods: {
                $( { $( $method: tt)+ } )*
            }
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone)]
        pub struct $service {
            connection: Connection,
        }

        impl $service {
            /// Creates a new service using the given `connection`.
            pub fn new(connection: &Connection) -> Self {
                Self {
                    connection: connection.clone(),
                }
            }

            paste::item! {
                /// Get a stream instance that allows access to stream version of the methods
                /// and properties for this service.
                pub fn stream(&self) -> [<$service Stream>] {
                    [<$service Stream>]::new(&self.connection)
                }
            }

            // Properties
            $(
                remote_type!(@property(service=$service) $( $property )+ );
            )*

            // Methods
            $(
                remote_type!(@method(service=$service) $( $method )+ );
            )*
        }

        paste::item! {
            #[derive(Debug, Clone)]
            pub struct [<$service Stream>] {
                connection: Connection,
            }

            impl [<$service Stream>] {
                pub fn new(connection: &Connection) -> Self {
                    Self {
                        connection: connection.clone(),
                    }
                }

                // Properties
                $(
                    remote_type!(@stream_property(service=$service) $( $property )+ );
                )*

                // Methods
                $(
                    remote_type!(@stream_method(service=$service) $( $method )+ );
                )*
            }
        }
    };

    //
    // Properties
    //
    (
        @property(service=$service:tt)
            $prop_name: ident : Option<$prop_type: ty>,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident$(,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident)?
    ) => {
        remote_type!(@method(service=$service, prefix=get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> Option<$prop_type> {
                $prop_name()
            }
        );

        $(
            remote_type!(@method(service=$service, prefix=set_)
                $( #[$setter_meta] )*
                fn $setter_name(value: Option<&$prop_type>) {
                    $prop_name(value)
                }
            );
        )?
    };

    (
        @property(service=$service:tt)
            $prop_name: ident : Vec<$prop_type: ty>,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident$(,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident)?
    ) => {
        remote_type!(@method(service=$service, prefix=get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> Vec<$prop_type> {
                $prop_name()
            }
        );

        $(
            remote_type!(@method(service=$service, prefix=set_)
                $( #[$setter_meta] )*
                fn $setter_name(value: &[$prop_type])> {
                    $prop_name(value)
                }
            );
        )?
    };

    (
        @property(service=$service:tt)
            $prop_name: ident : String,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident$(,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident)?
    ) => {
        remote_type!(@method(service=$service, prefix=get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> String {
                $prop_name()
            }
        );

        $(
            remote_type!(@method(service=$service, prefix=set_)
                $( #[$setter_meta] )*
                fn $setter_name(value: &str) {
                    $prop_name(value)
                }
            );
        )?
    };

    (
        @property(service=$service:tt)
            $prop_name: ident : f32,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident$(,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident)?
    ) => {
        remote_type!(@method(service=$service, prefix=get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> f32 {
                $prop_name()
            }
        );

        $(
            remote_type!(@method(service=$service, prefix=_set)
                $( #[$setter_meta] )*
                fn $setter_name(value: f32) {
                    $prop_name(value)
                }
            );
        )?
    };

    (
        @property(service=$service:tt)
            $prop_name: ident : f64,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident$(,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident)?
    ) => {
        remote_type!(@method(service=$service, prefix=get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> f64 {
                $prop_name()
            }
        );

        $(
            remote_type!(@method(service=$service, prefix=set_)
                $( #[$setter_meta] )*
                fn $setter_name(value: f64) {
                    $prop_name(value)
                }
            );
        )?
    };

    (
        @property(service=$service:tt)
            $prop_name: ident : i32,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident$(,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident)?
    ) => {
        remote_type!(@method(service=$service, prefix=get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> i32 {
                $prop_name()
            }
        );

        $(
            remote_type!(@method(service=$service, prefix=set_)
                $( #[$setter_meta] )*
                fn $setter_name(value: i32) {
                    $prop_name(value)
                }
            );
        )?
    };

    (
        @property(service=$service:tt)
            $prop_name: ident : i64,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident$(,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident)?
    ) => {
        remote_type!(@method(service=$service, prefix=get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> i64 {
                $prop_name()
            }
        );

        $(
            remote_type!(@method(service=$service, prefix=set_)
                $( #[$setter_meta] )*
                fn $setter_name(value: i64) {
                    $prop_name(value)
                }
            );
        )?
    };

    (
        @property(service=$service:tt)
            $prop_name: ident : u32,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident$(,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident)?
    ) => {
        remote_type!(@method(service=$service, prefix=get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> u32 {
                $prop_name()
            }
        );

        $(
            remote_type!(@method(service=$service, prefix=set_)
                $( #[$setter_meta] )*
                fn $setter_name(value: u32) {
                    $prop_name(value)
                }
            );
        )?
    };

    (
        @property(service=$service:tt)
            $prop_name: ident : u64,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident$(,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident)?
    ) => {
        remote_type!(@method(service=$service, prefix=get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> u64 {
                $prop_name()
            }
        );

        $(
            remote_type!(@method(service=$service, prefix=set_)
                $( #[$setter_meta] )*
                fn $setter_name(value: u64) {
                    $prop_name(value)
                }
            );
        )?
    };

    (
        @property(service=$service:tt)
            $prop_name: ident : bool,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident$(,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident)?
    ) => {
        remote_type!(@method(service=$service, prefix=get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> bool {
                $prop_name()
            }
        );

        $(
            remote_type!(@method(service=$service, prefix=set_)
                $( #[$setter_meta] )*
                fn $setter_name(value: bool) {
                    $prop_name(value)
                }
            );
        )?
    };

    (
        @property(service=$service:tt)
            $prop_name: ident : $prop_type: ty,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident$(,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident)?
    ) => {
        remote_type!(@method(service=$service, prefix=get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> $prop_type {
                $prop_name()
            }
        );

        $(
            remote_type!(@method(service=$service, prefix=set_)
                $( #[$setter_meta] )*
                fn $setter_name(value: &$prop_type) {
                    $prop_name(value)
                }
            );
        )?
    };

    //
    // Stream Properties
    //
    (
        @stream_property(service=$service:tt)
            $prop_name: ident : $prop_type: ty,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident $(,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident)?
    ) => {
        remote_type!(@stream_method(service=$service, prefix=get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> $prop_type {
                $prop_name()
            });
    };

    //
    // Methods
    //
    (
        @method(service=$service:tt $(, prefix=$prefix:tt)?)
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
            Ok(decode(&response, &self.connection)?)
        }
    };

    (
        @method(service=$service:tt $(, prefix=$prefix:tt)?)
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
    // Stream Methods
    //
    (
        @stream_method(service=$service:tt $(, prefix=$prefix:tt)?)
        $(#[$meta:meta])*
        fn $method_name: ident ($( $arg_name: ident : $arg_type: ty), *) -> $return_type: ty {
            $rpc_name: tt($( $arg_expr: expr ),* )
        }
    ) => {
        $(#[$meta])*
        pub fn $method_name(&self $(, $arg_name : $arg_type)*) -> StreamResult<StreamValue<$return_type>> {
            let args: Vec<Vec<u8>> = vec![$($arg_expr.encode()?),*];

            let stream_raw_value = self.connection.add_stream(
                stringify!($service),
                concat!( $( stringify!($prefix), )? stringify!($rpc_name)),
                &args,
                false
            )?;

            Ok(StreamValue::new(&self.connection, stream_raw_value))
        }
    };

    (
        @stream_method(service=$service:tt $(, prefix=$prefix:tt)?)
        $(#[$meta:meta])*
        fn $method_name: ident ($( $arg_name: ident : $arg_type: ty), *) {
            $rpc_name: tt($( $arg_expr: expr ),* )
        }
    ) => {
        // This space intentionally left blank
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
        #[derive(Debug, Clone)]
        pub struct $object_name {
            #[allow(dead_code)]
            connection: Connection,
            id: u64
        }

        paste::item! {
            #[derive(Debug, Clone)]
            pub struct [<$object_name Stream>] {
                connection: Connection,
                id: u64
            }

            impl [<$object_name Stream>] {
                pub fn new(remote_object: &$object_name) -> Self {
                    Self {
                        connection: remote_object.connection.clone(),
                        id: remote_object.id
                    }
                }

                // Properties
                $(
                    $(
                        remote_type!(@stream_property(service=$service, class=$object_name) $( $property )+ );
                    )*
                )?

                // Methods
                $(
                    $(
                        remote_type!(@stream_method(service=$service, class=$object_name, separator=_) $( $method )+ );
                    )*
                )?
            }
        }

        impl RemoteObject for $object_name {
            fn new(connection: &Connection, id: u64) -> Self {
                Self {
                    connection: connection.clone(),
                    id
                }
            }

            fn id(&self) -> u64 { self.id }
        }

        impl Decode for $object_name {
            fn decode(bytes: &Vec<u8>, connection: &Connection) -> Result<Self, CodecError> {
                let id: u64 = decode(bytes, connection)?;
                if id == 0 {
                    Err(CodecError::NullValue)
                } else {
                    Ok($object_name::new(connection, id))
                }
            }
        }

        impl Decode for Option<$object_name> {
            fn decode(bytes: &Vec<u8>, connection: &Connection) -> Result<Self, CodecError> {
                let id: u64 = decode(bytes, connection)?;
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

        impl Encode for Option<&$object_name> {
            fn encode(&self) -> Result<Vec<u8>, CodecError> {
                match self {
                    None => (0 as u64).encode(),
                    Some(ref obj) => obj.id().encode()
                }
            }
        }

        impl $object_name {
            paste::item! {
                /// Get a stream instance that allows access to stream version of the methods
                /// and properties for this remote object.
                pub fn stream(&self) -> [<$object_name Stream>] {
                    [<$object_name Stream>]::new(self)
                }
            }

            // Properties
            $(
                $(
                    remote_type!(@property(service=$service, class=$object_name) $( $property )+ );
                )*
            )?

            // Methods
            $(
                $(
                    remote_type!(@method(service=$service, class=$object_name, separator=_) $( $method )+ );
                )*
            )?

            // Static Methods
            $(
                $(
                    remote_type!(@static_method(service=$service, class=$object_name) $( $static_method )+ );
                )*
            )?
        }
    };

    //
    // Properties
    //
    (
        @property(service=$service:tt, class=$class:tt)
            $prop_name: ident : Option<$prop_type: ty>,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident$(,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident)?
    ) => {
        remote_type!(@method(service=$service, class=$class, separator=_get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> Option<$prop_type> {
                $prop_name()
            }
        );

        $(
            remote_type!(@method(service=$service, class=$class, separator=_set_)
                $( #[$setter_meta] )*
                fn $setter_name(value: Option<&$prop_type>) {
                    $prop_name(value)
                }
            );
        )?
    };

    (
        @property(service=$service:tt, class=$class:tt)
            $prop_name: ident : Vec<$prop_type: ty>,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident$(,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident)?
    ) => {
        remote_type!(@method(service=$service, class=$class, separator=_get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> Vec<$prop_type> {
                $prop_name()
            }
        );

        $(
            remote_type!(@method(service=$service, class=$class, separator=_set_)
                $( #[$setter_meta] )*
                fn $setter_name(value: &[$prop_type])> {
                    $prop_name(value)
                }
            );
        )?
    };

    (
        @property(service=$service:tt, class=$class:tt)
            $prop_name: ident : String,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident$(,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident)?
    ) => {
        remote_type!(@method(service=$service, class=$class, separator=_get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> String {
                $prop_name()
            }
        );

        $(
            remote_type!(@method(service=$service, class=$class, separator=_set_)
                $( #[$setter_meta] )*
                fn $setter_name(value: &str) {
                    $prop_name(value)
                }
            );
        )?
    };

    (
        @property(service=$service:tt, class=$class:tt)
            $prop_name: ident : f32,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident$(,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident)?
    ) => {
        remote_type!(@method(service=$service, class=$class, separator=_get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> f32 {
                $prop_name()
            }
        );

        $(
            remote_type!(@method(service=$service, class=$class, separator=_set_)
                $( #[$setter_meta] )*
                fn $setter_name(value: f32) {
                    $prop_name(value)
                }
            );
        )?
    };

    (
        @property(service=$service:tt, class=$class:tt)
            $prop_name: ident : f64,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident$(,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident)?
    ) => {
        remote_type!(@method(service=$service, class=$class, separator=_get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> f64 {
                $prop_name()
            }
        );

        $(
            remote_type!(@method(service=$service, class=$class, separator=_set_)
                $( #[$setter_meta] )*
                fn $setter_name(value: f64) {
                    $prop_name(value)
                }
            );
        )?
    };

    (
        @property(service=$service:tt, class=$class:tt)
            $prop_name: ident : i32,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident$(,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident)?
    ) => {
        remote_type!(@method(service=$service, class=$class, separator=_get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> i32 {
                $prop_name()
            }
        );

        $(
            remote_type!(@method(service=$service, class=$class, separator=_set_)
                $( #[$setter_meta] )*
                fn $setter_name(value: i32) {
                    $prop_name(value)
                }
            );
        )?
    };

    (
        @property(service=$service:tt, class=$class:tt)
            $prop_name: ident : i64,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident$(,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident)?
    ) => {
        remote_type!(@method(service=$service, class=$class, separator=_get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> i64 {
                $prop_name()
            }
        );

        $(
            remote_type!(@method(service=$service, class=$class, separator=_set_)
                $( #[$setter_meta] )*
                fn $setter_name(value: i64) {
                    $prop_name(value)
                }
            );
        )?
    };

    (
        @property(service=$service:tt, class=$class:tt)
            $prop_name: ident : u32,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident$(,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident)?
    ) => {
        remote_type!(@method(service=$service, class=$class, separator=_get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> u32 {
                $prop_name()
            }
        );

        $(
            remote_type!(@method(service=$service, class=$class, separator=_set_)
                $( #[$setter_meta] )*
                fn $setter_name(value: u32) {
                    $prop_name(value)
                }
            );
        )?
    };

    (
        @property(service=$service:tt, class=$class:tt)
            $prop_name: ident : u64,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident$(,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident)?
    ) => {
        remote_type!(@method(service=$service, class=$class, separator=_get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> u64 {
                $prop_name()
            }
        );

        $(
            remote_type!(@method(service=$service, class=$class, separator=_set_)
                $( #[$setter_meta] )*
                fn $setter_name(value: u64) {
                    $prop_name(value)
                }
            );
        )?
    };

    (
        @property(service=$service:tt, class=$class:tt)
            $prop_name: ident : bool,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident$(,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident)?
    ) => {
        remote_type!(@method(service=$service, class=$class, separator=_get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> bool {
                $prop_name()
            }
        );

        $(
            remote_type!(@method(service=$service, class=$class, separator=_set_)
                $( #[$setter_meta] )*
                fn $setter_name(value: bool) {
                    $prop_name(value)
                }
            );
        )?
    };

    (
        @property(service=$service:tt, class=$class:tt)
            $prop_name: ident : $prop_type: ty,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident$(,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident)?
    ) => {
        remote_type!(@method(service=$service, class=$class, separator=_get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> $prop_type {
                $prop_name()
            }
        );

        $(
            remote_type!(@method(service=$service, class=$class, separator=_set_)
                $( #[$setter_meta] )*
                fn $setter_name(value: &$prop_type) {
                    $prop_name(value)
                }
            );
        )?
    };

    //
    // Stream Properties
    //
    (
        @stream_property(service=$service:tt, class=$class:tt)
            $prop_name: ident : $prop_type: ty,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident $(,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident)?
    ) => {
        remote_type!(@stream_method(service=$service, class=$class, separator=_get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> $prop_type {
                $prop_name()
            });
    };

    //
    // Methods
    //
    (
        @method(service=$service:tt, class=$class:tt, separator=$separator:tt)
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
            Ok(decode(&response, &self.connection)?)
        }
    };

    (
        @method(service=$service:tt, class=$class:tt, separator=$separator:tt)
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
    // Stream Methods
    //
    (
        @stream_method(service=$service:tt, class=$class:tt, separator=$separator:tt)
        $(#[$meta:meta])*
        fn $method_name: ident ($( $arg_name: ident : $arg_type: ty), *) -> $return_type: ty {
            $rpc_name: tt($( $arg_expr: expr ),* )
        }
    ) => {
        $(#[$meta])*
        pub fn $method_name(&self $(, $arg_name : $arg_type)*) -> StreamResult<StreamValue<$return_type>> {
            let args: Vec<Vec<u8>> = vec![self.id.encode()? $(, $arg_expr.encode()?)*];

            let stream_raw_value = self.connection.add_stream(
                stringify!($service),
                concat!( stringify!($class), stringify!($separator), stringify!($rpc_name)),
                &args,
                false
            )?;

            Ok(StreamValue::new(&self.connection, stream_raw_value))
        }
    };

    (
        @stream_method(service=$service:tt, class=$class:tt, separator=$separator:tt)
        $(#[$meta:meta])*
        fn $method_name: ident ($( $arg_name: ident : $arg_type: ty), *) {
            $rpc_name: tt($( $arg_expr: expr ),* )
        }
    ) => {
        // This space intentionally left blank
    };

    //
    // Static Methods
    //
    (
        @static_method(service=$service:tt, class=$class:tt)
        $(#[$meta:meta])*
        fn $method_name: ident ($( $arg_name: ident : $arg_type: ty), *) -> $return_type: ty {
            $rpc_name: tt($( $arg_expr: expr ),* )
        }
    ) => {
        $(#[$meta])*
        pub fn $method_name(connection: &Connection $(, $arg_name : $arg_type)*) -> RpcResult<$return_type> {
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
        $( $(#[$variant_meta:meta])* $value_name: ident = $value_int : expr),+ $(,)?
    }) => {
        $(#[$enum_meta])*
        #[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
        pub enum $enum_name {
            $(
                $(#[$variant_meta])*
                $value_name = $value_int
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
                *self as i64
            }
        }

        impl Decode for $enum_name {
            fn decode(bytes: &Vec<u8>, connection: &Connection) -> Result<Self, CodecError> {
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