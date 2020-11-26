#![allow(unused_imports)]

use notcurses::{sys, Align, DirectMode, Error, Style};

fn main() -> Result<(), Error> {
    let mut ncd = DirectMode::new()?;

    // INFO

    let t_rows = ncd.rows();
    let t_cols = ncd.cols();
    println!("Terminal rows={0}, cols={1}", t_rows, t_cols);

    println!(
        "Can open images: {0}\nCan UTF-8: {1}",
        ncd.can_open_images(),
        ncd.can_utf8()
    );

    println!("palette_size: {}", ncd.palette_size());

    // TEXT & STYLE

    let stylesv = vec![
        ("[DIM]", Style::Dim),
        ("[UNDERLINE]", Style::Underline),
        ("[ITALIC]", Style::Italic),
        ("[BOLD]", Style::Bold),
        ("[STRUCK]", Style::Struck),
        ("[REVERSE]", Style::Reverse),
        ("[BLINK]", Style::Blink),
        ("[INVIS]", Style::Invis),
        ("[PROTECT]", Style::Protect),
        ("[STANDOUT]", Style::Standout),
    ];

    ncd.print_colored(0, "\nSingle styles:\n")?;

    ncd.print_colored(0, "[DEFAULT]")?;
    for (label, style) in stylesv.iter() {
        ncd.styles_on(*style)?;
        ncd.print_colored(0, label)?;
        ncd.styles_off(*style)?;
    }

    ncd.print_colored(0, "\nJoint styles:\n")?;

    ncd.print_colored(0, "[DEFAULT ")?;
    for (label, style) in stylesv.iter() {
        ncd.styles_on(*style)?;
        ncd.print_colored(
            0,
            &label
                .chars()
                .map(|c| match c {
                    '[' | ']' => ' ',
                    _ => c,
                })
                .collect::<String>(),
        )?;
        if let Style::Blink = style {
            break;
        }
    }
    ncd.styles_off_all()?;
    ncd.print_colored(0, "]")?;

    // TEXT mixing Rust's print!() & println!() and notcurses' print_colored() & print()
    //
    ncd.print_colored(0, "\n\n1")?;
    println!("2 < instead of printing this concatenated AFTER, it appears BEFORE 1");

    ncd.print_colored(0, "\n\n1 \n")?;
    println!("2 < it does work (better) with a `\\n` after 1");

    // TODO: more tests with styles_set & bold+italic
    //
    //ncd.styles_off(Style::Bold)?;
    //ncd.styles_on(Style::Italic)?;

    // COLORS & TEXT (WIP)

    ncd.bg(0x00FF00 as u32)?; // FIXME: colors don't seem to work
    ncd.fg(0xFF0000 as u32)?;
    println!("\nhello colors? (investigate)");
    ncd.print_colored(
        sys::channels_combine(0xFF008800, 0xFFBB0099),
        "hello colors 2",
    )?;
    ncd.print_colored(0, "...")?;

    // TODO: should be able to use print!() & println!()
    // ncd.clear()?;
    // ncd.print_aligned(0, Align::Center, "PRINTED")?;
    // ncd.print_aligned(40, Align::Left, "PRINTED")?;
    // ncd.print_aligned(5, Align::Right, "PRINTED")?;

    // WIP----------------------- ↓

    // CURSOR & TEXT

    // println!("Cursor position: {:?}", ncd.cursor_yx()?);
    // ncd.cursor_move_yx(200,100)?;
    // ncd.cursor_move_yx(yx.0, yx.1)?;
    // ncd.cursor_disable()?;
    // ncd.cursor_enable()?;

    Ok(())
}
