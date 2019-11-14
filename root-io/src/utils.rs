/// Zip `n` streams together analogous to the `zip()` function. This
/// is useful when iterating over multiple branches of a TTree. See
/// the examples of this crate for the suggested use-case.
#[macro_export]
macro_rules! stream_zip {
    // @closure creates a tuple-flattening closure for .map() call. usage:
    // @closure partial_pattern => partial_tuple , rest , of , iterators
    // eg. izip!( @closure ((a, b), c) => (a, b, c) , dd , ee )
    ( @closure $p:pat => $tup:expr ) => {
        |$p| $tup
    };

    // The "b" identifier is a different identifier on each recursion level thanks to hygiene.
    ( @closure $p:pat => ( $($tup:tt)* ) , $_iter:expr $( , $tail:expr )* ) => {
        stream_zip!(@closure ($p, b) => ( $($tup)*, b ) $( , $tail )*)
    };

    // unary
    ($first:expr $(,)*) => {
        $first
    };

    // binary
    ($first:expr, $second:expr $(,)*) => {
        stream_zip!($first)
            .zip($second)
    };

    // n-ary where n > 2
    ( $first:expr $( , $rest:expr )* $(,)* ) => {
        stream_zip!($first)
            $(
                .zip($rest)
            )*
            .map(
                stream_zip!(@closure a => (a) $( , $rest )*)
            )
    };
}
