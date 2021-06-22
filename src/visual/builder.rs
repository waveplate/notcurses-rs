//!

// TODO:
// - separate the Scale from the VisualBuilder _plane finishers?
// - add matching set_ methods to Visual

use crate::sys::{self, NcPlane, NcVisual, NcVisualOptions};
use crate::{
    Align, Blitter, Dimension, NotcursesError, NotcursesResult as Result, Plane, Scale, Visual,
};

/// A [`Visual`] builder.
#[derive(Default)] // TEMP do manually
pub struct VisualBuilder<'a, 'b> {
    ncvisual: Option<&'a mut NcVisual>,

    plane: Option<&'a mut Plane<'b>>,
    scale: Option<Scale>,

    x: Dimension,
    y: Dimension,

    halign: Option<Align>,
    valign: Option<Align>,

    begx: Dimension,
    begy: Dimension,
    lenx: Dimension,
    leny: Dimension,

    blitter: Blitter,

    flags: u32,
    transcolor: u32, // NcRgba,
}

impl<'a, 'b> VisualBuilder<'a, 'b> {
    /// Prepares a `Visual` based off RGBA content in memory at `rgba`.
    #[allow(clippy::wrong_self_convention)]
    pub fn from_rgba(mut self, rgba: &[u8], cols: Dimension, rows: Dimension) -> Result<Self> {
        self.ncvisual = Some(NcVisual::from_rgba(rgba, rows, cols * 4, cols)?);
        Ok(self)
    }

    /// Prepares a `Visual` based off RGB content in memory at `rgb`, providing
    /// the alpha to assign to all the pixels.
    #[allow(clippy::wrong_self_convention)]
    pub fn from_rgb(
        mut self,
        rgb: &[u8],
        cols: Dimension,
        rows: Dimension,
        alpha: u8,
    ) -> Result<Self> {
        self.ncvisual = Some(NcVisual::from_rgb_packed(rgb, rows, cols * 3, cols, alpha)?);
        Ok(self)
    }

    /// Prepares a `Visual` based off RGBX content in memory at `rgbx`,
    /// overriding the *alpha* byte *X* for all the pixels.
    #[allow(clippy::wrong_self_convention)]
    pub fn from_rgbx(
        mut self,
        rgbx: &[u8],
        cols: Dimension,
        rows: Dimension,
        alpha: u8,
    ) -> Result<Self> {
        self.ncvisual = Some(NcVisual::from_rgb_loose(rgbx, rows, cols * 4, cols, alpha)?);
        Ok(self)
    }

    /// Prepares a `Visual` based off BGRA content in memory at `bgra`.
    ///
    /// This is slower than [`from_rgba`][VisualBuilder#method.rgba], since it
    /// has to convert the pixels to the rgba format used internally.
    #[allow(clippy::wrong_self_convention)]
    pub fn from_bgra(mut self, bgra: &[u8], cols: Dimension, rows: Dimension) -> Result<Self> {
        self.ncvisual = Some(NcVisual::from_bgra(bgra, rows, cols * 4, cols)?);
        Ok(self)
    }

    /// Prepares a `Visual` from a `file`, extracts the codec and paramenters
    /// and decodes the first image to memory.
    ///
    /// It needs notcurses to be compiled with multimedia capabilities.
    #[allow(clippy::wrong_self_convention)]
    pub fn from_file(mut self, file: &str) -> Result<Self> {
        self.ncvisual = Some(NcVisual::from_file(file)?);
        Ok(self)
    }

