use lever_core::app::{App, Context, UpdateContext};
use lever_core::types::{Color, Point, SideOffsets, TextureId};
use lever_core::widget::Widget;
use lever_core::widgets::{BoxWidget, Flex, Overlay, Scroll, Spacer, ThemeToggle};
use lever_windowing::{AppConfig, Application};

mod sections;

#[derive(Debug, Clone)]
pub enum Message {
    ButtonClicked(String),
    ToggleChanged(bool),
    SliderChanged(f32),
    DiscreteSliderChanged(f32),
    PercentSliderChanged(f32),
    CheckboxChanged(bool),
    TextInput1Changed(String, usize),
    TextInput2Changed(String, usize),
    TextInput3Changed(String, usize),
    TextInput4Changed(String, usize),
    ThemeModeChanged(lever_core::theme::ThemeMode),
    TogglePulse(bool),
    ToggleFloat(bool),
    Scrolled(Point),
    GridScrolled(Point),
    DropdownSelected(usize),
    RadioSelected(usize),
    OpenModal,
    CloseModal,
    TabChanged(usize),
    TabPillChanged(usize),
    TabFullChanged(usize),
}

pub struct GalleryApp {
    pub toggle_on: bool,
    pub slider_value: f32,
    pub discrete_slider: f32,
    pub percent_slider: f32,
    pub checkbox_checked: bool,
    pub text_input1: String,
    pub cursor1: usize,
    pub text_input2: String,
    pub cursor2: usize,
    pub text_input3: String,
    pub cursor3: usize,
    pub text_input4: String,
    pub cursor4: usize,
    pub theme_mode: lever_core::theme::ThemeMode,
    pub is_pulsing: bool,
    pub is_floating: bool,
    pub time: f32,
    pub scroll_offset: Point,
    pub grid_offset: Point,
    pub selected_dropdown: Option<usize>,
    pub selected_radio: usize,
    pub test_image: Option<TextureId>,
    pub is_modal_open: bool,
    pub active_tab: usize,
    pub active_pill_tab: usize,
    pub active_full_tab: usize,
}

impl Default for GalleryApp {
    fn default() -> Self {
        Self {
            toggle_on: true,
            slider_value: 0.5,
            discrete_slider: 50.0,
            percent_slider: 0.75,
            checkbox_checked: true,
            text_input1: "Hello, Lever!".to_string(),
            cursor1: 13,
            text_input2: String::new(),
            cursor2: 0,
            text_input3: String::new(),
            cursor3: 0,
            text_input4: String::new(),
            cursor4: 0,
            theme_mode: lever_core::theme::ThemeMode::Dark,
            is_pulsing: false,
            is_floating: false,
            time: 0.0,
            scroll_offset: Point::default(),
            grid_offset: Point::default(),
            selected_dropdown: Some(0),
            selected_radio: 0,
            test_image: None,
            is_modal_open: false,
            active_tab: 0,
            active_pill_tab: 0,
            active_full_tab: 0,
        }
    }
}

impl App for GalleryApp {
    type Message = Message;

    fn init(&mut self, ctx: &mut Context<Self::Message>) {
        let tex = ctx.load_image(include_bytes!("test_pattern.png"));
        self.test_image = Some(tex);
    }

    fn update(&mut self, message: Self::Message, context: &mut UpdateContext) {
        match message {
            Message::TextInput1Changed(text, cursor) => {
                self.text_input1 = text;
                self.cursor1 = cursor;
            }
            Message::TextInput2Changed(text, cursor) => {
                self.text_input2 = text;
                self.cursor2 = cursor;
            }
            Message::TextInput3Changed(text, cursor) => {
                self.text_input3 = text;
                self.cursor3 = cursor;
            }
            Message::TextInput4Changed(text, cursor) => {
                self.text_input4 = text;
                self.cursor4 = cursor;
            }
            Message::ButtonClicked(label) => {
                println!("Button clicked: {}", label);
            }
            Message::ToggleChanged(val) => {
                self.toggle_on = val;
            }
            Message::SliderChanged(val) => {
                self.slider_value = val;
            }
            Message::DiscreteSliderChanged(val) => {
                self.discrete_slider = val;
            }
            Message::PercentSliderChanged(val) => {
                self.percent_slider = val;
            }
            Message::CheckboxChanged(val) => {
                self.checkbox_checked = val;
            }
            Message::ThemeModeChanged(mode) => {
                self.theme_mode = mode;
                context.set_theme(mode);
            }
            Message::TogglePulse(val) => {
                self.is_pulsing = val;
            }
            Message::ToggleFloat(val) => {
                self.is_floating = val;
            }
            Message::Scrolled(offset) => {
                self.scroll_offset = offset;
            }
            Message::GridScrolled(offset) => {
                self.grid_offset = offset;
            }
            Message::DropdownSelected(idx) => {
                self.selected_dropdown = Some(idx);
            }
            Message::RadioSelected(idx) => {
                self.selected_radio = idx;
            }
            Message::OpenModal => {
                self.is_modal_open = true;
            }
            Message::CloseModal => {
                self.is_modal_open = false;
            }
            Message::TabChanged(idx) => {
                self.active_tab = idx;
            }
            Message::TabPillChanged(idx) => {
                self.active_pill_tab = idx;
            }
            Message::TabFullChanged(idx) => {
                self.active_full_tab = idx;
            }
        }
    }

