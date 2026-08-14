use chrono::Local;
use eframe::egui;
use rusqlite::Connection;

use crate::crypto;
use crate::db;
use crate::models::{note_avatar_color, AppTheme, NoteTab};
use crate::settings;
use crate::theme;

pub const VERSION: &str = "0.1.0";

pub struct NoteApp {
    pub db: Connection,
    pub notes: Vec<crate::models::Note>,
    pub search_query: String,
    pub master_password: String,
    pub open_tabs: Vec<NoteTab>,
    pub selected_tab: Option<usize>,
    pub selected_note: Option<usize>,
    pub sidebar_collapsed: bool,
    pub show_menu: bool,
    pub is_maximized: bool,
    pub theme: AppTheme,
    pub tab_renaming: bool,
    pub tab_rename_buf: String,
    pub drag_idx: Option<usize>,
    pub context_note_id: Option<i64>,
    pub context_menu_pos: egui::Pos2,
    pub show_about: bool,
    pub menu_just_opened: bool,
    pub about_just_opened: bool,
    pub font_size: f32,
    pub word_wrap: bool,
    pub formatting_enabled: bool,
    pub restore_tabs: bool,
}

impl NoteApp {
    pub fn new() -> Self {
        let db_path = db::get_db_path();
        let conn = Connection::open(&db_path).expect("Failed to open database");
        db::init_db(&conn);

        let mut app = NoteApp {
            db: conn,
            notes: Vec::new(),
            search_query: String::new(),
            master_password: String::new(),
            open_tabs: Vec::new(),
            selected_tab: None,
            selected_note: None,
            sidebar_collapsed: false,
            show_menu: false,
            is_maximized: false,
            theme: settings::load_theme(),
            tab_renaming: false,
            tab_rename_buf: String::new(),
            drag_idx: None,
            context_note_id: None,
            context_menu_pos: egui::Pos2::ZERO,
            show_about: false,
            menu_just_opened: false,
            about_just_opened: false,
            font_size: settings::load_font_size(),
            word_wrap: settings::load_word_wrap(),
            formatting_enabled: settings::load_formatting_enabled(),
            restore_tabs: settings::load_restore_tabs(),
        };
        app.refresh_list();

        if app.notes.is_empty() {
            app.new_note();
        }

        if app.restore_tabs {
            let saved_ids = settings::load_open_tab_ids();
            for id in saved_ids {
                if let Some(note) = app.notes.iter().find(|n| n.id == id) {
                    let content = if note.is_locked {
                        crypto::decrypt(&note.content, &app.master_password)
                    } else {
                        note.content.clone()
                    };
                    let display_title = if note.is_locked {
                        crypto::decrypt(&note.title, &app.master_password)
                    } else {
                        note.title.clone()
                    };
                    app.open_tabs.push(NoteTab {
                        note_id: note.id,
                        title: display_title,
                        content,
                        is_locked: note.is_locked,
                    });
                }
            }
        }

        if app.open_tabs.is_empty() {
            if let Some(note) = app.notes.first() {
                let content = if note.is_locked {
                    crypto::decrypt(&note.content, &app.master_password)
                } else {
                    note.content.clone()
                };
                let display_title = if note.is_locked {
                    crypto::decrypt(&note.title, &app.master_password)
                } else {
                    note.title.clone()
                };
                app.open_tabs.push(NoteTab {
                    note_id: note.id,
                    title: display_title,
                    content,
                    is_locked: note.is_locked,
                });
            }
        }

        if !app.open_tabs.is_empty() {
            app.selected_tab = Some(0);
        }

        app
    }

    pub fn refresh_list(&mut self) {
        self.notes = db::load_notes(&self.db, &self.search_query, &self.master_password);
    }

    pub fn new_note(&mut self) {
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        db::insert_note(&self.db, "Yeni Not", "", false, &now);
        self.refresh_list();
    }

    pub fn open_tab(&mut self, note: &crate::models::Note) {
        let content = if note.is_locked {
            crypto::decrypt(&note.content, &self.master_password)
        } else {
            note.content.clone()
        };

        if let Some(pos) = self.open_tabs.iter().position(|t| t.note_id == note.id) {
            self.selected_tab = Some(pos);
            return;
        }

        let display_title = if note.is_locked {
            crypto::decrypt(&note.title, &self.master_password)
        } else {
            note.title.clone()
        };

        self.open_tabs.push(NoteTab {
            note_id: note.id,
            title: display_title,
            content,
            is_locked: note.is_locked,
        });
        self.selected_tab = Some(self.open_tabs.len() - 1);
        self.save_open_tabs();
    }

    fn save_open_tabs(&self) {
        let ids: Vec<i64> = self
            .open_tabs
            .iter()
            .map(|t| t.note_id)
            .filter(|&id| id != -1)
            .collect();
        settings::save_open_tab_ids(&ids);
    }

    pub fn save_tab_content(&mut self, tab_idx: usize, new_content: &str) {
        let note_id = self.open_tabs[tab_idx].note_id;
        if note_id == -1 {
            return;
        }
        let is_locked = self.open_tabs[tab_idx].is_locked;
        let final_content = if is_locked {
            crypto::encrypt(new_content, &self.master_password)
        } else {
            new_content.to_string()
        };
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        db::update_note_content(&self.db, &final_content, &now, note_id);
        self.open_tabs[tab_idx].content = new_content.to_string();
    }

    #[allow(dead_code)]
    pub fn delete_note(&mut self) {
        if let Some(tab_idx) = self.selected_tab {
            let note_id = self.open_tabs[tab_idx].note_id;
            db::delete_note(&self.db, note_id);
            self.open_tabs.remove(tab_idx);
            self.selected_tab = if self.open_tabs.is_empty() {
                None
            } else {
                Some(self.open_tabs.len().saturating_sub(1))
            };
            self.selected_note = None;
            self.refresh_list();
            self.save_open_tabs();
        }
    }
}

