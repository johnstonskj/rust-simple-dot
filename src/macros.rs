/*!
One-line description.

More detailed description, with

# Example

 */

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

macro_rules! builder_property {
    ($v:vis $name:ident, $rtype:ty, $ptype:ty) => {
        paste! {
            $v fn $name(&self) -> &$rtype {
                &self.$name
            }

            $v fn [<set_ $name>](mut self, $name: &$ptype) -> Self {
                self.$name = $name;
                self
            }
        }
    };
    ($v:vis $name:ident, $type:ty => $delegate:expr) => {
        paste! {
            $v fn $name(&self) -> &$type {
                &self.$delegate.$name
            }

            $v fn [<set_ $name>](mut self, $name: &$type) -> Self {
                self.$delegate.$name = $name;
                self
            }
        }
    };
    ($v:vis $name:ident, $type:ty) => {
        paste! {
            $v fn $name(&self) -> &$type {
                &self.$name
            }

            $v fn [<set_ $name>](mut self, $name: &$type) -> Self {
                self.$name = $name;
                self
            }
        }
    };
}

macro_rules! vec_builder_property {
    ($v:vis $name:ident, $type:ty => $delegate:expr) => {
        paste! {
            $v fn [<$name s>](&self) -> impl Iterator<Item = &$type> {
                self.$delegate.[<$name s>].iter()
            }

            $v fn [<add_ $name>](mut self, $name: $type) -> Self {
                self.$delegate.[<$name s>].push($name);
                self
            }
        }
    };
    ($v:vis $name:ident, $type:ty) => {
        paste! {
            $v fn [<$name s>](&self) -> impl Iterator<Item = &$type> {
                self.[<$name s>].iter()
            }

            $v fn [<add_ $name>](mut self, $name: $type) -> Self {
                self.[<$name s>].push($name);
                self
            }
        }
    };
}

macro_rules! optional_builder_property {
    ($v:vis $name:ident, $type:ty => $delegate:expr) => {
        paste! {
            $v fn $name(&self) -> Option<&$type> {
                self.$delegate.$name.as_ref()
            }

            $v fn [<set_ $name>](mut self, $name: $type) -> Self {
                self.$delegate.$name = Some($name);
                self
            }

            $v fn [<unset_ $name>](mut self) -> Self {
                self.$delegate.$name = None;
                self
            }
        }
    };
    ($v:vis $name:ident, $type:ty) => {
        paste! {
            $v fn $name(&self) -> Option<&$type> {
                self.$name.as_ref()
            }

            $v fn [<set_ $name>](mut self, $name: $type) -> Self {
                self.$name = Some($name);
                self
            }

            $v fn [<unset_ $name>](mut self) -> Self {
                self.$name = None;
                self
            }
        }
    };
    ($v:vis string $name:ident => $delegate:expr) => {
        paste! {
            $v fn $name(&self) -> Option<&String> {
                self.$delegate.$name.as_ref()
            }

            $v fn [<set_ $name>](mut self, $name: &str) -> Self {
                self.$delegate.$name = Some($name.to_owned());
                self
            }

            $v fn [<unset_ $name>](mut self) -> Self {
                self.$delegate.$name = None;
                self
            }
        }
    };
    ($v:vis string $name:ident) => {
        paste! {
            $v fn $name(&self) -> Option<&String> {
                self.$name.as_ref()
            }

            $v fn [<set_ $name>](mut self, $name: &str) -> Self {
                self.$name = Some($name.to_owned());
                self
            }

            $v fn [<unset_ $name>](mut self) -> Self {
                self.$name = None;
                self
            }
        }
    };
}

macro_rules! optional_property {
    ($name:ident, $type:ty) => {
        paste! {
            pub fn $name(&self) -> Option<&$type> {
                self.$name.as_ref()
            }

            pub fn [<set_ $name>](&mut self, $name: $type) -> &mut Self {
                self.$name = Some($name);
                self
            }

            pub fn [<unset $name>](&mut self) -> &mut Self {
                self.$name = None;
                self
            }
        }
    };
}

macro_rules! write_optional_property {
    ($fn_name:ident => $display_name:expr) => {
        paste! {
            #[inline(always)]
            fn [<write_$fn_name>](
                &self,
                f: &mut std::fmt::Formatter<'_>,
                _indent_level: u32,
                in_block: bool,) -> std::fmt::Result {
                if let Some(v) = self.$fn_name() {
                    write!(f, "{} = {};{}", $display_name, v, if in_block { "\n" } else { " " })?;
                }
                Ok(())
            }
        }
    };
}

macro_rules! display_to_inner {
    ($type:ty, $block:expr) => {
        impl Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.inner_fmt(f, 0, $block)
            }
        }
    };
    ($type:ty) => {
        impl Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.inner_fmt(f, 0, false)
            }
        }
    };
}

macro_rules! shared_graph_impl {
    () => {
        pub fn id(&self) -> &String {
            &self.inner.id
        }

        pub fn set_id(mut self, id: &str) -> Self {
            assert!(!id.is_empty());
            self.inner.id = id.to_owned();
            self
        }

        optional_builder_property!(pub string label => inner);

        optional_builder_property!(pub style, GraphStyle => inner);

        optional_builder_property!(pub options, GraphOptions => inner);

        optional_builder_property!(pub default_node_style, NodeStyle => inner);

        optional_builder_property!(pub default_edge_style, EdgeStyle => inner);

        pub fn node(&self, id: &str) -> Option<&Node> {
            self.inner.nodes.get(id)
        }

        pub fn nodes(&self) -> impl Iterator<Item = &Node> {
            self.inner.nodes.values()
        }

        pub fn add_node(&mut self, node: Node) {
            let _ = self.inner.nodes.insert(node.id().to_owned(), node);
        }

        vec_builder_property!(pub edge, Edge => inner);

        vec_builder_property!(pub sub_graph, SubGraph => inner);

        fn graph_fmt(
            &self,
            f: &mut std::fmt::Formatter<'_>,
            _indent_level: u32,
            _in_block: bool,
        ) -> std::fmt::Result {
            if let Some(label) = &self.label() {
                writeln!(f, "label={:?}", label)?;
            }

            if self.options().is_some() || self.style().is_some() {
                writeln!(f, "graph [")?;
                if let Some(options) = &self.options() {
                    writeln!(f, "{}", options)?;
                }
                if let Some(style) = &self.style() {
                    writeln!(f, "{}", style)?;
                }
                writeln!(f, "]")?;
            }

            if let Some(default_node_style) = &self.default_node_style() {
                writeln!(f, "node [")?;
                writeln!(f, "{}", default_node_style)?;
                writeln!(f, "]")?;
            }
            if let Some(default_edge_style) = &self.default_edge_style() {
                writeln!(f, "edge [")?;
                writeln!(f, "{}", default_edge_style)?;
                writeln!(f, "]")?;
            }
            for node in self.nodes() {
                writeln!(f, "{}", node)?;
            }
            for edge in self.edges() {
                writeln!(f, "{}", edge)?;
            }
            for sub_graph in self.sub_graphs() {
                writeln!(f, "{}", sub_graph)?;
            }
            Ok(())
        }

    };
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
