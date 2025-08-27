use std::path::Path;

pub trait RenderScheme {
    /// Render schema to templates got from folder
    /// returns HashMap<template_name, Vec<(name, rendered_content)>>
    fn render_tables<P: AsRef<Path>>(&self, templates: P) -> anyhow::Result<hashbrown::HashMap<String, Vec<(String, String)>>> ;
    fn render<P: AsRef<Path>>(&self, template: P) -> anyhow::Result<String>;
}