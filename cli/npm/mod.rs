// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.

mod cache;
mod installer;
mod registry;
mod resolution;
mod resolvers;
mod tarball;

pub use cache::NpmCache;
pub use cache::NpmCacheDir;
pub use installer::PackageJsonDepsInstaller;
pub use registry::CliNpmRegistryApi;
pub use resolution::NpmResolution;
pub use resolvers::create_npm_fs_resolver;
pub use resolvers::CliNpmResolver;
pub use resolvers::InnerCliNpmResolverRef;
pub use resolvers::ManagedCliNpmResolver;
pub use resolvers::NpmPackageFsResolver;
pub use resolvers::NpmProcessState;
