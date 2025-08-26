use std::path::Path;

pub trait RenderScheme {
    /// Render schema to templates got from folder
    /// returns HashMap<template_name, Vec<(name, rendered_content)>>
    fn render_all<P: AsRef<Path>>(&self, templates: P) -> anyhow::Result<hashbrown::HashMap<String, Vec<(String, String)>>> ;
}