    /// Prepares a `Visual` from a [`Plane`].
    ///
    /// The plane may contain only spaces, half blocks, and full blocks.
    /// This will be checked, and any other glyph will result in an error.
    ///
    /// This function exists so that planes can be subjected to NcVisual transformations.
    /// If possible, it's better to create the ncvisual from memory using
    /// [from_rgba][Visual#method.from_rgba].
    ///
    // RETHINK whether to name it y1,x1 or leny, lenx
    #[allow(clippy::wrong_self_convention)]
    pub fn from_plane(
        mut self,
        plane: &Plane<'b>,
        blitter: Blitter,
        x0: Dimension,
        y0: Dimension,
        x1: Dimension,
        y1: Dimension,
    ) -> Result<Self> {
        self.ncvisual = Some(NcVisual::from_plane(
            plane.raw,
            blitter.into(),
            y0,
            x0,
            y1,
            x1,
        )?);
        Ok(self)
    }

    // MAYBE
    // /// Creates a `VisualBuilder` from an already configured [`Visual`].
    // pub fn from_visual(visual: Visual<'a>) -> Self {
    //     // if let visua
    //     // let plane =
    //     Self {
    //         //ncvisual: Some(visual.raw),
    //         ..Default::default()
    //     }
    // }

    /// The X and Y coordinates.
    ///
    /// - If you don't provide a pre-existing `Plane`, they will be relative to
    ///   the terminal size.
    /// - If you do provide a pre-existing `Plane` via the
    ///   [`plane`][VisualBuilder#method.plane] method,
    ///   they indicate where in that `Plane` to start drawing the `Visual`.
    /// - If you provide a [`parent_plane`][VisualBuilder#method.parent_plane]
    ///   they are interpreted relative to that `Plane`.
    ///
    /// This will override any relative [`halign`][VisualBuilder#method.halign]
    /// and [`valign`][VisualBuilder#method.halign] positioning.
    pub fn xy(mut self, x: Dimension, y: Dimension) -> Self {
        self.x = x;
        self.y = y;
        self.flags &= !sys::NCVISUAL_OPTION_HORALIGNED;
        self.flags &= !sys::NCVISUAL_OPTION_VERALIGNED;
        self
    }

    /// Set the `x` cooordinate.
    ///
    /// This will override any relative [`halign`][VisualBuilder#method.halign]
    /// vertical positioning.
    pub fn x(mut self, x: Dimension) -> Self {
        self.x = x;
        self.halign = None;
        self.flags &= !sys::NCVISUAL_OPTION_HORALIGNED;
        self
    }

    /// Set the `y` cooordinate.
    ///
    /// This will override any relative [`valign`][VisualBuilder#method.valign]
    /// vertical positioning.
    pub fn y(mut self, y: Dimension) -> Self {
        self.y = y;
        self.valign = None;
        self.flags &= !sys::NCVISUAL_OPTION_VERALIGNED;
        self
    }

