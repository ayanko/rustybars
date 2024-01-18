use magnus::{function, exception, prelude::*, value::Lazy, Error, Ruby, RModule};
use handlebars::Handlebars;
use serde_json;


static RUSTYBARS: Lazy<RModule> = Lazy::new(|ruby| {
    ruby
        .define_module("Rustybars")
        .unwrap()
});

static RUSTYBARS_ERROR: Lazy<exception::ExceptionClass> = Lazy::new(|ruby| {
    ruby
        .get_inner(&RUSTYBARS)
        .define_error("Error", ruby.exception_standard_error())
        .unwrap()
});

static RUSTYBARS_DATA_ERROR: Lazy<exception::ExceptionClass> = Lazy::new(|ruby| {
    ruby
        .get_inner(&RUSTYBARS)
        .define_error("DataError", ruby.get_inner(&RUSTYBARS_ERROR))
        .unwrap()
});

static RUSTYBARS_TEMPLATE_ERROR: Lazy<exception::ExceptionClass> = Lazy::new(|ruby| {
    ruby
        .get_inner(&RUSTYBARS)
        .define_error("TemplateError", ruby.get_inner(&RUSTYBARS_ERROR))
        .unwrap()
});


fn render(ruby: &Ruby, template: String, data: String) -> Result<String, Error> {
    let variables: serde_json::Value = serde_json::from_str(&data).map_err(|e| {
        Error::new(
            ruby.get_inner(&RUSTYBARS_DATA_ERROR),
            e.to_string(),
        )
    })?;

    let hbs = Handlebars::new();
    hbs.render_template(&template, &variables).map_err(|e| {
        Error::new(
            ruby.get_inner(&RUSTYBARS_TEMPLATE_ERROR),
            e.to_string()
        )
    })
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    Lazy::force(&RUSTYBARS_DATA_ERROR, ruby);
    Lazy::force(&RUSTYBARS_TEMPLATE_ERROR, ruby);

    ruby
        .get_inner(&RUSTYBARS)
        .define_singleton_method("render", function!(render, 2))?;

    Ok(())
}
