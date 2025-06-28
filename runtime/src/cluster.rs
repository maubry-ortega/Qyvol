// # VolleyDevByMaubry [5/∞] "Un clúster une nodos como ideas en una revolución distribuida."
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClusterError {
    #[error("Acción no soportada: {0}")]
    InvalidAction(String),
    #[error("Nodo requerido")]
    NodeRequired,
}

pub fn manage_cluster(action: &str, node: Option<String>) -> Result<(), ClusterError> {
    match action {
        "add-node" => {
            let node = node.ok_or(ClusterError::NodeRequired)?;
            println!("Añadiendo nodo: {node}");
            Ok(())
        }
        _ => Err(ClusterError::InvalidAction(action.to_string())),
    }
}
