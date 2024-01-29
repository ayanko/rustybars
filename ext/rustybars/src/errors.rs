use magnus::{exception::ExceptionClass, value::Lazy, Error, Module, Ruby};

use super::namespace::RUSTYBARS_MODULE;

pub static RUSTYBARS_ERROR: Lazy<ExceptionClass> = Lazy::new(|ruby| {
    ruby
        .get_inner(&RUSTYBARS_MODULE)
        .define_error("Error", ruby.exception_standard_error())
        .unwrap()
});

pub static RUSTYBARS_JSON_ERROR: Lazy<ExceptionClass> = Lazy::new(|ruby| {
    ruby
        .get_inner(&RUSTYBARS_MODULE)
        .define_error("JsonError", ruby.get_inner(&RUSTYBARS_ERROR))
        .unwrap()
});

pub static RUSTYBARS_COMPILE_ERROR: Lazy<ExceptionClass> = Lazy::new(|ruby| {
    ruby
        .get_inner(&RUSTYBARS_MODULE)
        .define_error("CompileError", ruby.get_inner(&RUSTYBARS_ERROR))
        .unwrap()
});

pub static RUSTYBARS_RENDER_ERROR: Lazy<ExceptionClass> = Lazy::new(|ruby| {
    ruby
        .get_inner(&RUSTYBARS_MODULE)
        .define_error("RenderError", ruby.get_inner(&RUSTYBARS_ERROR))
        .unwrap()
});

pub fn init(ruby: &Ruby) -> Result<(), Error> {
    Lazy::force(&RUSTYBARS_ERROR, ruby);
    Lazy::force(&RUSTYBARS_JSON_ERROR, ruby);
    Lazy::force(&RUSTYBARS_COMPILE_ERROR, ruby);
    Lazy::force(&RUSTYBARS_RENDER_ERROR, ruby);
    Ok(())
}
