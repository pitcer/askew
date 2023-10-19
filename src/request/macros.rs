macro_rules! declare_requests {
    ($($declare:tt),+ $(,)?) => {
        $(declare_requests!(@declare $declare);)+
    };

    (@declare { mut $name:ident $request:tt -> $response:ty }) => {
        declare_requests!(@declare_struct $name, $request);

        impl $crate::request::RequestMut for $name {
            type Response = $response;
        }
    };

    (@declare { $name:ident $request:tt -> $response:ty }) => {
        declare_requests!(@declare_struct $name, $request);

        impl $crate::request::Request for $name {
            type Response = $response;
        }
    };

    (@declare_struct $name:ident, { $($field:ident: $field_type:ty),+ $(,)? }) => {
        #[derive(Debug, Default)]
        pub struct $name {
            $(pub $field: $field_type,)+
        }

        impl $name {
            #[must_use]
            pub fn new($($field: $field_type),+) -> Self {
                Self { $($field),+ }
            }
        }
    };

    (@declare_struct $name:ident, ()) => {
        #[derive(Debug, Default)]
        pub struct $name;
    };

    (@declare_struct $name:ident, ($($field_type:ty),+ $(,)?)) => {
        #[derive(Debug, Default)]
        pub struct $name(
            $(pub $field_type,)+
        );
    };
}

macro_rules! delegate_requests {
    (<$generic:ident> $handler:ty { $($delegate:tt),+ $(,)? }) => {
        $(delegate_requests!(@delegate $handler, $delegate, $generic);)+
    };

    ($handler:ty { $($delegate:tt),+ $(,)? }) => {
        $(delegate_requests!(@delegate $handler, $delegate);)+
    };

    (@delegate $handler:ty, { mut $request:ty => ! } $(, $generic:ident)?) => {
        impl $(<$generic>)? $crate::request::RequestHandlerMut<$request> for $handler {
            fn handle_mut(&mut self, request: $request) -> $crate::request::ResponseMut<$request> {
                delegate_requests!(@unimplemented_error $handler, |request|)
            }
        }
    };

    (@delegate $handler:ty, { $request:ty => ! } $(, $generic:ident)?) => {
        impl $(<$generic>)? $crate::request::RequestHandler<$request> for $handler {
            fn handle(&self, request: $request) -> $crate::request::Response<$request> {
                delegate_requests!(@unimplemented_error $handler, |request|)
            }
        }
    };

    (@delegate $handler:ty, { mut $request:ty => $sub_handler:ty } $(, $generic:ident)?) => {
        impl $(<$generic>)? $crate::request::RequestHandlerMut<$request> for $handler
        where
            $($generic: $crate::request::RequestHandlerMut<$request>)?
        {
            fn handle_mut(&mut self, request: $request) -> $crate::request::ResponseMut<$request> {
                $crate::request::RequestSubHandlerMut::<$sub_handler>::sub_handler_mut(self)
                    .handle_mut(request)
            }
        }
    };

    (@delegate $handler:ty, { $request:ty => $sub_handler:ty } $(, $generic:ident)?) => {
        impl $(<$generic>)? $crate::request::RequestHandler<$request> for $handler
        where
             $($generic: $crate::request::RequestHandler<$request>)?
        {
            fn handle(&self, request: $request) -> $crate::request::Response<$request> {
                $crate::request::RequestSubHandler::<$sub_handler>::sub_handler(self)
                    .handle(request)
            }
        }
    };

    (@unimplemented_error $handler:ty, |$request:ident|) => {
        Err($crate::request::Error::Unimplemented {
            request: format!("{:?}", $request),
            handler: stringify!($handler),
        })
    };
}

pub(crate) use declare_requests;
pub(crate) use delegate_requests;
