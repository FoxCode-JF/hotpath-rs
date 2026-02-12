use proc_macro::TokenStream;

/// No-op version of `#[hotpath_meta::main]` when profiling is disabled.
///
/// This macro simply returns the original function unchanged.
pub fn main_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// No-op version of `#[hotpath_meta::measure]` when profiling is disabled.
///
/// This macro simply returns the original function unchanged.
pub fn measure_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// No-op version of `#[hotpath_meta::future_fn]` when profiling is disabled.
///
/// This macro simply returns the original function unchanged.
pub fn future_fn_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// No-op version of `#[hotpath_meta::skip]` when profiling is disabled.
///
/// This macro simply returns the original function unchanged.
pub fn skip_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// No-op version of `#[hotpath_meta::measure_all]` when profiling is disabled.
///
/// This macro simply returns the original item unchanged.
pub fn measure_all_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
