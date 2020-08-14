#![allow(unused_imports)]

use notcurses::{NcDirect, NcError, NcStyle};
use libnotcurses_sys as nc;

fn main() -> Result<(), NcError> {
    let mut ncd = NcDirect::new()?;


    // INFO

    println!("Terminal rows={0}, cols={1}", ncd.dim_y(), ncd.dim_x());

    println!("Can open images: {0}\nCan UTF-8: {1}",
        ncd.can_open_images(), ncd.can_utf8());

    println!("palette_size: {}", ncd.palette_size());


    // TEXT & STYLE

    ncd.putstr(0, "\nSingle styles:\n")?;
    ncd.putstr(0, "[DEFAULT]")?;

    ncd.styles_on(NcStyle::Dim)?;
    ncd.putstr(0, "[DIM]")?;
    ncd.styles_off(NcStyle::Dim)?;

    ncd.styles_on(NcStyle::Underline)?;
    ncd.putstr(0, "[UNDERLINE]")?;
    ncd.styles_off(NcStyle::Underline)?;

    ncd.styles_on(NcStyle::Bold)?;
    ncd.putstr(0, "[BOLD]")?;
    ncd.styles_off(NcStyle::Bold)?;

    ncd.styles_on(NcStyle::Reverse)?;
    ncd.putstr(0, "[REVERSE]")?;
    ncd.styles_off(NcStyle::Reverse)?;

    ncd.styles_on(NcStyle::Blink)?;
    ncd.putstr(0, "[BLINK]")?;
    ncd.styles_off(NcStyle::Blink)?;

    ncd.styles_on(NcStyle::Invis)?;
    ncd.putstr(0, "[INVIS]")?;
    ncd.styles_off(NcStyle::Invis)?;

    ncd.styles_on(NcStyle::Italic)?;
    ncd.putstr(0, "[ITALIC]")?;
    ncd.styles_off(NcStyle::Italic)?;

    ncd.styles_on(NcStyle::Protect)?;
    ncd.putstr(0, "[PROTECT]")?;
    ncd.styles_off(NcStyle::Protect)?;

    ncd.styles_on(NcStyle::Standout)?;
    ncd.putstr(0, "[STANDOUT]")?;
    ncd.styles_off(NcStyle::Standout)?;


    ncd.putstr(0, "\nJoint styles:\n")?;

    ncd.putstr(0, "[")?;
    ncd.putstr(0, "DEFAULT ")?;
    ncd.styles_on(NcStyle::Dim)?;
    ncd.putstr(0, " DIM ")?;
    ncd.styles_on(NcStyle::Underline)?;
    ncd.putstr(0, " UNDERLINE ")?;
    ncd.styles_on(NcStyle::Bold)?;
    ncd.putstr(0, " BOLD ")?;
    ncd.styles_on(NcStyle::Reverse)?;
    ncd.putstr(0, " REVERSE ")?;
    ncd.styles_on(NcStyle::Blink)?;
    ncd.putstr(0, " BLINK ")?;
    ncd.styles_off_all()?;  // FIXME makes it blink!!
    ncd.putstr(0, "]")?;
    ncd.putstr(0, "\nshouldn't be italic, lol")?;

    // WIP----------------------- ↓

    // CURSOR & TEXT

    //println!("Cursor position: {:?}", ncd.cursor_yx()?);
    // ncd.cursor_move_yx(200,100)?;
    // ncd.cursor_move_yx(yx.0, yx.1)?;
    // ncd.cursor_disable()?;
    // ncd.cursor_enable()?;

    // COLORS & TEXT (WIP)


    // ncd.bg(0x00FF00)?; // FIXME: colors don't seem to work
    // ncd.fg(0xFF0000)?;
    // println!("hello colors?");
    //
    // // TODO: implement Channel & Channels (or ChannelPair)
    // ncd.putstr(nc::channels_combine(0x004400, 0x1100dd), "hello colors 2?")?; // FIXME: colors don't seem to work


    // ncd.clear()?;

    Ok(())
}