    /// The `Visual` will be horizontally aligned.
    ///
    /// This will override any absolute [`y`][VisualBuilder#method.y] positioning.
    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self.flags |= sys::NCVISUAL_OPTION_HORALIGNED;
        self
    }

    /// The `Visual` will be horizontally aligned.
    ///
    /// This will override any absolute [`y`][VisualBuilder#method.y] positioning.
    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self.flags |= sys::NCVISUAL_OPTION_VERALIGNED;
        self
    }

    ///
    /// - `x0`,`y0` are the origin coordinates of the rendering section
    /// - `x1`,`y1` are the size of the rendering section
    pub fn section(mut self, x0: Dimension, y0: Dimension, x1: Dimension, y1: Dimension) -> Self {
        self.begx = x0;
        self.begy = y0;
        self.lenx = x1;
        self.leny = y1;
        self
    }

    /// Sets the blitter
    pub fn blitter(mut self, blitter: Blitter) -> Self {
        self.blitter = blitter;
        self
    }

    /// Sets whether the scaling should be done with interpolation or not.
    ///
    /// The default is to interpolate.
    pub fn interpolate(mut self, interpolate: bool) -> Self {
        if interpolate {
            self.flags &= !sys::NCVISUAL_OPTION_NOINTERPOLATE;
        } else {
            self.flags |= sys::NCVISUAL_OPTION_NOINTERPOLATE;
        }
        self
    }

    // BUILD FINISHERS

    // NOTE This is not implemented, to avoid unnecessary complexity, including
    // making sure that render() only returns the plane if it's a new child.
    //
    // /// Finishes the build returning a `Visual` configured to be rendered
    // /// in a child [`Plane`] of the one provided.
    // pub fn into_plane_child(mut self, plane: &mut Plane<'b>, scale: Scale) -> Result<Visual<'a>> {
    //     if self.ncvisual.is_some() {
    //         self.scale = Some(scale);
    //         self.flags |= sys::NCVISUAL_OPTION_CHILDPLANE;
    //         Ok(Visual {
    //             options: self.assemble_options_with_plane(plane.raw),
    //             raw: self.ncvisual.unwrap(),
    //         })
    //     } else {
    //         Err(NotcursesError::BuildIncomplete(
    //             "It's necessary to prepare the Visual
    //             first by calling one of the `from_*` methods."
    //                 .into(),
    //         ))
    //     }
    // }

    /// Finishes the build returning a `Visual` configured to be rendered
    /// in the provided [`Plane`], with the provided [`Scale`].
    pub fn into_plane(mut self, plane: &mut Plane<'b>, scale: Scale) -> Result<Visual<'a>> {
        if self.ncvisual.is_some() {
            self.scale = Some(scale);
            self.flags &= !sys::NCVISUAL_OPTION_CHILDPLANE;
            Ok(Visual {
                options: self.assemble_options_with_plane(plane.raw),
                raw: self.ncvisual.unwrap(),
            })
        } else {
            Err(NotcursesError::BuildIncomplete(
                "It's necessary to prepare the Visual
                first by calling one of the `from_*` methods."
                    .into(),
            ))
        }
    }

    // // NOTE: related: https://github.com/dankamongmen/notcurses/issues/1462
    // //
    // /// Finishes the build returning a `Visual` configured to be rendered in the
    // /// provided [`Plane`], using the provided [`Scale`] mode for it.
    // pub fn new_plane(mut self, plane: &mut Plane<'b>, scale: Scale) -> Result<Visual<'a>> {
    //     if self.ncvisual.is_some() {
    //         self.scale = Some(scale);
    //         self.flags &= !sys::NCVISUAL_OPTION_CHILDPLANE;
    //
    //         Ok(Visual {
    //             options: self.assemble_options_without_plane(plane.raw),
    //             raw: self.ncvisual.unwrap(),
    //         })
    //     } else {
    //         Err(NotcursesError::BuildIncomplete("It's necessary to prepare the Visual
    //             first by calling any of the `from_*` methods.".into()))
    //     }
    // }

    // PRIVATE METHODS

    /// Prepares an `NcVisualOptions` from the relevant `VisualBuilder` fields.
    fn assemble_options_with_plane(&self, plane: &mut NcPlane) -> NcVisualOptions {
        // TODO if halign, valign…
        NcVisualOptions::with_plane(
            plane,
            self.scale.expect("Couldn't find a prepared scale.") as u32,
            self.y,
            self.x,
            self.begy,
            self.begx,
            self.leny,
            self.lenx,
            self.blitter.into(),
            self.flags,
            self.transcolor,
        )
    }

    /// Prepares an `NcVisualOptions` from the relevant `VisualBuilder` fields.
    fn assemble_options_without_plane(&self) -> NcVisualOptions {
        // TODO: if halign, valign…
        NcVisualOptions::without_plane(
            self.y,
            self.x,
            self.begy,
            self.begx,
            self.leny,
            self.lenx,
            self.blitter.into(),
            self.flags,
            self.transcolor,
        )
    }

    // MAYBE
    // /// Fills the relevant `VisualBuilder` fields from an `NcVisualOptions`.
    // fn disassemble_options(&mut self, &NcVisualOptions) {
    // }
}
