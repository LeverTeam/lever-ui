use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static STATE_STORE: RefCell<HashMap<String, Box<dyn Any>>> = RefCell::new(HashMap::new());
    static GLOBAL_TIME: RefCell<f32> = RefCell::new(0.0);
}

pub fn get_time() -> f32 {
    GLOBAL_TIME.with(|t| *t.borrow())
}

pub fn set_time(time: f32) {
    GLOBAL_TIME.with(|t| *t.borrow_mut() = time);
}

pub fn get_state<T: Any + Clone>(id: &str) -> Option<T> {
    STATE_STORE.with(|store| {
        store
            .borrow()
            .get(id)
            .and_then(|b| b.downcast_ref::<T>())
            .cloned()
    })
}

pub fn set_state<T: Any>(id: &str, value: T) {
    STATE_STORE.with(|store| {
        store.borrow_mut().insert(id.to_string(), Box::new(value));
    });
}

pub fn get_or_set_state<T: Any + Clone, F: FnOnce() -> T>(id: &str, f: F) -> T {
    if let Some(state) = get_state::<T>(id) {
        state
    } else {
        let state = f();
        set_state(id, state.clone());
        state
    }
}

pub fn update_state<T: Any, F: FnOnce(&mut T)>(id: &str, f: F) {
    STATE_STORE.with(|store| {
        if let Some(val) = store.borrow_mut().get_mut(id) {
            if let Some(state) = val.downcast_mut::<T>() {
                f(state);
            }
        }
    });
}

pub fn tick_animations(dt: f32) {
    GLOBAL_TIME.with(|t| *t.borrow_mut() += dt);
    STATE_STORE.with(|store| {
        for val in store.borrow_mut().values_mut() {
            if let Some(anim) = val.downcast_mut::<crate::animation::AnimationController>() {
                anim.tick(dt);
            }
            if let Some(anim) = val.downcast_mut::<crate::animation::SpringController>() {
                anim.tick(dt);
            }
        }
    });
}
