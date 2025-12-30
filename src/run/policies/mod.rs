use std::{collections::HashSet, path::Path};

use crate::run::{
    config::config::Config,
    policies::{config_policy::ConfigPolicy, effective_policy::EffectivePolicy},
};

pub mod config_policy;
pub mod effective_policy;

pub fn setup_policy_for_walk(config: &Config, base_dir: &Path) -> EffectivePolicy {
    let config_policy: ConfigPolicy = ConfigPolicy::new(config, base_dir);
    let mut effective_policy = EffectivePolicy {
        effective_includes: Vec::new(),
        effective_excludes: Vec::new(),
        effective_includes_hash: HashSet::new(),
        effective_excludes_hash: HashSet::new(),
    };
    effective_policy.set_effective_policy(config_policy.include_roots, config_policy.exclude_roots);
    effective_policy
}

pub fn should_process(policy: &EffectivePolicy, current_path: &Path) -> bool {
    for current_path_ancestor in current_path.ancestors() {
        if policy
            .effective_excludes_hash
            .contains(current_path_ancestor)
        {
            return false;
        }
    }
    return true;
}

pub fn should_descend(policy: &EffectivePolicy, current_path: &Path) -> bool {
    for current_path_ancestor in current_path.ancestors() {
        if policy
            .effective_excludes_hash
            .contains(current_path_ancestor)
        {
            return false;
        }
    }
    return true;
}
