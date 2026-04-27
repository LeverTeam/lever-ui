use crate::theme::ThemeMode;
use crate::types::TextureId;
use crate::widget::Widget;

/// A proxy for sending messages to the application from background threads or async tasks.
pub trait MessageProxy<M>: Send + Sync {
    /// Sends a message to the application's update loop.
    fn send(&self, message: M);
}

/// The initialization context provided to [`App::init`].
///
/// This context allows the application to load assets and set up initial state
/// before the main event loop begins.
pub struct Context<'a, M> {
    renderer: &'a mut dyn TextureLoader,
    _phantom: std::marker::PhantomData<M>,
}

impl<'a, M> Context<'a, M> {
    /// Creates a new initialization context.
    pub fn new(renderer: &'a mut dyn TextureLoader) -> Self {
        Self {
            renderer,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Loads an image from raw bytes and returns a [`TextureId`] that can be used with [`ImageWidget`].
    ///
    /// # Arguments
    ///
    /// * `bytes` - The encoded image data (e.g., PNG, JPEG).
    ///
    /// # Examples
    ///
    /// ```rust
    /// fn init(&mut self, ctx: &mut Context<Self::Message>) {
    ///     let tex = ctx.load_image(include_bytes!("logo.png"));
    ///     self.logo = Some(tex);
    /// }
    /// ```
    pub fn load_image(&mut self, bytes: &[u8]) -> TextureId {
        self.renderer.load_texture(bytes)
    }
}

/// The update context provided to [`App::update`].
///
/// This context allows the application to trigger global framework actions,
/// such as switching the theme mode.
pub struct UpdateContext {
    /// The requested theme mode change, if any.
    pub theme_mode: Option<ThemeMode>,
}

impl UpdateContext {
    /// Creates a new empty update context.
    pub fn new() -> Self {
        Self { theme_mode: None }
    }

    /// Requests a global theme change.
    pub fn set_theme(&mut self, mode: ThemeMode) {
        self.theme_mode = Some(mode);
    }
}

/// Interface for loading textures into the renderer.
pub trait TextureLoader {
    /// Loads a texture from bytes and returns its identifier.
    fn load_texture(&mut self, bytes: &[u8]) -> TextureId;
}

/// The core trait for Lever applications.
///
/// Implementing this trait defines how your application initializes,
/// processes messages, updates its state, and renders its user interface.
pub trait App: Sized + 'static {
    /// The type of messages processed by this application.
    type Message: Send + 'static;

    /// Called once when the application starts. Use this to load assets.
    fn init(&mut self, _ctx: &mut Context<Self::Message>) {}

    /// Processes a message and updates the application state.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to process.
    /// * `ctx` - The update context for framework-level actions.
    fn update(&mut self, message: Self::Message, _ctx: &mut UpdateContext);

    /// Called every frame before rendering. Use this for time-based animations.
    ///
    /// # Arguments
    ///
    /// * `dt` - The time elapsed since the last frame in seconds.
    fn tick(&mut self, _dt: f32) {}

    /// Returns the root widget representing the current state of the UI.
    ///
    /// This is called whenever the framework needs to redraw the application.
    fn view(&self) -> Box<dyn Widget<Self::Message>>;
}