    fn tick(&mut self, dt: f32) {
        self.time += dt;
    }

    fn view(&self) -> Box<dyn Widget<Self::Message>> {
        let theme = lever_core::theme::Theme::for_mode(self.theme_mode);

        let header = Box::new(Flex::row(vec![
            Box::new(
                Flex::column(vec![
                    Box::new(lever_core::widgets::Label::new("Lever UI Gallery").with_size(42.0)),
                    Box::new(
                        lever_core::widgets::Label::new(
                            "A modern, high-performance UI toolkit for Rust.",
                        )
                        .with_size(16.0)
                        .with_color(theme.text_muted),
                    ),
                ])
                .with_flex(1),
            ),
            Box::new(
                ThemeToggle::new("theme-toggle", self.theme_mode)
                    .on_changed(|mode| Message::ThemeModeChanged(mode)),
            ),
        ]));

        let content = Flex::column(vec![
            header,
            Box::new(Spacer::new().with_size(10.0, 48.0)),
            sections::typography::view(self, &theme),
            Box::new(Spacer::new().with_size(10.0, 24.0)),
            sections::animation::view(self, &theme),
            Box::new(Spacer::new().with_size(10.0, 24.0)),
            sections::media::view(self, &theme),
            Box::new(Spacer::new().with_size(10.0, 24.0)),
            sections::controls::view(self, &theme),
            Box::new(Spacer::new().with_size(10.0, 40.0)),
            sections::inputs::view(self, &theme),
            Box::new(Spacer::new().with_size(10.0, 40.0)),
            sections::data_grid::view(self, &theme),
            Box::new(Spacer::new().with_size(10.0, 40.0)),
            sections::progress::view(self, &theme),
            Box::new(Spacer::new().with_size(10.0, 40.0)),
            sections::navigation::view(self, &theme),
            Box::new(Spacer::new().with_size(10.0, 40.0)),
            sections::layout::view(self, &theme),
            Box::new(Spacer::new().with_size(10.0, 40.0)),
            sections::layout_advanced::view(self, &theme),
            Box::new(Spacer::new().with_size(10.0, 100.0)),
        ])
        .with_gap(24.0)
        .with_cross_axis_alignment(lever_core::layout::CrossAxisAlignment::Stretch);

        let scroll = Scroll::new(
            "main-scroll",
            Box::new(
                BoxWidget::new(Color::TRANSPARENT)
                    .with_padding(
                        SideOffsets::default()
                            .with_horizontal(40.0)
                            .with_vertical(60.0),
                    )
                    .with_child(Box::new(content)),
            ),
        )
        .with_flex(1)
        .on_scroll(|offset| Message::Scrolled(offset))
        .with_offset(self.scroll_offset);

        let root = lever_core::widgets::ConstraintLayout::new()
            .with_id("root")
            .with_child(
                Box::new(BoxWidget::new(theme.background).with_child(Box::new(scroll))),
                |set| set.fill_parent(),
            );

        if self.is_modal_open {
            Box::new(
                root.with_child(
                    Box::new(
                        Overlay::new()
                            .with_child(
                                BoxWidget::new(theme.surface)
                                    .with_radius(12.0)
                                    .with_size(400.0, 300.0)
                                    .with_padding(SideOffsets::all(32.0))
                                    .with_child(Box::new(
                                        Flex::column(vec![
                                            Box::new(lever_core::widgets::Label::new("Example Modal").with_size(24.0)),
                                            Box::new(Spacer::new().with_size(10.0, 16.0)),
                                            Box::new(
                                                lever_core::widgets::Label::new(
                                                    "This is a modal overlay that blocks interaction with the main content.",
                                                )
                                                .with_color(theme.text_muted),
                                            ),
                                            Box::new(Spacer::flex()),
                                            Box::new(
                                                lever_core::widgets::Button::new("modal-close", "Close Modal")
                                                    .with_variant(lever_core::widgets::ButtonVariant::Primary)
                                                    .on_click(|| Message::CloseModal),
                                            ),
                                        ])
                                        .with_cross_axis_alignment(
                                            lever_core::layout::CrossAxisAlignment::Stretch,
                                        ),
                                    )),
                            )
                            .on_dismiss(|| Message::CloseModal),
                    ),
                    |set| set.fill_parent(),
                ),
            ) as Box<dyn Widget<Self::Message>>
        } else {
            Box::new(root) as Box<dyn Widget<Self::Message>>
        }
    }
}

fn main() {
    let app = Application::new(
        AppConfig {
            title: "Lever UI Gallery".to_string(),
            width: 1200,
            height: 900,
            ..Default::default()
        },
        GalleryApp::default(),
    );

    app.run();
}
