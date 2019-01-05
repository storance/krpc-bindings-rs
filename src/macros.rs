#[macro_export]
macro_rules! rpc_method {
    // RPC Class Method
    // Passes self to rpc method
    // Maps the decoded value
    // Returns Err on no response
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

    // RPC Class Method
    // Maps the decoded value
    // Returns Err on no response
    (
        $(#[$meta:meta])*
        fn $func_name: ident (&self $(, $arg_name : ident : $arg_type : ty)* ) -> $ret : ty {
            $service : tt.$( $method : tt),+ ( $( $method_arg : expr ),* )
                .map(|$value: ident : $value_type : ty| $success_expr: expr)
                .ok_or($fail_expr: expr)
        }
    ) => {
        rpc_method!(
            $(#[$meta])*
            fn $func_name(&self $(, $arg_name : $arg_type)* ) -> $ret {
                if let Some($value) = $service.$( $method ),+( $( $method_arg ),* ) as $value_type {
                    $success_expr
                } else {
                    return Err($fail_expr)
                }
            }
        );
    };

    // RPC Class Method
    // Passes self to rpc method
    // Maps the decoded value
    // Returns None on no response
    (
        $(#[$meta:meta])*
        fn $func_name: ident (&self $(, $arg_name : ident : $arg_type : ty)* ) -> Option<$ret : ty> {
            $service : tt.$( $method : tt),+ ( self $(, $method_arg : expr )* )
                .map(|$value: ident : $value_type : ty| $success_expr: expr)
        }
    ) => {
        rpc_method!(
            $(#[$meta])*
            fn $func_name(&self $(, $arg_name : $arg_type)* ) -> Option<$ret> {
                if let Some($value) = $service.$( $method ),+( self $(, $method_arg )* ) as $value_type {
                    $success_expr
                } else {
                    None
                }
            }
        );
    };

    // RPC Class Method
    // Maps the decoded value
    // Returns None on no response
    (
        $(#[$meta:meta])*
        fn $func_name: ident (&self $(, $arg_name : ident : $arg_type : ty)* ) -> Option<$ret : ty> {
            $service : tt.$( $method : tt),+ ( $( $method_arg : expr ),* )
                .map(|$value: ident : $value_type : ty| $success_expr: expr)
        }
    ) => {
        rpc_method!(
            $(#[$meta])*
            fn $func_name(&self $(, $arg_name : $arg_type)* ) -> Option<$ret> {
                if let Some($value) = $service.$( $method ),+( $( $method_arg ),* ) as $value_type {
                    $success_expr
                } else {
                    None
                }
            }
        );
    };

    // RPC Class Method
    // Passes self to rpc method
    // Returns decoded value as is
    // Returns None on no response
    (
        $(#[$meta:meta])*
        fn $func_name: ident (&self $(, $arg_name : ident : $arg_type : ty)* ) -> Option<$ret : ty> {
            $service : tt.$( $method : tt),+ ( self $(, $method_arg : expr )* )
        }
    ) => {
        rpc_method!(
            $(#[$meta])*
            fn $func_name(&self $(, $arg_name : $arg_type)* ) -> Option<$ret> {
                if let Some(value) = $service.$( $method ),+( self $(, $method_arg )* ) as Option<$ret> {
                    value
                } else {
                    None
                }
            }
        );
    };

    // RPC Class Method
    // Returns the decoded value as is
    // Returns None on no response
    (
        $(#[$meta:meta])*
        fn $func_name: ident (&self $(, $arg_name : ident : $arg_type : ty)* ) -> Option<$ret : ty> {
            $service : tt.$( $method : tt),+ ( $( $method_arg : expr ),* )
        }
    ) => {
        rpc_method!(
            $(#[$meta])*
            fn $func_name(&self $(, $arg_name : $arg_type)* ) -> Option<$ret> {
                if let Some(value) = $service.$( $method ),+( $( $method_arg ),* ) as Option<$ret> {
                    value
                } else {
                    None
                }
            }
        );
    };

    // RPC Class Method
    // Passes self to rpc method
    // Returns the decoded value
    // Returns Err on no response
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

    // RPC Class Method
    // Returns the decoded value
    // Returns Err on no response
    (
        $(#[$meta:meta])*
        fn $func_name: ident (&self $(, $arg_name : ident : $arg_type : ty)* ) -> $ret : ty {
            $service : tt.$( $method : tt),+ ( $( $method_arg : expr ),* )
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

    // RPC Class Method
    // Passes self to rpc method
    // Custom success case handling
    // Custom no response case handling
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
    // Custom success case handling
    // Custom no response case handling
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

    // RPC Class Method
    // Passes self to rpc method
    // No return
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

    // RPC Class Method
    // No return
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


    // RPC Static Method
    // Returns the decoded value as is
    // Returns Err on no response
    (
        $(#[$meta:meta])*
        fn $func_name: ident ($client: ident : &KrpcClient $(, $arg_name : ident : $arg_type : ty)* ) -> $ret : ty {
            $service : tt.$( $method : tt),+ ( $( $method_arg : expr ),* )
                .ok_or($fail_expr: expr)
        }
    ) => {
        rpc_method!(
            $(#[$meta])*
            fn $func_name($client: &KrpcClient  $(, $arg_name : $arg_type)* ) -> $ret {
                if let Some(value) = $service.$( $method),+( $( $method_arg ),* ) as $ret {
                    value
                } else {
                    return Err($fail_expr)
                }
            }
        );
    };

    // RPC Static Method
    // Maps the decoded value
    // Returns Err on no response
    (
        $(#[$meta:meta])*
        fn $func_name: ident ($client: ident : &KrpcClient $(, $arg_name : ident : $arg_type : ty)* ) -> $ret : ty {
            $service : tt.$( $method : tt),+ ( $( $method_arg : expr ),* )
                .map(|$value: ident : $value_type: ident| $success_expr: expr)
                .ok_or($fail_expr: expr)
        }
    ) => {
        rpc_method!(
            $(#[$meta])*
            fn $func_name($client: &KrpcClient  $(, $arg_name : $arg_type)* ) -> $ret {
                if let Some($value) = $service.$( $method),+( $( $method_arg ),* ) as $value_type {
                    $success_expr
                } else {
                    return Err($fail_expr)
                }
            }
        );
    };

    // RPC Static Method
    // Custom success case handling
    // Custom no response case handling
    (
        $(#[$meta:meta])*
        fn $func_name: ident ($client: ident : &KrpcClient $(, $arg_name : ident : $arg_type : ty)* ) -> $ret : ty {
            if let Some($value: ident) = $service : tt.$( $method : tt),+ ( $($method_arg : expr ),* ) as $value_type: ty {
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

            Ok(if let Some(response) = rpc.invoke(stringify!($service).to_owned(), concat!( $( stringify!($method) ),+).to_owned(), args)? {
                let $value : $value_type = decode(&response, $client)?;
                $success_expr
            } else {
                $fail_expr
            })
        }
    };

    // RPC Static Method
    // No return
    (
        $(#[$meta:meta])*
        fn $func_name: ident ($client: ident : &KrpcClient $(, $arg_name : ident : $arg_type : ty)* ) {
            $service : tt.$( $method : tt),+ ( $($method_arg : expr ),* )
        }
    ) => {
        $(#[$meta])*
        pub fn $func_name($client : &Rc<RefCell<KrpcClient>> $(, $arg_name : $arg_type)*) -> KrpcResult<()> {
            let rpc = &mut (*$client).borrow_mut().rpc;
            let args: Vec<Vec<u8>> = vec![$($method_arg.encode()?),*];

            rpc.invoke(stringify!($service).to_owned(), concat!( $( stringify!($method) ),+).to_owned(), args)?;
            Ok(())
        }
    };
}


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
            client : Rc<RefCell<KrpcClient>>
        }

        impl $service {
            /// Creates a new service using the given `client`.
            pub fn new(client: Rc<RefCell<KrpcClient>>) -> Self {
                $service{client}
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
        @service_property(service=$service:tt) $prop_name: ident : Option<$prop_type: ty> =>
            $(#[$getter_meta:meta])*
            get: $getter_name: ident,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident
    ) => {
        rpc_method!(
        $( #[$getter_meta] )*
        fn $getter_name(&self) -> Option<$prop_type> {
            $service.get_,$prop_name()
        });

        rpc_method!(
        $(#[$setter_meta])*
        fn $setter_name(&self, value: $prop_type) {
            $service.set_,$prop_name(value)
        });
    };

    (
        @service_property(service=$service:tt)
            $prop_name: ident : Option<$prop_type: ty>,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident
    ) => {
        rpc_method!(
        $( #[$getter_meta] )*
        fn $getter_name(&self) -> Option<$prop_type> {
            $service.get_,$prop_name()
        });
    };

    (
        @service_property(service=$service:tt)
            $prop_name: ident : $prop_type: ty,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident
    ) => {
        rpc_method!(
        $( #[$getter_meta] )*
        fn $getter_name(&self) -> $prop_type {
            $service.get_,$prop_name()
                .ok_or(KrpcError::NullResponseValue)
        });

        rpc_method!(
        $(#[$setter_meta])*
        fn $setter_name(&self, value: $prop_type) {
            $service.set_,$prop_name(value)
        });
    };

    (
        @service_property(service=$service:tt)
            $prop_name: ident : $prop_type: ty,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident
    ) => {
        rpc_method!(
        $( #[$getter_meta] )*
        fn $getter_name(&self) -> $prop_type {
            $service.get_,$prop_name()
                .ok_or(KrpcError::NullResponseValue)
        });
    };

    //
    // Methods
    //
    (
        @service_method(service=$service:tt)
        $(#[$meta:meta])*
        fn $method_name: ident ($( $arg_name: ident : $arg_type: ty), *) -> Option<$return_type: ty> {
            $rpc_name: tt($( $arg_expr: expr ),* )
        }
    ) => {
        rpc_method!(
        $( #[$meta] )*
        fn $method_name(&self $(, $arg_name: $arg_type)*) -> Option<$return_type> {
            $service.$rpc_name($($arg_expr),*)
        });
    };

    (
        @service_method(service=$service:tt)
        $(#[$meta:meta])*
        fn $method_name: ident ($( $arg_name: ident : $arg_type: ty), *) -> $return_type: ty {
            $rpc_name: tt($( $arg_expr: expr ),* )
        }
    ) => {
        rpc_method!(
        $( #[$meta] )*
        fn $method_name(&self $(, $arg_name: $arg_type)*) -> $return_type {
            $service.$rpc_name($($arg_expr),*)
                .ok_or(KrpcError::NullResponseValue)
        });
    };

    (
        @service_method(service=$service:tt)
        $(#[$meta:meta])*
        fn $method_name: ident ($( $arg_name: ident : $arg_type: ty), *) {
            $rpc_name: tt($( $arg_expr: expr ),* )
        }
    ) => {
        rpc_method!(
        $( #[$meta] )*
        fn $method_name(&self $(, $arg_name: $arg_type)*) {
            $service.$rpc_name($($arg_expr),*)
        });
    };

    //
    // Remote Object
    //
    (
        $(#[$meta:meta])* object $object_name: ident {}
    ) => {
        remote_type! {
            $(#[$meta])*
            object $object_name {
                service: Unknown,
                properties: {},
                methods: {},
                static_methods: {}
            }
        }
    };

    (
        $(#[$meta:meta])*
        object $object_name: ident {
            service: $service: tt,
            properties: {
                $( { $( $property: tt)+ } )*
            },
            methods: {
                $( { $( $method: tt)+ } )*
            }
        }
    ) => {
        remote_type! {
            $(#[$meta])*
            object $object_name {
                service: $service,
                properties: {
                    $( { $( $property)+ } )*
                },
                methods: {
                    $( { $( $method)+ } )*
                },
                static_methods: {}
            }
        }
    };

    (
        $(#[$meta:meta])*
        object $object_name: ident {
            service: $service: tt,
            properties: {
                $( { $( $property: tt)+ } )*
            }
        }
    ) => {
        remote_type! {
            $(#[$meta])*
            object $object_name {
                service: $service,
                properties: {
                    $( { $( $property)+ } )*
                },
                methods: {},
                static_methods: {}
            }
        }
    };

    (
        $(#[$meta:meta])*
        object $object_name: ident {
            service: $service: tt,
            methods: {
                $( { $( $method: tt)+ } )*
            }
        }
    ) => {
        remote_type! {
            $(#[$meta])*
            object $object_name {
                service: $service,
                properties: {},
                methods: {
                    $( { $( $method)+ } )*
                },
                static_methods: {}
            }
        }
    };

    (
        $(#[$meta:meta])*
        object $object_name: ident {
            service: $service: tt,
            static_methods: {
                $( { $( $static_method: tt)+ } )*
            }
        }
    ) => {
        remote_type! {
            $(#[$meta])*
            object $object_name {
                service: $service,
                properties: {},
                methods: {},
                static_methods: {
                    $( { $( $static_method)+ } )*
                }
            }
        }
    };

    (
        $(#[$meta:meta])*
        object $object_name: ident {
            service: $service: tt,
            properties: {
                $( { $( $property: tt)+ } )*
            },
            methods: {
                $( { $( $method: tt)+ } )*
            },
            static_methods: {
                $( { $( $static_method: tt)+ } )*
            }
        }
     ) => {
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

        impl $object_name {
            // Properties
            $(
                remote_type!(@object_property(service=$service, class=$object_name) $( $property )+ );
            )*

            // Methods
            $(
                remote_type!(@object_method(service=$service, class=$object_name) $( $method )+ );
            )*

            // Static Methods
            $(
                remote_type!(@object_static_method(service=$service, class=$object_name) $( $static_method )+ );
            )*
        }
    };

    //
    // Properties
    //
    (
        @object_property(service=$service:tt, class=$class:tt) $prop_name: ident : Option<$prop_type: ty> =>
            $(#[$getter_meta:meta])*
            get: $getter_name: ident,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident
    ) => {
        rpc_method!(
        $( #[$getter_meta] )*
        fn $getter_name(&self) -> Option<$prop_type> {
            $service.$class,_get_,$prop_name(self)
        });

        rpc_method!(
        $(#[$setter_meta])*
        fn $setter_name(&self, value: $prop_type) {
            $service.$class,_set_,$prop_name(self, value)
        });
    };

    (
        @object_property(service=$service:tt, class=$class:tt)
            $prop_name: ident : Option<$prop_type: ty>,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident
    ) => {
        rpc_method!(
        $( #[$getter_meta] )*
        fn $getter_name(&self) -> Option<$prop_type> {
            $service.$class,_get_,$prop_name(self)
        });
    };

    (
        @object_property(service=$service:tt, class=$class:tt)
            $prop_name: ident : $prop_type: ty,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident,
            $(#[$setter_meta:meta])*
            set: $setter_name: ident
    ) => {
        rpc_method!(
        $( #[$getter_meta] )*
        fn $getter_name(&self) -> $prop_type {
            $service.$class,_get_,$prop_name(self)
                .ok_or(KrpcError::NullResponseValue)
        });

        rpc_method!(
        $(#[$setter_meta])*
        fn $setter_name(&self, value: $prop_type) {
            $service.$class,_set_,$prop_name(self, value)
        });
    };

    (
        @object_property(service=$service:tt, class=$class:tt)
            $prop_name: ident : $prop_type: ty,
            $(#[$getter_meta:meta])*
            get: $getter_name: ident
    ) => {
        rpc_method!(
        $( #[$getter_meta] )*
        fn $getter_name(&self) -> $prop_type {
            $service.$class,_get_,$prop_name(self)
                .ok_or(KrpcError::NullResponseValue)
        });
    };

    //
    // Methods
    //
    (
        @object_method(service=$service:tt, class=$class:tt)
        $(#[$meta:meta])*
        fn $method_name: ident ($( $arg_name: ident : $arg_type: ty), *) -> Option<$return_type: ty> {
            $rpc_name: tt($( $arg_expr: expr ),* )
        }
    ) => {
        rpc_method!(
        $( #[$meta] )*
        fn $method_name(&self $(, $arg_name: $arg_type)*) -> Option<$return_type> {
            $service.$class,_,$rpc_name(self $(, $arg_expr)*)
        });
    };

    (
        @object_method(service=$service:tt, class=$class:tt)
        $(#[$meta:meta])*
        fn $method_name: ident ($( $arg_name: ident : $arg_type: ty), *) -> $return_type: ty {
            $rpc_name: tt($( $arg_expr: expr ),* )
        }
    ) => {
        rpc_method!(
        $( #[$meta] )*
        fn $method_name(&self $(, $arg_name: $arg_type)*) -> $return_type {
            $service.$class,_,$rpc_name(self $(, $arg_expr)*)
                .ok_or(KrpcError::NullResponseValue)
        });
    };

    (
        @object_method(service=$service:tt, class=$class:tt)
        $(#[$meta:meta])*
        fn $method_name: ident ($( $arg_name: ident : $arg_type: ty), *) {
            $rpc_name: tt($( $arg_expr: expr ),* )
        }
    ) => {
        rpc_method!(
        $( #[$meta] )*
        fn $method_name(&self $(, $arg_name: $arg_type)*) {
            $service.$class,_,$rpc_name(self $(, $arg_expr)*)
        });
    };

    //
    // Static Methods
    //
    (
        @object_static_method(service=$service:tt, class=$class:tt)
        $(#[$meta:meta])*
        fn $method_name: ident ($( $arg_name: ident : $arg_type: ty), *) -> Option<$return_type: ty> {
            $rpc_name: tt($( $arg_expr: expr ),* )
        }
    ) => {
        rpc_method!(
        $( #[$meta] )*
        fn $method_name(client: &KrpcClient $(, $arg_name: $arg_type)*) -> Option<$return_type> {
            $service.$class,_static_,$rpc_name($( $arg_expr),*)
        });
    };

    (
        @object_static_method(service=$service:tt, class=$class:tt)
        $(#[$meta:meta])*
        fn $method_name: ident ($( $arg_name: ident : $arg_type: ty), *) -> $return_type: ty {
            $rpc_name: tt($( $arg_expr: expr ),* )
        }
    ) => {
        rpc_method!(
        $( #[$meta] )*
        fn $method_name(client: &KrpcClient $(, $arg_name: $arg_type)*) -> $return_type {
            $service.$class,_static_,$rpc_name($($arg_expr),*)
                .ok_or(KrpcError::NullResponseValue)
        });
    };

    (
        @object_static_method(service=$service:tt, class=$class:tt)
        $(#[$meta:meta])*
        fn $method_name: ident ($( $arg_name: ident : $arg_type: ty), *) {
            $rpc_name: tt($( $arg_expr: expr ),* )
        }
    ) => {
        rpc_method!(
        $( #[$meta] )*
        fn $method_name(client: &KrpcClient $(, $arg_name: $arg_type)*) {
            $service.$class,_static_,$rpc_name($($arg_expr),*)
        });
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