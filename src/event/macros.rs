macro_rules! unimplemented_handlers {
    ($handler:ty { $($event:ty),+ $(,)? }) => {
        $(
        impl $crate::event::EventHandler<$event> for $handler {
            fn handle(&mut self, _event: $event) -> $crate::event::HandlerResult<$event> {
                Err($crate::event::Error::Unimplemented)
            }
        }
        )+
    };
}

macro_rules! delegate_handlers {
    ($handler:ty { $($event:ty),+ $(,)? }) => {
        $(
        impl $crate::event::EventHandler<$event> for $handler {
            fn handle(&mut self, event: $event) -> $crate::event::HandlerResult<$event> {
                DelegateEventHandler::<$event>::delegate_handler(self).handle(event)
            }
        }
        )+
    };
}

macro_rules! delegate_events {
    ($handler:ty { $($event:ty),+ $(,)? }) => {
        $(
        impl $crate::event::EventHandler<$event> for $handler {
            fn handle(&mut self, event: $event) -> $crate::event::HandlerResult<$event> {
                DelegateEvent::<$event>::delegate(self, event)
            }
        }
        )+
    };
}

macro_rules! declare_events {
    ($($handler:ty $(: $($other_event:ty),+ $(,)? )? {
        $( $event_name:ident $event:tt -> $event_return:ty ),* $(,)?
    })*) => {
        $(
        $(
        declare_events!(@declare_struct $event_name $event);

        impl $crate::event::Event for $event_name {
            type Return = $event_return;
        }

        static_assertions::assert_impl_all!($handler: $crate::event::EventHandler<$event_name>);
        )*
        $($(
            static_assertions::assert_impl_all!(
                $handler: $crate::event::EventHandler<$other_event>
            );
        )+)?
        )*
    };
    (@declare_struct $name:ident {
        $($field:ident: $field_type:ty),+ $(,)?
    }) => {
        #[derive(Debug)]
        pub struct $name {
            $(pub $field: $field_type,)+
        }

        impl $name {
            pub fn new($($field: $field_type),+) -> Self {
                Self { $($field),+ }
            }
        }
    };
    (@declare_struct $name:ident()) => {
        #[derive(Debug)]
        pub struct $name;
    };
    (@declare_struct $name:ident (
        $($field_type:ty),+ $(,)?
    )) => {
        #[derive(Debug)]
        pub struct $name(
            $(pub $field_type,)+
        );
    };
}

pub(super) use declare_events;

pub(crate) use delegate_events;
pub(crate) use delegate_handlers;
pub(crate) use unimplemented_handlers;
