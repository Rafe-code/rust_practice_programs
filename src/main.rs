mod blog_oop;
mod blog_type;
mod fibonacci;
mod median_mode;
mod org_builder;
mod pig_latin;
mod temp_conversion;
mod xmas_song;
const RUN_TEMP_COVERSION: bool = false;
const RUN_FIB_TEST: bool = false;
const RUN_XMAS_SONG: bool = false;
const RUN_MEDIAN_MODE: bool = false;
const RUN_PIG_LATIN: bool = false;
const RUN_ORG_BUILDER: bool = false;
fn main() {
    // org builder
    if RUN_ORG_BUILDER {
        let mut org = org_builder::Organisation::new();
        org.add_employee("karen", "software");
        // should see ["karen"]
        org.print_dep("software");
        org.print_dep("HR");
        org.add_employee("Bea", "HR");
        org.add_employee("Andy", "HR");
        // should see: HR: ["Andy", "Bea"]
        //             software: ["karen"]
        org.print_all();
    }

    //  pig latin
    if RUN_PIG_LATIN {
        pig_latin::test_pig_latin();
    }
    // median, mode
    if RUN_MEDIAN_MODE {
        median_mode::test_median();

        median_mode::test_mode();
    }
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
        _ = xmas_song::print_lyrics_xmas();
    }
}
