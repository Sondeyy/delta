use std::env;

const COLORTERM: &str = "COLORTERM";
const BAT_THEME: &str = "BAT_THEME";
const GIT_CONFIG_PARAMETERS: &str = "GIT_CONFIG_PARAMETERS";
const GIT_PREFIX: &str = "GIT_PREFIX";
const DELTA_FEATURES: &str = "DELTA_FEATURES";
const DELTA_NAVIGATE: &str = "DELTA_NAVIGATE";
const DELTA_EXPERIMENTAL_MAX_LINE_DISTANCE_FOR_NAIVELY_PAIRED_LINES: &str =
    "DELTA_EXPERIMENTAL_MAX_LINE_DISTANCE_FOR_NAIVELY_PAIRED_LINES";
const DELTA_PAGER: &str = "DELTA_PAGER";

#[derive(Default, Clone)]
pub struct DeltaEnv {
    pub bat_theme: Option<String>,
    pub colorterm: Option<String>,
    pub current_dir: Option<std::path::PathBuf>,
    pub experimental_max_line_distance_for_naively_paired_lines: Option<String>,
    pub features: Option<String>,
    pub git_config_parameters: Option<String>,
    pub git_prefix: Option<String>,
    pub navigate: Option<String>,
    pub pagers: (Option<String>, Option<String>),
}

impl DeltaEnv {
    /// Create a structure with current environment variable
    pub fn init() -> Self {
        let bat_theme = env::var(BAT_THEME).ok();
        let colorterm = env::var(COLORTERM).ok();
        let experimental_max_line_distance_for_naively_paired_lines =
            env::var(DELTA_EXPERIMENTAL_MAX_LINE_DISTANCE_FOR_NAIVELY_PAIRED_LINES).ok();
        let features = env::var(DELTA_FEATURES).ok();
        let git_config_parameters = env::var(GIT_CONFIG_PARAMETERS).ok();
        let git_prefix = env::var(GIT_PREFIX).ok();
        let navigate = env::var(DELTA_NAVIGATE).ok();

        let current_dir = env::current_dir().ok();
        let pagers = (
            env::var(DELTA_PAGER).ok(),
            // We're using `bat::config::get_pager_executable` here instead of just returning
            // the pager from the environment variables, because we want to make sure
            // that the pager is a valid pager from env and handle the case of
            // the PAGER being set to something invalid like "most" and "more".
            bat::config::get_pager_executable(None),
        );

        Self {
            bat_theme,
            colorterm,
            current_dir,
            experimental_max_line_distance_for_naively_paired_lines,
            features,
            git_config_parameters,
            git_prefix,
            navigate,
            pagers,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::DeltaEnv;
    use std::env;

    #[test]
    fn test_env_parsing() {
        let feature = "Awesome Feature";
        env::set_var("DELTA_FEATURES", feature);
        let env = DeltaEnv::init();
        assert_eq!(env.features, Some(feature.into()));
    }

    #[test]
    fn test_env_parsing_with_pager_set_to_bat() {
        env::set_var("PAGER", "bat");
        let env = DeltaEnv::init();
        assert_eq!(env.pagers.1, Some("bat".into()));
    }

    #[test]
    fn test_env_parsing_with_pager_set_to_more() {
        env::set_var("PAGER", "more");
        let env = DeltaEnv::init();
        assert_eq!(env.pagers.1, Some("less".into()));
    }

    #[test]
    fn test_env_parsing_with_pager_set_to_most() {
        env::set_var("PAGER", "most");
        let env = DeltaEnv::init();
        assert_eq!(env.pagers.1, Some("less".into()));
    }
}
