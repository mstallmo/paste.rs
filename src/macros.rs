#[macro_export]
macro_rules! ignore {
    ( $( $v:ident ), +) => {
        $(let _ = $v;) +
    }
}