impl eframe::App for NoteApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.menu_just_opened = false;
        self.about_just_opened = false;

        let effective_dark = self.theme.is_dark(&ctx);

        theme::apply_win11_style(&ctx, effective_dark);
        let c = theme::colors(effective_dark);

        let prev_query = self.search_query.clone();
        let mut action_open: Option<usize> = None;
        let mut action_new = false;

        // --- TAB BAR (en üst) ---
        let bar_bg = theme::glass(effective_dark);
        let active_bg = c.surface;
        let inactive_bg = egui::Color32::TRANSPARENT;
        let active_text = c.text;
        let inactive_text = c.text_secondary;

        let mut menu_btn_pos = egui::Pos2::ZERO;

        egui::Panel::top("tab_bar")
            .min_size(36.0)
            .frame(egui::Frame::new().fill(bar_bg))
            .show(ui, |ui| {
                let mut to_close: Option<usize> = None;
                let mut tab_rects: Vec<egui::Rect> = Vec::new();
                let mut tab_rename_commit = false;

                // --- Window drag handle + double-click maximize ---
                let drag_sense = ui.interact(
                    egui::Rect::from_min_max(ui.max_rect().min, ui.max_rect().max),
                    ui.id().with("bar_drag"),
                    egui::Sense::click_and_drag(),
                );
                if drag_sense.drag_started_by(egui::PointerButton::Primary) {
                    ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }
                if drag_sense.double_clicked_by(egui::PointerButton::Primary) {
                    self.is_maximized = !self.is_maximized;
                    ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(self.is_maximized));
                }

                ui.horizontal(|ui| {
                    // --- Render tabs ---
                    for i in 0..self.open_tabs.len() {
                        let text = self.open_tabs[i].title.clone();
                        let is_selected = self.selected_tab == Some(i);

                        let text_w = ctx.fonts_mut(|f| {
                            let galley = f.layout_no_wrap(
                                text.as_str().into(),
                                egui::FontId::proportional(13.0),
                                egui::Color32::WHITE,
                            );
                            galley.rect.width()
                        });
                        let tab_w = (text_w + 50.0).max(70.0).min(200.0);
                        let tab_size = egui::vec2(tab_w, 30.0);
                        let (tab_rect, tab_resp) =
                            ui.allocate_exact_size(tab_size, egui::Sense::click_and_drag());

                        let display_text = if text.len() > 15 {
                            format!("{}…", &text[..15])
                        } else {
                            text.clone()
                        };
                        tab_rects.push(tab_rect);

let corner = if is_selected {
    egui::CornerRadius {
        nw: 6,
        ne: 6,
        sw: 0,
        se: 0,
    }
} else {
    egui::CornerRadius::same(6)
};
let tab_bg = if is_selected { active_bg } else { inactive_bg };
let text_color = if is_selected {
    active_text
} else {
    inactive_text
};

let painter = ui.painter_at(tab_rect);
painter.rect_filled(tab_rect, corner, tab_bg);

// Active tab accent top indicator
if is_selected {
    painter.rect_filled(
        egui::Rect::from_min_max(
            egui::pos2(tab_rect.min.x, tab_rect.min.y),
            egui::pos2(tab_rect.max.x, tab_rect.min.y + 2.0),
        ),
        2.0,
        c.accent,
    );
}

let is_locked_tab = self.open_tabs[i].is_locked;

// Close button rect (right side of tab)
let close_rect = egui::Rect::from_min_size(
    egui::pos2(tab_rect.max.x - 30.0, tab_rect.center().y - 14.0),
    egui::vec2(28.0, 28.0),
);
let close_hover_bg = c.surface_pressed;

                        if is_selected && self.tab_renaming {
                            // Rename mode: use TextEdit widget
                            let rename_rect = egui::Rect::from_min_max(
                                egui::pos2(tab_rect.min.x + 8.0, tab_rect.min.y + 3.0),
                                egui::pos2(tab_rect.max.x - 32.0, tab_rect.max.y - 3.0),
                            );
                            let ui_builder = egui::UiBuilder::new()
                                .max_rect(rename_rect)
                                .layout(*ui.layout());
                            let mut child_ui = ui.new_child(ui_builder);
                            let resp = child_ui.add_sized(
                                rename_rect.size(),
                                egui::TextEdit::singleline(&mut self.tab_rename_buf)
                                    .text_color(text_color)
                                    .frame(egui::Frame::default()),
                            );
                            resp.request_focus();

                            if ui.input_mut(|i| {
                                i.consume_key(egui::Modifiers::NONE, egui::Key::Enter)
                            }) {
                                tab_rename_commit = true;
                            }
                            if ui.input_mut(|i| {
                                i.consume_key(egui::Modifiers::NONE, egui::Key::Escape)
                            }) {
                                self.tab_renaming = false;
                            }
} else {
    // Lock icon for locked tabs
    let title_start_x = tab_rect.min.x + 10.0;
    if is_locked_tab {
        draw_lock(&painter, egui::pos2(title_start_x, tab_rect.center().y), text_color);
    }
    // Title text
    painter.text(
        egui::pos2(
            title_start_x + if is_locked_tab { 14.0 } else { 0.0 },
            tab_rect.center().y,
        ),
        egui::Align2::LEFT_CENTER,
        &display_text,
        egui::FontId::proportional(13.0),
        text_color,
    );
}

                        // Close tab X
                        let pointer_pos = ctx.pointer_interact_pos();
                        let is_close_hover = pointer_pos.map_or(false, |p| close_rect.contains(p));
                        if is_close_hover {
                            painter.rect_filled(close_rect, 4.0, close_hover_bg);
                        }
                        let xc = close_rect.center();
                        let xs = 5.0;
                        let x_stroke = egui::Stroke::new(2.0, text_color);
                        painter.line_segment(
                            [
                                egui::pos2(xc.x - xs, xc.y - xs),
                                egui::pos2(xc.x + xs, xc.y + xs),
                            ],
                            x_stroke,
                        );
                        painter.line_segment(
                            [
                                egui::pos2(xc.x + xs, xc.y - xs),
                                egui::pos2(xc.x - xs, xc.y + xs),
                            ],
                            x_stroke,
                        );

                        // Tab interactions
                        if tab_resp.drag_started() {
                            self.drag_idx = Some(i);
                        }

                        if tab_resp.clicked() {
                            let is_close = pointer_pos.map_or(false, |p| close_rect.contains(p));
                            if is_close {
                                to_close = Some(i);
                            } else if is_selected && self.open_tabs[i].note_id != -1 {
                                self.tab_rename_buf = text;
                                self.tab_renaming = true;
                            } else {
                                self.selected_tab = Some(i);
                                self.tab_renaming = false;
                            }
                        }
                    }

                    ui.add_space(4.0);

                    // --- + button ---
                    let btn_size = egui::vec2(28.0, 28.0);
                    let (plus_rect, plus_resp) =
                        ui.allocate_exact_size(btn_size, egui::Sense::click());
                    let plus_hover = plus_resp.hovered();
                    let btn_text_color = c.text;
                    let painter = ui.painter_at(plus_rect);
                    if plus_hover {
                        let plus_hover_bg = c.surface_hover;
                        painter.rect_filled(plus_rect, 4.0, plus_hover_bg);
                    }
                    painter.text(
                        plus_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "+",
                        egui::FontId::proportional(20.0),
                        btn_text_color,
                    );
                    if plus_resp
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        action_new = true;
                    }

                    // --- Chevron ▼ button ---
                    ui.add_space(4.0);
                    let chev_size = egui::vec2(28.0, 28.0);
                    let (chev_rect, chev_resp) =
                        ui.allocate_exact_size(chev_size, egui::Sense::click());
                    menu_btn_pos = chev_rect.left_bottom();
                    let chev_hover = chev_resp.hovered() || self.show_menu;
                    let chev_painter = ui.painter_at(chev_rect);
                    if chev_hover {
                        let chev_hover_bg = c.surface_hover;
                        chev_painter.rect_filled(chev_rect, 4.0, chev_hover_bg);
                    }
                    let chev_color = c.text;
                    let cc = chev_rect.center();
                    let cw = 4.0;
                    let ch = 2.5;
                    let chev_stroke = egui::Stroke::new(2.0, chev_color);
                    chev_painter.line_segment(
                        [
                            egui::pos2(cc.x - cw, cc.y - ch),
                            egui::pos2(cc.x, cc.y + ch),
                        ],
                        chev_stroke,
                    );
                    chev_painter.line_segment(
                        [
                            egui::pos2(cc.x + cw, cc.y - ch),
                            egui::pos2(cc.x, cc.y + ch),
                        ],
                        chev_stroke,
                    );
                    if chev_resp
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        self.show_menu = !self.show_menu;
                        self.menu_just_opened = true;
                        self.context_note_id = None;
                    }

                    ui.add_space(12.0);

                    // --- Search bar + window controls (right-aligned) ---
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Close button (en sağ)
                        let wc_size = egui::vec2(46.0, 30.0);
                        let (close_rect, close_resp) =
                            ui.allocate_exact_size(wc_size, egui::Sense::click());
                        let close_hover = close_resp.hovered();
                        let close_painter = ui.painter_at(close_rect);
                        if close_hover {
                            close_painter.rect_filled(
                                close_rect,
                                0.0,
                                egui::Color32::from_rgb(232, 17, 35),
                            );
                        }
                        let xc = close_rect.center();
                        let xs2 = 5.0;
                        let x_stroke2 = egui::Stroke::new(
                            2.0,
                            if close_hover {
                                egui::Color32::WHITE
                            } else {
                                c.text
                            },
                        );
                        close_painter.line_segment(
                            [egui::pos2(xc.x - xs2, xc.y - xs2), egui::pos2(xc.x + xs2, xc.y + xs2),],
                            x_stroke2,
                        );
                        close_painter.line_segment(
                            [egui::pos2(xc.x + xs2, xc.y - xs2), egui::pos2(xc.x - xs2, xc.y + xs2),],
                            x_stroke2,
                        );
                        if close_resp
                            .on_hover_cursor(egui::CursorIcon::PointingHand)
                            .clicked()
                        {
                            std::process::exit(0);
                        }

                        // Maximize button
                        let (max_rect, max_resp) =
                            ui.allocate_exact_size(wc_size, egui::Sense::click());
                        let max_hover = max_resp.hovered();
                        let max_painter = ui.painter_at(max_rect);
                        if max_hover {
                            max_painter.rect_filled(max_rect, 0.0, c.surface_hover);
                        }
                        let mrect = max_rect.shrink(8.0);
                        max_painter.rect_stroke(
                            mrect,
                            1.0,
                            egui::Stroke::new(1.0, c.text),
                            egui::StrokeKind::Inside,
                        );
                        if max_resp
                            .on_hover_cursor(egui::CursorIcon::PointingHand)
                            .clicked()
                        {
                            self.is_maximized = !self.is_maximized;
                            ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(
                                self.is_maximized,
                            ));
                        }

                        // Minimize button
                        let (min_rect, min_resp) =
                            ui.allocate_exact_size(wc_size, egui::Sense::click());
                        let min_hover = min_resp.hovered();
                        let min_painter = ui.painter_at(min_rect);
                        if min_hover {
                            min_painter.rect_filled(min_rect, 0.0, c.surface_hover);
                        }
                        let min_c = min_rect.center();
                        min_painter.line_segment(
                            [
                                egui::pos2(min_c.x - 6.0, min_c.y),
                                egui::pos2(min_c.x + 6.0, min_c.y),
                            ],
                            egui::Stroke::new(1.4, c.text),
                        );
                        if min_resp
                            .on_hover_cursor(egui::CursorIcon::PointingHand)
                            .clicked()
                        {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                        }

                        ui.add_space(10.0);

                        // Search field
                        let search_w = 200.0;
                        let (search_rect, _) = ui.allocate_exact_size(
                            egui::vec2(search_w, 30.0),
                            egui::Sense::hover(),
                        );
                        let search_focused =
                            ctx.memory(|m| m.has_focus(egui::Id::new("search_field")));
                        let search_stroke = if search_focused {
                            egui::Stroke::new(1.0, c.accent)
                        } else {
                            egui::Stroke::new(1.0, c.border)
                        };
                        let sp = ui.painter_at(search_rect);
                        sp.rect(
                            search_rect,
                            10.0,
                            c.surface,
                            search_stroke,
                            egui::StrokeKind::Inside,
                        );
                        let sc = search_rect.center();
                        sp.circle_stroke(
                            egui::pos2(search_rect.min.x + 15.0, sc.y - 1.0),
                            4.0,
                            egui::Stroke::new(1.5, c.text_secondary),
                        );
                        sp.line_segment(
                            [
                                egui::pos2(search_rect.min.x + 18.5, sc.y + 2.5),
                                egui::pos2(search_rect.min.x + 22.0, sc.y + 6.0),
                            ],
                            egui::Stroke::new(1.5, c.text_secondary),
                        );
                        let ui_builder = egui::UiBuilder::new().max_rect(
                            search_rect.shrink2(egui::vec2(26.0, 2.0)),
                        );
                        let mut child = ui.new_child(ui_builder);
                        child.add(
                            egui::TextEdit::singleline(&mut self.search_query)
                                .id(egui::Id::new("search_field"))
                                .hint_text("Notlarda ara...")
                                .text_color(c.text)
                                .font(egui::FontId::proportional(13.0))
                                .frame(egui::Frame::NONE),
                        );
                    });
                });

                // --- DnD reorder (after horizontal, inside panel) ---
                if let Some(from) = self.drag_idx {
                    if let Some(pos) = ctx.pointer_interact_pos() {
                        for (to, rect) in tab_rects.iter().enumerate() {
                            if rect.contains(pos) && to != from && to < self.open_tabs.len() {
                                let tab = self.open_tabs.remove(from);
                                self.open_tabs.insert(to, tab);
                                self.drag_idx = Some(to);

                                if let Some(sel) = self.selected_tab {
                                    if sel == from {
                                        self.selected_tab =
                                            Some(to.min(self.open_tabs.len().saturating_sub(1)));
                                    } else if from < sel && to >= sel {
                                        self.selected_tab = Some(sel - 1);
                                    } else if from > sel && to <= sel {
                                        self.selected_tab = Some(sel + 1);
                                    }
                                }
                                break;
                            }
                        }
                    }

                    let any_down = ctx.input(|i| i.pointer.any_down());
                    if !any_down {
                        self.drag_idx = None;
                        self.save_open_tabs();
                    }
                }

                // --- Tab rename commit ---
                if tab_rename_commit {
                    let new_title = self.tab_rename_buf.trim().to_string();
                    if !new_title.is_empty() {
                        if let Some(tab_idx) = self.selected_tab {
                            let note_id = self.open_tabs[tab_idx].note_id;
                            let is_locked = self.open_tabs[tab_idx].is_locked;
                            let final_title = if is_locked {
                                crypto::encrypt(&new_title, &self.master_password)
                            } else {
                                new_title.clone()
                            };
                            let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                            db::update_note_title(&self.db, &final_title, &now, note_id);
                            self.open_tabs[tab_idx].title = new_title;
                            self.refresh_list();
                        }
                    }
                    self.tab_renaming = false;
                }

                // --- Close tab ---
                if let Some(idx) = to_close {
                    let note_id = self.open_tabs[idx].note_id;
                    let content_empty = self.open_tabs[idx].content.trim().is_empty();
                    self.open_tabs.remove(idx);
                    self.selected_tab = if self.open_tabs.is_empty() {
                        None
                    } else {
                        Some(self.open_tabs.len().saturating_sub(1))
                    };
                    self.tab_renaming = false;
                    if content_empty && note_id != -1 {
                        db::delete_note(&self.db, note_id);
                        self.refresh_list();
                    }
                    self.save_open_tabs();
                    // if self.open_tabs.is_empty() {
                    //     ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    // }
                }
            });

        // --- Status Bar ---
        egui::Panel::bottom("status_bar")
            .min_size(28.0)
            .frame(egui::Frame::new().fill(theme::glass(effective_dark)))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(format!("uNote v{}", VERSION))
                            .color(c.text_secondary)
                            .size(11.5),
                    );
                    ui.label(
                        egui::RichText::new("·")
                            .color(c.text_secondary)
                            .size(11.5),
                    );
                    ui.label(
                        egui::RichText::new(format!("{} not", self.notes.len()))
                            .color(c.text_secondary)
                            .size(11.5),
                    );
                    if let Some(tab_idx) = self.selected_tab {
                        if let Some(tab) = self.open_tabs.get(tab_idx) {
                            ui.label(
                                egui::RichText::new("·")
                                    .color(c.text_secondary)
                                    .size(11.5),
                            );
                            ui.label(
                                egui::RichText::new(&tab.title)
                                    .color(c.text_secondary)
                                    .size(11.5),
                            );
                        }
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let (icon, name) = match self.theme {
                            AppTheme::Light => ("☀", "Açık"),
                            AppTheme::Dark => ("🌙", "Koyu"),
                            AppTheme::System => ("🖥", "Sistem"),
                        };
                        ui.label(
                            egui::RichText::new(format!("{} {}", icon, name))
                                .color(c.text_secondary)
                                .size(11.5),
                        );
                    });
                });
            });

        // --- Menu dropdown (Win11 modern) ---
        if self.show_menu {
            let menu_text_color = c.text;
            let menu_bg = theme::glass_strong(effective_dark);
            let menu_hover_bg = c.surface_hover;
            let menu_stroke = theme::separator_stroke(effective_dark);

            let menu_frame = egui::Frame::new()
                .fill(menu_bg)
                .corner_radius(8.0)
                .stroke(menu_stroke)
                .inner_margin(egui::Margin::symmetric(4, 2));

            let menu_resp = egui::Area::new(egui::Id::new("menu_dropdown"))
                .fixed_pos(menu_btn_pos)
                .show(&ctx, |ui| {
                    let shadow = egui::Frame::new().fill(menu_bg).corner_radius(8.0);
                    shadow.show(ui, |ui| {
                        menu_frame.show(ui, |ui| {
                            ui.set_width(180.0);
                            let item_h = 32.0;

                            // Settings
                            let (sr, srsp) = ui.allocate_exact_size(
                                egui::vec2(ui.available_width(), item_h),
                                egui::Sense::click(),
                            );
                            if srsp.hovered() {
                                ui.painter_at(sr).rect_filled(
                                    sr.shrink2(egui::vec2(4.0, 0.0)),
                                    6.0,
                                    menu_hover_bg,
                                );
                            }
                            ui.painter_at(sr).text(
                                egui::pos2(sr.min.x + 12.0, sr.center().y),
                                egui::Align2::LEFT_CENTER,
                                "⚙  Ayarlar",
                                egui::FontId::proportional(13.0),
                                menu_text_color,
                            );
                            if srsp
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .clicked()
                            {
                                if let Some(pos) =
                                    self.open_tabs.iter().position(|t| t.note_id == -1)
                                {
                                    self.selected_tab = Some(pos);
                                } else {
                                    self.open_tabs.push(NoteTab {
                                        note_id: -1,
                                        title: "⚙  Ayarlar".to_string(),
                                        content: String::new(),
                                        is_locked: false,
                                    });
                                    self.selected_tab = Some(self.open_tabs.len() - 1);
                                }
                                self.show_menu = false;
                            }

                            ui.separator();

                            // About
                            let (ar, arsp) = ui.allocate_exact_size(
                                egui::vec2(ui.available_width(), item_h),
                                egui::Sense::click(),
                            );
                            if arsp.hovered() {
                                ui.painter_at(ar).rect_filled(
                                    ar.shrink2(egui::vec2(4.0, 0.0)),
                                    6.0,
                                    menu_hover_bg,
                                );
                            }
                            ui.painter_at(ar).text(
                                egui::pos2(ar.min.x + 12.0, ar.center().y),
                                egui::Align2::LEFT_CENTER,
                                "ℹ  Hakkında",
                                egui::FontId::proportional(13.0),
                                menu_text_color,
                            );
                            if arsp
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .clicked()
                            {
                                self.show_about = true;
                                self.about_just_opened = true;
                                self.show_menu = false;
                            }
                        });
                    });
                });

            if !self.menu_just_opened && menu_resp.response.clicked_elsewhere()
                || ctx.input(|i| i.key_pressed(egui::Key::Escape))
            {
                self.show_menu = false;
            }
        }

        // --- About popup ---
        if self.show_about {
            let about_bg = c.surface;
            let about_stroke = theme::separator_stroke(effective_dark);

            let about_resp = egui::Area::new(egui::Id::new("about_popup"))
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .show(&ctx, |ui| {
                    let about_frame = egui::Frame::new()
                        .fill(about_bg)
                        .corner_radius(10.0)
                        .stroke(about_stroke)
                        .inner_margin(egui::Margin::symmetric(24, 22));
                    about_frame.show(ui, |ui| {
                        ui.set_min_width(300.0);
                        ui.vertical_centered(|ui| {
                            ui.label(
                                egui::RichText::new("uNote")
                                    .color(c.accent)
                                    .size(34.0)
                                    .strong(),
                            );
                            ui.add_space(6.0);
                            ui.label(
                                egui::RichText::new("Güvenli Not Alma Kasası")
                                    .color(c.text)
                                    .size(17.0)
                                    .strong(),
                            );
                            ui.add_space(8.0);
                            ui.label(
                                egui::RichText::new(
                                    "Notlarınız AES-256-GCM ile şifrelenir ve yerel olarak saklanır.",
                                )
                                .color(c.text_secondary)
                                .size(12.5),
                            );
                            ui.add_space(4.0);
                            ui.label(
                                egui::RichText::new(format!(
                                    "Sürüm {} · Rust + egui",
                                    VERSION
                                ))
                                .color(c.text_secondary)
                                .size(11.5),
                            );
                            ui.add_space(16.0);
                            let ok_size = egui::vec2(120.0, 34.0);
                            let (okr, okresp) =
                                ui.allocate_exact_size(ok_size, egui::Sense::click());
                            let okp = ui.painter_at(okr);
                            let ok_bg = if okresp.hovered() {
                                c.accent_strong
                            } else {
                                c.accent
                            };
                            okp.rect_filled(okr, 8.0, ok_bg);
                            okp.text(
                                okr.center(),
                                egui::Align2::CENTER_CENTER,
                                "Tamam",
                                egui::FontId::proportional(13.0),
                                egui::Color32::WHITE,
                            );
                            if okresp
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .clicked()
                            {
                                self.show_about = false;
                            }
                        });
                    });
                });

            if !self.about_just_opened && about_resp.response.clicked_elsewhere()
                || ctx.input(|i| i.key_pressed(egui::Key::Escape))
            {
                self.show_about = false;
            }
        }

        // --- Sidebar ---
        let is_settings_tab = self
            .selected_tab
            .and_then(|i| self.open_tabs.get(i))
            .map_or(false, |t| t.note_id == -1);

        if !is_settings_tab {
            let sidebar_anim =
                ctx.animate_bool(egui::Id::new("sidebar_anim"), !self.sidebar_collapsed);
            let sidebar_w = 52.0 + 168.0 * sidebar_anim;

            let notes_clone = self.notes.clone();
            let master_pwd = self.master_password.clone();

            egui::Panel::left("sidebar")
                .resizable(false)
                .exact_size(sidebar_w)
                .frame(
                    egui::Frame::new()
                        .fill(theme::glass(effective_dark))
                        .inner_margin(egui::Margin::symmetric(6, 0)),
                )
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        let btn_size = egui::vec2(28.0, 28.0);
                        let (btn_rect, btn_resp) =
                            ui.allocate_exact_size(btn_size, egui::Sense::click());
                        let btn_hover_bg = if btn_resp.hovered() {
                            c.surface_hover
                        } else {
                            egui::Color32::TRANSPARENT
                        };
                        let painter = ui.painter_at(btn_rect);
                        if btn_hover_bg != egui::Color32::TRANSPARENT {
                            painter.rect_filled(btn_rect, 6.0, btn_hover_bg);
                        }
                        let btn_text_color = c.text;
                        // Hamburger lines
                        let sc = btn_rect.center();
                        let sw = 12.0;
                        let sg = 4.0;
                        let s_stroke = egui::Stroke::new(2.0, btn_text_color);
                        painter.line_segment(
                            [
                                egui::pos2(sc.x - sw / 2.0, sc.y - sg),
                                egui::pos2(sc.x + sw / 2.0, sc.y - sg),
                            ],
                            s_stroke,
                        );
                        painter.line_segment(
                            [
                                egui::pos2(sc.x - sw / 2.0, sc.y),
                                egui::pos2(sc.x + sw / 2.0, sc.y),
                            ],
                            s_stroke,
                        );
                        painter.line_segment(
                            [
                                egui::pos2(sc.x - sw / 2.0, sc.y + sg),
                                egui::pos2(sc.x + sw / 2.0, sc.y + sg),
                            ],
                            s_stroke,
                        );
                        if btn_resp
                            .on_hover_cursor(egui::CursorIcon::PointingHand)
                            .clicked()
                        {
                            self.sidebar_collapsed = !self.sidebar_collapsed;
                        }

                        // "Notlar" title + count (expanded only)
                        if !self.sidebar_collapsed {
                            ui.add_space(2.0);
                            ui.label(
                                egui::RichText::new("Notlar")
                                    .color(c.text)
                                    .size(13.0)
                                    .strong(),
                            );
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    ui.label(
                                        egui::RichText::new(format!("{} not", notes_clone.len()))
                                            .color(c.text_secondary)
                                            .size(11.5),
                                    );
                                },
                            );
                        }
                    });
                    ui.add_space(4.0);

                    // Header separator
                    let head_rect = ui.max_rect();
                    ui.painter().line_segment(
                        [
                            egui::pos2(head_rect.min.x + 2.0, head_rect.min.y),
                            egui::pos2(head_rect.max.x - 2.0, head_rect.min.y),
                        ],
                        theme::separator_stroke(effective_dark),
                    );

                    egui::ScrollArea::vertical()
                        .auto_shrink(false)
                        .show(ui, |ui| {
                            if self.sidebar_collapsed {
                                ui.add_space(4.0);
                                for (i, note) in notes_clone.iter().enumerate() {
                                    let display_title = if note.is_locked {
                                        crypto::decrypt(&note.title, &master_pwd)
                                    } else {
                                        note.title.clone()
                                    };
                                    let avatar_text: String = display_title
                                        .chars()
                                        .take(3)
                                        .collect::<String>()
                                        .to_uppercase();

                                    let (bg, fg) = note_avatar_color(i, effective_dark);
                                    let (av_rect, av_resp) = ui.allocate_exact_size(
                                        egui::vec2(36.0, 30.0),
                                        egui::Sense::click(),
                                    );
                                    let p = ui.painter_at(av_rect);
                                    let sq = egui::Rect::from_center_size(
                                        av_rect.center(),
                                        egui::vec2(26.0, 26.0),
                                    );
                                    p.rect_filled(sq, 6.0, bg);
                                    p.text(
                                        sq.center(),
                                        egui::Align2::CENTER_CENTER,
                                        &avatar_text,
                                        egui::FontId::proportional(10.0),
                                        fg,
                                    );
                                    if av_resp
                                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                                        .clicked()
                                    {
                                        action_open = Some(i);
                                    }
                                    ui.add_space(2.0);
                                }
                                if notes_clone.is_empty() {
                                    ui.add_space(8.0);
                                    ui.vertical_centered(|ui| {
                                        ui.label(
                                            egui::RichText::new("-")
                                                .color(c.text_secondary)
                                                .size(11.0),
                                        );
                                    });
                                }
                            } else {
                                if notes_clone.is_empty() {
                                    ui.add_space(40.0);
                                    ui.vertical_centered(|ui| {
                                        ui.label(
                                            egui::RichText::new("Henüz not yok")
                                                .color(c.text_secondary),
                                        );
                                    });
                                    return;
                                }
                                for (i, note) in notes_clone.iter().enumerate() {
                                    let is_sel = self.selected_note == Some(i);

                                    let display_title = if note.is_locked {
                                        crypto::decrypt(&note.title, &master_pwd)
                                    } else {
                                        note.title.clone()
                                    };
                                    let avatar_text: String = display_title
                                        .chars()
                                        .take(3)
                                        .collect::<String>()
                                        .to_uppercase();

                                    let item_w = ui.available_width();
                                    let item_h = 34.0;
                                    let (item_rect, item_resp) = ui.allocate_exact_size(
                                        egui::vec2(item_w.max(0.0), item_h),
                                        egui::Sense::click(),
                                    );

                                    let item_bg = if is_sel {
                                        c.accent_soft
                                    } else if item_resp.hovered() {
                                        c.surface_hover
                                    } else {
                                        egui::Color32::TRANSPARENT
                                    };

                                    let painter = ui.painter_at(item_rect);
                                    if item_bg != egui::Color32::TRANSPARENT {
                                        painter.rect_filled(item_rect, 6.0, item_bg);
                                    }
                                    // Left accent bar for selected
                                    if is_sel {
                                        painter.rect_filled(
                                            egui::Rect::from_min_max(
                                                egui::pos2(
                                                    item_rect.min.x,
                                                    item_rect.min.y + 7.0,
                                                ),
                                                egui::pos2(
                                                    item_rect.min.x + 3.0,
                                                    item_rect.max.y - 7.0,
                                                ),
                                            ),
                                            1.5,
                                            c.accent,
                                        );
                                    }
                                    // Avatar
                                    let av_rect = egui::Rect::from_min_size(
                                        egui::pos2(
                                            item_rect.min.x + 8.0,
                                            item_rect.center().y - 13.0,
                                        ),
                                        egui::vec2(26.0, 26.0),
                                    );
                                    let (bg, fg) = note_avatar_color(i, effective_dark);
                                    painter.rect_filled(av_rect, 6.0, bg);
                                    painter.text(
                                        av_rect.center(),
                                        egui::Align2::CENTER_CENTER,
                                        &avatar_text,
                                        egui::FontId::proportional(10.0),
                                        fg,
                                    );
                                    // Title
                                    let text_color = if is_sel { c.text } else { c.text };
                                    painter.text(
                                        egui::pos2(
                                            item_rect.min.x + 42.0,
                                            item_rect.center().y,
                                        ),
                                        egui::Align2::LEFT_CENTER,
                                        &display_title,
                                        egui::FontId::proportional(12.5),
                                        text_color,
                                    );
                                    // Lock icon
                                    if note.is_locked {
                                        draw_lock(
                                            &painter,
                                            egui::pos2(item_rect.max.x - 13.0, item_rect.center().y),
                                            c.text_secondary,
                                        );
                                    }

                                    let was_secondary = item_resp.secondary_clicked();
                                    if item_resp
                                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                                        .clicked()
                                    {
                                        action_open = Some(i);
                                    }
                                    if was_secondary {
                                        self.context_note_id = Some(note.id);
                                        self.context_menu_pos = ctx
                                            .pointer_interact_pos()
                                            .unwrap_or(ctx.content_rect().center());
                                        self.show_menu = false;
                                    }
                                }
                            }
                        });
                });
        }

        // --- Context menu (right-click, Win11 style) ---
        if self.context_note_id.is_some() {
            let ctx_bg = theme::glass_strong(effective_dark);
            let ctx_stroke = theme::separator_stroke(effective_dark);
            let ctx_text_color = c.text;

            let ctx_id = egui::Id::new("context_menu");
            let ctx_resp = egui::Area::new(ctx_id)
                .fixed_pos(self.context_menu_pos)
                .show(&ctx, |ui| {
                    let shadow = egui::Frame::new().fill(ctx_bg).corner_radius(8.0);
                    shadow.show(ui, |ui| {
                        let ctx_frame = egui::Frame::new()
                            .fill(ctx_bg)
                            .corner_radius(8.0)
                            .stroke(ctx_stroke)
                            .inner_margin(egui::Margin::symmetric(2, 3));
                        ctx_frame.show(ui, |ui| {
                            ui.set_width(180.0);
                            let item_h = 32.0;

                            // Note title header
                            if let Some(note_id) = self.context_note_id {
                                if let Some(note) = self.notes.iter().find(|n| n.id == note_id) {
                                    let title_text = if note.title.len() > 35 {
                                        format!("{}…", &note.title[..35])
                                    } else {
                                        note.title.clone()
                                    };
                                    let (tr, _) = ui.allocate_exact_size(
                                        egui::vec2(ui.available_width(), item_h),
                                        egui::Sense::hover(),
                                    );
                                    ui.painter_at(tr).text(
                                        egui::pos2(tr.min.x + 8.0, tr.center().y),
                                        egui::Align2::LEFT_CENTER,
                                        &title_text,
                                        egui::FontId::proportional(11.0),
                                        ctx_text_color,
                                    );
                                }
                            }
                            ui.separator();

                            // Delete Note
                            let (r, resp) = ui.allocate_exact_size(
                                egui::vec2(ui.available_width(), item_h),
                                egui::Sense::click(),
                            );
                            let p = ui.painter_at(r);
                            if resp.hovered() {
                                p.rect_filled(r.shrink2(egui::vec2(3.0, 0.0)), 6.0, c.danger_soft);
                            }
                            p.text(
                                egui::pos2(r.min.x + 12.0, r.center().y),
                                egui::Align2::LEFT_CENTER,
                                "🗑  Notu Sil",
                                egui::FontId::proportional(13.0),
                                c.danger,
                            );
                            if resp
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .clicked()
                            {
                                if let Some(note_id) = self.context_note_id {
                                    db::delete_note(&self.db, note_id);
                                    self.open_tabs.retain(|t| t.note_id != note_id);
                                    self.selected_tab = if self.open_tabs.is_empty() {
                                        None
                                    } else {
                                        Some(self.open_tabs.len().saturating_sub(1))
                                    };
                                    self.selected_note = None;
                                    self.refresh_list();
                                    self.save_open_tabs();
                                    // if self.open_tabs.is_empty() {
                                    //     ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                                    // }
                                }
                                self.context_note_id = None;
                            }
                        });
                    });
                });
            if ctx_resp.response.clicked_elsewhere()
                || ctx.input(|i| i.key_pressed(egui::Key::Escape))
            {
                self.context_note_id = None;
            }
        }

        // --- Handle sidebar actions ---
        if action_new {
            self.new_note();
            if let Some(note) = self.notes.first().cloned() {
                self.open_tab(&note);
            }
        }
        if let Some(idx) = action_open {
            self.selected_note = Some(idx);
            self.tab_renaming = false;
            if let Some(note) = self.notes.get(idx).cloned() {
                self.open_tab(&note);
            }
        }

        // --- Central Editor ---
        let mut saved: Option<(usize, String)> = None;
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ui, |ui| {
                if let Some(tab_idx) = self.selected_tab {
                    if let Some(tab) = self.open_tabs.get(tab_idx) {
                        if tab.note_id == -1 {
                            // --- Settings UI ---
                            let text_color = c.text;
                            let settings_bg = c.bg;
                            let section_bg = c.surface;

                            let full_rect = ui.available_rect_before_wrap();
                            ui.painter().rect_filled(full_rect, 0.0, settings_bg);
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                    ui.add_space(16.0);

                                    ui.label(
                                        egui::RichText::new("Ayarlar")
                                            .color(c.text)
                                            .size(24.0)
                                            .strong(),
                                    );
                                    ui.add_space(16.0);

                                    // Theme
                                    let theme_frame = egui::Frame::new()
                                        .fill(section_bg)
                                        .corner_radius(10.0)
                                        .inner_margin(egui::Margin::symmetric(14, 12));
                                    theme_frame.show(ui, |ui| {
                                        ui.label(
                                            egui::RichText::new("TEMA")
                                                .color(c.text_secondary)
                                                .size(12.0)
                                                .strong(),
                                        );
                                        ui.add_space(8.0);
                                        ui.horizontal(|ui| {
                                            let themes = [
                                                (AppTheme::Light, "☀  Açık"),
                                                (AppTheme::Dark, "🌙  Koyu"),
                                                (AppTheme::System, "🖥  Sistem"),
                                            ];
                                            let seg_pad = 4.0;
                                            let btn_w = 84.0;
                                            let btn_h = 32.0;
                                            let seg_w = themes.len() as f32 * btn_w + seg_pad * 2.0;
                                            let (seg_rect, _) = ui.allocate_exact_size(
                                                egui::vec2(seg_w, btn_h + seg_pad * 2.0),
                                                egui::Sense::hover(),
                                            );
                                            let sp = ui.painter_at(seg_rect);
                                            sp.rect_filled(seg_rect, 6.0, c.surface_hover);
                                            for (k, (variant, label)) in themes.iter().enumerate() {
                                                let is_selected = self.theme == *variant;
                                                let b_rect = egui::Rect::from_min_size(
                                                    egui::pos2(
                                                        seg_rect.min.x
                                                            + seg_pad
                                                            + k as f32 * btn_w,
                                                        seg_rect.min.y + seg_pad,
                                                    ),
                                                    egui::vec2(btn_w, btn_h),
                                                );
                                                let b_resp = ui.interact(
                                                    b_rect,
                                                    ui.id().with(("theme_btn", k)),
                                                    egui::Sense::click(),
                                                );
                                                let bp = ui.painter_at(b_rect);
                                                if is_selected {
                                                    bp.rect_filled(b_rect, 4.0, c.surface);
                                                    bp.rect_stroke(
                                                        b_rect,
                                                        4.0,
                                                        egui::Stroke::new(1.0, c.accent_border),
                                                        egui::StrokeKind::Inside,
                                                    );
                                                } else if b_resp.hovered() {
                                                    bp.rect_filled(b_rect, 4.0, c.surface_pressed);
                                                }
                                                bp.text(
                                                    b_rect.center(),
                                                    egui::Align2::CENTER_CENTER,
                                                    label,
                                                    egui::FontId::proportional(13.0),
                                                    text_color,
                                                );
                                                if b_resp
                                                    .on_hover_cursor(
                                                        egui::CursorIcon::PointingHand,
                                                    )
                                                    .clicked()
                                                {
                                                    self.theme = *variant;
                                                    settings::save_theme(self.theme);
                                                }
                                            }
                                        });
                                    });

                                    ui.add_space(12.0);

                                    // --- Text Formatting ---
                                    let tf_frame = egui::Frame::new()
                                        .fill(section_bg)
                                        .corner_radius(10.0)
                                        .inner_margin(egui::Margin::symmetric(14, 12));
                                    tf_frame.show(ui, |ui| {
                                        ui.label(
                                            egui::RichText::new("METİN BİÇİMLENDİRME")
                                                .color(c.text_secondary)
                                                .size(12.0)
                                                .strong(),
                                        );
                                        ui.add_space(8.0);

                                        // Font: Consolas (Monospace)
                                        ui.horizontal(|ui| {
                                            ui.label(
                                                egui::RichText::new("Yazı Tipi:")
                                                    .color(text_color)
                                                    .size(13.0),
                                            );
                                            ui.label(
                                                egui::RichText::new("Consolas (Monospace)")
                                                    .color(c.text_secondary)
                                                    .size(13.0),
                                            );
                                        });

                                        // Font size
                                        ui.add_space(4.0);
                                        ui.horizontal(|ui| {
                                            ui.label(
                                                egui::RichText::new("Boyut:")
                                                    .color(text_color)
                                                    .size(13.0),
                                            );
                                            let mut fs = self.font_size;
                                            ui.add(
                                                egui::Slider::new(&mut fs, 8.0..=24.0)
                                                    .clamping(egui::SliderClamping::Always)
                                                    .text(""),
                                            );
                                            if fs != self.font_size {
                                                self.font_size = fs.round();
                                                settings::save_font_size(self.font_size);
                                            }
                                            ui.label(
                                                egui::RichText::new(format!(
                                                    "{:.0}px",
                                                    self.font_size
                                                ))
                                                .color(c.text_secondary)
                                                .size(11.0),
                                            );
                                        });

                                        // Preview
                                        ui.add_space(6.0);
                                        let preview_bg = c.bg;
                                        let preview_frame = egui::Frame::new()
                                            .fill(preview_bg)
                                            .corner_radius(6.0)
                                            .stroke(theme::separator_stroke(effective_dark))
                                            .inner_margin(egui::Margin::symmetric(10, 8));
                                        preview_frame.show(ui, |ui| {
                                            ui.label(
                                                egui::RichText::new(
                                                    "AaBbCc 123!@# — The quick brown fox",
                                                )
                                                .font(egui::FontId::monospace(self.font_size))
                                                .color(text_color),
                                            );
                                        });

                                        // Word wrap toggle
                                        ui.add_space(8.0);
                                        let mut ww = self.word_wrap;
                                        ui.checkbox(&mut ww, "Satır Kaydırma");
                                        if ww != self.word_wrap {
                                            self.word_wrap = ww;
                                            settings::save_word_wrap(self.word_wrap);
                                        }

                                        // Formatting toggle
                                        let mut fe = self.formatting_enabled;
                                        ui.checkbox(&mut fe, "Biçimlendirme (Zengin Metin)");
                                        if fe != self.formatting_enabled {
                                            self.formatting_enabled = fe;
                                            settings::save_formatting_enabled(
                                                self.formatting_enabled,
                                            );
                                        }
                                    });

                                    ui.add_space(8.0);

                                    // --- Advanced ---
                                    let adv_frame = egui::Frame::new()
                                        .fill(section_bg)
                                        .corner_radius(10.0)
                                        .inner_margin(egui::Margin::symmetric(14, 12));
                                    adv_frame.show(ui, |ui| {
                                        ui.label(
                                            egui::RichText::new("GELİŞMİŞ")
                                                .color(c.text_secondary)
                                                .size(12.0)
                                                .strong(),
                                        );
                                        ui.add_space(8.0);

                                        // Restore tabs toggle
                                        let mut rt = self.restore_tabs;
                                        ui.checkbox(&mut rt, "Başlangıçta sekmeleri geri yükle");
                                        if rt != self.restore_tabs {
                                            self.restore_tabs = rt;
                                            settings::save_restore_tabs(self.restore_tabs);
                                        }
                                    });
                                });
                        } else if let Some(tab) = self.open_tabs.get(tab_idx).cloned() {
                            let tab_title = tab.title.clone();
                            let tab_locked = tab.is_locked;
                            let mut edit_content = tab.content.clone();

                            // --- Editor meta header ---
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new(&tab_title)
                                        .color(c.text)
                                        .size(20.0)
                                        .strong(),
                                );
                                if tab_locked {
                                    ui.add_space(8.0);
                                    let (br, _) = ui.allocate_exact_size(
                                        egui::vec2(70.0, 22.0),
                                        egui::Sense::hover(),
                                    );
                                    let bp = ui.painter_at(br);
                                    bp.rect(
                                        br,
                                        11.0,
                                        c.surface,
                                        egui::Stroke::new(1.0, c.border),
                                        egui::StrokeKind::Inside,
                                    );
                                    draw_lock(
                                        &bp,
                                        egui::pos2(br.min.x + 11.0, br.center().y),
                                        c.text_secondary,
                                    );
                                    bp.text(
                                        egui::pos2(br.min.x + 22.0, br.center().y),
                                        egui::Align2::LEFT_CENTER,
                                        "Kilitli",
                                        egui::FontId::proportional(11.0),
                                        c.text_secondary,
                                    );
                                }
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        ui.label(
                                            egui::RichText::new("Otomatik kaydedildi")
                                                .color(c.text_secondary)
                                                .size(11.5),
                                        );
                                    },
                                );
                            });
                            ui.add_space(10.0);

                            // --- Editor frame ---
                            let mut te = egui::TextEdit::multiline(&mut edit_content)
                                .desired_width(f32::INFINITY)
                                .desired_rows(0)
                                .frame(egui::Frame::NONE)
                                .margin(egui::Margin::ZERO);
                            if !self.word_wrap {
                                te = te.desired_rows(1);
                            }
                            if !self.formatting_enabled {
                                te = te.code_editor();
                            }
                            te = te.font(egui::FontId::monospace(self.font_size));
                            let editor_frame = egui::Frame::new()
                                .fill(c.surface)
                                .stroke(egui::Stroke::new(1.0, c.border))
                                .corner_radius(10.0)
                                .inner_margin(egui::Margin::same(16));
                            let resp = editor_frame.show(ui, |ui| {
                                ui.add_sized(ui.available_size(), te)
                            });
                            if resp.inner.changed() {
                                saved = Some((tab_idx, edit_content));
                            }
                        }
                    }
                } else {
                    let avail = ui.available_height();
                    ui.add_space((avail * 0.3).max(60.0));
                    ui.vertical_centered(|ui| {
                        ui.label(
                            egui::RichText::new("uNote")
                                .color(c.accent)
                                .size(56.0)
                                .strong(),
                        );
                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new("Bir not seçin veya yeni bir not oluşturun")
                                .color(c.text_secondary)
                                .size(14.0),
                        );
                    });
                }
            });
        if let Some((idx, content)) = saved {
            self.save_tab_content(idx, &content);
        }

        if self.search_query != prev_query {
            self.refresh_list();
        }

        // (window border removed)
    }
}

fn draw_lock(painter: &egui::Painter, center: egui::Pos2, color: egui::Color32) {
    let size = 10.0;
    let x = center.x - size / 2.0;
    let y = center.y - size / 2.0;
    let stroke = egui::Stroke::new(1.5, color);
    painter.circle_stroke(egui::pos2(center.x, y + 4.5), 3.0, stroke);
    painter.rect_filled(
        egui::Rect::from_min_size(
            egui::pos2(x + 0.5, y + 4.0),
            egui::vec2(size - 1.0, size - 4.0),
        ),
        egui::CornerRadius::same(2),
        color,
    );
}
