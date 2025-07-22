mod fibonacci;
// mod median_mode;
mod temp_conversion;
mod xmas_song;

const RUN_TEMP_COVERSION: bool = false;
const RUN_FIB_TEST: bool = true;
const RUN_XMAS_SONG: bool = false;
// const RUN_MEDIAN_MODE: bool = true;
fn main() {
    // // median, mode
    // if RUN_MEDIAN_MODE {
    //     median_mode::test_median();

    //     median_mode::test_mode();
    // }
    // TEMPERATURE CONVERSION
    if RUN_TEMP_COVERSION {
        temp_conversion::test_temp_conversion();
    }
    // FIBONACCI TEST
    if RUN_FIB_TEST {
        fibonacci::test_fibonnaci();
    }

    // no test for this one, just print out the lyrics and look
    if RUN_XMAS_SONG {
        xmas_song::print_lyrics_xmas()
    }
}
