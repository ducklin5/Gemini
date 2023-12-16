

#[macro_export]
macro_rules! config_node_core {
    ($node:expr, $($name:expr)?) => {
        $(
            $node.set_name($name.into());
        )?
    };
}

#[macro_export]
macro_rules! config_node_props {
    ($node:expr, [ $($key:expr => $value:expr),* ]) => {
        use godot::prelude::Variant;
        $(
            $node.set_indexed($key.into(), Variant::from($value) );
        )*
    };
}

#[macro_export(local_inner_macros)]
macro_rules! config_node  {
    ($node:expr, ) => {};
    ($node:expr, {$($children:tt)*} ) => {
        scene!($node, { $($children)*} );
    };
    ($node:expr, [$($props:tt)*] $($other:tt)*) => {
        config_node_props!($node, [ $($props)* ]);
        config_node!($node, $($other)*);
    };
    ($node:expr, ($($name:expr)?) $($other:tt)*) => {
        $(
            config_node_core!($node, $name);
        )?
        config_node!($node, $($other)*);
    };
}

#[macro_export(local_inner_macros)]
macro_rules! add_new_child {
    ($root:expr, ) => {};
    ($root:expr, $node_type:ident $($options:tt)*) => {
        {
            
            #[allow(unused_mut)]
            let mut node = $node_type::new_alloc();
            config_node!(node, $($options)*);
            let mut base = node.upcast::<Node>();
            base.set_meta("is_inherited".into(), Variant::from(true));
            $root.add_child(base);
        }
    };
}

#[macro_export(local_inner_macros)]
macro_rules! scene {
    // TT muncher

    // inner base case
    (@inner $root:expr, [ $($accum:tt)* ]) => {
        add_new_child!($root, $($accum)*);
    };

    // inner end of node_tokens handler (handles comma token)
    (@inner $root:expr, [ $($accum:tt)* ] , $($rest:tt)* ) => {
        add_new_child!($root, $($accum)*);
        scene!(@inner $root, [] $($rest)*);
    };

    // inner token accummulator
    (@inner $root:expr, [ $($accum:tt)* ] $current:tt $($rest:tt)* ) => {
        scene!( @inner $root, [ $($accum)* $current ] $($rest)*);
    };

    // {} Wrapper and inner entry point
    ($root:expr, { $($tokens:tt)* }) => {
        scene!( @inner $root, [] $($tokens)*);
    };
}
