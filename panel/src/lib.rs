// Copyright (c) 2026 Ward-Beck Systems (Ward-Beck.com - Ward-Beck.ca). All rights reserved.
//! Ward-Beck module front panels for plugin editors.
//!
//! A module is laid out the way the rack drawing (art/concept-rack-gemini.svg) draws it: along its length
//! `x` (0 at the thumb-screw end … `len` at the WBS badge) and across its width `y` (0 … 159). On screen
//! the module stands upright as it does in the console: the WBS badge at the top, the lettering level.
//! Everything is drawn with egui shapes (no images), so it is sharp at any size.

pub use nih_plug_egui;
pub use nih_plug_egui::egui;
use egui::epaint::{Mesh, Vertex, WHITE_UV};
use egui::{Align2, Color32, FontId, Painter, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2};
use nih_plug::prelude::{BoolParam, Param, ParamSetter};

/// Panel width across the module in drawing units.
pub const WIDTH: f32 = 159.0;

// ---------------------------------------------------------------------------------------------
// Colours (from the rack drawing)

pub mod ink {
    use super::Color32;
    pub const RACK: Color32 = Color32::from_rgb(0x09, 0x0a, 0x0c);
    pub const LABEL: Color32 = Color32::from_rgb(0xe6, 0xea, 0xee);
    pub const SUB: Color32 = Color32::from_rgb(0xdc, 0xe1, 0xe6);
    pub const TINY: Color32 = Color32::from_rgb(0xcf, 0xd5, 0xdb);
    pub const TICK: Color32 = Color32::from_rgb(0xd5, 0xdb, 0xe0);
    pub const BRAND: Color32 = Color32::from_rgb(0xee, 0xf1, 0xf4);
    pub const DARK: Color32 = Color32::from_rgb(0x11, 0x13, 0x15);
}

/// Knob cap colours (Elma-style caps on the drawing).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cap {
    Green,
    Yellow,
    Amber,
    Red,
    Black,
    White,
}

impl Cap {
    /// (highlight, body, edge, pointer)
    fn colours(self) -> [Color32; 4] {
        let c = Color32::from_rgb;
        match self {
            Cap::Green => [c(0x34, 0xd0, 0x7d), c(0x16, 0x98, 0x53), c(0x0b, 0x5c, 0x31), Color32::WHITE],
            Cap::Yellow => [c(0xff, 0xea, 0x4d), c(0xe5, 0xb8, 0x00), c(0x9c, 0x7a, 0x00), c(0x18, 0x1b, 0x1e)],
            Cap::Amber => [c(0xff, 0xbf, 0x5a), c(0xe8, 0x8a, 0x00), c(0x9c, 0x5a, 0x00), c(0x18, 0x1b, 0x1e)],
            Cap::Red => [c(0xff, 0x66, 0x4d), c(0xd9, 0x32, 0x18), c(0x87, 0x19, 0x08), Color32::WHITE],
            Cap::Black => [c(0x48, 0x4d, 0x54), c(0x20, 0x23, 0x27), c(0x0e, 0x10, 0x12), Color32::WHITE],
            Cap::White => [c(0xff, 0xff, 0xff), c(0xdc, 0xe0, 0xe4), c(0x9d, 0xa3, 0xa9), c(0x18, 0x1b, 0x1e)],
        }
    }
}

/// Push-button cap colours.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Button {
    Red,
    White,
}

/// LED colours.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Led {
    Red,
    Green,
    Amber,
}

