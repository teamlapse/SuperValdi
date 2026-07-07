use valdi_rust_ir::{
    ids::{ComponentId, NodeId, SourceSpanId},
    tree::{RootNode, TreeChild},
};

use crate::{
    diagnostics::{RuntimeDiagnostic, RuntimeResult},
    tree::{RuntimeChild, RuntimeNode, RuntimeNodeDeclaration},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeDocumentDeclaration {
    pub fixture_id: &'static str,
    pub component_id: ComponentId,
    pub root: RootNode,
    pub nodes: &'static [RuntimeNodeDeclaration],
    pub children: &'static [RuntimeChild],
    pub source_span_id: Option<SourceSpanId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeDocument {
    pub fixture_id: &'static str,
    pub component_id: ComponentId,
    pub root: RootNode,
    pub nodes: Vec<RuntimeNode>,
    pub children: Vec<RuntimeChild>,
    pub source_span_id: Option<SourceSpanId>,
}

impl RuntimeDocument {
    pub fn node(&self, node_id: NodeId) -> Option<&RuntimeNode> {
        self.nodes.iter().find(|node| node.node_id() == node_id)
    }

    pub fn child_for(&self, child_id: NodeId) -> Option<&RuntimeChild> {
        self.children
            .iter()
            .find(|child| child.tree.child_id == child_id)
    }

    pub fn contains_node(&self, node_id: NodeId) -> bool {
        self.node(node_id).is_some()
    }
}

pub fn load_runtime_document(
    declaration: &RuntimeDocumentDeclaration,
) -> RuntimeResult<RuntimeDocument> {
    let document = RuntimeDocument {
        fixture_id: declaration.fixture_id,
        component_id: declaration.component_id,
        root: declaration.root,
        nodes: declaration
            .nodes
            .iter()
            .copied()
            .map(RuntimeNode::from_declaration)
            .collect(),
        children: declaration.children.to_vec(),
        source_span_id: declaration.source_span_id,
    };
    validate_document(&document)?;
    Ok(document)
}

fn validate_document(document: &RuntimeDocument) -> RuntimeResult<()> {
    if document.node(document.root.node_id).is_none() {
        return Err(RuntimeDiagnostic::error(
            "RUNTIME_DOCUMENT_INVALID",
            "$.root.node_id",
            format!("root node {} is missing", document.root.node_id.as_str()),
            document.source_span_id,
        ));
    }

    for (index, node) in document.nodes.iter().enumerate() {
        if document.nodes[index + 1..]
            .iter()
            .any(|other| other.node_id() == node.node_id())
        {
            return Err(RuntimeDiagnostic::error(
                "RUNTIME_NODE_ID_DUPLICATE",
                "$.nodes[].node_id",
                format!("duplicate runtime node {}", node.node_id().as_str()),
                node.source_span_id,
            ));
        }
    }

    for child in &document.children {
        require_child_endpoint(
            document,
            child.tree.parent_id,
            "$.children[].parent_id",
            child.tree,
        )?;
        require_child_endpoint(
            document,
            child.tree.child_id,
            "$.children[].child_id",
            child.tree,
        )?;
    }
    Ok(())
}

fn require_child_endpoint(
    document: &RuntimeDocument,
    node_id: NodeId,
    path: &'static str,
    child: TreeChild,
) -> RuntimeResult<()> {
    if document.contains_node(node_id) {
        return Ok(());
    }
    Err(RuntimeDiagnostic::error(
        "RUNTIME_CHILD_ENDPOINT_MISSING",
        path,
        format!(
            "child edge {} -> {} references missing node {}",
            child.parent_id.as_str(),
            child.child_id.as_str(),
            node_id.as_str()
        ),
        document.source_span_id,
    ))
}
