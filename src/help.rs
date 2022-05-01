use serenity::prelude::TypeMapKey;

mod command_list;
pub use command_list::{CommandList, CommandSearchResult};
pub mod embed;

pub struct CommandListKey;
impl TypeMapKey for CommandListKey {
    type Value = CommandList;
}
