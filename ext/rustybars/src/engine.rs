use std::fmt::{self, Debug, Formatter};

use handlebars::Handlebars;
use magnus::{function, method, prelude::*, scan_args::{get_kwargs, scan_args}, Error, Object, RHash, Ruby, Value};

use crate::{namespace::RUSTYBARS_MODULE, template::CompiledTemplate};

#[magnus::wrap(class = "Rustybars::Engine")]
pub struct Engine {
    pub registry: Handlebars<'static>
}

impl Debug for Engine {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("Engine")
            .field("strict_mode", &self.registry.strict_mode())
            .field("dev_mode", &self.registry.dev_mode())
            .field("prevent_indent", &self.registry.prevent_indent())
            .finish()
    }
}

impl Engine {
    fn new(args: &[Value]) -> Result<Self, Error> {
        let args = scan_args::<(), (), (), (), RHash, ()>(args)?;
        let args = get_kwargs(
            args.keywords,
            &[],
            &["strict_mode", "dev_mode", "prevent_indent"]
        )?;
        let _: () = args.required;
        let (strict_mode, dev_mode, prevent_indent): (Option<bool>, Option<bool>, Option<bool>,) = args.optional;
        let _: () = args.splat;

        let mut hbs = Handlebars::new();
        hbs.set_strict_mode(strict_mode.unwrap_or(false));
        hbs.set_dev_mode(dev_mode.unwrap_or(false));
        hbs.set_prevent_indent(prevent_indent.unwrap_or(false));
        Ok(Self { registry: hbs })
    }

    fn render(ruby: &Ruby, rb_self: &'static Self, source: String, data: String) -> Result<String, Error> {
        let template = Self::compile(ruby, rb_self, source)?;
        CompiledTemplate::render(ruby, &template, data)
    }

    fn compile(ruby: &Ruby, rb_self: &'static Self, source: String) -> Result<CompiledTemplate, Error> {
        CompiledTemplate::compile(ruby, &rb_self.registry, source)
    }

    fn inspect(&self) -> Result<String, Error> {
        Ok(format!("<Rustybars::{:?}>", self))
    }
}

pub fn init(ruby: &Ruby) -> Result<(), Error> {
    let class = ruby.get_inner(&RUSTYBARS_MODULE).define_class("Engine", ruby.class_object())?;
    class.define_singleton_method("new", function!(Engine::new, -1))?;
    class.define_method("compile", method!(Engine::compile, 1))?;
    class.define_method("render", method!(Engine::render, 2))?;
    class.define_method("inspect", method!(Engine::inspect, 0))?;
    Ok(())
}
