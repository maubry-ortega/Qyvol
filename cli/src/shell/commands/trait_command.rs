// # VolleyDevByMaubry [37/∞] "Un comando es una pieza modular, lista para ser extendida."
#[allow(dead_code)]
pub trait Command {
    fn name(&self) -> &'static str;
    fn aliases(&self) -> &[&'static str] {
        &[]
    }
    fn description(&self) -> &'static str;
    fn execute(
        &self, args: &[&str], context: &mut crate::shell::context::ShellContext,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
