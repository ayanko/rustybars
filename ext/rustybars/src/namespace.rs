use magnus::{value::Lazy, Error, RModule, Ruby};

pub static RUSTYBARS_MODULE: Lazy<RModule> = Lazy::new(|ruby| {
    ruby
        .define_module("Rustybars")
        .unwrap()
});

pub fn init(ruby: &Ruby) -> Result<(), Error> {
    Lazy::force(&RUSTYBARS_MODULE, ruby);
    Ok(())
}
