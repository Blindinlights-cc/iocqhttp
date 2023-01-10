use std::{collections::HashMap};
pub trait EventHandler {
    type Output;
    type Args;
    fn handle_event(&self, args: Self::Args) -> Self::Output;
}
type AsyncFunc<T> = Box<dyn EventHandler<Output = (), Args = T>>;
type AsyncFuncMap<T> = HashMap<String, AsyncFunc<T>>;
#[derive(Default)]
pub struct EventBus<T> {
    subscribers: AsyncFuncMap<T>,
    hooks_before: AsyncFuncMap<T>,
}
impl<T> EventBus<T> {
    pub fn new() -> Self {
        Self {
            subscribers: AsyncFuncMap::new(),
            hooks_before: AsyncFuncMap::new(),
        }
    }
    pub fn subscribe(&mut self, name: &str, func: AsyncFunc<T>) {
        self.subscribers.insert(name.to_string(), func);
    }
    pub fn hook_before(&mut self, name: &str, func: AsyncFunc<T>) {
        self.hooks_before.insert(name.to_string(), func);
    }
    pub fn unsubscribe(&mut self, name: &str) {
        self.subscribers.remove(name);
    }
    pub fn unhook_before(&mut self, name: &str) {
        self.hooks_before.remove(name);
    }
    pub async fn emit(&self, name: &str, args: T) {
        if let Some(handler) = self.subscribers.get(name) {
            handler.handle_event(args);
        } else if let Some(handler) = self.hooks_before.get(name) {
            handler.handle_event(args);
        }
    }
}
