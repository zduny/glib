use glium::glutin::event::MouseScrollDelta;
use glium::glutin::event::VirtualKeyCode;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::game::KeyboardEvent;
use crate::game::MouseEvent;
use crate::math::vector::dis2;
use crate::math::vector::Displacement2;

#[derive(Hash, Eq, PartialEq)]
pub enum VirtualGamepadButton {
    South,
    East,
    North,
    West,
    Left,
    LeftTrigger,
    Right,
    RightTrigger,
    Select,
    Start,
    Mode,
    LeftStick,
    RightStick,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
}

pub enum VirtualGamepadStick {
    Left,
    Right,
}

pub trait VirtualGamepad {
    fn is_pressed(&self, button: VirtualGamepadButton) -> bool;
    fn button_value(&self, button: VirtualGamepadButton) -> f32;
    fn stick_value(&self, stick: VirtualGamepadStick) -> Displacement2;
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub enum RealInput {
    KeyboardKey(VirtualKeyCode),
    MouseButton(u32),
    MouseWheelUp,
    MouseWheelDown,
}

#[derive(Copy, Clone)]
pub enum StickBinding {
    Key {
        left: RealInput,
        up: RealInput,
        right: RealInput,
        down: RealInput,
    },
    Mouse,
}

pub struct KeyboardMouseVirtualGamepad {
    pub mouse_sensitivity: f32,
    pub mouse_invert_x: bool,
    pub mouse_invert_y: bool,
    mouse_delta: Displacement2,
    left_stick_value: Displacement2,
    right_stick_value: Displacement2,
    left_stick_bindings: Vec<StickBinding>,
    right_stick_bindings: Vec<StickBinding>,
    button_bindings: HashMap<VirtualGamepadButton, Vec<RealInput>>,
    currently_pressed: HashSet<RealInput>,
}

impl KeyboardMouseVirtualGamepad {
    pub fn new() -> KeyboardMouseVirtualGamepad {
        let mut button_bindings = HashMap::new();
        button_bindings.insert(VirtualGamepadButton::South, vec![]);
        button_bindings.insert(VirtualGamepadButton::South, vec![]);
        button_bindings.insert(VirtualGamepadButton::East, vec![]);
        button_bindings.insert(VirtualGamepadButton::North, vec![]);
        button_bindings.insert(VirtualGamepadButton::West, vec![]);
        button_bindings.insert(VirtualGamepadButton::Left, vec![]);
        button_bindings.insert(VirtualGamepadButton::LeftTrigger, vec![]);
        button_bindings.insert(VirtualGamepadButton::Right, vec![]);
        button_bindings.insert(VirtualGamepadButton::RightTrigger, vec![]);
        button_bindings.insert(VirtualGamepadButton::Select, vec![]);
        button_bindings.insert(VirtualGamepadButton::Start, vec![]);
        button_bindings.insert(VirtualGamepadButton::Mode, vec![]);
        button_bindings.insert(VirtualGamepadButton::LeftStick, vec![]);
        button_bindings.insert(VirtualGamepadButton::RightStick, vec![]);
        button_bindings.insert(VirtualGamepadButton::DPadUp, vec![]);
        button_bindings.insert(VirtualGamepadButton::DPadDown, vec![]);
        button_bindings.insert(VirtualGamepadButton::DPadLeft, vec![]);
        button_bindings.insert(VirtualGamepadButton::DPadRight, vec![]);

        KeyboardMouseVirtualGamepad {
            mouse_sensitivity: 0.3,
            mouse_invert_x: false,
            mouse_invert_y: false,
            mouse_delta: dis2(0.0, 0.0),
            left_stick_value: dis2(0.0, 0.0),
            right_stick_value: dis2(0.0, 0.0),
            left_stick_bindings: vec![],
            right_stick_bindings: vec![],
            button_bindings,
            currently_pressed: HashSet::new(),
        }
    }

    pub fn borrow_button_binding(&self, button: VirtualGamepadButton) -> &Vec<RealInput> {
        self.button_bindings.get(&button).unwrap()
    }

    pub fn borrow_stick_binding(&self, stick: VirtualGamepadStick) -> &Vec<StickBinding> {
        match stick {
            VirtualGamepadStick::Left => &self.left_stick_bindings,
            VirtualGamepadStick::Right => &self.right_stick_bindings,
        }
    }

    pub fn bind_stick(&mut self, stick: VirtualGamepadStick, binding: StickBinding) {
        match stick {
            VirtualGamepadStick::Left => self.left_stick_bindings.push(binding),
            VirtualGamepadStick::Right => self.right_stick_bindings.push(binding),
        }
    }

    pub fn bind_button(&mut self, button: VirtualGamepadButton, input: RealInput) {
        self.button_bindings.get_mut(&button).unwrap().push(input);
    }

    pub fn bind_default(&mut self) {
        self.bind_stick(
            VirtualGamepadStick::Left,
            StickBinding::Key {
                left: RealInput::KeyboardKey(VirtualKeyCode::A),
                up: RealInput::KeyboardKey(VirtualKeyCode::W),
                right: RealInput::KeyboardKey(VirtualKeyCode::D),
                down: RealInput::KeyboardKey(VirtualKeyCode::S),
            },
        );
        self.bind_stick(VirtualGamepadStick::Right, StickBinding::Mouse);
    }

