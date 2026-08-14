use eframe::egui::{self, Color32, CornerRadius, Margin, Shadow, Stroke};

pub struct Win11Colors {
    pub bg: Color32,
    pub surface: Color32,
    pub surface_hover: Color32,
    pub surface_pressed: Color32,
    pub accent: Color32,
    pub text: Color32,
    pub text_secondary: Color32,
    pub border: Color32,
    pub separator: Color32,
    pub shadow_color: Color32,
    pub selection: Color32,
    pub danger: Color32,
}

pub fn colors(dark: bool) -> Win11Colors {
    if dark {
        Win11Colors {
            bg: Color32::from_rgb(32, 32, 32),
            surface: Color32::from_rgb(45, 45, 45),
            surface_hover: Color32::from_rgb(56, 56, 56),
            surface_pressed: Color32::from_rgb(68, 68, 68),
            accent: Color32::from_rgb(96, 205, 255),
            text: Color32::from_rgb(255, 255, 255),
            text_secondary: Color32::from_rgb(158, 158, 158),
            border: Color32::from_rgb(64, 64, 64),
            separator: Color32::from_rgb(55, 55, 55),
            shadow_color: Color32::from_rgba_premultiplied(0, 0, 0, 80),
            selection: Color32::from_rgba_premultiplied(96, 205, 255, 60),
            danger: Color32::from_rgb(255, 100, 100),
        }
    } else {
        Win11Colors {
            bg: Color32::from_rgb(243, 243, 243),
            surface: Color32::from_rgb(255, 255, 255),
            surface_hover: Color32::from_rgb(249, 249, 249),
            surface_pressed: Color32::from_rgb(240, 240, 240),
            accent: Color32::from_rgb(0, 120, 212),
            text: Color32::from_rgb(26, 26, 26),
            text_secondary: Color32::from_rgb(97, 97, 97),
            border: Color32::from_rgb(224, 224, 224),
            separator: Color32::from_rgb(230, 230, 230),
            shadow_color: Color32::from_rgba_premultiplied(0, 0, 0, 30),
            selection: Color32::from_rgba_premultiplied(0, 120, 212, 40),
            danger: Color32::from_rgb(200, 50, 50),
        }
    }
}

