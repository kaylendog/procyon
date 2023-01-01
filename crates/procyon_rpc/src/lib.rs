//! Defines the Procyon RPC definition.

use rspc::Router;

/// The router context.
pub struct Context {}

/// Build and return the router.
pub fn router() -> Router<Context> {
    Router::new().query("version", |t| t(|_, _: ()| env!("VERGEN_BUILD_SEMVER"))).build()
}
