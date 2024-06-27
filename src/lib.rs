//! A simple crate that adds a variety of Godot (specifically gdext) related macros for convenience.

// TODO: Readme!

/// Macro for quickly getting a non-mutable reference to a node from its path. Can choose to specify node type, otherwise defaults to Node.
///
/// # Panics
/// 
/// Panics if node_path cannot be found.
/// 
/// # Example
/// 
/// ```
/// let label: Gd<Node> = n!(self, "Player"); // Reference to a child Node named "Player"
/// let label: Gd<Label> = n!(self, Label, "Player"); // Reference to a child Label named "Player"
/// let label: Gd<Label> = n!(self, "Player", Label); // Same but with type and name switched
/// 
/// // Expanded
/// let label: Gd<Node> = self.base().get_node_as::<Node>("Player");
/// let label: Gd<Label> = self.base().get_node_as::<Label>("Player");
/// ```
#[macro_export]
macro_rules! n {
    ($self:ident, $node_path:expr) => {
        $self.base().get_node_as::<Node>($node_path)
    };
    ($self:ident, $node_type:ty, $node_path:expr) => {
        $self.base().get_node_as::<$node_type>($node_path)
    };
    ($self:ident, $node_path:expr, $node_type:ty) => {
        $self.base().get_node_as::<$node_type>($node_path)
    };
}

/// Macro for quickly getting a mutable reference to a node from its path. Can choose to specify node type, otherwise defaults to Node.
///
/// # Panics
/// 
/// Panics if node_path cannot be found.
/// 
/// # Example
/// 
/// ```
/// let mut label: Gd<Node> = nm!(self, "Player"); // Mutable reference to a child Node named "Player"
/// let mut label: Gd<Label> = nm!(self, Label, "Player"); // Mutable reference to a child Label named "Player"
/// let mut label: Gd<Label> = nm!(self, "Player", Label); // Same but with type and name switched
/// 
/// // Expanded
/// let mut label: Gd<Node> = self.base_mut().get_node_as::<Node>("Player");
/// let mut label: Gd<Label> = self.base_mut().get_node_as::<Label>("Player");
/// ```
#[macro_export]
macro_rules! nm {
    ($self:ident, $node_path:expr) => {
        $self.base_mut().get_node_as::<Node>($node_path)
    };
    ($self:ident, $node_type:ty, $node_path:expr) => {
        $self.base_mut().get_node_as::<$node_type>($node_path)
    };
    ($self:ident, $node_path:expr, $node_type:ty) => {
        $self.base_mut().get_node_as::<$node_type>($node_path)
    };
}

/// Simplifies connecting a signal on a node to to a callback on either self or a second provided node.
/// 
/// Note: Callback function must be registered as a function with Godot, either in GDScript or by #\[func\].
/// 
/// # Panics
/// 
/// Panics if any of the named strings do not correspond to anything in the engine.
/// 
/// # Example
/// 
/// ```
/// // Connect MobDetector.body_entered -> self.on_body_entered
/// connect!(self, "MobDetector", "body_entered", "on_body_entered");
/// // Expanded
/// n!(self, "MobDetector").connect("body_entered".into(), self.base().callable("on_body_entered"));
/// 
/// // Connect MobDetector.body_entered -> Ui.game_over
/// connect!(self, "MobDetector", "body_entered", "Ui", "game_over");
/// // Expanded
/// n!(self, "MobDetector").connect("body_entered".into(), n!(self, "Ui").callable("game_over"));
/// ```
#[macro_export]
macro_rules! connect {
    ($self:ident, $node_path:expr, $signal:expr, $callback_name:expr) => {
        n!($self, $node_path).connect($signal.into(), $self.base().callable($callback_name))
    };
    ($self:ident, $node_path_1:expr, $signal:expr, $node_path_2:expr, $callback_name:expr) => {
        n!($self, $node_path_1).connect($signal.into(), n!($self, $node_path_2).callable($callback_name))
    };
}

/// Returns whether any input event has been pressed.
/// 
/// # Example
/// 
/// ```
/// // See if player wants to continue game
/// let continue: bool = any_press!();
/// 
/// // Expanded
/// let continue: bool = Input::singleton().is_anything_pressed();
/// ```
#[macro_export]
macro_rules! any_press {
    () => {
        Input::singleton().is_anything_pressed()
    };
}

/// Returns whether the provided key has been pressed.
/// 
/// # Example
/// 
/// ```
/// // See if player jumped
/// let should_jump: bool = key_press!(Key::UP);
/// 
/// // Expanded
/// let should_jump: bool = Input::singleton().is_key_pressed(Key::UP);
/// ```
#[macro_export]
macro_rules! key_press {
    ($keycode:expr) => {
        Input::singleton().is_key_pressed($keycode)
    };
}

/// Returns whether the provided key has been pressed physically.
/// 
/// # Example
/// 
/// ```
/// // See if player jumped
/// let should_jump: bool = key_press_phys!(Key::UP);
/// 
/// // Expanded
/// let should_jump: bool = Input::singleton().is_physical_key_pressed(Key::UP);
/// ```
#[macro_export]
macro_rules! key_press_phys {
    ($keycode:expr) => {
        Input::singleton().is_physical_key_pressed($keycode)
    };
}

/// Returns whether the provided key's label has been pressed.
/// 
/// # Example
/// 
/// ```
/// // See if player jumped
/// let should_jump: bool = key_press_label!(Key::UP);
/// 
/// // Expanded
/// let should_jump: bool = Input::singleton().is_key_label_pressed(Key::UP);
/// ```
#[macro_export]
macro_rules! key_press_label {
    ($keycode:expr) => {
        Input::singleton().is_key_label_pressed($keycode)
    };
}

