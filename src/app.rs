use chrono::Local;
use eframe::egui;
use rusqlite::Connection;

use crate::crypto;
use crate::db;
use crate::models::{note_avatar_color, AppTheme, NoteTab};
use crate::settings;

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
        };
        app.refresh_list();

        if app.notes.is_empty() {
            app.new_note();
        }

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
        let ids: Vec<i64> = self.open_tabs.iter().map(|t| t.note_id).collect();
        settings::save_open_tab_ids(&ids);
    }

    pub fn save_tab_content(&mut self, tab_idx: usize, new_content: &str) {
        let note_id = self.open_tabs[tab_idx].note_id;
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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.theme {
            AppTheme::Light => ctx.set_visuals(egui::Visuals::light()),
            AppTheme::Dark => ctx.set_visuals(egui::Visuals::dark()),
        }

        let prev_query = self.search_query.clone();
        let mut action_open: Option<usize> = None;
        let mut action_new = false;

        // --- TAB BAR (en üst) ---
        let (bar_bg, active_bg, inactive_bg, active_text, inactive_text) = if self.theme == AppTheme::Dark {
            (
                egui::Color32::from_rgb(22, 22, 26),
                egui::Color32::from_rgb(30, 30, 36),
                egui::Color32::from_rgb(26, 26, 32),
                egui::Color32::WHITE,
                egui::Color32::from_gray(160),
            )
        } else {
            (
                egui::Color32::from_rgb(235, 235, 240),
                egui::Color32::WHITE,
                egui::Color32::from_rgb(245, 245, 248),
                egui::Color32::BLACK,
                egui::Color32::from_gray(90),
            )
        };

        let mut menu_btn_pos = egui::Pos2::ZERO;

        egui::TopBottomPanel::top("tab_bar")
            .min_height(34.0)
            .frame(egui::Frame::new().fill(bar_bg))
            .show(ctx, |ui| {
                let mut to_close: Option<usize> = None;
                let mut tab_rects: Vec<egui::Rect> = Vec::new();
                let mut tab_rename_commit = false;

                // --- Window drag handle + double-click maximize ---
                let drag_sense = ui.interact(
                    egui::Rect::from_min_max(ui.max_rect().min, egui::pos2(ui.max_rect().max.x - 36.0, ui.max_rect().max.y)),
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
                    // --- Menu button ---
                    let menu_btn_size = egui::vec2(34.0, 30.0);
                    let (menu_btn_rect, menu_btn_resp) =
                        ui.allocate_exact_size(menu_btn_size, egui::Sense::click());
                    menu_btn_pos = menu_btn_rect.left_bottom();

                    let menu_hover_bg = if self.theme == AppTheme::Dark {
                        egui::Color32::from_rgb(50, 50, 60)
                    } else {
                        egui::Color32::from_rgb(210, 210, 220)
                    };
                    let m_painter = ui.painter_at(menu_btn_rect);
                    if menu_btn_resp.hovered() || self.show_menu {
                        m_painter.rect_filled(menu_btn_rect, 4.0, menu_hover_bg);
                    }
                    let menu_btn_text_color = if self.theme == AppTheme::Dark {
                        egui::Color32::WHITE
                    } else {
                        egui::Color32::BLACK
                    };
                    // Hamburger icon (3 lines)
                    let mc = menu_btn_rect.center();
                    let mw = 14.0;
                    let mg = 4.0;
                    let line_stroke = egui::Stroke::new(2.0, menu_btn_text_color);
                    m_painter.line_segment(
                        [egui::pos2(mc.x - mw / 2.0, mc.y - mg), egui::pos2(mc.x + mw / 2.0, mc.y - mg)],
                        line_stroke,
                    );
                    m_painter.line_segment(
                        [egui::pos2(mc.x - mw / 2.0, mc.y), egui::pos2(mc.x + mw / 2.0, mc.y)],
                        line_stroke,
                    );
                    m_painter.line_segment(
                        [egui::pos2(mc.x - mw / 2.0, mc.y + mg), egui::pos2(mc.x + mw / 2.0, mc.y + mg)],
                        line_stroke,
                    );
                    if menu_btn_resp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                        self.show_menu = !self.show_menu;
                        self.context_note_id = None;
                    }

                    // --- Render tabs ---
                    for i in 0..self.open_tabs.len() {
                        let text = self.open_tabs[i].title.clone();
                        let is_selected = self.selected_tab == Some(i);

                        let text_w = ctx.fonts(|f| {
                            let galley = f.layout_no_wrap(
                                text.as_str().into(),
                                egui::FontId::proportional(13.0),
                                egui::Color32::WHITE,
                            );
                            galley.rect.width()
                        });
                        let tab_w = (text_w + 50.0).max(60.0).min(200.0);
                        let tab_size = egui::vec2(tab_w, 30.0);
                        let (tab_rect, tab_resp) =
                            ui.allocate_exact_size(tab_size, egui::Sense::click_and_drag());
                        tab_rects.push(tab_rect);

                        let corner = if is_selected {
                            egui::CornerRadius { nw: 6, ne: 6, sw: 0, se: 0 }
                        } else {
                            egui::CornerRadius::same(6)
                        };
                        let tab_bg = if is_selected { active_bg } else { inactive_bg };
                        let text_color = if is_selected { active_text } else { inactive_text };

                        let painter = ui.painter_at(tab_rect);
                        painter.rect_filled(tab_rect, corner, tab_bg);

                        // Close button rect (right side of tab)
                        let close_rect = egui::Rect::from_min_size(
                            egui::pos2(tab_rect.max.x - 30.0, tab_rect.center().y - 14.0),
                            egui::vec2(28.0, 28.0),
                        );
                        let close_hover_bg = if self.theme == AppTheme::Dark {
                            egui::Color32::from_rgb(80, 40, 40)
                        } else {
                            egui::Color32::from_rgb(230, 180, 180)
                        };

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
                                    .frame(true),
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
                            // Title text
                            painter.text(
                                egui::pos2(tab_rect.min.x + 10.0, tab_rect.center().y),
                                egui::Align2::LEFT_CENTER,
                                &text,
                                egui::FontId::proportional(13.0),
                                text_color,
                            );
                        }

                        // Close X
                        let pointer_pos = ctx.pointer_interact_pos();
                        let is_close_hover = pointer_pos.map_or(false, |p| close_rect.contains(p));
                        if is_close_hover {
                            painter.rect_filled(close_rect, 4.0, close_hover_bg);
                        }
                        let xc = close_rect.center();
                        let xs = 5.0;
                        let x_stroke = egui::Stroke::new(2.0, text_color);
                        painter.line_segment(
                            [egui::pos2(xc.x - xs, xc.y - xs), egui::pos2(xc.x + xs, xc.y + xs)],
                            x_stroke,
                        );
                        painter.line_segment(
                            [egui::pos2(xc.x + xs, xc.y - xs), egui::pos2(xc.x - xs, xc.y + xs)],
                            x_stroke,
                        );

                        // Tab interactions
                        if tab_resp.drag_started() {
                            self.drag_idx = Some(i);
                        }

                        if tab_resp.clicked() {
                            let is_close =
                                pointer_pos.map_or(false, |p| close_rect.contains(p));
                            if is_close {
                                to_close = Some(i);
                            } else if is_selected {
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
                    let btn_text_color = if self.theme == AppTheme::Dark {
                        egui::Color32::WHITE
                    } else {
                        egui::Color32::BLACK
                    };
                    let painter = ui.painter_at(plus_rect);
                    if plus_hover {
                        let plus_hover_bg = if self.theme == AppTheme::Dark {
                            egui::Color32::from_rgb(50, 50, 60)
                        } else {
                            egui::Color32::from_rgb(210, 210, 220)
                        };
                        painter.rect_filled(plus_rect, 4.0, plus_hover_bg);
                    }
                    painter.text(
                        plus_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "+",
                        egui::FontId::proportional(20.0),
                        btn_text_color,
                    );
                    if plus_resp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                        action_new = true;
                    }

                });

                // --- Close (exit) button on far right ---
                let btn_text_color = if self.theme == AppTheme::Dark {
                    egui::Color32::WHITE
                } else {
                    egui::Color32::BLACK
                };
                let close_btn_size = egui::vec2(34.0, 30.0);
                let close_x = ui.available_rect_before_wrap().right() - close_btn_size.x;
                let close_y = ui.min_rect().min.y;
                let close_rect = egui::Rect::from_min_size(
                    egui::pos2(close_x, close_y),
                    close_btn_size,
                );
                let close_resp = ui.allocate_rect(close_rect, egui::Sense::click());
                let pointer_pos = ctx.pointer_interact_pos();
                let close_hover = pointer_pos.map_or(false, |p| close_rect.contains(p));
                let close_painter = ui.painter_at(close_rect);
                if close_hover {
                    close_painter.rect_filled(close_rect, 0.0, egui::Color32::from_rgb(232, 17, 35));
                }
                let xc = close_rect.center();
                let xs2 = 7.0;
                let x_stroke2 = egui::Stroke::new(2.0, if close_hover { egui::Color32::WHITE } else { btn_text_color });
                close_painter.line_segment(
                    [egui::pos2(xc.x - xs2, xc.y - xs2), egui::pos2(xc.x + xs2, xc.y + xs2)],
                    x_stroke2,
                );
                close_painter.line_segment(
                    [egui::pos2(xc.x + xs2, xc.y - xs2), egui::pos2(xc.x - xs2, xc.y + xs2)],
                    x_stroke2,
                );
                if close_resp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                    std::process::exit(0);
                }

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
                                        self.selected_tab = Some(to.min(self.open_tabs.len().saturating_sub(1)));
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
                    if content_empty {
                        db::delete_note(&self.db, note_id);
                        self.refresh_list();
                    }
                    self.save_open_tabs();
                    if self.open_tabs.is_empty() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                }
            });

        // --- Menu dropdown (Win11 modern) ---
        if self.show_menu {
            let menu_text_color = if self.theme == AppTheme::Dark {
                egui::Color32::WHITE
            } else {
                egui::Color32::BLACK
            };
            let menu_bg = if self.theme == AppTheme::Dark {
                egui::Color32::from_rgb(32, 32, 38)
            } else {
                egui::Color32::WHITE
            };
            let menu_hover_bg = if self.theme == AppTheme::Dark {
                egui::Color32::from_rgba_premultiplied(128, 128, 128, 30)
            } else {
                egui::Color32::from_rgba_premultiplied(128, 128, 128, 18)
            };
            let menu_stroke = if self.theme == AppTheme::Dark {
                egui::Stroke::new(1.0, egui::Color32::from_gray(60))
            } else {
                egui::Stroke::new(1.0, egui::Color32::from_gray(200))
            };

            let menu_frame = egui::Frame::new()
                .fill(menu_bg)
                .corner_radius(8.0)
                .stroke(menu_stroke)
                .inner_margin(egui::Margin::symmetric(4, 4));

            let menu_resp = egui::Area::new(egui::Id::new("menu_dropdown"))
                .fixed_pos(menu_btn_pos)
                .show(ctx, |ui| {
                    let shadow = egui::Frame::new()
                        .fill(menu_bg)
                        .corner_radius(8.0);
                    shadow.show(ui, |ui| {
                        menu_frame.show(ui, |ui| {
                            ui.set_min_width(200.0);
                            let item_h = 32.0;

                            // Light
                            let is_light = self.theme == AppTheme::Light;
                            let (lr, lrsp) = ui.allocate_exact_size(
                                egui::vec2(ui.available_width(), item_h),
                                egui::Sense::click(),
                            );
                            if lrsp.hovered() {
                                ui.painter_at(lr).rect_filled(
                                    lr.shrink2(egui::vec2(4.0, 0.0)), 4.0, menu_hover_bg,
                                );
                            }
                            ui.painter_at(lr).text(
                                egui::pos2(lr.min.x + 14.0, lr.center().y),
                                egui::Align2::LEFT_CENTER,
                                if is_light { "✓  ☀  Light" } else { "   ☀  Light" },
                                egui::FontId::proportional(13.0),
                                menu_text_color,
                            );
                            if lrsp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                self.theme = AppTheme::Light;
                                settings::save_theme(self.theme);
                                self.show_menu = false;
                            }

                            // Dark
                            let is_dark = self.theme == AppTheme::Dark;
                            let (dr, drsp) = ui.allocate_exact_size(
                                egui::vec2(ui.available_width(), item_h),
                                egui::Sense::click(),
                            );
                            if drsp.hovered() {
                                ui.painter_at(dr).rect_filled(
                                    dr.shrink2(egui::vec2(4.0, 0.0)), 4.0, menu_hover_bg,
                                );
                            }
                            ui.painter_at(dr).text(
                                egui::pos2(dr.min.x + 14.0, dr.center().y),
                                egui::Align2::LEFT_CENTER,
                                if is_dark { "✓  🌙  Dark" } else { "   🌙  Dark" },
                                egui::FontId::proportional(13.0),
                                menu_text_color,
                            );
                            if drsp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                self.theme = AppTheme::Dark;
                                settings::save_theme(self.theme);
                                self.show_menu = false;
                            }

                            ui.separator();

                            // Delete Note
                            let (dlr, dlrsp) = ui.allocate_exact_size(
                                egui::vec2(ui.available_width(), item_h),
                                egui::Sense::click(),
                            );
                            if dlrsp.hovered() {
                                ui.painter_at(dlr).rect_filled(
                                    dlr.shrink2(egui::vec2(4.0, 0.0)), 4.0, menu_hover_bg,
                                );
                            }
                            ui.painter_at(dlr).text(
                                egui::pos2(dlr.min.x + 14.0, dlr.center().y),
                                egui::Align2::LEFT_CENTER,
                                "🗑  Delete Note",
                                egui::FontId::proportional(13.0),
                                if self.theme == AppTheme::Dark {
                                    egui::Color32::from_rgb(255, 100, 100)
                                } else {
                                    egui::Color32::from_rgb(200, 50, 50)
                                },
                            );
                            if dlrsp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                self.delete_note();
                                self.show_menu = false;
                            }

                            ui.separator();

                            // Exit
                            let (exr, exrsp) = ui.allocate_exact_size(
                                egui::vec2(ui.available_width(), item_h),
                                egui::Sense::click(),
                            );
                            if exrsp.hovered() {
                                ui.painter_at(exr).rect_filled(
                                    exr.shrink2(egui::vec2(4.0, 0.0)), 4.0, menu_hover_bg,
                                );
                            }
                            ui.painter_at(exr).text(
                                egui::pos2(exr.min.x + 14.0, exr.center().y),
                                egui::Align2::LEFT_CENTER,
                                "✕  Exit",
                                egui::FontId::proportional(13.0),
                                menu_text_color,
                            );
                            if exrsp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                std::process::exit(0);
                            }
                        });
                    });
                });

            if menu_resp.response.clicked_elsewhere()
                || ctx.input(|i| i.key_pressed(egui::Key::Escape))
            {
                self.show_menu = false;
            }
        }

        // --- Sidebar ---
        let sidebar_anim = ctx.animate_bool(egui::Id::new("sidebar_anim"), !self.sidebar_collapsed);
        let sidebar_w = 40.0 + 220.0 * sidebar_anim;

        let notes_clone = self.notes.clone();
        let master_pwd = self.master_password.clone();

        egui::SidePanel::left("sidebar")
            .resizable(false)
            .exact_width(sidebar_w)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let btn_text = if self.sidebar_collapsed { "☰" } else { "≡" };
                    let btn_size = egui::vec2(28.0, 28.0);
                    let (btn_rect, btn_resp) =
                        ui.allocate_exact_size(btn_size, egui::Sense::click());
                    let btn_hover_bg = if btn_resp.hovered() {
                        if self.theme == AppTheme::Dark {
                            egui::Color32::from_rgb(50, 50, 60)
                        } else {
                            egui::Color32::from_rgb(210, 210, 220)
                        }
                    } else {
                        egui::Color32::TRANSPARENT
                    };
                    let painter = ui.painter_at(btn_rect);
                    if btn_hover_bg != egui::Color32::TRANSPARENT {
                        painter.rect_filled(btn_rect, 4.0, btn_hover_bg);
                    }
                    let btn_text_color = if self.theme == AppTheme::Dark {
                        egui::Color32::WHITE
                    } else {
                        egui::Color32::BLACK
                    };
                    painter.text(
                        btn_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        btn_text,
                        egui::FontId::proportional(18.0),
                        btn_text_color,
                    );
                    if btn_resp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                        self.sidebar_collapsed = !self.sidebar_collapsed;
                    }

                    if !self.sidebar_collapsed {
                        ui.add(
                            egui::TextEdit::singleline(&mut self.search_query)
                                .hint_text("🔍 Ara...")
                                .desired_width(f32::INFINITY),
                        );
                    }
                });

                if !self.sidebar_collapsed {
                    ui.add_space(4.0);
                    if ui.button("➕ Yeni Not").clicked() {
                        action_new = true;
                    }
                    ui.add_space(4.0);
                    ui.separator();
                }

                egui::ScrollArea::vertical().show(ui, |ui| {
                    if self.sidebar_collapsed {
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

                            let (bg, fg) = note_avatar_color(i, self.theme);
                            egui::Frame::new()
                                .fill(bg)
                                .corner_radius(6.0)
                                .inner_margin(egui::Margin::symmetric(1, 4))
                                .show(ui, |ui| {
                                    ui.vertical_centered(|ui| {
                                        ui.set_max_width(34.0);
                                        let label = egui::Label::new(
                                            egui::RichText::new(&avatar_text)
                                                .color(fg)
                                                .size(9.0)
                                                .strong(),
                                        )
                                        .sense(egui::Sense::click());
                                        if ui
                                            .add(label)
                                            .on_hover_cursor(egui::CursorIcon::PointingHand)
                                            .clicked()
                                        {
                                            action_open = Some(i);
                                        }
                                    });
                                });
                        }
                        if notes_clone.is_empty() {
                            ui.add_space(8.0);
                            ui.vertical_centered(|ui| {
                                ui.label(
                                    egui::RichText::new("-")
                                        .color(egui::Color32::from_gray(120))
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
                                        .color(egui::Color32::from_gray(160)),
                                );
                            });
                            return;
                        }
                        for (i, note) in notes_clone.iter().enumerate() {
                            let is_sel = self.selected_note == Some(i);

                            let display_title = if note.is_locked {
                                "🔒 ".to_string()
                                    + &crypto::decrypt(&note.title, &master_pwd)
                            } else {
                                note.title.clone()
                            };

                            let item_w = ui.available_width();
                            let (item_rect, item_resp) = ui.allocate_exact_size(
                                egui::vec2(item_w.max(0.0), 26.0),
                                egui::Sense::click(),
                            );

                            let item_bg = if is_sel {
                                if self.theme == AppTheme::Dark {
                                    egui::Color32::from_rgb(50, 50, 60)
                                } else {
                                    egui::Color32::from_rgb(225, 225, 235)
                                }
                            } else if item_resp.hovered() {
                                if self.theme == AppTheme::Dark {
                                    egui::Color32::from_rgb(40, 40, 50)
                                } else {
                                    egui::Color32::from_rgb(235, 235, 245)
                                }
                            } else {
                                egui::Color32::TRANSPARENT
                            };

                            let painter = ui.painter_at(item_rect);
                            if item_bg != egui::Color32::TRANSPARENT {
                                painter.rect_filled(item_rect, 4.0, item_bg);
                            }
                            let text_color = if self.theme == AppTheme::Dark {
                                egui::Color32::WHITE
                            } else {
                                egui::Color32::BLACK
                            };
                            painter.text(
                                egui::pos2(item_rect.min.x + 8.0, item_rect.center().y),
                                egui::Align2::LEFT_CENTER,
                                &display_title,
                                egui::FontId::proportional(13.0),
                                text_color,
                            );

                            let was_secondary = item_resp.secondary_clicked();
                            if item_resp
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .clicked()
                            {
                                action_open = Some(i);
                            }
                            if was_secondary {
                                self.context_note_id = Some(note.id);
                                self.context_menu_pos = ctx.pointer_interact_pos()
                                    .unwrap_or(ctx.screen_rect().center());
                                self.show_menu = false;
                            }
                        }
                    }
                });
            });

        // --- Context menu (right-click, Win11 style) ---
        if self.context_note_id.is_some() {
            let ctx_bg = if self.theme == AppTheme::Dark {
                egui::Color32::from_rgb(32, 32, 38)
            } else {
                egui::Color32::WHITE
            };
            let ctx_stroke = if self.theme == AppTheme::Dark {
                egui::Stroke::new(1.0, egui::Color32::from_gray(60))
            } else {
                egui::Stroke::new(1.0, egui::Color32::from_gray(200))
            };
            let ctx_hover_bg = if self.theme == AppTheme::Dark {
                egui::Color32::from_rgba_premultiplied(128, 128, 128, 30)
            } else {
                egui::Color32::from_rgba_premultiplied(128, 128, 128, 18)
            };

            let ctx_id = egui::Id::new("context_menu");
            let ctx_resp = egui::Area::new(ctx_id)
                .fixed_pos(self.context_menu_pos)
                .show(ctx, |ui| {
                    let shadow = egui::Frame::new().fill(ctx_bg).corner_radius(8.0);
                    shadow.show(ui, |ui| {
                        let ctx_frame = egui::Frame::new()
                            .fill(ctx_bg)
                            .corner_radius(8.0)
                            .stroke(ctx_stroke)
                            .inner_margin(egui::Margin::symmetric(4, 4));
                        ctx_frame.show(ui, |ui| {
                            ui.set_min_width(200.0);
                            let item_h = 32.0;

                            // Delete Note
                            let (r, resp) = ui.allocate_exact_size(
                                egui::vec2(ui.available_width(), item_h),
                                egui::Sense::click(),
                            );
                            let p = ui.painter_at(r);
                            if resp.hovered() {
                                p.rect_filled(r.shrink2(egui::vec2(4.0, 0.0)), 4.0, ctx_hover_bg);
                            }
                            p.text(
                                egui::pos2(r.min.x + 14.0, r.center().y),
                                egui::Align2::LEFT_CENTER,
                                "🗑  Delete Note",
                                egui::FontId::proportional(13.0),
                                if self.theme == AppTheme::Dark {
                                    egui::Color32::from_rgb(255, 100, 100)
                                } else {
                                    egui::Color32::from_rgb(200, 50, 50)
                                },
                            );
                            if resp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
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
                                    if self.open_tabs.is_empty() {
                                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                                    }
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
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(tab_idx) = self.selected_tab {
                if let Some(txt) = self.open_tabs.get(tab_idx).map(|t| t.content.clone()) {
                    let mut edit_content = txt;
                    let response = ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::multiline(&mut edit_content)
                            .font(egui::TextStyle::Monospace),
                    );
                    if response.changed() {
                        saved = Some((tab_idx, edit_content));
                    }
                }
            } else {
                let avail = ui.available_height();
                ui.add_space((avail * 0.35).max(60.0));
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new("uNote")
                            .color(egui::Color32::from_gray(180))
                            .size(36.0),
                    );
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new("Bir not seçin veya yeni bir not oluşturun")
                            .color(egui::Color32::from_gray(140)),
                    );
                });
            }
        });
        if let Some((idx, content)) = saved {
            self.save_tab_content(idx, &content);
        }

        // --- Status Bar ---
        egui::TopBottomPanel::bottom("status_bar")
            .min_height(22.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(format!("{} notes", self.notes.len()));
                    if let Some(tab_idx) = self.selected_tab {
                        if let Some(tab) = self.open_tabs.get(tab_idx) {
                            ui.separator();
                            ui.label(&tab.title);
                        }
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let icon = if self.theme == AppTheme::Light {
                            "☀"
                        } else {
                            "🌙"
                        };
                        let name = if self.theme == AppTheme::Light {
                            "Light"
                        } else {
                            "Dark"
                        };
                        ui.label(format!("{} {}", icon, name));
                    });
                });
            });

        if self.search_query != prev_query {
            self.refresh_list();
        }
    }
}
