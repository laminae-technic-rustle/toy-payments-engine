/* This is inspired by Haskell's 'sequence' (Monad m => [m a] -> m [a])
 * https://hackage.haskell.org/package/base-4.15.0.0/docs/GHC-Base.html#v:sequence
 * But instead of a list, we invert a pair ((Option a), Option(b))->Option(a,b)
 *
 * Extremely handy when trying to rely on the result of two options being Some
 * */
pub fn sequence<A, B>((a, b): (Option<A>, Option<B>)) -> Option<(A, B)> {
    match (a, b) {
        (Some(a), Some(b)) => Some((a, b)),
        _ => None,
    }
}
