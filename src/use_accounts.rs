use std::collections::{HashMap, HashSet};
use std::io::Result;

use crate::baresip::{Arg, UserAgent};

pub fn handler(arg: Arg, user_agents: &HashMap<String, UserAgent>) -> Result<()> {
    let user_agent_names: HashSet<&str> = arg.param.split(',').collect();
    for (name, user_agent) in user_agents.iter() {
        if user_agent_names.contains(name.as_str()) {
            if !user_agent.is_registered() {
                user_agent.register()?;
            }
        } else if user_agent.is_registered() {
            user_agent.unregister();
        }
    }
    Ok(())
}
