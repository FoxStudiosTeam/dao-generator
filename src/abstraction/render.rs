use std::path::Path;

pub trait RenderScheme {
    /// Render tables to template
    /// returns Vec<(table_name, rendered_content)>
    fn render_tables<P: AsRef<Path>>(&self, template: P) -> anyhow::Result<Vec<(String, String)>>;
    fn render<P: AsRef<Path>>(&self, template: P) -> anyhow::Result<String>;
}