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

        impl<'a> std::fmt::Debug for $service<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                write!(f, "{}", stringify!($service))
            }
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
        $prop_name: ident {
            $(#[$getter_meta:meta])*
            get: $getter_name: ident -> $getter_type: ty $(,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident ($setter_type: ty) )?
        }
    ) => {
        remote_type!(@method(service=$service, prefix=get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> $getter_type {
                $prop_name()
            }
        );

        $(
            remote_type!(@method(service=$service, prefix=set_)
                $( #[$setter_meta] )*
                fn $setter_name(value: $setter_type) {
                    $prop_name(value)
                }
            );
        )?
    };

    (
        @property(service=$service:tt)
        $( $props: tt)*
    ) => {
        compile_error!(concat!("Invalid Service property definition:\n", stringify!($($props)*)));
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
        pub fn $method_name(&self $(, $arg_name : $arg_type)*) -> $crate::client::KrpcResult<$return_type> {
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
        pub fn $method_name(&self $(, $arg_name : $arg_type)*) -> $crate::client::KrpcResult<()> {
            let args: Vec<Vec<u8>> = vec![$($arg_expr.encode()?),*];

            self.connection.invoke(stringify!($service),
                concat!( $( stringify!($prefix), )? stringify!($rpc_name)),
                &args)?;
            Ok(())
        }
    };

    (
        @method(service=$service:tt $(, prefix=$prefix:tt)?)
        $( $methods: tt)*
    ) => {
        compile_error!(concat!("Invalid Service method definition:\n", stringify!($($methods)*)));
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
            #[derive(Debug, Clone)]
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
        $prop_name: ident {
            $(#[$getter_meta:meta])*
            get: $getter_name: ident -> $getter_type: ty $(,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident ($setter_type: ty) )?
        }
    ) => {
        remote_type!(@stream_method(service=$service, prefix=get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> $getter_type {
                $prop_name()
            });
    };

    (
        @stream_property(service=$service:tt)
        $( $props: tt)*
    ) => {

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
        pub fn $method_name(&self $(, $arg_name : $arg_type)*) -> $crate::client::KrpcResult<$crate::client::Stream<$return_type>> {
            let args: Vec<Vec<u8>> = vec![$($arg_expr.encode()?),*];

            Ok(self.connection.add_stream(
                stringify!($service),
                concat!( $( stringify!($prefix), )? stringify!($rpc_name)),
                &args
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

    (
        @stream_method(service=$service:tt $(, prefix=$prefix:tt)?)
        $( $methods: tt)*
    ) => {

    };

    //
    // Service Call
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
            #[derive(Debug, Clone)]
            pub struct [<$service Call>]<'a> {
                connection: &'a $crate::client::Connection,
            }

            impl<'a> $service<'a> {
                /// Returns a call instance that provides versions of the properties
                /// and methods as `ProcedureCall`s.
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
    // Call Properties
    //
    (
        @call_property(service=$service:tt)
        $prop_name: ident {
            $(#[$getter_meta:meta])*
            get: $getter_name: ident -> $getter_type: ty $(,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident ($setter_type: ty) )?
        }
    ) => {
        remote_type!(
            @call_method(service=$service, prefix=get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> $getter_type {
                $prop_name()
            }
        );

        $(
            remote_type!(
                @call_method(service=$service, prefix=set_)
                $( #[$setter_meta] )*
                fn $setter_name(value: $setter_type) {
                    $prop_name(value)
                }
            );
        )?
    };

    (
        @call_property(service=$service:tt)
        $( $props: tt)*
    ) => {

    };

    //
    // Call Methods
    //
    (
        @call_method(service=$service:tt $(, prefix=$prefix:tt)?)
        $(#[$meta:meta])*
        fn $method_name: ident ($( $arg_name: ident : $arg_type: ty), *) $( -> $return_type: ty )? {
            $rpc_name: tt($( $arg_expr: expr ),* )
        }
    ) => {
        $(#[$meta])*
        pub fn $method_name(&self $(, $arg_name : $arg_type)*) -> $crate::client::KrpcResult<$crate::client::ProcedureCall> {
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
        $( $methods: tt)*
    ) => {
        
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
            fn decode(bytes: &Vec<u8>, connection: &'a $crate::client::Connection) -> $crate::codec::CodecResult<Self> {
                let id = u64::decode(bytes, connection)?;
                if id == 0 {
                    Err(failure::Error::from($crate::codec::CodecError::NullValue))
                } else {
                    Ok($object_name::new(connection, id))
                }
            }
        }

        impl<'a> $crate::codec::Decode<'a> for Option<$object_name<'a>> {
            fn decode(bytes: &Vec<u8>, connection: &'a $crate::client::Connection) -> $crate::codec::CodecResult<Self> {
                let id = u64::decode(bytes, connection)?;
                if id == 0 {
                    Ok(None)
                } else {
                    Ok(Some($object_name::new(connection, id)))
                }
            }
        }

        impl<'a> $crate::codec::Encode for $object_name<'a> {
            fn encode(&self) -> $crate::codec::CodecResult<Vec<u8>> {
                self.id().encode()
            }
        }

        impl<'a> $crate::codec::Encode for Option<$object_name<'a>> {
            fn encode(&self) -> $crate::codec::CodecResult<Vec<u8>> {
                match self {
                    None => (0 as u64).encode(),
                    Some(obj) => obj.id().encode()
                }
            }
        }

        impl<'a> $crate::codec::Encode for Option<&$object_name<'a>> {
            fn encode(&self) -> $crate::codec::CodecResult<Vec<u8>> {
                match self {
                    None => (0 as u64).encode(),
                    Some(ref obj) => obj.id().encode()
                }
            }
        }

        impl<'a> std::fmt::Debug for $object_name<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                write!(f, "{}.{}{{ id: {} }}", stringify!($service), stringify!($object_name), self.id)
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
        $prop_name: ident {
            $(#[$getter_meta:meta])*
            get: $getter_name: ident -> $getter_type: ty $(,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident ($setter_type: ty) )?
        }
    ) => {
        remote_type!(@method(service=$service, class=$class, separator=_get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> $getter_type {
                $prop_name()
            }
        );

        $(
            remote_type!(@method(service=$service, class=$class, separator=_set_)
                $( #[$setter_meta] )*
                fn $setter_name(value: $setter_type) {
                    $prop_name(value)
                }
            );
        )?
    };

    (
        @property(service=$service:tt, class=$class:tt)
        $( $props: tt)*
    ) => {
        compile_error!(concat!("Invalid Remote Object property definition:\n", stringify!($($props)*)));
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
        pub fn $method_name(&self $(, $arg_name : $arg_type)*) -> $crate::client::KrpcResult<$return_type> {
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
        pub fn $method_name(&self $(, $arg_name : $arg_type)*) -> $crate::client::KrpcResult<()> {
            let args: Vec<Vec<u8>> = vec![self.encode()? $(, $arg_expr.encode()?)*];

            self.connection.invoke(stringify!($service),
                concat!( stringify!($class), stringify!($separator), stringify!($rpc_name)),
                &args)?;
            Ok(())
        }
    };

    (
        @method(service=$service:tt, class=$class:tt, separator=$separator:tt)
        $( $methods: tt)*
    ) => {
        compile_error!(concat!("Invalid Remote Object method definition:\n", stringify!($($methods)*)));
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
        pub fn $method_name(connection: &'a $crate::client::Connection $(, $arg_name : $arg_type)*) -> $crate::client::KrpcResult<$return_type> {
            let args: Vec<Vec<u8>> = vec![$($arg_expr.encode()?),*];

            let response = connection.invoke(stringify!($service),
                concat!( stringify!($class), "_static_", stringify!($rpc_name)),
                &args)?;
            Ok(<$return_type>::decode(&response, connection)?)
        }
    };

    (
        @static_method(service=$service:tt, class=$class:tt)
        $( $methods: tt)*
    ) => {
        compile_error!(concat!("Invalid Remote Object static method definition:\n", stringify!($($methods)*)));
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
            #[derive(Debug, Clone)]
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
        $prop_name: ident {
            $(#[$getter_meta:meta])*
            get: $getter_name: ident -> $getter_type: ty $(,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident ($setter_type: ty) )?
        }
    ) => {
        remote_type!(@stream_method(service=$service, class=$class, separator=_get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> $getter_type {
                $prop_name()
            });
    };

    (
        @stream_property(service=$service:tt, class=$class:tt)
        $( $props: tt)*
    ) => {
        // silently ignore since it should be caught by the normal property definition
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
        pub fn $method_name(&self $(, $arg_name : $arg_type)*) -> $crate::client::KrpcResult<$crate::client::Stream<$return_type>> {
            let args: Vec<Vec<u8>> = vec![self.id.encode()? $(, $arg_expr.encode()?)*];

            Ok(self.connection.add_stream(
                stringify!($service),
                concat!( stringify!($class), stringify!($separator), stringify!($rpc_name)),
                &args
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

    (
        @stream_method(service=$service:tt, class=$class:tt, separator=$separator:tt)
        $( $methods: tt)*
    ) => {

    };

    //
    // Remote Object Call
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
            #[derive(Debug, Clone)]
            pub struct [<$object_name Call>]<'a> {
                connection: &'a $crate::client::Connection,
                id: u64
            }

            impl<'a> $object_name<'a> {
                /// Returns a call instance that provides versions of the properties
                /// and methods as `ProcedureCall`s.
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
    // Call Properties
    //
    (
        @call_property(service=$service:tt, class=$class:tt)
        $prop_name: ident {
            $(#[$getter_meta:meta])*
            get: $getter_name: ident -> $getter_type: ty $(,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident ($setter_type: ty) )?
        }
    ) => {
        remote_type!(
            @call_method(service=$service, class=$class, separator=_get_)
            $( #[$getter_meta] )*
            fn $getter_name() -> $getter_type {
                $prop_name()
            }
        );

        $(
            remote_type!(
                @call_method(service=$service, class=$class, separator=_set_)
                $( #[$setter_meta] )*
                fn $setter_name(value: $setter_type) {
                    $prop_name(value)
                }
            );
        )?
    };

    (
        @call_property(service=$service:tt, class=$class:tt)
        $( $props: tt)*
    ) => {
        // silently ignore since it should be caught by the normal property definition
    };

    //
    // Call Methods
    //
    (
        @call_method(service=$service:tt, class=$class:tt, separator=$separator:tt)
        $(#[$meta:meta])*
        fn $method_name: ident ($( $arg_name: ident : $arg_type: ty), *) $( -> $return_type: ty )? {
            $rpc_name: tt($( $arg_expr: expr ),* )
        }
    ) => {
        $(#[$meta])*
        pub fn $method_name(&self $(, $arg_name : $arg_type)*) -> $crate::client::KrpcResult<$crate::client::ProcedureCall> {
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
        $( $methods: tt)*
    ) => {

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
            fn decode(bytes: &Vec<u8>, connection: &'a $crate::client::Connection) -> $crate::codec::CodecResult<Self> {
                let value = i64::decode(bytes, connection)?;
                Ok(Self::from_value(value)
                    .ok_or($crate::codec::CodecError::InvalidEnumValue(value))?)
            }
        }

        impl $crate::codec::Encode for $enum_name {
            fn encode(&self) -> $crate::codec::CodecResult<Vec<u8>> {
                self.value().encode()
            }
        }
    }
}
