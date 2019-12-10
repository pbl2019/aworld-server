use super::button::Button;
use serde_json::value::Value;

#[derive(Debug, Clone)]
pub struct Controller {
    pub login: Button,
    pub up: Button,
    pub down: Button,
    pub right: Button,
    pub left: Button,
    pub spacebar: Button,
    pub a: Button,
    pub i: Button,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            login: Button::new(),
            up: Button::new(),
            down: Button::new(),
            right: Button::new(),
            left: Button::new(),
            spacebar: Button::new(),
            a: Button::new(),
            i: Button::new(),
        }
    }
}

impl<'a> std::iter::IntoIterator for &'a Controller {
    type Item = (String, bool, Value);
    type IntoIter = ControllerIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ControllerIntoIterator {
            controller: self,
            index: 0,
        }
    }
}

pub struct ControllerIntoIterator<'a> {
    controller: &'a Controller,
    index: usize,
}

impl<'a> std::iter::Iterator for ControllerIntoIterator<'a> {
    type Item = (String, bool, Value);
    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => (
                "login".to_owned(),
                self.controller.login.status,
                self.controller.login.optional.clone(),
            ),
            1 => (
                "forward".to_owned(),
                self.controller.up.status,
                self.controller.up.optional.clone(),
            ),
            2 => (
                "backward".to_owned(),
                self.controller.down.status,
                self.controller.down.optional.clone(),
            ),
            3 => (
                "turn_right".to_owned(),
                self.controller.right.status,
                self.controller.right.optional.clone(),
            ),
            4 => (
                "turn_left".to_owned(),
                self.controller.left.status,
                self.controller.left.optional.clone(),
            ),
            5 => (
                "pickup".to_owned(),
                self.controller.spacebar.status,
                self.controller.spacebar.optional.clone(),
            ),
            6 => (
                "attack".to_owned(),
                self.controller.a.status,
                self.controller.a.optional.clone(),
            ),
            7 => (
                "use_item".to_owned(),
                self.controller.i.status,
                self.controller.i.optional.clone(),
            ),
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}
