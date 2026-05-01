use crate::theme::ThemeMode;
use crate::types::TextureId;
use crate::widget::Widget;

pub trait MessageProxy<M>: Send + Sync {
    fn send(&self, message: M);
}

pub struct Context<'a, M> {
    renderer: &'a mut dyn TextureLoader,
    _phantom: std::marker::PhantomData<M>,
}

impl<'a, M> Context<'a, M> {
    pub fn new(renderer: &'a mut dyn TextureLoader) -> Self {
        Self {
            renderer,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn load_image(&mut self, bytes: &[u8]) -> TextureId {
        self.renderer.load_texture(bytes)
    }
}

pub struct UpdateContext {
    pub theme_mode: Option<ThemeMode>,
}

impl UpdateContext {
    pub fn new() -> Self {
        Self { theme_mode: None }
    }

    pub fn set_theme(&mut self, mode: ThemeMode) {
        self.theme_mode = Some(mode);
    }
}

pub trait TextureLoader {
    fn load_texture(&mut self, bytes: &[u8]) -> TextureId;
}

pub trait App: Sized + 'static {
    type Message: Send + 'static;

    fn init(&mut self, _ctx: &mut Context<Self::Message>) {}

    fn update(&mut self, message: Self::Message, _ctx: &mut UpdateContext);

    fn tick(&mut self, _dt: f32) {}

    fn view(&self) -> Box<dyn Widget<Self::Message>>;
}
