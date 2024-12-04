use symphonia::core::codecs::CodecParameters;
use symphonia::core::units::Time;
use symphonia::default::get_codecs;
use crate::print::{debug_option, display, display_option};

/// Print all properties in a given audio parameter struct.
/// 
/// Raw properties, no interpretation beyond the choice of Display/Debug and
/// bringing values out of Options.
/// 
/// This function mainly exists for the sake of documentation; calling it may
/// not be very useful.
pub fn full(properties: CodecParameters) {
    // needs `symphonia::codecs::CodecRegistry::get_codec` to be legible
    display("codec", properties.codec);
    display_option("sample rate", properties.sample_rate);
    // just the inverse of the sample rate I think but VERY useful, see below
    display_option("timebase", properties.time_base);
    // pass to `TimeBase::calc_time` to get the duration
    display_option("number of frames", properties.n_frames);

    // Debug is more legible than Display despite the format
    // call `count` on the underlying value to get an actual number
    debug_option("channels", properties.channels);
    // often None
    debug_option("channel layout", properties.channel_layout);
    
    // often None
    debug_option("sample format", properties.sample_format);
    display_option("bits per sample", properties.bits_per_sample);
    // often None
    display_option("bits per coded sample", properties.bits_per_coded_sample);

    // often None
    display_option("padding", properties.padding);
    // often None
    display_option("delay", properties.delay);
    // often 0
    display("start timestamp", properties.start_ts);

    // often None
    display_option("frames per block", properties.frames_per_block);
    // often None
    display_option("max frames per packet", properties.max_frames_per_packet);
    // probably doesn't have many use cases outside of Symphonia internals
    display("packet data integrity", properties.packet_data_integrity);
    // literally just a bunch of bytes, probably some ultra nerd shit
    debug_option("extra data", properties.extra_data);
}

/// Print the most useful information about an audio file.
/// 
/// "Most useful" is defined in terms of what I want to do. This includes
/// the following:
/// - the name of the codec
/// - sample rate
/// - timebase
/// - number of frames
/// - number of channels
pub fn core(properties: CodecParameters) {
    if let Some(codec_info) = get_codecs().get_codec(properties.codec) {
        println!("codec: {} ({})", codec_info.short_name, codec_info.long_name);
    } else {
        println!("unidentified codec");
    }

    if let Some(channels) = properties.channels {
        display("channels", channels.count());
    }

    debug_option("duration (s)", get_duration(&properties));

    display_option("sample rate", properties.sample_rate);
    display_option("timebase", properties.time_base);
    display_option("number of frames", properties.n_frames);
    display_option("number of samples", get_n_samples(&properties));
}

/// Find the duration of an audio file if possible.
fn get_duration(properties: &CodecParameters) -> Option<Time> {
    if let Some(timebase) = properties.time_base {
        if let Some(n_frames) = properties.n_frames {
            Some(timebase.calc_time(n_frames))
        } else {
            None
        }
    } else {
        None
    }
}

/// Find the total number of samples in an audio file if possible.
fn get_n_samples(properties: &CodecParameters) -> Option<u64> {
    if let Some(channels) = properties.channels {
        if let Some(n_frames) = properties.n_frames {
            Some(n_frames * channels.count() as u64)
        } else {
            None
        }
    } else {
        None
    }
}
