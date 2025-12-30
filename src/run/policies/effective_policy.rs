use std::{collections::HashSet, path::PathBuf};

pub struct EffectivePolicy {
    pub effective_includes: Vec<PathBuf>,
    pub effective_excludes: Vec<PathBuf>,
    pub effective_includes_hash: HashSet<PathBuf>,
    pub effective_excludes_hash: HashSet<PathBuf>,
}

impl EffectivePolicy {
    fn skip_paths(
        &mut self,
        vector_paths: &[PathBuf],
        is_include: bool,
        mut kept_set: HashSet<PathBuf>,
    ) -> HashSet<PathBuf> {
        for path in vector_paths {
            let mut skip = false;
            if kept_set.contains(path) {
                continue;
            }
            for ancestor in path.ancestors().skip(1) {
                if kept_set.contains(ancestor) {
                    skip = true;
                    break;
                }
            }
            if !skip {
                let correct_path: PathBuf = path.clone();
                kept_set.insert(correct_path.clone());
                match is_include {
                    true => {
                        self.effective_includes.push(correct_path);
                    }
                    false => {
                        self.effective_excludes.push(correct_path);
                    }
                }
            }
        }
        kept_set
    }

    fn retain(vector_paths: &[PathBuf], kept_set: &HashSet<PathBuf>) -> HashSet<PathBuf> {
        let mut other_set: HashSet<PathBuf> = HashSet::new();
        for path in vector_paths {
            for ancestor in path.ancestors() {
                if kept_set.contains(ancestor) {
                    other_set.insert(path.clone());
                    break;
                }
            }
        }
        other_set
    }

    pub fn set_effective_policy(
        &mut self,
        mut include_roots: Vec<PathBuf>,
        mut exclude_roots: Vec<PathBuf>,
    ) -> &EffectivePolicy {
        // Initialize effective include to an empty vector
        self.effective_includes = Vec::new();
        self.effective_excludes = Vec::new();

        // sort based on the depth of the path
        include_roots.sort_by_key(|included_root| included_root.components().count());
        exclude_roots.sort_by_key(|excluded_root| excluded_root.components().count());

        // skip paths and build effective include policy roots
        let kept_set = self.skip_paths(&include_roots, true, HashSet::new());

        // only retain the excluded folders which are in effective includes
        let excluded_set = Self::retain(&exclude_roots, &kept_set);
        let mut updated_excluded_roots: Vec<PathBuf> = Vec::new();
        for path in excluded_set {
            updated_excluded_roots.push(path);
        }

        // skip paths and build effective exclude policy roots
        updated_excluded_roots.sort_by_key(|excluded_root| excluded_root.components().count());
        let excluded_hash = self.skip_paths(&updated_excluded_roots, false, HashSet::new());

        self.effective_includes_hash = kept_set;
        self.effective_excludes_hash = excluded_hash;
        self
    }
}
