use crate::empty_children;
use crate::parser::expectation::StatementKind;
use crate::parser::graph::{GraphNode, GraphNodeId};
use crate::parser::tree::{ParseContext, ParseTree, ParseTreeFactory, TreeFn};

use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug)]
pub struct GraphBuilder<C>
where
    C: ParseContext,
{
    nodes: Vec<GraphNode<C>>,
}

impl<C> GraphBuilder<C>
where
    C: ParseContext,
{
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add_node(
        &mut self,
        statement_expectation: StatementKind,
        function: TreeFn<C>,
    ) -> GraphNodeId {
        let id = self.nodes.len();
        let node = GraphNode::new(statement_expectation, function);

        self.nodes.push(node);

        id
    }

    pub fn add_tree(&mut self, tree: &ParseTree<C>) -> GraphNodeId {
        let id = self.add_node(*tree.expectation(), *tree.function());

        for child in tree.children() {
            let child_id = self.add_tree(child);
            self.add_child(id, child_id);
        }

        id
    }

    pub fn add_child(&mut self, parent: GraphNodeId, child: GraphNodeId) {
        let parent = self.nodes.get_mut(parent).unwrap();
        parent.push_child(child);
    }

    pub fn add_children<I>(&mut self, parent: GraphNodeId, children: I)
    where
        I: IntoIterator<Item = GraphNodeId>,
    {
        let parent = self.nodes.get_mut(parent).unwrap();

        for child in children {
            parent.push_child(child);
        }
    }

    pub fn extend(&mut self, parent: GraphNodeId, child: GraphNodeId) {
        self.extend_if(parent, child, |_| true);
    }

    pub fn extend_if<P>(&mut self, parent: GraphNodeId, child: GraphNodeId, predicate: P)
    where
        P: Fn(&GraphNode<C>) -> bool,
    {
        let mut visited = HashSet::new();
        self.extend_if_impl(parent, child, &predicate, &mut visited);
    }

    fn extend_if_impl<P>(
        &mut self,
        current: GraphNodeId,
        child: GraphNodeId,
        predicate: &P,
        visited: &mut HashSet<GraphNodeId>,
    ) where
        P: Fn(&GraphNode<C>) -> bool,
    {
        if !visited.insert(current) {
            return;
        }

        let node = &mut self.nodes[current];

        if node.children().next().is_none() {
            if predicate(node) {
                node.push_child(child);
            }

            return;
        }

        let children: Vec<_> = node.children().copied().collect();
        for child_id in children {
            self.extend_if_impl(child_id, child, predicate, visited);
        }
    }

    pub fn extend_children<I>(&mut self, parent: GraphNodeId, children: I)
    where
        I: IntoIterator<Item = GraphNodeId>,
    {
        self.extend_children_if(parent, children, |_| true);
    }

    pub fn extend_children_if<I, P>(&mut self, parent: GraphNodeId, children: I, predicate: P)
    where
        I: IntoIterator<Item = GraphNodeId>,
        P: Fn(&GraphNode<C>) -> bool,
    {
        let children: Vec<_> = children.into_iter().collect();
        let mut visited = HashSet::new();
        self.extend_children_if_impl(parent, &children, &predicate, &mut visited);
    }

    fn extend_children_if_impl<P>(
        &mut self,
        current: GraphNodeId,
        children: &[GraphNodeId],
        predicate: &P,
        visited: &mut HashSet<GraphNodeId>,
    ) where
        P: Fn(&GraphNode<C>) -> bool,
    {
        if !visited.insert(current) {
            return;
        }

        let node = &mut self.nodes[current];

        if node.children().next().is_none() {
            if predicate(node) {
                for &child in children {
                    node.push_child(child);
                }
            }

            return;
        }

        let next: Vec<_> = node.children().copied().collect();
        for child_id in next {
            self.extend_children_if_impl(child_id, children, predicate, visited);
        }
    }

    pub fn build_from(
        self,
        tree_factory: Rc<ParseTreeFactory<C>>,
        root: GraphNodeId,
    ) -> Rc<ParseTree<C>> {
        let mut trees: Vec<Rc<ParseTree<C>>> = Vec::with_capacity(self.nodes.len());

        for node in &self.nodes {
            trees.push(Rc::new(ParseTree::new(
                Rc::clone(&tree_factory),
                *node.statement_expectation(),
                empty_children!(),
                node.function(),
            )));
        }

        unsafe {
            for (id, node) in self.nodes.iter().enumerate() {
                let tree_ptr = Rc::as_ptr(&trees[id]) as *mut ParseTree<C>;

                (*tree_ptr)
                    .push_children(node.children().map(|&child_id| Rc::clone(&trees[child_id])));
            }
        }

        Rc::clone(&trees[root])
    }
}

impl<C> Default for GraphBuilder<C>
where
    C: ParseContext,
{
    fn default() -> Self {
        Self::new()
    }
}
