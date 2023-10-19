#[deprecated]
macro_rules! unimplemented_handlers {
    ($handler:ty { $($event:ty),+ $(,)? }) => {
        $(
        impl $crate::event::EventHandler<$event> for $handler {
            fn handle(&self, _event: $event) -> $crate::event::HandlerResult<$event> {
                Err($crate::event::Error::Unimplemented)
            }
        }

        impl $crate::event::UnimplementedHandler<$event> for $handler {}
        )+
    };
}

#[deprecated]
macro_rules! unimplemented_handlers_mut {
    ($handler:ty { $($event:ty),+ $(,)? }) => {
        $(
        impl $crate::event::EventHandlerMut<$event> for $handler {
            fn handle_mut(&mut self, _event: $event) -> $crate::event::HandlerResult<$event> {
                Err($crate::event::Error::Unimplemented)
            }
        }

        impl $crate::event::UnimplementedHandler<$event> for $handler {}
        )+
    };
}

#[deprecated]
macro_rules! delegate_handlers {
    ($handler:ty { $($event:ty),+ $(,)? }) => {
        $(
        impl $crate::event::EventHandler<$event> for $handler {
            fn handle(&self, event: $event) -> $crate::event::HandlerResult<$event> {
                $crate::event::DelegateEventHandler::<$event>::delegate_handler(self).handle(event)
            }
        }
        )+
    };
}

#[deprecated]
macro_rules! delegate_handlers_mut {
    ($handler:ty { $($event:ty),+ $(,)? }) => {
        $(
        impl $crate::event::EventHandlerMut<$event> for $handler {
            fn handle_mut(&mut self, event: $event) -> $crate::event::HandlerResult<$event> {
                $crate::event::DelegateEventHandlerMut::<$event>::delegate_handler_mut(self)
                    .handle_mut(event)
            }
        }
        )+
    };
}

#[deprecated]
// TODO: merge delegare_events_mut and delegate_events like in declare macro
macro_rules! delegate_events_mut {
    ($handler:ty { $($event:ty),+ $(,)? }) => {
        $(
        impl $crate::event::EventHandlerMut<$event> for $handler {
            fn handle_mut(&mut self, event: $event) -> $crate::event::HandlerResult<$event> {
                $crate::event::DelegateEventMut::<$event>::delegate_mut(self, event)
            }
        }
        )+
    };
}

#[deprecated]
macro_rules! delegate_events {
    ($handler:ty { $($event:ty),+ $(,)? }) => {
        $(
        impl $crate::event::EventHandler<$event> for $handler {
            fn handle(&self, event: $event) -> $crate::event::HandlerResult<$event> {
                $crate::event::DelegateEvent::<$event>::delegate(self, event)
            }
        }
        )+
    };
}

#[deprecated]
macro_rules! declare_handler {
    ( $handler:ty {
        $('inherited: $inherited:tt)?
        $('unimplemented: $unimplemented:tt)?
        $('events_mut: $events_mut:tt)?
        $('events: $events:tt)?
    } ) => {
        $( declare_handler!(@events_mut $handler, $events_mut); )?
        $( declare_handler!(@events $handler, $events); )?
        $( declare_handler!(@unimplemented $handler, $unimplemented ); )?
        // $( declare_handler!(@inherited $handler, $inherited ); )?
    };

    (@events $handler:ty, { $( $event_name:ident $event:tt -> $event_return:ty ),+ $(,)? } ) => {
        $(
        declare_handler!(@declare_event $handler, $event_name, $event -> $event_return);

        impl $crate::event::Event for $event_name {}

        static_assertions::assert_impl_all!($handler: $crate::event::EventHandler<$event_name>);
        )+
    };

    (@events_mut $handler:ty, { $( $event_name:ident $event:tt -> $event_return:ty ),+ $(,)? }) => {
        $(
        declare_handler!(@declare_event $handler, $event_name, $event -> $event_return);

        impl $crate::event::EventMut for $event_name {}

        static_assertions::assert_impl_all!($handler: $crate::event::EventHandlerMut<$event_name>);
        )+
    };

    (@unimplemented $handler:ty, { $($other_event:ty),+ $(,)? } ) => {
        $(
        static_assertions::assert_impl_all!($handler:
            // $crate::event::EventHandler<$other_event>,
            $crate::event::UnimplementedHandler<$other_event>
        );
        )+
    };

    (@inherited $handler:ty, { $($other_event:ty),+ $(,)? } ) => {
        $(
        static_assertions::assert_impl_all!($handler:
            $crate::event::EventHandler<$other_event>
        );
        )+
    };

    (@declare_event $handler:ty, $event_name:ident, $event:tt -> $event_return:ty ) => {
        declare_handler!(@declare_struct $event_name $event);

        impl $crate::event::Ret for $event_name {
            type Return = $event_return;
        }
    };

    (@declare_struct $name:ident {
        $($field:ident: $field_type:ty),+ $(,)?
    }) => {
        #[deprecated]
        #[derive(Debug)]
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

    (@declare_struct $name:ident()) => {
        #[deprecated]
        #[derive(Debug)]
        pub struct $name;
    };

    (@declare_struct $name:ident (
        $($field_type:ty),+ $(,)?
    )) => {
        #[deprecated]
        #[derive(Debug)]
        pub struct $name(
            $(pub $field_type,)+
        );
    };
}

pub(super) use declare_handler;

pub(crate) use delegate_events;
pub(crate) use delegate_events_mut;
pub(crate) use delegate_handlers;
pub(crate) use delegate_handlers_mut;
pub(crate) use unimplemented_handlers;
pub(crate) use unimplemented_handlers_mut;
