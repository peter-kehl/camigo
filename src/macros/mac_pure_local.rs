/// NOT for public (for now). Otherwise make [crate::Locality::debug_fail_unreachable_for_non_local] public, too.
#[macro_export]
macro_rules! pure_local_cpartial_eq {
    ($T:ident) => {
        impl $crate::CPartialEq for $T {
            const LOCALITY: $crate::Locality = $crate::Locality::PureLocal;

            fn eq_local(&self, other: &Self) -> bool {
                self == other
            }
            fn eq_non_local(&self, other: &Self) -> bool {
                $crate::locality::debug_fail_unreachable_for_non_local();
                self == other
            }
            fn eq_full(&self, other: &Self) -> bool {
                self == other
            }
        }
    };
}

/// NOT for public (for now). Otherwise make [crate::Locality::debug_fail_unreachable_for_non_local] public, too.
#[macro_export]
macro_rules! pure_local_cord {
    ($T:ident) => {
        impl $crate::COrd for $T {
            fn cmp_local(&self, other: &Self) -> core::cmp::Ordering {
                self.cmp(other)
            }

            fn cmp_non_local(&self, other: &Self) -> core::cmp::Ordering {
                $crate::locality::debug_fail_unreachable_for_non_local();
                self.cmp(other)
            }

            fn cmp_full(&self, other: &Self) -> core::cmp::Ordering {
                self.cmp(other)
            }
        }
    };
}
