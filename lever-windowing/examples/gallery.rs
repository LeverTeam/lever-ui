use lever_core::app::{App, Context, UpdateContext};
use lever_core::layout::{Alignment, GridTrack};
use lever_core::types::{Color, ImageFit, Point, SideOffsets, TextureId};
use lever_core::widget::Widget;
use lever_core::widgets::{
    child, AnimatedOpacity, AnimatedScale, AnimatedTranslation, BoxWidget, Button, ButtonVariant,
    Checkbox, CircularProgress, ConstraintLayout, Dropdown, Flex, Grid, ImageWidget, Label,
    Overlay, ProgressBar, RadioGroup, RadioOption, Scroll, Slider, Spacer, SpacerOrientation,
    TabItem, TabStyle, Tabs, TextInput, ThemeToggle, Toggle, PARENT,
};
use lever_core::MainAxisAlignment;
use lever_windowing::{AppConfig, Application};

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
    DropdownSelected(usize),
    RadioSelected(usize),
    OpenModal,
    CloseModal,
    TabChanged(usize),
    TabPillChanged(usize),
    TabFullChanged(usize),
}

pub struct GalleryApp {
    toggle_on: bool,
    slider_value: f32,
    discrete_slider: f32,
    percent_slider: f32,
    checkbox_checked: bool,
    text_input1: String,
    cursor1: usize,
    text_input2: String,
    cursor2: usize,
    text_input3: String,
    cursor3: usize,
    text_input4: String,
    cursor4: usize,
    theme_mode: lever_core::theme::ThemeMode,
    is_pulsing: bool,
    is_floating: bool,
    time: f32,
    scroll_offset: Point,
    selected_dropdown: Option<usize>,
    selected_radio: usize,
    test_image: Option<TextureId>,
    is_modal_open: bool,
    active_tab: usize,
    active_pill_tab: usize,
    active_full_tab: usize,
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

        let animated_pulse = lever_core::animated::animated_spring(
            "pulse-scale",
            if self.is_pulsing { 1.1 } else { 1.0 },
            lever_core::animation::Spring::SMOOTH,
        );

        let animated_float = lever_core::animated::animated_spring(
            "float-val",
            if self.is_floating { -20.0 } else { 0.0 },
            lever_core::animation::Spring::SMOOTH,
        );

        let animated_opacity = lever_core::animated::animated_spring(
            "fade-val",
            if self.toggle_on { 1.0 } else { 0.4 },
            lever_core::animation::Spring::SMOOTH,
        );

        let section_card = |title: &str,
                            subtitle: &str,
                            child: Box<dyn Widget<Self::Message>>,
                            flex: u32| {
            Box::new(
                BoxWidget::new(theme.surface)
                    .with_radius(12.0)
                    .with_padding(SideOffsets::all(24.0))
                    .with_flex(flex)
                    .with_child(Box::new(
                        Flex::column(vec![
                            Box::new(
                                Flex::column(vec![
                                    Box::new(Label::new(title).with_size(24.0)),
                                    Box::new(Spacer::new().with_size(10.0, 6.0)),
                                    Box::new(Label::new(subtitle).with_color(theme.text_muted)),
                                ])
                                .with_cross_axis_alignment(
                                    lever_core::layout::CrossAxisAlignment::Stretch,
                                ),
                            ),
                            Box::new(Spacer::new().with_size(10.0, 32.0)),
                            child,
                        ])
                        .with_cross_axis_alignment(lever_core::layout::CrossAxisAlignment::Stretch),
                    )),
            ) as Box<dyn Widget<Self::Message>>
        };

