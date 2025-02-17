use crate::{image_data::ImageData, render_char_to_png::str_to_png, AsciiImageOptions};

use super::render_char_to_png::{str_to_transparent_png, ColoredStr};
use ab_glyph::FontRef;
use image::open;
use rascii_art::{render_image_to, RenderOptions};
use regex::Regex;

// read bytes for the font
const FONT_BYTES: &[u8] = include_bytes!("../fonts/UbuntuMono.ttf");

pub fn parse_ascii(
    input_file_name: &str,
    rascii_options: &RenderOptions,
    ascii_image_options: &AsciiImageOptions,
) -> Vec<Vec<ImageData>> {
    // set up font and font sizes for rendering
    let font = FontRef::try_from_slice(FONT_BYTES).unwrap();
    let font_size = ascii_image_options.get_font_size();

    // render the ascii text with RASCII
    let mut ascii_text = String::new();
    let loaded_img =
        open(input_file_name).expect(format!("Could not open file ({})", input_file_name).as_str());
    render_image_to(&loaded_img, &mut ascii_text, &rascii_options)
        .expect("Error converting image to ASCII");

    // contains lines of images
    // starting at 0 is the top, first line of the vector
    // inside an inner vec, 0 starts at the leftmost character of the line
    let mut lines = vec![];

    // read every line in the file
    for line in ascii_text.lines() {
        let mut char_images = vec![];

        // we need to find each character that we are going to write
        // we assume that there's only one character for each color
        let pattern = r"\[38;2;([0-9]+);([0-9]+);([0-9]+)m(.)";
        let _control_char = '\u{1b}'; // another way to represent the ansi escape character `\033`
        let re = Regex::new(pattern)
            .expect(format!("Error creating regex pattern ({})", pattern).as_str());

        // create the image for this character
        for (full_str, [r, g, b, the_str]) in re.captures_iter(&line).map(|c| c.extract()) {
            let red = r.parse::<u8>().expect(
                format!(
                    "Error parsing red from string: ({}), full string: ({}). Improper encoding?",
                    r, full_str
                )
                .as_str(),
            );
            let green = g.parse::<u8>().expect(
                format!(
                    "Error parsing green from string: ({}), full string: ({}). Improper encoding?",
                    g, full_str
                )
                .as_str(),
            );
            let blue = b.parse::<u8>().expect(
                format!(
                    "Error parsing blue from string ({}), full string ({}). Improper encoding?",
                    b, full_str
                )
                .as_str(),
            );

            let generated_png = {
                if the_str.trim().is_empty() {
                    // create a transparent png for a space
                    str_to_transparent_png(font_size)
                } else {
                    // render the actual text if it's not empty
                    let colored = ColoredStr {
                        red,
                        green,
                        blue,
                        string: String::from(the_str),
                    };

                    str_to_png(colored, &font, font_size)
                        .expect(format!("Could not convert str ({}) to PNG", the_str).as_str())
                }
            };

            char_images.push(generated_png);
        }

        lines.push(char_images);
    }

    lines
}
