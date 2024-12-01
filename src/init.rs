
use std::fs::File;
use symphonia::default::get_probe;
use symphonia::core::{
    codecs::{CODEC_TYPE_NULL, CodecParameters},
    formats::FormatOptions,
    io::MediaSourceStream,
    meta::{MetadataOptions, Limit},
    probe::Hint
};


/// Get a handle for the file to be probed from CLI arguments.
/// 
/// # Errors
/// 
/// Returns an error if no file is specified or the specified file cannot be
/// opened.
pub fn get_source_file() -> Result<File, String> {
    let args: Vec<String> = std::env::args().collect();
    let input_path = match args.get(1) {
        Some(path) => path,
        None => {
            return Err("specify a file to probe".into())
        }
    };
    match std::fs::File::open(&input_path) {
        Ok(file) => Ok(file),
        Err(err) => {
            return Err(format!("failed to open '{}': {}", input_path, err))
        }
    }
}

/// Do all the ugly setup and probe the given file for audio parameters.
pub fn probe_audio(source_file: File) -> CodecParameters {
    let mss = MediaSourceStream::new(
        Box::new(source_file),
        Default::default()
    );
    let fmt_opts: FormatOptions = Default::default();
    let meta_opts = MetadataOptions {
        limit_metadata_bytes: Limit::Maximum(0),
        limit_visual_bytes: Limit::Maximum(0)
    };
    
    // trust me this works
    get_probe()
        .format(
            &Hint::new(),
            mss,
            &fmt_opts,
            &meta_opts
        )
        .expect("unsupported format")
        .format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .expect("no supported audio tracks")
        .codec_params
        .clone()
}
