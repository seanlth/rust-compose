
#[macro_export]
macro_rules! compose {
    ( $c:expr, ) => { $c };
    ( $c:expr, . $x:ident $(.$y:ident)* ) => { $x ( compose!($c, $(. $y)* ) ) };
    ( $x:ident $(. $y:ident)* ) => { |arg| $x ( compose!(arg, $(. $y)* ) ) };
}
