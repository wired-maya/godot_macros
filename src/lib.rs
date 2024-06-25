#[macro_export]
macro_rules! nm {
    ($r#node_name:ident) => {
        {
            self.base_mut().get_node_as::<Node>(stringify!($node_name))
        }
    };
}