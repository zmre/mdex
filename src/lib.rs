#[macro_use]
extern crate log;
extern crate anyhow;
extern crate serde;
extern crate simplelog;

use anyhow::Result;
use input::Input;
use std::io::BufRead;
use std::io::Read;
use std::io::{stdout, Write};
use termimad::rgb;
use termimad::Alignment;
use termimad::MadSkin;

pub mod input;
pub mod output;

pub fn pretty_print_md(input: &mut Input) -> Result<()> {
    let mut buffer = String::new();
    input.read_to_string(&mut buffer)?;

    let mut skin = MadSkin::default();
    skin.set_headers_fg(rgb(255, 255, 0));
    skin.inline_code.set_fg(rgb(208, 85, 146));
    //skin.italic.add_attr(Italic);
    //skin.bold.set_fg(Yellow);
    //skin.italic.set_fgbg(Magenta, rgb(30, 30, 40));
    // skin.headers.align = Alignment::Left;
    // skin.paragraph.align = Alignment::Center;
    skin.table.align = Alignment::Center;

    //let markdown = format!("{}", &content);

    println!("\n");
    println!("{}", skin.term_text(&buffer));
    println!("\n");
    //termimad::print_inline("**some** *nested **style*** and `some(code)`");
    Ok(())
}
