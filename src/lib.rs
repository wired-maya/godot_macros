//! A simple crate that adds a variety of godot (specifically gdext) related macros for convenience.

// TODO: Write docs that pass tests

/// Macro for quickly getting a non-mutable reference to a node from its path. Can choose to specify node type, otherwise defaults to Node.
///
/// # Panics
/// 
/// Panics if node_path cannot be found.
/// 
/// # Examples
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
/// # Examples
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
/// Note: Callback function must be registered as a function with godot, either in GDScript or by #\[func\].
/// 
/// # Panics
/// 
/// Panics if any of the named strings do not correspond to anything in the engine.
/// 
/// # Examples
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
        n!($self, $node_path).connect($signal.into(), $self.base().callable($callback_name));
    };
    ($self:ident, $node_path_1:expr, $signal:expr, $node_path_2:expr, $callback_name:expr) => {
        n!($self, $node_path_1).connect($signal.into(), n!($self, $node_path_2).callable($callback_name));
    };
}