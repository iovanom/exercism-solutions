pub mod graph {

    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;

    type Attributes = HashMap<String, String>;

    fn collect_attributes(attrs: &[(&str, &str)]) -> Attributes {
        attrs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    macro_rules! impl_with_attrs {
        () => {
            pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                self.attrs = crate::graph::collect_attributes(attrs);
                self
            }
        };
    }

    #[derive(Default)]
    pub struct Graph<'a> {
        pub nodes: Vec<Node<'a>>,
        pub edges: Vec<Edge<'a>>,
        pub attrs: Attributes,
    }

    impl<'a> Graph<'a> {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_nodes(mut self, nodes: &Vec<Node<'a>>) -> Self {
            self.nodes.extend_from_slice(&nodes);
            self
        }

        pub fn with_edges(mut self, edges: &Vec<Edge<'a>>) -> Self {
            self.edges.extend_from_slice(&edges);
            self
        }

        impl_with_attrs!();

        pub fn get_node(&self, node: &str) -> Option<Node<'a>> {
            self.nodes.iter().cloned().find(|n| n.name == node)
        }
    }

    pub mod graph_items {

        pub mod node {

            #[derive(Debug, Clone, PartialEq, Default)]
            pub struct Node<'a> {
                pub(crate) name: &'a str,
                attrs: super::super::Attributes,
            }

            impl<'a> Node<'a> {
                pub fn new(name: &'a str) -> Self {
                    Self {
                        name,
                        ..Self::default()
                    }
                }

                impl_with_attrs!();

                pub fn get_attr(&mut self, attr: &'a str) -> Option<&str> {
                    Some(self.attrs.entry(attr.to_string()).or_default())
                }
            }
        }

        pub mod edge {

            #[derive(Debug, Clone, PartialEq, Default)]
            pub struct Edge<'a> {
                points: (&'a str, &'a str),
                attrs: super::super::Attributes,
            }

            impl<'a> Edge<'a> {
                pub fn new(p1: &'a str, p2: &'a str) -> Self {
                    Self {
                        points: (p1, p2),
                        ..Self::default()
                    }
                }

                impl_with_attrs!();
            }
        }
    }
}
