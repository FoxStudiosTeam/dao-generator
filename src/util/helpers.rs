use handlebars::Handlebars;
use handlebars::Helper;
use handlebars::Context;
use handlebars::RenderContext;
use handlebars::Output;
use handlebars::HelperResult;

pub fn upper_first(
    h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output
) -> HelperResult {
    let Some(param) = h.param(0) else {return Ok(())};
    let Some(param) = param.value().as_str() else {return Ok(());};
    let mut chars = param.chars();
    let result = match chars.next() {
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        None => "".to_string(),
    };
    out.write(&result)?;
    Ok(())
}

pub fn snake_to_camel(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let Some(param) = h.param(0) else { return Ok(()); };
    let Some(s) = param.value().as_str() else { return Ok(()); };

    let mut camel = String::new();
    let mut uppercase_next = true;

    for c in s.chars() {
        if c == '_' {
            uppercase_next = true;
        } else if uppercase_next {
            camel.push(c.to_ascii_uppercase());
            uppercase_next = false;
        } else {
            camel.push(c);
        }
    }

    out.write(&camel)?;
    Ok(())
}

pub trait DefaultHelpers {
    fn register_default_helpers(&mut self);
}

impl DefaultHelpers for Handlebars<'_> {
    fn register_default_helpers(&mut self) {
        self.register_helper("upperFirst", Box::new(upper_first));
        self.register_helper("snakeToCamel", Box::new(snake_to_camel));
    }
}