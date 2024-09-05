#![feature(register_tool)]
#![register_tool(tarpaulin)]
#![allow(clippy::map_entry)]

use reactive_graph_model_system_environment as model_system_environment;

pub mod factory;
pub mod plugin;
pub mod providers;

// #[cfg(test)]
// #[tarpaulin::ignore]
// pub mod tests;

// #[macro_use]
// extern crate query_interface;
//
// use std::sync::Arc;
//
// use reactive_graph_core_builder as builder;
// use reactive_graph_core_di as di;
// use reactive_graph_model_system_environment as model_system_environment;
// use reactive_graph_plugin_api as plugins;
// use log::error;
//
// use crate::di::profiles;
// use crate::di::Container;
// use crate::di::Provider;
// use crate::plugin::SystemEnvironmentPlugin;
// use crate::plugins::Plugin;
// use crate::plugins::PluginDependency;
// use crate::plugins::PluginLoadingError;
//
// pub mod plugin;
// pub mod providers;
//
// pub static PLUGIN_NAME: &str = env!("CARGO_PKG_NAME");
// pub static PLUGIN_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
// pub static PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");
//
// pub fn get<T>() -> Container<T> {
//     Container::<T>::new()
// }
//
// pub fn construct_plugin() -> Result<Arc<dyn Plugin>, PluginLoadingError> {
//     let mut container = get::<profiles::Default>();
//     let container = &mut container;
//     let plugin = Provider::<dyn SystemEnvironmentPlugin>::create(container);
//     let plugin = Arc::new(plugin);
//     let plugin: Result<Arc<dyn Plugin>, _> = <dyn query_interface::Object>::query_arc(plugin);
//     if plugin.is_err() {
//         error!("Failed to construct plugin");
//         return Err(PluginLoadingError::PluginContainerInitializationError);
//     }
//     Ok(plugin.unwrap())
// }
//
// plugins::export_plugin!(register, get_dependencies, PLUGIN_NAME, PLUGIN_DESCRIPTION, PLUGIN_VERSION);
//
// #[allow(improper_ctypes_definitions)]
// extern "C" fn register(registrar: &mut dyn plugins::PluginRegistrar) {
//     if let Err(error) = log4rs::init_file("config/logging.toml", Default::default()) {
//         println!("Failed to configure logger in {}: {}", PLUGIN_NAME, error);
//     }
//     if let Ok(plugin) = construct_plugin() {
//         registrar.register_plugin(Box::new(plugin));
//     }
// }
//
// #[allow(improper_ctypes_definitions)]
// extern "C" fn get_dependencies() -> Vec<PluginDependency> {
//     vec![
//         PluginDependency::new("reactive-graph-plugin-base", ">=0.9.0, <0.10.0"),
//         PluginDependency::new("reactive-graph-plugin-value", ">=0.9.0, <0.10.0"),
//     ]
// }
//
// #[cfg(test)]
// #[tarpaulin::ignore]
// pub mod tests;
