use std::{
    any::{Any, TypeId},
    cell::{Cell, RefCell, RefMut},
    collections::HashMap,
    error::Error,
    rc::Rc,
    time::Duration,
};

use crate::FromStates;

use crate::{Chunks, Events};

pub struct State<T: ?Sized>(Rc<RefCell<T>>);

impl<T: Sized> State<T> {
    pub fn new(inner: T) -> Self {
        Self(Rc::new(RefCell::new(inner)))
    }

    pub fn get(&mut self) -> RefMut<T> {
        self.0.borrow_mut()
    }
}

impl<T: Sized> Clone for State<T> {
    fn clone(&self) -> Self {
        State(self.0.clone())
    }
}

pub struct States {
    states: HashMap<TypeId, Box<dyn Any>>,
}

impl Default for States {
    fn default() -> Self {
        let mut states = Self {
            states: HashMap::new(),
        };

        states.register(Time::default());
        states.register(Events::default());
        states.register(Chunks::default());

        states
    }
}

impl States {
    pub fn register<S: Any>(&mut self, state: S) {
        self.states
            .insert(state.type_id(), Box::new(State::new(state)));
    }

    pub fn get_option<S: Any>(&mut self) -> Option<State<S>> {
        if let Some(state) = self.states.get_mut(&TypeId::of::<S>()) {
            state.downcast_mut::<State<S>>().map(|state| state.clone())
        } else {
            None
        }
    }

    pub fn get<S: Any>(&mut self) -> Result<State<S>, Box<dyn Error>> {
        match self.get_option::<S>() {
            Some(item) => Ok(item),
            None => Err(anyhow!("Item didn't exist").into()),
        }
    }
}

pub trait FromState {
    fn from_state(states: &mut States) -> &mut Self;
}

// ---------- Guarenteed States --------- //

#[derive(Default, Clone, FromState)]
pub struct Time {
    frame_duration: Duration,
}

impl Time {
    pub fn set_duration(&mut self, duration: Duration) {
        self.frame_duration = duration
    }

    pub fn frame_time(&self) -> Duration {
        self.frame_duration
    }
}