impl Led {
    /// (hot centre, body, dark rim)
    fn colours(self) -> [Color32; 3] {
        let c = Color32::from_rgb;
        match self {
            Led::Red => [c(0xff, 0x8a, 0x75), c(0xd4, 0x1e, 0x08), c(0x5e, 0x09, 0x00)],
            Led::Green => [c(0x75, 0xff, 0xad), c(0x12, 0xa8, 0x4e), c(0x05, 0x4a, 0x20)],
            Led::Amber => [c(0xff, 0xe8, 0x75), c(0xd9, 0x9b, 0x00), c(0x66, 0x47, 0x00)],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Size {
    Large,
    Small,
}

// ---------------------------------------------------------------------------------------------
// Drawing helpers

fn lerp(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    let m = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t).round() as u8;
    Color32::from_rgba_unmultiplied(m(a.r(), b.r()), m(a.g(), b.g()), m(a.b(), b.b()), m(a.a(), b.a()))
}

/// A circle filled like an SVG radial gradient lit from the top left: `stops` run from the
/// highlight (0) to the rim (1).
fn radial(p: &Painter, c: Pos2, r: f32, stops: &[(f32, Color32)], light: Vec2) {
    const N: usize = 18;
    for i in 0..N {
        let t = i as f32 / N as f32; // 0 = rim … 1 = centre
        let rr = r * (1.0 - t);
        let at = 1.0 - t;
        let mut col = stops[0].1;
        for w in stops.windows(2) {
            if at >= w[0].0 && at <= w[1].0 {
                col = lerp(w[0].1, w[1].1, (at - w[0].0) / (w[1].0 - w[0].0).max(1e-6));
            }
        }
        if at > stops[stops.len() - 1].0 {
            col = stops[stops.len() - 1].1;
        }
        p.circle_filled(c + light * r * t, rr, col);
    }
}

/// A rectangle with a gradient across it (`along_x` = colour changes left → right on screen).
fn gradient_rect(p: &Painter, r: Rect, stops: &[(f32, Color32)], along_x: bool) {
    let mut mesh = Mesh::default();
    for (i, (t, col)) in stops.iter().enumerate() {
        let (a, b) = if along_x {
            (Pos2::new(r.left() + r.width() * t, r.top()), Pos2::new(r.left() + r.width() * t, r.bottom()))
        } else {
            (Pos2::new(r.left(), r.top() + r.height() * t), Pos2::new(r.right(), r.top() + r.height() * t))
        };
        mesh.vertices.push(Vertex { pos: a, uv: WHITE_UV, color: *col });
        mesh.vertices.push(Vertex { pos: b, uv: WHITE_UV, color: *col });
        if i > 0 {
            let k = (i as u32) * 2;
            mesh.add_triangle(k - 2, k - 1, k);
            mesh.add_triangle(k - 1, k + 1, k);
        }
    }
    p.add(mesh);
}

fn shadow(p: &Painter, c: Pos2, r: f32, s: f32) {
    for i in 0..5 {
        let a = 38 - i * 7;
        p.circle_filled(c + Vec2::new(1.0, 2.5) * s, r + i as f32 * 0.9 * s, Color32::from_black_alpha(a as u8));
    }
}

// ---------------------------------------------------------------------------------------------
// The face of one module

/// One module's face on screen.
pub struct Face {
    /// Where the module's top-left corner (WBS end, y = 0 edge) sits on screen.
    pub origin: Pos2,
    /// Screen pixels per drawing unit.
    pub scale: f32,
    /// Module length in drawing units (702 on the rack drawing).
    pub len: f32,
    /// Module width across, in drawing units (159 on the rack drawing).
    pub width: f32,
}

impl Face {
    /// The face for a module of `len` drawing units, fitted to `rect` (upright).
    pub fn fit(rect: Rect, len: f32) -> Self {
        Self::fit_size(rect, WIDTH, len)
    }

    /// The face for a module `width` × `len` drawing units, fitted to `rect` (upright).
    pub fn fit_size(rect: Rect, width: f32, len: f32) -> Self {
        let scale = (rect.width() / width).min(rect.height() / len);
        let size = Vec2::new(width * scale, len * scale);
        let origin = rect.center() - size / 2.0;
        Face { origin, scale, len, width }
    }

    /// Drawing coordinates for a point given the other way: `across` from the left edge, `down` from
    /// the top (the WBS badge end), as the VST suite's layouts (176 × 770) give them.
    pub fn at(&self, across: f32, down: f32) -> (f32, f32) {
        (self.len - down, across)
    }

    /// Screen point for drawing coordinates (x along the module, y across it).
    pub fn p(&self, x: f32, y: f32) -> Pos2 {
        Pos2::new(self.origin.x + y * self.scale, self.origin.y + (self.len - x) * self.scale)
    }

    /// Screen length for a drawing length.
    pub fn s(&self, v: f32) -> f32 {
        v * self.scale
    }

    pub fn rect(&self) -> Rect {
        Rect::from_min_size(self.origin, Vec2::new(self.width * self.scale, self.len * self.scale))
    }

    /// Anodised panel, silver trims down both long edges, thumb screws at both ends.
    pub fn panel(&self, p: &Painter, badge: &str) {
        let r = self.rect();
        p.rect_filled(r.expand(self.s(6.0)), 0.0, ink::RACK);
        let c = Color32::from_rgb;
        gradient_rect(
            p,
            r,
            &[(0.0, c(0x38, 0x3e, 0x44)), (0.06, c(0x2c, 0x31, 0x36)), (0.94, c(0x25, 0x29, 0x2e)), (1.0, c(0x1d, 0x20, 0x24))],
            true,
        );
        let trim = [(0.0, Color32::WHITE), (0.45, c(0xd2, 0xd6, 0xda)), (0.55, c(0x9c, 0xa2, 0xa8)), (1.0, c(0xe4, 0xe7, 0xea))];
        let w = self.s(4.5);
        gradient_rect(p, Rect::from_min_size(r.min, Vec2::new(w, r.height())), &trim, true);
        gradient_rect(p, Rect::from_min_size(Pos2::new(r.right() - w, r.top()), Vec2::new(w, r.height())), &trim, true);
        p.rect_stroke(r, 0.0, Stroke::new(1.0, ink::DARK), egui::StrokeKind::Inside);
        let mid = self.width / 2.0;
        self.screw(p, 19.0, mid);
        self.screw(p, self.len - 19.0, mid);
        self.text(p, self.len - 18.0, self.width * 0.29, "WBS", 13.0, ink::BRAND);
        self.text(p, self.len - 18.0, self.width * 0.71, badge, 12.5, ink::LABEL);
    }

    pub fn screw(&self, p: &Painter, x: f32, y: f32) {
        let c = self.p(x, y);
        p.circle(c, self.s(10.5), Color32::from_rgb(0x11, 0x13, 0x15), Stroke::new(1.0, Color32::from_rgb(0x45, 0x4b, 0x52)));
        p.circle(c, self.s(7.5), Color32::from_rgb(0x2c, 0x31, 0x36), Stroke::new(1.2, Color32::from_rgb(0x68, 0x70, 0x78)));
        let k = self.s(5.0);
        let st = Stroke::new(self.s(1.8), ink::DARK);
        p.line_segment([c - Vec2::new(k, 0.0), c + Vec2::new(k, 0.0)], st);
        p.line_segment([c - Vec2::new(0.0, k), c + Vec2::new(0.0, k)], st);
    }

    /// Level lettering centred on (x, y).
    pub fn text(&self, p: &Painter, x: f32, y: f32, s: &str, size: f32, col: Color32) {
        p.text(self.p(x, y), Align2::CENTER_CENTER, s, FontId::proportional(self.s(size)), col);
    }

    /// An LED at (x, y); `on` lights it with its glow.
    pub fn led(&self, p: &Painter, x: f32, y: f32, colour: Led, on: bool) {
        let c = self.p(x, y);
        let r = self.s(6.0);
        let [hot, body, rim] = colour.colours();
        if on {
            for i in 0..6 {
                p.circle_filled(c, r * (1.6 + i as f32 * 0.35), Color32::from_rgba_unmultiplied(body.r(), body.g(), body.b(), 40 - i * 6));
            }
            radial(p, c, r, &[(0.0, hot), (0.5, body), (1.0, rim)], Vec2::new(-0.3, -0.3));
        } else {
            radial(p, c, r, &[(0.0, lerp(body, Color32::BLACK, 0.55)), (0.6, lerp(rim, Color32::BLACK, 0.5)), (1.0, Color32::BLACK)], Vec2::new(-0.3, -0.3));
        }
        p.circle_stroke(c, r, Stroke::new(self.s(1.2), Color32::from_rgb(0x14, 0x16, 0x18)));
    }

    /// A latching push-button bound to a `BoolParam`; pressed in = on.
    pub fn push(&self, ui: &mut Ui, setter: &ParamSetter, param: &BoolParam, x: f32, y: f32, size: Size, colour: Button) -> Response {
        let c = self.p(x, y);
        let (ring, cap) = match size {
            Size::Large => (19.5, 17.0),
            Size::Small => (14.0, 11.8),
        };
        let resp = ui.interact(Rect::from_center_size(c, Vec2::splat(self.s(ring * 2.0))), ui.id().with(param.name()), Sense::click());
        if resp.clicked() {
            setter.begin_set_parameter(param);
            setter.set_parameter(param, !param.value());
            setter.end_set_parameter(param);
        }
        let on = param.value();
        let p = ui.painter();
        if !on {
            shadow(p, c, self.s(ring), self.scale);
        }
        p.circle_filled(c, self.s(ring), ink::DARK);
        let cc = Color32::from_rgb;
        let (stops, edge) = match colour {
            Button::Red => (vec![(0.0, cc(0xff, 0x6a, 0x52)), (0.65, cc(0xdf, 0x33, 0x18)), (1.0, cc(0x8b, 0x19, 0x07))], cc(0xff, 0x8b, 0x78)),
            Button::White => (vec![(0.0, Color32::WHITE), (0.65, cc(0xd5, 0xd9, 0xde)), (1.0, cc(0x8c, 0x92, 0x98))], Color32::WHITE),
        };
        // Pressed in: the cap sits lower (smaller, darker, no shadow).
        let (r, dim) = if on { (self.s(cap) * 0.93, 0.22) } else { (self.s(cap), 0.0) };
        let stops: Vec<(f32, Color32)> = stops.into_iter().map(|(t, col)| (t, lerp(col, Color32::BLACK, dim))).collect();
        radial(p, c, r, &stops, Vec2::new(-0.3, -0.4));
        p.circle_stroke(c, r, Stroke::new(self.s(0.8), lerp(edge, Color32::BLACK, dim)));
        resp.on_hover_text(param.name())
    }

    /// A bat-handle lever switch bound to a `BoolParam`: down-left = off, up-right = on.
    pub fn lever(&self, ui: &mut Ui, setter: &ParamSetter, param: &BoolParam, x: f32, y: f32) -> Response {
        let c = self.p(x, y);
        let resp = ui.interact(Rect::from_center_size(c, Vec2::splat(self.s(28.0))), ui.id().with(param.name()), Sense::click());
        if resp.clicked() {
            setter.begin_set_parameter(param);
            setter.set_parameter(param, !param.value());
            setter.end_set_parameter(param);
        }
        let p = ui.painter();
        shadow(p, c, self.s(11.0), self.scale);
        p.circle(c, self.s(11.0), Color32::from_rgb(0x16, 0x18, 0x1b), Stroke::new(self.s(1.5), Color32::from_rgb(0x6c, 0x74, 0x7c)));
        p.circle_filled(c, self.s(6.5), Color32::from_rgb(0xc8, 0xcf, 0xd6));
        let tip = if param.value() { Vec2::new(7.0, -7.0) } else { Vec2::new(-7.0, 7.0) } * self.scale;
        p.line_segment([c, c + tip], Stroke::new(self.s(4.0), Color32::from_rgb(0xee, 0xf2, 0xf6)));
        p.circle_filled(c + tip, self.s(3.0), Color32::WHITE);
        resp.on_hover_text(format!("{}: {}", param.name(), if param.value() { "on" } else { "off" }))
    }

    /// Interlocked push-buttons (one in at a time), e.g. COMP TIE OUT / 1 / 2, at `xs` along y.
    #[allow(clippy::too_many_arguments)]
    pub fn push_radio(&self, ui: &mut Ui, setter: &ParamSetter, params: &[&BoolParam], xs: &[f32], y: f32, size: Size, colour: Button) {
        for (i, (param, x)) in params.iter().zip(xs).enumerate() {
            let was = param.value();
            if self.push(ui, setter, param, *x, y, size, colour).clicked() {
                // the one pressed stays in; the others come out
                if !was {
                    for (j, other) in params.iter().enumerate() {
                        if j != i && other.value() {
                            setter.begin_set_parameter(*other);
                            setter.set_parameter(*other, false);
                            setter.end_set_parameter(*other);
                        }
                    }
                } else {
                    setter.begin_set_parameter(*param);
                    setter.set_parameter(*param, true);
                    setter.end_set_parameter(*param);
                }
            }
        }
    }

    /// A knob bound to any parameter. `scale` = (normalised position, legend) printed round it.
    /// Drag up / down to turn (Shift: fine), double-click for the default, scroll to nudge.
    #[allow(clippy::too_many_arguments)]
    pub fn knob<P: Param>(
        &self,
        ui: &mut Ui,
        setter: &ParamSetter,
        param: &P,
        x: f32,
        y: f32,
        size: Size,
        cap: Cap,
        scale: &[(f32, &str)],
        ticks: usize,
    ) -> Response {
        let c = self.p(x, y);
        let (skirt, inner, capr, tick_r, legend_r, font) = match size {
            Size::Large => (29.0, 23.0, 21.0, 32.0, 44.0, 10.0),
            Size::Small => (21.0, 16.5, 14.5, 22.0, 32.0, 9.5),
        };
        let id = ui.id().with(param.name());
        let resp = ui.interact(Rect::from_center_size(c, Vec2::splat(self.s(skirt * 2.2))), id, Sense::click_and_drag());
        let norm = param.modulated_normalized_value();
        if resp.drag_started() {
            setter.begin_set_parameter(param);
        }
        if resp.dragged() {
            let fine = ui.input(|i| i.modifiers.shift);
            let k = if fine { 0.0008 } else { 0.004 };
            let v = (param.unmodulated_normalized_value() - resp.drag_delta().y * k).clamp(0.0, 1.0);
            setter.set_parameter_normalized(param, v);
        }
        if resp.drag_stopped() {
            setter.end_set_parameter(param);
        }
        if resp.double_clicked() {
            setter.begin_set_parameter(param);
            setter.set_parameter_normalized(param, param.default_normalized_value());
            setter.end_set_parameter(param);
        }
        if resp.hovered() {
            let dy = ui.input(|i| i.raw_scroll_delta.y);
            if dy != 0.0 {
                setter.begin_set_parameter(param);
                // a stepped switch moves one position per notch; a pot 1 %
                let step = param.step_count().map_or(0.01, |n| 1.0 / n.max(1) as f32);
                let v = (param.unmodulated_normalized_value() + dy.signum() * step).clamp(0.0, 1.0);
                setter.set_parameter_normalized(param, v);
                setter.end_set_parameter(param);
            }
        }

        let p = ui.painter();
        let ang = |n: f32| (-135.0 + 270.0 * n).to_radians(); // 0 = straight up, clockwise
        let dir = |a: f32| Vec2::new(a.sin(), -a.cos());
        // Scale: ticks, then the legends
        let st = Stroke::new(self.s(1.2), ink::TICK);
        if ticks > 1 {
            for i in 0..ticks {
                let d = dir(ang(i as f32 / (ticks - 1) as f32));
                p.line_segment([c + d * self.s(tick_r), c + d * self.s(tick_r + 5.0)], st);
            }
        }
        for (n, s) in scale {
            p.text(c + dir(ang(*n)) * self.s(legend_r), Align2::CENTER_CENTER, *s, FontId::proportional(self.s(font)), ink::TINY);
        }
        // Skirt with its flutes, then the coloured cap and its pointer line
        shadow(p, c, self.s(skirt), self.scale);
        let cc = Color32::from_rgb;
        radial(p, c, self.s(skirt), &[(0.0, cc(0x3b, 0x40, 0x46)), (0.7, cc(0x1b, 0x1e, 0x21)), (1.0, cc(0x0c, 0x0e, 0x10))], Vec2::new(-0.2, -0.3));
        p.circle_stroke(c, self.s(skirt), Stroke::new(self.s(1.5), cc(0x09, 0x0a, 0x0b)));
        let flutes = if size == Size::Large { 16 } else { 8 };
        for i in 0..flutes {
            let d = dir(i as f32 / flutes as f32 * std::f32::consts::TAU + ang(norm));
            p.line_segment([c + d * self.s(inner), c + d * self.s(skirt)], Stroke::new(self.s(2.0), ink::DARK));
        }
        p.circle_filled(c, self.s(inner), cc(0x18, 0x1b, 0x1e));
        let [hi, body, edge, pointer] = cap.colours();
        radial(p, c, self.s(capr), &[(0.0, hi), (0.65, body), (1.0, edge)], Vec2::new(-0.3, -0.4));
        let d = dir(ang(norm));
        p.line_segment([c + d * self.s(capr * 0.24), c + d * self.s(capr)], Stroke::new(self.s(2.4), pointer));
        resp.on_hover_text(format!("{}: {}", param.name(), param.to_string()))
    }

    /// An edgewise VU (WBS cream face, red from 0 VU) between drawing x0 … x1 at y, needle at `vu`
    /// (VU dB, −20 … +3). The scale runs along the module.
    pub fn edgewise_vu(&self, p: &Painter, x0: f32, x1: f32, y: f32, depth: f32, vu: f32) {
        let a = self.p(x1, y);
        let b = self.p(x0, y + depth);
        let outer = Rect::from_two_pos(a, b);
        p.rect_filled(outer, self.s(4.0), ink::DARK);
        let face = outer.shrink(self.s(6.0));
        let cc = Color32::from_rgb;
        gradient_rect(
            p,
            face,
            &[(0.0, cc(0xd4, 0xbd, 0x7d)), (0.2, cc(0xf5, 0xe7, 0xb8)), (0.5, cc(0xff, 0xf4, 0xcf)), (0.85, cc(0xeb, 0xd7, 0x9c)), (1.0, cc(0xbf, 0xa6, 0x63))],
            true,
        );
        // −20 … +3 VU mapped bottom (x0) → top (x1); the arc of a VU is about dB-linear near 0, log below
        let pos = |db: f32| {
            let lin = 10f32.powf(db / 20.0);
            let t = ((lin - 0.1) / (1.4125 - 0.1)).clamp(0.0, 1.0);
            let m = self.s(9.0);
            face.bottom() - m - t * (face.height() - 2.0 * m)
        };
        let mid = face.center().x;
        let ink_c = cc(0x2b, 0x23, 0x16);
        let red = cc(0xc4, 0x2b, 0x14);
        p.rect_filled(Rect::from_min_max(Pos2::new(mid - self.s(2.0), pos(3.0)), Pos2::new(mid + self.s(2.0), pos(0.0))), 0.0, red);
        for (db, s) in [(-20.0, "20"), (-10.0, "10"), (-7.0, "7"), (-5.0, "5"), (-3.0, "3"), (-1.0, "1"), (0.0, "0"), (1.0, "1"), (2.0, "2"), (3.0, "3")] {
            let y = pos(db);
            let col = if db >= 0.0 { red } else { ink_c };
            p.line_segment([Pos2::new(mid - self.s(6.0), y), Pos2::new(mid + self.s(1.0), y)], Stroke::new(self.s(1.2), col));
            p.text(Pos2::new(mid - self.s(12.0), y), Align2::CENTER_CENTER, s, FontId::proportional(self.s(8.5)), col);
        }
        p.text(Pos2::new(mid + self.s(12.0), face.center().y), Align2::CENTER_CENTER, "VU", FontId::proportional(self.s(10.0)), ink_c);
        let ny = pos(vu);
        p.line_segment([Pos2::new(face.left() + self.s(2.0), ny), Pos2::new(face.right() - self.s(2.0), ny)], Stroke::new(self.s(2.2), cc(0xd4, 0x24, 0x0e)));
        gradient_rect(
            p,
            face,
            &[(0.0, Color32::from_white_alpha(110)), (0.18, Color32::from_white_alpha(30)), (0.5, Color32::TRANSPARENT), (0.88, Color32::from_black_alpha(38)), (1.0, Color32::from_white_alpha(60))],
            true,
        );
    }
}
