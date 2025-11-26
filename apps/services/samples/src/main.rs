use ffmpeg_next::format;

fn main() {
    let format_context = format::input("samples/sample.mp4").unwrap();

    println!("BitRate: {}", format_context.bit_rate());
    println!("Number of chapters: {}", format_context.chapters().len());
    println!("Number of streams: {}", format_context.streams().len());

    let duration = format_context.duration();
    println!("Duration: {} microseconds", duration);

    let format = format_context.format();
    println!("Format name: {}", format.name());
    println!("Format description: {}", format.description());
    println!("Format extentions: {:?}", format.extensions());
    println!("Format mime types: {:?}", format.mime_types());

    let metadata = format_context.metadata();
    for (key, value) in metadata.iter() {
        println!("Metadata: {}: {}", key, value);
    }

    for stream in format_context.streams() {
        println!("Stream {}:", stream.index());
        println!("  Time Base: {}/{}", stream.time_base().numerator(), stream.time_base().denominator());
        println!("  Duration: {}", stream.duration());
        println!("  Frames: {}", stream.frames());
        println!("  Rate: {}", stream.rate());
        println!("  Disposition: {:?}", stream.disposition());
        println!("  Avg Frame Rate: {:?}", stream.avg_frame_rate());
        println!("  Start Time: {}", stream.start_time());
        println!("  Discard: {:?}", stream.discard());

        for (key, value) in stream.metadata().iter() {
            println!("  Metadata: {}: {}", key, value);
        }

        let parameters = stream.parameters();
        println!("  Codec ID Name: {}", parameters.id().name());
        println!("  Codec Type: {:?}", parameters.medium());
        let width = unsafe { parameters.as_ptr().as_ref().and_then(|p| Some(p.width)).unwrap_or(0) };
        println!("  Width: {}", width);
        let height = unsafe { parameters.as_ptr().as_ref().and_then(|p| Some(p.height)).unwrap_or(0) };
        println!("  Height: {}", height);
        let bit_rate = unsafe { parameters.as_ptr().as_ref().and_then(|p| Some(p.bit_rate)).unwrap_or(0) };
        println!("  Bit Rate: {}", bit_rate);
        let pixel_format = unsafe { parameters.as_ptr().as_ref().and_then(|p| Some(p.format)).unwrap_or(-1) };
        println!("  Pixel Format: {}", pixel_format);

        

        let side_data_iter = stream.side_data();
        for side_data in side_data_iter {
            println!("  Side Data Type: {:?}", side_data.kind());
        }

        
        
    };
}
