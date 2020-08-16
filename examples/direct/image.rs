use notcurses::{Align, Blitter, Direct, Error, Scale};

fn main() -> Result<(), Error> {
    let mut ncd = Direct::new()?;

    if !ncd.can_open_images() {
        println!("Sorry, your terminal doesn't support images.");
        std::process::exit(1);
    }

    let img = abspath("examples/res/image-16x16.png");

    // Alignment
    ncd.render_image(&img, Align::Left, Blitter::_1x1, Scale::None)?;
    ncd.render_image(&img, Align::Center, Blitter::_1x1, Scale::None)?;
    ncd.render_image(&img, Align::Right, Blitter::_1x1, Scale::None)?;

    // Scale
    ncd.render_image(&img, Align::Center, Blitter::_1x1, Scale::Stretch)?;
    ncd.render_image(&img, Align::Center, Blitter::_1x1, Scale::Scale)?;

    // Blitter
    //
    // NOTE: Blitter::_1x1x4 & Blitter::_4x1 are still unimplemented,
    // they both ought be falling back to 1x1 with a top half.
    ncd.render_image(&img, Align::Center, Blitter::_1x1, Scale::None)?;
    ncd.render_image(&img, Align::Center, Blitter::_1x1x4, Scale::None)?;  // WIP
    ncd.render_image(&img, Align::Center, Blitter::_2x1, Scale::None)?; 
    ncd.render_image(&img, Align::Center, Blitter::_2x2, Scale::None)?;
    ncd.render_image(&img, Align::Center, Blitter::_4x1, Scale::None)?;    // WIP
    ncd.render_image(&img, Align::Center, Blitter::_8x1, Scale::None)?;    // BUG: doesn't show
    ncd.render_image(&img, Align::Center, Blitter::Braille, Scale::None)?;

    Ok(())
}

/// Return an absolute path to a file
///
/// file = the relative path from the root
pub fn abspath(file: &str) -> String {
    let mut dir = std::env::current_exe().unwrap().to_path_buf();
    for _ in 0..4 {
        dir.pop();
    }
    dir.push(file);
    format!("{}", dir.display())
}
