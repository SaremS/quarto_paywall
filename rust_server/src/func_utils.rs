//Utility functions based on concepts from functional programming
//Primarily used to declutter some of the main code
//
//
pub mod extractable_tuples {
    /*
        Convert `(Option<T1>, Option<T2>,..)` (possibly other monads later on) 
        into an `Option<(T1,T2,..)>` that resolves to `Some()` only if all elements
        of the tuple of Options resolve to `Some()`. Otherwise, resolves to `None`.
    */
    pub trait ExtractableOptionTuple2<T1, T2> {
        fn extract(self) -> Option<(T1, T2)>;
    }

    impl<T1: Clone, T2: Clone> ExtractableOptionTuple2<T1, T2> for (Option<T1>, Option<T2>) {
        fn extract(self) -> Option<(T1, T2)> {
            match self {
                (Some(a), Some(b)) => Some((a.clone(), b.clone())),
                _ => None,
            }
        }
    }

    pub trait ExtractableOptionTuple3<T1, T2, T3> {
        fn extract(self) -> Option<(T1, T2, T3)>;
    }

    impl<T1: Clone, T2: Clone, T3: Clone> ExtractableOptionTuple3<T1, T2, T3> for (Option<T1>, Option<T2>, Option<T3>) {
        fn extract(self) -> Option<(T1, T2, T3)> {
            match self {
                (Some(a), Some(b), Some(c)) => Some((a, b, c)),
                _ => None,
            }
        }
    }
}
