use symphoprobe::init;

fn main() -> Result<(), String> {
    let source_file = init::get_source_file()?;
    let properties = init::probe_audio(source_file);
    let output_fn = init::select_output()?;

    output_fn(&properties);

    Ok(())
}