        let header = Box::new(Flex::row(vec![
            Box::new(
                Flex::column(vec![
                    Box::new(Label::new("Lever UI Gallery").with_size(42.0)),
                    Box::new(
                        Label::new("A modern, high-performance UI toolkit for Rust.")
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

        let animation_section = section_card(
            "Motion & Animation",
            "Smooth, spring-based transitions for reactive interfaces.",
            Box::new(
                Flex::row(vec![
                    Box::new(AnimatedScale::new(
                        animated_pulse,
                        Box::new(
                            BoxWidget::new(theme.primary.with_alpha(0.15))
                                .with_radius(12.0)
                                .with_size(140.0, 100.0)
                                .with_alignment(Alignment::Center)
                                .with_child(Box::new(
                                    Label::new("Pulse").with_color(theme.primary),
                                )),
                        ),
                    )),
                    Box::new(AnimatedTranslation::new(
                        Point {
                            x: 0.0,
                            y: animated_float,
                        },
                        Box::new(
                            BoxWidget::new(theme.success.with_alpha(0.15))
                                .with_radius(12.0)
                                .with_size(140.0, 100.0)
                                .with_alignment(Alignment::Center)
                                .with_child(Box::new(
                                    Label::new("Float").with_color(theme.success),
                                )),
                        ),
                    )),
                    Box::new(AnimatedOpacity::new(
                        animated_opacity,
                        Box::new(
                            BoxWidget::new(theme.danger.with_alpha(0.15))
                                .with_radius(12.0)
                                .with_size(140.0, 100.0)
                                .with_alignment(Alignment::Center)
                                .with_child(Box::new(Label::new("Fade").with_color(theme.danger))),
                        ),
                    )),
                    Box::new(
                        Flex::column(vec![
                            Box::new(
                                Flex::row(vec![
                                    Box::new(Label::new("Enable Pulse")),
                                    Box::new(
                                        Toggle::new("pulse-toggle", self.is_pulsing)
                                            .on_changed(|v| Message::TogglePulse(v)),
                                    ),
                                ])
                                .with_gap(12.0),
                            ),
                            Box::new(
                                Flex::row(vec![
                                    Box::new(Label::new("Enable Float")),
                                    Box::new(
                                        Toggle::new("float-toggle", self.is_floating)
                                            .on_changed(|v| Message::ToggleFloat(v)),
                                    ),
                                ])
                                .with_gap(12.0),
                            ),
                            Box::new(
                                Flex::row(vec![
                                    Box::new(Label::new("Enable Fade")),
                                    Box::new(
                                        Toggle::new("fade-toggle", self.toggle_on)
                                            .on_changed(|v| Message::ToggleChanged(v)),
                                    ),
                                ])
                                .with_gap(12.0),
                            ),
                        ])
                        .with_gap(12.0),
                    ),
                ])
                .with_gap(24.0),
            ),
            0,
        );

        let controls_section = section_card(
            "Interactive Components",
            "Essential building blocks for user input and actions.",
            Box::new(Flex::column(vec![
                Box::new(
                    Flex::row(vec![
                        Box::new(
                            Button::new("btn-p", "Primary Action")
                                .with_variant(ButtonVariant::Primary)
                                .on_click(|| Message::ButtonClicked("P".into())),
                        ),
                        Box::new(
                            Button::new("btn-s", "Secondary")
                                .with_variant(ButtonVariant::Secondary),
                        ),
                        Box::new(
                            Button::new("btn-o", "Outline").with_variant(ButtonVariant::Outline),
                        ),
                        Box::new(
                            Button::new("btn-modal", "Show Modal")
                                .with_variant(ButtonVariant::Primary)
                                .on_click(|| Message::OpenModal),
                        ),
                    ])
                    .with_gap(12.0),
                ),
                Box::new(Spacer::new().with_size(10.0, 20.0)),
                Box::new(
                    Flex::row(vec![
                        Box::new(
                            Checkbox::new("c1", self.checkbox_checked)
                                .with_label("Agree to terms")
                                .on_changed(|v| Message::CheckboxChanged(v)),
                        ),
                        Box::new(
                            Checkbox::new("c2", !self.checkbox_checked)
                                .with_label("Opt-in to news"),
                        ),
                    ])
                    .with_gap(32.0),
                ),
                Box::new(Spacer::new().with_size(10.0, 20.0)),
                Box::new(
                    Flex::row(vec![
                        Box::new(
                            Dropdown::new(
                                "g-drop",
                                vec![
                                    "Standard Mode".into(),
                                    "Compact Mode".into(),
                                    "Experimental".into(),
                                    "Debug".into(),
                                ],
                                self.selected_dropdown,
                            )
                            .on_select(|idx| Message::DropdownSelected(idx)),
                        ),
                        Box::new(
                            RadioGroup::new(
                                "radio-group-demo",
                                vec![
                                    RadioOption::new("Option A", 0),
                                    RadioOption::new("Option B", 1),
                                    RadioOption::new("Disabled Option", 2).with_disabled(true),
                                ],
                            )
                            .with_selected(self.selected_radio)
                            .with_direction(lever_core::layout::FlexDirection::Row)
                            .with_gap(24.0)
                            .on_changed(|v| Message::RadioSelected(v)),
                        ),
                    ])
                    .with_gap(32.0),
                ),
                Box::new(Spacer::new().with_size(10.0, 32.0)),
                Box::new(Flex::column(vec![
                    Box::new(Label::new("Sliders").with_size(16.0)),
                    Box::new(Spacer::new().with_size(10.0, 16.0)),
                    Box::new(
                        Flex::row(vec![
                            Box::new(
                                Flex::column(vec![
                                    Box::new(
                                        Label::new("Continuous (0-1)")
                                            .with_size(12.0)
                                            .with_color(theme.text_muted),
                                    ),
                                    Box::new(Spacer::new().with_size(10.0, 8.0)),
                                    Box::new(
                                        Slider::new("s1", self.slider_value)
                                            .on_changed(|v| Message::SliderChanged(v)),
                                    ),
                                ])
                                .with_flex(1),
                            ),
                            Box::new(
                                Flex::column(vec![
                                    Box::new(
                                        Label::new("Discrete (0-100, step 10)")
                                            .with_size(12.0)
                                            .with_color(theme.text_muted),
                                    ),
                                    Box::new(Spacer::new().with_size(10.0, 8.0)),
                                    Box::new(
                                        Slider::new("s2", self.discrete_slider)
                                            .with_range(0.0, 100.0)
                                            .with_step(10.0)
                                            .on_changed(|v| Message::DiscreteSliderChanged(v)),
                                    ),
                                ])
                                .with_flex(1),
                            ),
                        ])
                        .with_gap(24.0),
                    ),
                    Box::new(Spacer::new().with_size(10.0, 24.0)),
                    Box::new(
                        Flex::row(vec![
                            Box::new(
                                Flex::column(vec![
                                    Box::new(
                                        Label::new("Percent Formatter")
                                            .with_size(12.0)
                                            .with_color(theme.text_muted),
                                    ),
                                    Box::new(Spacer::new().with_size(10.0, 8.0)),
                                    Box::new(
                                        Slider::new("s3", self.percent_slider)
                                            .with_label_formatter(|v| format!("{:.0}%", v * 100.0))
                                            .on_changed(|v| Message::PercentSliderChanged(v)),
                                    ),
                                ])
                                .with_flex(1),
                            ),
                            Box::new(
                                Flex::column(vec![
                                    Box::new(
                                        Label::new("Disabled State")
                                            .with_size(12.0)
                                            .with_color(theme.text_muted),
                                    ),
                                    Box::new(Spacer::new().with_size(10.0, 8.0)),
                                    Box::new(Slider::new("s4", 0.3).with_disabled(true)),
                                ])
                                .with_flex(1),
                            ),
                        ])
                        .with_gap(24.0),
                    ),
                ])),
            ])),
            0,
        );

        let progress_section = section_card(
            "Progress Indicators",
            "Linear and circular indicators for status and loading states.",
            Box::new(
                Flex::column(vec![
                    Box::new(
                        Label::new("Linear Progress")
                            .with_size(12.0)
                            .with_color(theme.text_muted),
                    ),
                    Box::new(Spacer::new().with_size(10.0, 8.0)),
                    Box::new(ProgressBar::new("p1", self.slider_value)),
                    Box::new(Spacer::new().with_size(10.0, 24.0)),
                    Box::new(
                        Label::new("Indeterminate (Loading)")
                            .with_size(12.0)
                            .with_color(theme.text_muted),
                    ),
                    Box::new(Spacer::new().with_size(10.0, 8.0)),
                    Box::new(ProgressBar::indeterminate("p2").with_color(theme.success)),
                    Box::new(Spacer::new().with_size(10.0, 32.0)),
                    Box::new(
                        Flex::row(vec![
                            Box::new(
                                Flex::column(vec![
                                    Box::new(
                                        Label::new("Circular")
                                            .with_size(12.0)
                                            .with_color(theme.text_muted),
                                    ),
                                    Box::new(Spacer::new().with_size(10.0, 12.0)),
                                    Box::new(CircularProgress::new("c1", self.slider_value)),
                                ])
                                .with_flex(1),
                            ),
                            Box::new(
                                Flex::column(vec![
                                    Box::new(
                                        Label::new("Indeterminate")
                                            .with_size(12.0)
                                            .with_color(theme.text_muted),
                                    ),
                                    Box::new(Spacer::new().with_size(10.0, 12.0)),
                                    Box::new(
                                        CircularProgress::indeterminate("c2")
                                            .with_color(theme.secondary),
                                    ),
                                ])
                                .with_flex(1),
                            ),
                            Box::new(
                                Flex::column(vec![
                                    Box::new(
                                        Label::new("Custom Style")
                                            .with_size(12.0)
                                            .with_color(theme.text_muted),
                                    ),
                                    Box::new(Spacer::new().with_size(10.0, 12.0)),
                                    Box::new(
                                        CircularProgress::new("c3", 0.75)
                                            .with_thickness(8.0)
                                            .with_color(theme.danger),
                                    ),
                                ])
                                .with_flex(1),
                            ),
                        ])
                        .with_gap(24.0),
                    ),
                ])
                .with_cross_axis_alignment(lever_core::layout::CrossAxisAlignment::Stretch),
            ),
            0,
        );

        let test_tex = self.test_image.unwrap_or(TextureId(0));

        let navigation_section = section_card(
            "Navigation & Tabs",
            "Modern, animated tab systems for switching views.",
            Box::new(Flex::column(vec![
                Box::new(
                    Label::new("Underline Style (Default)")
                        .with_size(12.0)
                        .with_color(theme.text_muted),
                ),
                Box::new(Spacer::new().with_size(10.0, 8.0)),
                Box::new(
                    Tabs::new(
                        "tabs-underline",
                        vec![
                            TabItem::new("Account"),
                            TabItem::new("Security"),
                            TabItem::new("Notifications"),
                            TabItem::new("Advanced").with_disabled(true),
                        ],
                        self.active_tab,
                    )
                    .on_change(|idx| Message::TabChanged(idx)),
                ),
                Box::new(Spacer::new().with_size(10.0, 32.0)),
                Box::new(
                    Label::new("Pill Style with Icons")
                        .with_size(12.0)
                        .with_color(theme.text_muted),
                ),
                Box::new(Spacer::new().with_size(10.0, 8.0)),
                Box::new(
                    Tabs::new(
                        "tabs-pill",
                        vec![
                            TabItem::new("Home").with_icon(test_tex),
                            TabItem::new("Search").with_icon(test_tex),
                            TabItem::new("Library").with_icon(test_tex),
                        ],
                        self.active_pill_tab,
                    )
                    .with_style(TabStyle::Pill)
                    .on_change(|idx| Message::TabPillChanged(idx)),
                ),
                Box::new(Spacer::new().with_size(10.0, 32.0)),
                Box::new(
                    Label::new("Full Width (Distributed)")
                        .with_size(12.0)
                        .with_color(theme.text_muted),
                ),
                Box::new(Spacer::new().with_size(10.0, 8.0)),
                Box::new(
                    Tabs::new(
                        "tabs-full",
                        vec![
                            TabItem::new("Left"),
                            TabItem::new("Middle"),
                            TabItem::new("Right"),
                        ],
                        self.active_full_tab,
                    )
                    .with_full_width(true)
                    .with_alignment(MainAxisAlignment::SpaceEvenly)
                    .on_change(|idx| Message::TabFullChanged(idx)),
                ),
            ])),
            0,
        );

        let image_section = section_card(
            "Images & Media",
            "High-performance image rendering with multiple fit modes.",
            Box::new(
                Flex::row(vec![
                    Box::new(
                        Flex::column(vec![
                            Box::new(
                                Label::new("Fill")
                                    .with_size(12.0)
                                    .with_color(theme.text_muted),
                            ),
                            Box::new(Spacer::new().with_size(10.0, 8.0)),
                            Box::new(
                                BoxWidget::new(theme.surface_variant)
                                    .with_radius(8.0)
                                    .with_size(120.0, 120.0)
                                    .with_child(Box::new(
                                        ImageWidget::new(test_tex)
                                            .with_size(120.0, 120.0)
                                            .with_fit(ImageFit::Fill),
                                    )),
                            ),
                        ])
                        .with_cross_axis_alignment(lever_core::layout::CrossAxisAlignment::Center),
                    ),
                    Box::new(
                        Flex::column(vec![
                            Box::new(
                                Label::new("Contain")
                                    .with_size(12.0)
                                    .with_color(theme.text_muted),
                            ),
                            Box::new(Spacer::new().with_size(10.0, 8.0)),
                            Box::new(
                                BoxWidget::new(theme.surface_variant)
                                    .with_radius(8.0)
                                    .with_size(120.0, 120.0)
                                    .with_child(Box::new(
                                        ImageWidget::new(test_tex)
                                            .with_size(120.0, 120.0)
                                            .with_fit(ImageFit::Contain),
                                    )),
                            ),
                        ])
                        .with_cross_axis_alignment(lever_core::layout::CrossAxisAlignment::Center),
                    ),
                    Box::new(
                        Flex::column(vec![
                            Box::new(
                                Label::new("Cover")
                                    .with_size(12.0)
                                    .with_color(theme.text_muted),
                            ),
                            Box::new(Spacer::new().with_size(10.0, 8.0)),
                            Box::new(
                                BoxWidget::new(theme.surface_variant)
                                    .with_radius(8.0)
                                    .with_size(120.0, 120.0)
                                    .with_child(Box::new(
                                        ImageWidget::new(test_tex)
                                            .with_size(120.0, 120.0)
                                            .with_fit(ImageFit::Cover),
                                    )),
                            ),
                        ])
                        .with_cross_axis_alignment(lever_core::layout::CrossAxisAlignment::Center),
                    ),
                ])
                .with_gap(24.0),
            ),
            0,
        );

        let layout_section = Box::new(
            Flex::row(vec![
                section_card(
                    "Constraint Layout",
                    "Powerful relative positioning system.",
                    Box::new(
                        BoxWidget::new(Color::TRANSPARENT)
                            .with_size(0.0, 160.0)
                            .with_child(Box::new(
                                ConstraintLayout::new()
                                    .with_id("demo-cl")
                                    .with_child(
                                        Box::new(
                                            BoxWidget::new(theme.primary)
                                                .with_radius(8.0)
                                                .with_size(60.0, 60.0),
                                        ),
                                        |set| set.center_x(PARENT, 0.0).center_y(PARENT, 0.0),
                                    )
                                    .with_child(
                                        Box::new(
                                            BoxWidget::new(theme.success.with_alpha(0.8))
                                                .with_radius(8.0)
                                                .with_size(40.0, 40.0),
                                        ),
                                        |set| {
                                            set.right_to_left(child(0), -12.0).center_y(PARENT, 0.0)
                                        },
                                    )
                                    .with_child(
                                        Box::new(
                                            BoxWidget::new(theme.danger.with_alpha(0.8))
                                                .with_radius(8.0)
                                                .with_size(40.0, 40.0),
                                        ),
                                        |set| {
                                            set.left_to_right(child(0), 12.0).center_y(PARENT, 0.0)
                                        },
                                    )
                                    .with_child(
                                        Box::new(
                                            BoxWidget::new(theme.secondary.with_alpha(0.8))
                                                .with_radius(8.0)
                                                .with_size(120.0, 32.0),
                                        ),
                                        |set| {
                                            set.top_to_bottom(child(0), 12.0).center_x(PARENT, 0.0)
                                        },
                                    ),
                            )),
                    ),
                    1,
                ),
                section_card(
                    "Grid System",
                    "Flexible track-based alignment.",
                    Box::new(
                        BoxWidget::new(Color::TRANSPARENT)
                            .with_size(0.0, 160.0)
                            .with_child(Box::new(
                                Grid::new()
                                    .with_column(GridTrack::Flex(1))
                                    .with_column(GridTrack::Flex(1))
                                    .with_child(Box::new(
                                        BoxWidget::new(theme.primary.with_alpha(0.2))
                                            .with_radius(6.0)
                                            .with_size(0.0, 60.0),
                                    ))
                                    .with_child(Box::new(
                                        BoxWidget::new(theme.success.with_alpha(0.2))
                                            .with_radius(6.0)
                                            .with_size(0.0, 60.0),
                                    ))
                                    .with_child(Box::new(
                                        BoxWidget::new(theme.danger.with_alpha(0.2))
                                            .with_radius(6.0)
                                            .with_size(0.0, 60.0),
                                    ))
                                    .with_child(Box::new(
                                        BoxWidget::new(theme.secondary.with_alpha(0.2))
                                            .with_radius(6.0)
                                            .with_size(0.0, 60.0),
                                    ))
                                    .with_gap(12.0),
                            )),
                    ),
                    1,
                ),
            ])
            .with_gap(24.0),
        );

        let spacing_section = section_card(
            "Spacing & Dividers",
            "Helpers for layout separation and visual structure.",
            Box::new(Flex::column(vec![
                Box::new(
                    Flex::row(vec![
                        Box::new(Label::new("Item 1")),
                        Box::new(
                            Spacer::horizontal(32.0)
                                .with_divider(theme.border)
                                .with_orientation(SpacerOrientation::Vertical),
                        ),
                        Box::new(Label::new("Item 2")),
                        Box::new(
                            Spacer::horizontal(32.0)
                                .with_divider(theme.primary)
                                .with_thickness(2.0)
                                .with_orientation(SpacerOrientation::Vertical),
                        ),
                        Box::new(Label::new("Item 3")),
                    ])
                    .with_cross_axis_alignment(lever_core::layout::CrossAxisAlignment::Stretch)
                    .with_gap(12.0),
                ),
                Box::new(Spacer::vertical(24.0)),
                Box::new(Flex::row(vec![
                    Box::new(Label::new("Pushed to Left")),
                    Box::new(Spacer::flex()),
                    Box::new(Label::new("Pushed to Right")),
                ])),
                Box::new(
                    Spacer::vertical(32.0)
                        .with_divider(theme.border)
                        .with_orientation(SpacerOrientation::Horizontal),
                ),
                Box::new(
                    Flex::row(vec![
                        Box::new(
                            BoxWidget::new(theme.surface_variant)
                                .with_size(100.0, 100.0)
                                .with_radius(8.0)
                                .with_child(Box::new(Label::new("A").with_size(24.0))),
                        ),
                        Box::new(
                            Spacer::horizontal(32.0)
                                .with_divider(theme.success)
                                .with_thickness(4.0)
                                .with_orientation(SpacerOrientation::Vertical),
                        ),
                        Box::new(
                            BoxWidget::new(theme.surface_variant)
                                .with_size(100.0, 100.0)
                                .with_radius(8.0)
                                .with_child(Box::new(Label::new("B").with_size(24.0))),
                        ),
                    ])
                    .with_cross_axis_alignment(lever_core::layout::CrossAxisAlignment::Stretch)
                    .with_gap(24.0),
                ),
            ])),
            0,
        );

        let typography_section = section_card(
            "Typography & Alignment",
            "Consistent sizing and flexible horizontal alignment.",
            Box::new(
                Flex::column(vec![
                    Box::new(
                        Label::new("Left Aligned (Default)")
                            .with_align(lever_core::types::TextAlign::Left),
                    ),
                    Box::new(Spacer::new().with_size(10.0, 12.0)),
                    Box::new(
                        Label::new("Center Aligned Text")
                            .with_align(lever_core::types::TextAlign::Center),
                    ),
                    Box::new(Spacer::new().with_size(10.0, 12.0)),
                    Box::new(
                        Label::new("Right Aligned Text")
                            .with_align(lever_core::types::TextAlign::Right),
                    ),
                    Box::new(Spacer::new().with_size(10.0, 24.0)),
                    Box::new(
                        Flex::row(vec![
                            Box::new(
                                Label::new("Small")
                                    .with_size(theme.font_size_sm)
                                    .with_color(theme.text_muted),
                            ),
                            Box::new(Label::new("Medium").with_size(theme.font_size_md)),
                            Box::new(
                                Label::new("Large")
                                    .with_size(theme.font_size_lg)
                                    .with_color(theme.primary),
                            ),
                        ])
                        .with_gap(24.0)
                        .with_cross_axis_alignment(lever_core::layout::CrossAxisAlignment::Center),
                    ),
                ])
                .with_cross_axis_alignment(lever_core::layout::CrossAxisAlignment::Stretch),
            ),
            0,
        );

        let inputs_section = section_card(
            "Form Inputs",
            "Modern text fields with validation, icons, and password support.",
            Box::new(
                Flex::column(vec![
                    Box::new(
                        TextInput::new("input-1")
                            .with_placeholder("Username or email")
                            .with_text(self.text_input1.clone())
                            .with_cursor(self.cursor1)
                            .on_input(|text, idx| Message::TextInput1Changed(text, idx)),
                    ),
                    Box::new(Spacer::new().with_size(10.0, 16.0)),
                    Box::new(
                        TextInput::new("input-2")
                            .with_placeholder("Password")
                            .with_text(self.text_input2.clone())
                            .with_cursor(self.cursor2)
                            .with_password(true)
                            .with_leading_icon(test_tex)
                            .on_input(|text, idx| Message::TextInput2Changed(text, idx)),
                    ),
                    Box::new(Spacer::new().with_size(10.0, 32.0)),
                    Box::new(
                        TextInput::new("input-3")
                            .with_placeholder("Search something...")
                            .with_text(self.text_input3.clone())
                            .with_cursor(self.cursor3)
                            .with_trailing_icon(test_tex)
                            .on_input(|text, idx| Message::TextInput3Changed(text, idx)),
                    ),
                    Box::new(Spacer::new().with_size(10.0, 16.0)),
                    Box::new(
                        TextInput::new("input-4")
                            .with_placeholder("Invalid input")
                            .with_text(self.text_input4.clone())
                            .with_cursor(self.cursor4)
                            .with_error("This field is required")
                            .on_input(|text, idx| Message::TextInput4Changed(text, idx)),
                    ),
                ])
                .with_cross_axis_alignment(lever_core::layout::CrossAxisAlignment::Stretch),
            ),
            0,
        );

        let content = Flex::column(vec![
            header,
            Box::new(Spacer::new().with_size(10.0, 48.0)),
            typography_section,
            Box::new(Spacer::new().with_size(10.0, 24.0)),
            spacing_section,
            Box::new(Spacer::new().with_size(10.0, 24.0)),
            animation_section,
            Box::new(Spacer::new().with_size(10.0, 24.0)),
            image_section,
            Box::new(Spacer::new().with_size(10.0, 24.0)),
            controls_section,
            Box::new(Spacer::new().with_size(10.0, 40.0)),
            inputs_section,
            Box::new(Spacer::new().with_size(10.0, 40.0)),
            progress_section,
            Box::new(Spacer::new().with_size(10.0, 40.0)),
            navigation_section,
            Box::new(Spacer::new().with_size(10.0, 40.0)),
            layout_section,
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

        let root = ConstraintLayout::new().with_id("root").with_child(
            Box::new(BoxWidget::new(theme.background).with_child(Box::new(scroll))),
            |set| set.fill_parent(),
        );

        if self.is_modal_open {
            Box::new(
                root.with_child(
                    Box::new(
                        Overlay::new()
                            .on_dismiss(|| Message::CloseModal)
                            .with_child(
                                BoxWidget::new(theme.surface)
                                    .with_radius(12.0)
                                    .with_shadow(lever_core::types::BoxShadow {
                                        offset: Point { x: 0.0, y: 20.0 },
                                        blur: 40.0,
                                        color: Color::rgba(0.0, 0.0, 0.0, 0.4),
                                    })
                                    .with_padding(SideOffsets::all(32.0))
                                    .with_child(Box::new(
                                        Flex::column(vec![
                                            Box::new(Label::new("Modal Dialog").with_size(24.0)),
                                            Box::new(Spacer::new().with_size(10.0, 16.0)),
                                            Box::new(Label::new(
                                                "This is a centered modal dialog rendered using\nthe improved Overlay widget with child support.",
                                            ).with_color(theme.text_muted)),
                                            Box::new(Spacer::new().with_size(10.0, 32.0)),
                                            Box::new(
                                                Button::new("modal-close", "Dismiss")
                                                    .with_variant(ButtonVariant::Secondary)
                                                    .on_click(|| Message::CloseModal),
                                            ),
                                        ])
                                        .with_cross_axis_alignment(
                                            lever_core::layout::CrossAxisAlignment::Center,
                                        ),
                                    )),
                            ),
                    ),
                    |set| set.fill_parent(),
                ),
            )
        } else {
            Box::new(root)
        }
    }
}

fn main() {
    let config = AppConfig {
        title: "Lever UI Gallery".to_string(),
        width: 1000,
        height: 850,
        clear_color: Color::rgb(0.05, 0.05, 0.05),
    };

    let app = GalleryApp::default();

    let application = Application::new(config, app);
    application.run();
}
