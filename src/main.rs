use symphoprobe::init::{get_source_file, probe_audio};
use symphoprobe::output;

fn main() -> Result<(), String> {
    let source_file = get_source_file()?;
    let properties = probe_audio(source_file);

    output::core(properties);

    Ok(())
}
