use crate::widget::Widget;

pub trait MessageProxy<M>: Send + Sync {
    fn send(&self, message: M);
}

pub trait App: Sized + 'static {
    type Message: Send + 'static;

    fn update(&mut self, message: Self::Message);
    fn tick(&mut self, _dt: f32) {}
    fn view(&self) -> Box<dyn Widget<Self::Message>>;
}
