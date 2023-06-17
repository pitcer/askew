pub trait Property {
    type Type;

    fn name(self) -> &'static str;

    fn default_value(self) -> Self::Type;
}

macro_rules! declare_properties {
    ($($struct_name:ident($name:literal, $property_type:ty, $default_value:expr)),*) => {
        $(
        pub struct $struct_name;

        impl Property for $struct_name {
            type Type = $property_type;

            fn name(self) -> &'static str {
                $name
            }

            fn default_value(self) -> Self::Type {
                $default_value
            }
        }
        )*
    };
}

declare_properties! {
    ConvexHull("show_convex_hull", bool, false),
    ChebyshevNodes("chebyshev_nodes", bool, false),
    LineWidth("line_width", f32, 2.0),
    Samples("samples", u32, 5000)
}