    pub fn clear_stick_binding(&mut self, stick: VirtualGamepadStick) {
        match stick {
            VirtualGamepadStick::Left => self.left_stick_bindings.clear(),
            VirtualGamepadStick::Right => self.right_stick_bindings.clear(),
        }
    }

    pub fn clear_button_binding(&mut self, button: VirtualGamepadButton) {
        self.button_bindings.get_mut(&button).unwrap().clear();
    }

    pub fn clear_all_buttons_bindings(&mut self) {
        for binding in self.button_bindings.iter_mut() {
            binding.1.clear();
        }
    }

    fn is_input_pressed(&self, input: &RealInput) -> bool {
        self.currently_pressed.contains(input)
    }

    fn stick_binding_value(&self, binding: &StickBinding) -> Displacement2 {
        match binding {
            StickBinding::Key {
                left,
                up,
                right,
                down,
            } => {
                let mut value = dis2(0.0, 0.0);
                if self.is_input_pressed(left) {
                    value = value + dis2(-1.0, 0.0);
                }
                if self.is_input_pressed(up) {
                    value = value + dis2(0.0, 1.0);
                }
                if self.is_input_pressed(right) {
                    value = value + dis2(1.0, 0.0);
                }
                if self.is_input_pressed(down) {
                    value = value + dis2(0.0, -1.0);
                }
                value.normalized() * 1.0
            }
            StickBinding::Mouse => {
                let mut x = self.mouse_delta.vector.x;
                let mut y = self.mouse_delta.vector.y;

                if self.mouse_invert_x {
                    x = -x;
                }

                if self.mouse_invert_y {
                    y = -y;
                }

                dis2(x, y) * self.mouse_sensitivity
            }
        }
    }

    fn stick_bindings_value(&self, bindings: &[StickBinding]) -> Displacement2 {
        let mut value = dis2(0.0, 0.0);
        for binding in bindings.iter() {
            let new_value = self.stick_binding_value(binding);
            if new_value.length_squared() > value.length_squared() {
                value = new_value;
            }
        }
        value
    }

    fn update(&mut self) {
        let new_value = self.stick_bindings_value(&self.left_stick_bindings);
        self.left_stick_value = new_value;
        let new_value = self.stick_bindings_value(&self.right_stick_bindings);
        self.right_stick_value = new_value;

        self.currently_pressed.remove(&RealInput::MouseWheelUp);
        self.currently_pressed.remove(&RealInput::MouseWheelDown);
    }

    pub fn on_after_update(&mut self) {
        self.update();
        self.mouse_delta = dis2(0.0, 0.0);
    }

    pub fn on_keyboard_event(&mut self, event: KeyboardEvent) {
        match event {
            KeyboardEvent::KeyDown { input, .. } => {
                if let Some(virtual_keycode) = input.virtual_keycode {
                    self.currently_pressed
                        .insert(RealInput::KeyboardKey(virtual_keycode));
                }
            }
            KeyboardEvent::KeyUp { input, .. } => {
                if let Some(virtual_keycode) = input.virtual_keycode {
                    self.currently_pressed
                        .remove(&RealInput::KeyboardKey(virtual_keycode));
                }
            }
        }
    }

    pub fn on_mouse_event(&mut self, event: MouseEvent) {
        match event {
            MouseEvent::MouseDown { button, .. } => {
                self.currently_pressed
                    .insert(RealInput::MouseButton(button));
            }
            MouseEvent::MouseUp { button, .. } => {
                self.currently_pressed
                    .remove(&RealInput::MouseButton(button));
            }
            MouseEvent::MouseMove { delta, .. } => {
                self.mouse_delta = self.mouse_delta + delta;
            }
            MouseEvent::MouseWheel { delta, .. } => match delta {
                MouseScrollDelta::LineDelta(_x, y) => {
                    if y > 0.0 {
                        self.currently_pressed.insert(RealInput::MouseWheelUp);
                    }
                    if y < 0.0 {
                        self.currently_pressed.insert(RealInput::MouseWheelDown);
                    }
                }
                MouseScrollDelta::PixelDelta(delta) => {
                    if delta.y > 0.0 {
                        self.currently_pressed.insert(RealInput::MouseWheelUp);
                    }
                    if delta.y < 0.0 {
                        self.currently_pressed.insert(RealInput::MouseWheelDown);
                    }
                }
            },
        }
    }
}

impl VirtualGamepad for KeyboardMouseVirtualGamepad {
    fn is_pressed(&self, button: VirtualGamepadButton) -> bool {
        let bindings = self.button_bindings.get(&button).unwrap();
        for input in bindings {
            if self.is_input_pressed(&input) {
                return true;
            }
        }
        false
    }

    fn button_value(&self, button: VirtualGamepadButton) -> f32 {
        let bindings = self.button_bindings.get(&button).unwrap();
        for input in bindings {
            if self.is_input_pressed(&input) {
                return 1.0;
            }
        }
        0.0
    }
    fn stick_value(&self, stick: VirtualGamepadStick) -> Displacement2 {
        match stick {
            VirtualGamepadStick::Left => self.left_stick_value,
            VirtualGamepadStick::Right => self.right_stick_value,
        }
    }
}
