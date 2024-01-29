use handlebars::{Context, Handlebars, RenderContext, Renderable, Template};
use magnus::{method, prelude::*, Class, Error, Ruby};

use crate::{errors::{RUSTYBARS_COMPILE_ERROR, RUSTYBARS_JSON_ERROR, RUSTYBARS_RENDER_ERROR}, namespace::RUSTYBARS_MODULE};

#[magnus::wrap(class = "Rustybars::CompiledTemplate")]
pub struct CompiledTemplate {
    pub handlebars: &'static Handlebars<'static>,
    pub template: Template
}

impl CompiledTemplate {
    pub fn compile(ruby: &Ruby, handlebars: &'static Handlebars, source: String) -> Result<Self, Error> {
        let template = Template::compile(&source).map_err(|e| {
            Error::new(
                ruby.get_inner(&RUSTYBARS_COMPILE_ERROR),
                e.to_string()
                )
        })?;
        Ok(Self{
            handlebars,
            template
        })
    }

    pub fn render(ruby: &Ruby, rb_self: &Self, data: String) -> Result<String, Error> {
        let hbs = &rb_self.handlebars;

        let data: serde_json::Value = serde_json::from_str(&data).map_err(|e| {
            Error::new(
                ruby.get_inner(&RUSTYBARS_JSON_ERROR),
                e.to_string(),
            )
        })?;

        let ctx = Context::from(data);

        let mut render_context = RenderContext::new(None);

        rb_self.template.renders(&hbs, &ctx, &mut render_context).map_err(|e| {
            Error::new(
                ruby.get_inner(&RUSTYBARS_RENDER_ERROR),
                e.to_string()
            )
        })
    }
}

pub fn init(ruby: &Ruby) -> Result<(), Error> {
    let class = ruby.get_inner(&RUSTYBARS_MODULE).define_class("CompiledTemplate", ruby.class_object())?;
    class.undef_default_alloc_func();
    class.define_method("render", method!(CompiledTemplate::render, 1))?;
    Ok(())
}