pub fn apply_win11_style(ctx: &egui::Context, dark: bool) {
    let c = colors(dark);

    let theme = if dark {
        egui::Theme::Dark
    } else {
        egui::Theme::Light
    };
    let mut style = (*ctx.style_of(theme)).clone();

    style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    style.spacing.window_margin = Margin::same(12);
    style.spacing.button_padding = egui::vec2(12.0, 6.0);
    style.spacing.indent = 18.0;

    let cr4 = CornerRadius::same(4);
    let cr8 = CornerRadius::same(8);

    style.visuals.window_corner_radius = cr8;
    style.visuals.menu_corner_radius = cr8;

    style.visuals.widgets.noninteractive.corner_radius = cr4;
    style.visuals.widgets.noninteractive.weak_bg_fill = c.surface;
    style.visuals.widgets.noninteractive.bg_fill = c.surface;
    style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, c.text_secondary);
    style.visuals.widgets.noninteractive.bg_stroke = Stroke::NONE;

    style.visuals.widgets.inactive.corner_radius = cr4;
    style.visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
    style.visuals.widgets.inactive.bg_fill = c.surface;
    style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, c.text);
    style.visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, c.border);

    style.visuals.widgets.hovered.corner_radius = cr4;
    style.visuals.widgets.hovered.weak_bg_fill = c.surface_hover;
    style.visuals.widgets.hovered.bg_fill = c.surface_hover;
    style.visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, c.text);
    style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, c.border);
    style.visuals.widgets.hovered.expansion = 0.0;

    style.visuals.widgets.active.corner_radius = cr4;
    style.visuals.widgets.active.weak_bg_fill = c.surface_pressed;
    style.visuals.widgets.active.bg_fill = c.surface_pressed;
    style.visuals.widgets.active.fg_stroke = Stroke::new(1.0, c.text);
    style.visuals.widgets.active.bg_stroke = Stroke::new(1.0, c.accent);
    style.visuals.widgets.active.expansion = 0.0;

    style.visuals.widgets.open.corner_radius = cr4;
    style.visuals.widgets.open.weak_bg_fill = c.surface_hover;
    style.visuals.widgets.open.bg_fill = c.surface_hover;
    style.visuals.widgets.open.fg_stroke = Stroke::new(1.0, c.text);
    style.visuals.widgets.open.bg_stroke = Stroke::NONE;

    style.visuals.extreme_bg_color = c.bg;
    style.visuals.faint_bg_color = c.surface;

    style.visuals.selection.bg_fill = c.selection;
    style.visuals.selection.stroke = Stroke::new(1.0, c.accent);

    style.visuals.window_fill = c.surface;
    style.visuals.window_stroke = Stroke::new(1.0, c.border);
    style.visuals.window_shadow = Shadow {
        offset: [0, 4],
        blur: 16,
        spread: 0,
        color: c.shadow_color,
    };
    style.visuals.popup_shadow = Shadow {
        offset: [0, 8],
        blur: 24,
        spread: 0,
        color: Color32::from_rgba_premultiplied(0, 0, 0, if dark { 120 } else { 50 }),
    };

    style.visuals.override_text_color = Some(c.text);
    style.visuals.warn_fg_color = Color32::from_rgb(255, 180, 0);
    style.visuals.error_fg_color = c.danger;
    style.visuals.panel_fill = c.bg;

    style.visuals.dark_mode = dark;

    ctx.set_style_of(theme, style);

    let mut visuals = egui::Visuals::default();
    visuals.dark_mode = dark;
    visuals.window_fill = c.surface;
    visuals.extreme_bg_color = c.bg;
    visuals.faint_bg_color = c.surface;
    visuals.panel_fill = c.bg;
    visuals.selection.bg_fill = c.selection;
    visuals.selection.stroke = Stroke::new(1.0, c.accent);
    visuals.window_shadow = Shadow {
        offset: [0, 4],
        blur: 16,
        spread: 0,
        color: c.shadow_color,
    };
    visuals.popup_shadow = Shadow {
        offset: [0, 8],
        blur: 24,
        spread: 0,
        color: Color32::from_rgba_premultiplied(0, 0, 0, if dark { 120 } else { 50 }),
    };
    visuals.window_corner_radius = CornerRadius::same(8);
    visuals.menu_corner_radius = CornerRadius::same(8);
    visuals.warn_fg_color = Color32::from_rgb(255, 180, 0);
    visuals.error_fg_color = c.danger;
    visuals.override_text_color = Some(c.text);
    visuals.widgets.noninteractive.corner_radius = cr4;
    visuals.widgets.noninteractive.weak_bg_fill = c.surface;
    visuals.widgets.noninteractive.bg_fill = c.surface;
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, c.text_secondary);
    visuals.widgets.noninteractive.bg_stroke = Stroke::NONE;
    visuals.widgets.inactive.corner_radius = cr4;
    visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
    visuals.widgets.inactive.bg_fill = c.surface;
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, c.text);
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, c.border);
    visuals.widgets.hovered.corner_radius = cr4;
    visuals.widgets.hovered.weak_bg_fill = c.surface_hover;
    visuals.widgets.hovered.bg_fill = c.surface_hover;
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, c.text);
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, c.border);
    visuals.widgets.hovered.expansion = 0.0;
    visuals.widgets.active.corner_radius = cr4;
    visuals.widgets.active.weak_bg_fill = c.surface_pressed;
    visuals.widgets.active.bg_fill = c.surface_pressed;
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, c.text);
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, c.accent);
    visuals.widgets.active.expansion = 0.0;
    visuals.widgets.open.corner_radius = cr4;
    visuals.widgets.open.weak_bg_fill = c.surface_hover;
    visuals.widgets.open.bg_fill = c.surface_hover;
    visuals.widgets.open.fg_stroke = Stroke::new(1.0, c.text);
    visuals.widgets.open.bg_stroke = Stroke::NONE;

    ctx.set_visuals_of(theme, visuals);
}

pub fn separator_stroke(dark: bool) -> Stroke {
    Stroke::new(1.0, colors(dark).separator)
}