/// Returns whether the provided mouse button has been pressed.
/// 
/// # Example
/// 
/// ```
/// // See if player is shooting
/// let shooting: bool = mouse_press!(MouseButton::LEFT);
/// 
/// // Expanded
/// let shooting: bool = Input::singleton().is_mouse_button_pressed(MouseButton::LEFT);
/// ```
#[macro_export]
macro_rules! mouse_press {
    ($button:expr) => {
        Input::singleton().is_mouse_button_pressed($button)
    };
}

/// Returns whether the provided joypad button on the provided gontroller has been pressed.
/// 
/// # Example
/// 
/// ```
/// // See if player 1 jumped
/// let should_jump: bool = joy_press!(0, JoyButton::A);
/// 
/// // Expanded
/// let should_jump: bool = Input::singleton().is_joy_button_pressed(0, JoyButton::A);
/// ```
#[macro_export]
macro_rules! joy_press {
    ($device:expr, $button:expr) => {
        Input::singleton().is_joy_button_pressed($device, $button)
    };
}

/// Returns whether the provided action has been pressed.
/// 
/// # Panics
/// 
/// Panics if provided action is not found in the Godot.
/// 
/// # Example
/// 
/// ```
/// // See if player jumped
/// let should_jump: bool = act_press!("jump");
/// 
/// // Expanded
/// let should_jump: bool = Input::singleton().is_action_pressed("jump".into());
/// ```
#[macro_export]
macro_rules! act_press {
    ($action:expr) => {
        Input::singleton().is_action_pressed($action.into())
    };
}

/// Returns whether the provided action has been pressed down.
/// 
/// # Panics
/// 
/// Panics if provided action is not found in the Godot.
/// 
/// # Example
/// 
/// ```
/// // See if player started jumping
/// let begun_jumping: bool = act_press_down!("jump");
/// 
/// // Expanded
/// let begun_jumping: bool = Input::singleton().is_action_just_pressed("jump".into());
/// ```
#[macro_export]
macro_rules! act_press_down {
    ($action:expr) => {
        Input::singleton().is_action_just_pressed($action.into())
    };
}

/// Returns whether the provided action has been unpressed.
/// 
/// # Panics
/// 
/// Panics if provided action is not found in the Godot.
/// 
/// # Example
/// 
/// ```
/// // See if player stopped jumping
/// let jump_stopped: bool = act_press_up!("jump");
/// 
/// // Expanded
/// let jump_stopped: bool = Input::singleton().is_action_just_released("jump".into());
/// ```
#[macro_export]
macro_rules! act_press_up {
    ($action:expr) => {
        Input::singleton().is_action_just_released($action.into())
    };
}

/// Returns the strength by which the provided action is pressed down.
/// 
/// # Panics
/// 
/// Panics if provided action is not found in the Godot.
/// 
/// # Example
/// 
/// ```
/// // Get speed so you can speed up the more you push the stick
/// let speed_multiplier: f32 = act_str!("move_right");
/// 
/// // Expanded
/// let speed_multiplier: f32 = Input::singleton().get_action_strength("move_right".into());
/// ```
#[macro_export]
macro_rules! act_str {
    ($action:expr) => {
        Input::singleton().get_action_strength($action.into())
    };
}

/// Returns the raw strength by which the provided action is pressed down.
/// 
/// # Panics
/// 
/// Panics if provided action is not found in the Godot.
/// 
/// # Example
/// 
/// ```
/// // Get speed so you can speed up the more you push the stick
/// let speed_multiplier: f32 = act_str_raw!("move_right");
/// 
/// // Expanded
/// let speed_multiplier: f32 = Input::singleton().get_action_raw_strength("move_right".into());
/// ```
#[macro_export]
macro_rules! act_str_raw {
    ($action:expr) => {
        Input::singleton().get_action_raw_strength($action.into())
    };
}

/// Returns the position on an a given access between two actions.
/// 
/// # Panics
/// 
/// Panics if either of the provided actions is not found in Godot.
/// 
/// # Example
/// 
/// ```
/// // Get controller's Y axis
/// let y_axis: f32 = act_axis!("move_left", "move_right");
/// 
/// // Expanded
/// let y_axis: f32 = Input::singleton().get_axis("move_left".into(), "move_right".into());
/// ```
#[macro_export]
macro_rules! act_axis {
    ($negative_action:expr, $positive_action:expr) => {
        Input::singleton().get_axis($negative_action.into(), $positive_action.into())
    };
}

/// Macro for quickly emitting signal with no arguments.
/// 
/// # Panics
/// 
/// Panics if the provided signal does not exist on self.
/// 
/// # Example
/// ```
/// // Emit that current node has been hit
/// emit!(self, "hit");
/// 
/// // Expanded
/// self.base_mut().emit_signal("hit".into(), &[]);
/// ```
#[macro_export]
macro_rules! emit {
    ($self:ident, $signal:expr) => {
        $self.base_mut().emit_signal($signal.into(), &[])
    };
}

/// Frees provided self.
/// 
/// # Example
/// 
/// ```
/// // Destroy self
/// free!(self);
/// 
/// // Expanded
/// self.base_mut().queue_free();
/// ```
#[macro_export]
macro_rules! free {
    ($self:ident) => {
        $self.base_mut().queue_free()
    };
}

/// Reloads the scene which this node is a part of.
/// 
/// # Panics
/// 
/// Panics if the tree of the current node cannot be reloaded.
/// 
/// # Example
/// 
/// ```
/// // Reload current scene
/// reload!(self);
/// 
/// // Expanded
/// self.base_mut().get_tree().expect("Node has no tree").reload_current_scene();
/// ```
#[macro_export]
macro_rules! reload {
    ($self:ident) => {
        $self.base_mut().get_tree().expect("Node has no tree").reload_current_scene()
    };
}