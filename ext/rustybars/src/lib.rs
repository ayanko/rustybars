mod namespace;
mod errors;
mod engine;
mod template;

#[magnus::init]
fn init(ruby: &magnus::Ruby) -> Result<(), magnus::Error> {
    namespace::init(ruby)?;
    errors::init(ruby)?;
    engine::init(ruby)?;
    template::init(ruby)?;
    Ok(())
}

