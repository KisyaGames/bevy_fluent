//! Plugins
//!
//! Any entity located directly in this module is [`Plugin`](bevy::app::Plugin).

use crate::{
    assets::{
        bundle::{BundleAssetLoader, ConcurrentFluentBundle},
        resource::ResourceAssetLoader,
    },
    BundleAsset, ResourceAsset,
};
use bevy::{app::PluginGroupBuilder, prelude::*};

/// Adds support for Fluent file loading to applications
pub struct FluentPlugin<CustomizeFn>
where
    CustomizeFn: Fn(&mut ConcurrentFluentBundle) + Send + Sync + 'static,
{
    pub customize_bundle_fn: CustomizeFn,
}

impl<CustomizeFn> FluentPlugin<CustomizeFn>
where
    CustomizeFn: Fn(&mut ConcurrentFluentBundle) + Send + Sync + 'static + Copy,
{
    pub fn new(customize_bundle_fn: CustomizeFn) -> Self {
        Self {
            customize_bundle_fn,
        }
    }
}

#[derive(Default)]
pub struct DefaultFluentPlugins;

impl PluginGroup for DefaultFluentPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().set(FluentPlugin::new(|_| {}))
    }
}

impl<CustomizeFn> Plugin for FluentPlugin<CustomizeFn>
where
    CustomizeFn: Fn(&mut ConcurrentFluentBundle) + Send + Sync + 'static + Copy,
{
    fn build(&self, app: &mut App) {
        app.register_asset_loader(ResourceAssetLoader)
            .init_asset::<ResourceAsset>()
            .register_asset_loader(BundleAssetLoader {
                customize_bundle_fn: self.customize_bundle_fn,
            })
            .init_asset::<BundleAsset>();
    }
}
