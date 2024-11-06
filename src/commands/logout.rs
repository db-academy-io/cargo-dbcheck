use super::CommandContext;

pub fn logout(context: &mut CommandContext) -> Result<(), anyhow::Error> {
    context.remove_token()?;
    println!("Successfully logged out");
    Ok(())
}
