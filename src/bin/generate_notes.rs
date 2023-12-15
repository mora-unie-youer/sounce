use itertools::Itertools;

/// This program is made for generation of scale I came up with 2 years ago
///
/// The main reason why this is needed is because I wanted note system with
/// the idea of 0-th octave would be close to 16Hz, which can be heard by people.
///
/// And with idea I wanted to calculate which note would be 27.5 Hz
/// (This note must be in 0-th octave)
fn main() {
    // Frequency of some note in scale, which I call "root note"
    const ROOT_FREQ: f32 = 27.5;

    // To find position of "root note" we have to solve "27.5 * 2^(-n/53) = 16"
    // This will give us "offset" of "root note" in a scale
    let root_offset = (53. * (ROOT_FREQ / 16.).log2()) as isize;
    println!("Root note has offset: {root_offset}");

    // Now when we found offset, we can generate the whole "root" scale
    let root_scale = (0..53)
        .map(|i| ROOT_FREQ * 2f32.powf((i - root_offset) as f32 / 53.))
        .collect_vec();
    println!("Root scale: {root_scale:?}");

    // Now with this information you can get any frequency in my scale.
    // Of course, it is "unlimited", starting from here, but it's obvious that
    // you should stop when you reach 20kHz, that's we there are only 10 octaves

    // Oh, also you may need some names for notes, so...
    #[rustfmt::skip]
    const NOTES: [&str; 53] = [
        // "Small" notes
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m",
        "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",

        // Median note
        "=",

        // "Big" notes
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M",
        "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
    ];

    // Printing note names and their frequencies
    println!();
    for (note, freq) in NOTES.into_iter().zip(root_scale) {
        println!("{note} -> {freq} Hz");
    }
}
