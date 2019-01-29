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
        #[derive(Clone)]
        pub struct $service<'a> {
            connection: &'a $crate::client::Connection,
        }

        impl<'a> $service<'a> {
            /// Creates a new service using the given `connection`.
            pub fn new(connection: &'a $crate::client::Connection) -> Self {
                Self { connection }
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

        remote_type!(
            @stream_service(service=$service)
            properties: {
                $( { $( $property)+ } )*
            }
            methods: {
                $( { $( $method)+ } )*
            }
        );

        remote_type!(
            @call_service(service=$service)
            properties: {
                $( { $( $property)+ } )*
            }
            methods: {
                $( { $( $method)+ } )*
            }
        );
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
        pub fn $method_name(&self $(, $arg_name : $arg_type)*) -> $crate::client::Result<$return_type> {
            let args: Vec<Vec<u8>> = vec![$($arg_expr.encode()?),*];

            let response = self.connection.invoke(stringify!($service),
                concat!( $( stringify!($prefix), )? stringify!($rpc_name)),
                &args)?;
            Ok(<$return_type>::decode(&response, self.connection)?)
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
        pub fn $method_name(&self $(, $arg_name : $arg_type)*) -> $crate::client::Result<()> {
            let args: Vec<Vec<u8>> = vec![$($arg_expr.encode()?),*];

            self.connection.invoke(stringify!($service),
                concat!( $( stringify!($prefix), )? stringify!($rpc_name)),
                &args)?;
            Ok(())
        }
    };

    //
    // Service Stream
    //
    (
        @stream_service(service=$service: ident)
        properties: {}
        methods: {}
    ) => {

    };

    (
        @stream_service(service=$service: ident)
        properties: {
            $( { $( $property: tt)+ } )*
        }
        methods: {
            $( { $( $method: tt)+ } )*
        }
    ) => {
        paste::item! {
            #[derive(Clone)]
            pub struct [<$service Stream>]<'a> {
                connection: &'a $crate::client::Connection,
            }

            impl<'a> $service<'a> {
                /// Returns a stream instance that provides streaming versions of the
                /// property getters and methods with return values.
                pub fn stream(&self) -> [<$service Stream>]<'a> {
                    [<$service Stream>]::new(self.connection)
                }
            }

            impl<'a> [<$service Stream>]<'a> {
                pub fn new(connection: &'a $crate::client::Connection) -> Self {
                    Self { connection }
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
        pub fn $method_name(&self $(, $arg_name : $arg_type)*) -> $crate::client::Result<$crate::client::Stream<$return_type>> {
            let args: Vec<Vec<u8>> = vec![$($arg_expr.encode()?),*];

            Ok(self.connection.add_stream(
                stringify!($service),
                concat!( $( stringify!($prefix), )? stringify!($rpc_name)),
                &args,
                false
            )?)
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
    // Service Expression
    //
    (
        @call_service(service=$service: ident)
        properties: {}
        methods: {}
    ) => {

    };

    (
        @call_service(service=$service: ident)
        properties: {
            $( { $( $property: tt)+ } )*
        }
        methods: {
            $( { $( $method: tt)+ } )*
        }
    ) => {
        paste::item! {
            #[derive(Clone)]
            pub struct [<$service Call>]<'a> {
                connection: &'a $crate::client::Connection,
            }

            impl<'a> $service<'a> {
                /// Returns a call instance that provides versions of the property getters
                /// and methods with return values as `ProcedureCall`s.
                pub fn call(&self) -> [<$service Call>] {
                    [<$service Call>]::new(self.connection)
                }
            }

            impl<'a> [<$service Call>]<'a> {
                pub fn new(connection: &'a $crate::client::Connection) -> Self {
                    Self { connection }
                }

                // Properties
                $(
                    remote_type!(@call_property(service=$service) $( $property )+ );
                )*

                // Methods
                $(
                    remote_type!(@call_method(service=$service) $( $method )+ );
                )*
            }
        }
    };

    //
    // Expression Properties
    //
    (
        @call_property(service=$service:tt)
            $prop_name: ident : $prop_type: ty,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident $(,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident)?
    ) => {
        remote_type!(@call_method(service=$service, prefix=get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> $prop_type {
                $prop_name()
            });
    };

    //
    // Expression Methods
    //
    (
        @call_method(service=$service:tt $(, prefix=$prefix:tt)?)
        $(#[$meta:meta])*
        fn $method_name: ident ($( $arg_name: ident : $arg_type: ty), *) -> $return_type: ty {
            $rpc_name: tt($( $arg_expr: expr ),* )
        }
    ) => {
        $(#[$meta])*
        pub fn $method_name(&self $(, $arg_name : $arg_type)*) -> $crate::client::Result<$crate::client::ProcedureCall> {
            let args: Vec<Vec<u8>> = vec![$($arg_expr.encode()?),*];

            Ok(self.connection.procedure_call(
                stringify!($service),
                concat!( $( stringify!($prefix), )? stringify!($rpc_name)),
                &args
            ))
        }
    };

    (
        @call_method(service=$service:tt $(, prefix=$prefix:tt)?)
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
        #[derive(Clone)]
        pub struct $object_name<'a> {
            #[allow(dead_code)]
            connection: &'a $crate::client::Connection,
            id: u64
        }

        impl<'a> $crate::RemoteObject<'a> for $object_name<'a> {
            fn new(connection: &'a $crate::client::Connection, id: u64) -> Self {
                Self { connection, id }
            }

            fn id(&self) -> u64 { self.id }
        }

        impl<'a> $crate::codec::Decode<'a> for $object_name<'a> {
            fn decode(bytes: &Vec<u8>, connection: &'a $crate::client::Connection) -> Result<Self, $crate::codec::CodecError> {
                let id = u64::decode(bytes, connection)?;
                if id == 0 {
                    Err($crate::codec::CodecError::NullValue)
                } else {
                    Ok($object_name::new(connection, id))
                }
            }
        }

        impl<'a> $crate::codec::Decode<'a> for Option<$object_name<'a>> {
            fn decode(bytes: &Vec<u8>, connection: &'a $crate::client::Connection) -> Result<Self, $crate::codec::CodecError> {
                let id = u64::decode(bytes, connection)?;
                if id == 0 {
                    Ok(None)
                } else {
                    Ok(Some($object_name::new(connection, id)))
                }
            }
        }

        impl<'a> $crate::codec::Encode for $object_name<'a> {
            fn encode(&self) -> Result<Vec<u8>, $crate::codec::CodecError> {
                self.id().encode()
            }
        }

        impl<'a> $crate::codec::Encode for Option<$object_name<'a>> {
            fn encode(&self) -> Result<Vec<u8>, $crate::codec::CodecError> {
                match self {
                    None => (0 as u64).encode(),
                    Some(obj) => obj.id().encode()
                }
            }
        }

        impl<'a> $crate::codec::Encode for Option<&$object_name<'a>> {
            fn encode(&self) -> Result<Vec<u8>, $crate::codec::CodecError> {
                match self {
                    None => (0 as u64).encode(),
                    Some(ref obj) => obj.id().encode()
                }
            }
        }

        impl<'a> $object_name<'a> {
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

        remote_type!(
            @stream_remote_object(service=$service, class=$object_name)
            properties: {
                $( $( { $( $property)+ } )* )?
            }
            methods: {
                $( $( { $( $method)+ } )* )?
            }
        );

        remote_type!(
            @call_remote_object(service=$service, class=$object_name)
            properties: {
                $( $( { $( $property)+ } )* )?
            }
            methods: {
                $( $( { $( $method)+ } )* )?
            }
        );
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
        pub fn $method_name(&self $(, $arg_name : $arg_type)*) -> $crate::client::Result<$return_type> {
            let args: Vec<Vec<u8>> = vec![self.encode()? $(, $arg_expr.encode()?)*];

            let response = self.connection.invoke(stringify!($service),
                concat!( stringify!($class), stringify!($separator), stringify!($rpc_name)),
                &args)?;
            Ok(<$return_type>::decode(&response, self.connection)?)
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
        pub fn $method_name(&self $(, $arg_name : $arg_type)*) -> $crate::client::Result<()> {
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
        @static_method(service=$service:tt, class=$class:tt)
        $(#[$meta:meta])*
        fn $method_name: ident ($( $arg_name: ident : $arg_type: ty), *) -> $return_type: ty {
            $rpc_name: tt($( $arg_expr: expr ),* )
        }
    ) => {
        $(#[$meta])*
        pub fn $method_name(connection: &'a $crate::client::Connection $(, $arg_name : $arg_type)*) -> $crate::client::Result<$return_type> {
            let args: Vec<Vec<u8>> = vec![$($arg_expr.encode()?),*];

            let response = connection.invoke(stringify!($service),
                concat!( stringify!($class), "_static_", stringify!($rpc_name)),
                &args)?;
            Ok(<$return_type>::decode(&response, connection)?)
        }
    };

    //
    // Remote Object Stream
    //
    (
        @stream_remote_object(service=$service: ident, class=$object_name: ident)
        properties: {}
        methods: {}
    ) => {

    };

    (
        @stream_remote_object(service=$service: ident, class=$object_name: ident)
        properties: {
            $( { $( $property: tt)+ } )*
        }
        methods: {
            $( { $( $method: tt)+ } )*
        }
    ) => {
        paste::item! {
            #[derive(Clone)]
            pub struct [<$object_name Stream>]<'a> {
                connection: &'a $crate::client::Connection,
                id: u64
            }

            impl<'a> $object_name<'a> {
                /// Returns a stream instance that provides streaming versions of the
                /// property getters and methods with return values.
                pub fn stream(&self) -> [<$object_name Stream>]<'a> {
                    [<$object_name Stream>]::new(self)
                }
            }

            impl<'a> [<$object_name Stream>]<'a> {
                pub fn new(remote_object: &$object_name<'a>) -> Self {
                    Self {
                        connection: remote_object.connection,
                        id: remote_object.id
                    }
                }

                // Properties
                $(
                    remote_type!(@stream_property(service=$service, class=$object_name) $( $property )+ );
                )*

                // Methods
                $(
                    remote_type!(@stream_method(service=$service, class=$object_name, separator=_) $( $method )+ );
                )*
            }
        }
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
        pub fn $method_name(&self $(, $arg_name : $arg_type)*) -> $crate::client::Result<$crate::client::Stream<$return_type>> {
            let args: Vec<Vec<u8>> = vec![self.id.encode()? $(, $arg_expr.encode()?)*];

            Ok(self.connection.add_stream(
                stringify!($service),
                concat!( stringify!($class), stringify!($separator), stringify!($rpc_name)),
                &args,
                false
            )?)
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
    // Remote Object Expression
    //
    (
        @call_remote_object(service=$service: ident, class=$object_name: ident)
        properties: {}
        methods: {}
    ) => {

    };

    (
        @call_remote_object(service=$service: ident, class=$object_name: ident)
        properties: {
            $( { $( $property: tt)+ } )*
        }
        methods: {
            $( { $( $method: tt)+ } )*
        }
    ) => {
        paste::item! {
            #[derive(Clone)]
            pub struct [<$object_name Call>]<'a> {
                connection: &'a $crate::client::Connection,
                id: u64
            }

            impl<'a> $object_name<'a> {
                /// Returns a call instance that provides versions of the property getters
                /// and methods with return values as `ProcedureCall`s.
                pub fn call(&self) -> [<$object_name Call>] {
                    [<$object_name Call>]::new(self)
                }
            }

            impl<'a> [<$object_name Call>]<'a> {
                pub fn new(remote_object: &$object_name<'a>) -> Self {
                    Self {
                        connection: remote_object.connection,
                        id: remote_object.id
                    }
                }

                // Propertie
                $(
                    remote_type!(@call_property(service=$service, class=$object_name) $( $property )+ );
                )*

                // Methods
                $(
                    remote_type!(@call_method(service=$service, class=$object_name, separator=_) $( $method )+ );
                )*
            }
        }
    };

    //
    // Expression Properties
    //
    (
        @call_property(service=$service:tt, class=$class:tt)
            $prop_name: ident : $prop_type: ty,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident $(,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident)?
    ) => {
        remote_type!(@call_method(service=$service, class=$class, separator=_get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> $prop_type {
                $prop_name()
            });
    };

    //
    // Expression Methods
    //
    (
        @call_method(service=$service:tt, class=$class:tt, separator=$separator:tt)
        $(#[$meta:meta])*
        fn $method_name: ident ($( $arg_name: ident : $arg_type: ty), *) -> $return_type: ty {
            $rpc_name: tt($( $arg_expr: expr ),* )
        }
    ) => {
        $(#[$meta])*
        pub fn $method_name(&self $(, $arg_name : $arg_type)*) -> $crate::client::Result<$crate::client::ProcedureCall> {
            let args: Vec<Vec<u8>> = vec![self.id.encode()? $(, $arg_expr.encode()?)*];

            Ok(self.connection.procedure_call(
                stringify!($service),
                concat!( stringify!($class), stringify!($separator), stringify!($rpc_name)),
                &args,
            ))
        }
    };

    (
        @call_method(service=$service:tt, class=$class:tt, separator=$separator:tt)
        $(#[$meta:meta])*
        fn $method_name: ident ($( $arg_name: ident : $arg_type: ty), *) {
            $rpc_name: tt($( $arg_expr: expr ),* )
        }
    ) => {
        // This space intentionally left blank
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

        impl $crate::RemoteEnum for $enum_name {
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

        impl<'a> $crate::codec::Decode<'a> for $enum_name {
            fn decode(bytes: &Vec<u8>, connection: &'a $crate::client::Connection) -> Result<Self, $crate::codec::CodecError> {
                let value = i64::decode(bytes, connection)?;
                $enum_name::from_value(value)
                    .ok_or($crate::codec::CodecError::InvalidEnumValue(value))
            }
        }

        impl $crate::codec::Encode for $enum_name {
            fn encode(&self) -> Result<Vec<u8>, $crate::codec::CodecError> {
                self.value().encode()
            }
        }
    }
}
