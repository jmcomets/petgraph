#[macro_export]
macro_rules! define_edges {
    ($G:ident, $($source:expr => $target:expr),* ) => {
        {
            use std::collections::HashMap;

            let mut g = $G::new();
            let mut nodes = HashMap::new();

            $(
                {
                    let s = *nodes.entry($source).or_insert_with(|| { g.add_node($source) });
                    let t = *nodes.entry($target).or_insert_with(|| { g.add_node($target) });
                    g.add_edge(s, t, ());
                }
             )*

            g
        }
    };
}

macro_rules! clone_fields {
    ($name:ident, $($field:ident),+ $(,)*) => (
        fn clone(&self) -> Self {
            $name {
                $(
                    $field : self . $field .clone()
                ),*
            }
        }
    );
}

