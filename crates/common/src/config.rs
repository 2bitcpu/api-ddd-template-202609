use argh::FromArgs;
use serde::{Deserialize, Deserializer, de::Error};
use std::{path::PathBuf, sync::OnceLock};
use tracing_subscriber::EnvFilter;

const CONFIG_FILE_NAME: &str = "config.yaml";
const UNKNOWN_NAME: &str = "unknown";

fn get_exe_name() -> String {
    std::env::current_exe()
        .ok()
        .and_then(|path| path.file_stem().map(|s| s.to_string_lossy().to_string()))
        .unwrap_or_else(|| UNKNOWN_NAME.to_string())
}

// region: get config paths
fn get_cur_config_file() -> PathBuf {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from(UNKNOWN_NAME))
        .join(CONFIG_FILE_NAME)
}

fn get_exe_config_file() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from(UNKNOWN_NAME))
        .join(CONFIG_FILE_NAME)
}

fn get_user_config_file() -> PathBuf {
    if let Ok(xdg_config_home) = std::env::var("XDG_CONFIG_HOME") {
        PathBuf::from(xdg_config_home)
            .join(get_exe_name())
            .join(CONFIG_FILE_NAME)
    } else {
        PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| UNKNOWN_NAME.to_string()))
            .join(".config")
            .join(get_exe_name())
            .join(CONFIG_FILE_NAME)
    }
}

fn get_system_config_file() -> PathBuf {
    PathBuf::from("/etc")
        .join(get_exe_name())
        .join(CONFIG_FILE_NAME)
}
// endregion: get config paths

// region: server config
#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default)]
    pub static_dir: Option<PathBuf>,
    #[serde(default)]
    pub cors: Vec<String>,
    #[serde(default = "default_lock_timeout_millis")]
    pub lock_timeout_millis: u64,
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    3000
}

fn default_lock_timeout_millis() -> u64 {
    50
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            host: default_host(),
            port: default_port(),
            static_dir: None,
            cors: vec![],
            lock_timeout_millis: default_lock_timeout_millis(),
        }
    }
}
// endregion: server config

// region: storage config
#[derive(Debug, Deserialize)]
pub struct StorageConfig {
    #[serde(default = "default_data_dir")]
    pub data_dir: PathBuf,
    #[serde(default = "default_queue_size")]
    pub queue_size: usize,
    #[serde(default = "default_wait_interval_seconds")]
    pub wait_interval_seconds: u64,
}

fn default_data_dir() -> PathBuf {
    PathBuf::from("data")
}

fn default_queue_size() -> usize {
    32
}

fn default_wait_interval_seconds() -> u64 {
    5
}

impl Default for StorageConfig {
    fn default() -> Self {
        StorageConfig {
            data_dir: default_data_dir(),
            queue_size: default_queue_size(),
            wait_interval_seconds: default_wait_interval_seconds(),
        }
    }
}
// endregion: storage config

// region: security config
#[derive(Debug, Deserialize)]
pub struct SecurityConfig {
    #[serde(default)]
    pub allow_signup: bool,
    #[serde(default = "default_jwt_secret")]
    pub jwt_secret: String,
    #[serde(default = "default_jwt_issuer")]
    pub jwt_issuer: String,
    #[serde(default = "default_jwt_expires_minutes")]
    pub jwt_expires_minutes: i64,
}

fn default_jwt_secret() -> String {
    uuid25::gen_v4().to_string()
}

fn default_jwt_issuer() -> String {
    get_exe_name()
}

fn default_jwt_expires_minutes() -> i64 {
    60 * 24 * 7
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            allow_signup: false,
            jwt_secret: default_jwt_secret(),
            jwt_issuer: default_jwt_issuer(),
            jwt_expires_minutes: default_jwt_expires_minutes(),
        }
    }
}
// endregion: security config

// region: logging config
#[derive(Debug, Deserialize, Default)]
pub struct LoggingConfig {
    #[serde(deserialize_with = "deserialize_env_filter")]
    pub level: Option<EnvFilter>,
}

fn deserialize_env_filter<'de, D>(deserializer: D) -> Result<Option<EnvFilter>, D::Error>
where
    D: Deserializer<'de>,
{
    let level = Option::<String>::deserialize(deserializer)?;

    level
        .map(|value| EnvFilter::try_new(value).map_err(D::Error::custom))
        .transpose()
}

// endregion: logging config

// region: config
#[derive(Debug, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub storage: StorageConfig,
    #[serde(default)]
    pub security: SecurityConfig,
    #[serde(default)]
    pub logging: LoggingConfig,
}

impl Config {
    fn new() -> Self {
        let configs = vec![
            get_cur_config_file(),
            get_exe_config_file(),
            get_user_config_file(),
            get_system_config_file(),
        ];

        for config in configs {
            if config.exists()
                && let Ok(file) = std::fs::File::open(&config)
                && let Ok(cfg) = serde_yaml::from_reader(file)
            {
                return cfg;
            }
        }

        Config::default()
    }
}
// endregion: config

// region: cli
#[derive(FromArgs, Debug, Clone)]
/// command line interface
struct Cli {
    #[argh(option)]
    /// server host
    host: Option<String>,

    #[argh(option)]
    /// server port
    port: Option<u16>,

    #[argh(option)]
    /// static directory
    static_dir: Option<PathBuf>,

    #[argh(option)]
    /// cors origin
    cors: Vec<String>,

    #[argh(switch)]
    /// no cors
    no_cors: bool,

    #[argh(switch)]
    /// no static
    no_static: bool,

    #[argh(option, long = "lock-timeout")]
    /// lock timeout millis
    lock_timeout_millis: Option<u64>,

    #[argh(option)]
    /// data directory
    data_dir: Option<PathBuf>,

    #[argh(option)]
    /// queue size
    queue_size: Option<usize>,

    #[argh(option, long = "wait-interval")]
    /// wait interval seconds
    wait_interval_seconds: Option<u64>,

    #[argh(switch)]
    /// allow signup
    allow_signup: bool,

    #[argh(switch)]
    /// deny signup
    deny_signup: bool,

    #[argh(option)]
    /// jwt secret
    jwt_secret: Option<String>,

    #[argh(option)]
    /// jwt issuer
    jwt_issuer: Option<String>,

    #[argh(option, long = "jwt-expires")]
    /// jwt expires minutes
    jwt_expires_minutes: Option<i64>,

    #[argh(option, long = "log")]
    /// log level
    log_level: Option<EnvFilter>,

    #[argh(switch)]
    /// no log
    no_log: bool,
}

fn get_cli() -> Cli {
    let args = std::env::args()
        .flat_map(|arg| {
            if arg.starts_with('-') {
                arg.split_once('=')
                    .map(|(key, value)| vec![key.to_string(), value.to_string()])
                    .unwrap_or_else(|| vec![arg])
            } else {
                vec![arg]
            }
        })
        .collect::<Vec<_>>();

    let command_name = [args[0].as_str()];
    let args = args[1..].iter().map(String::as_str).collect::<Vec<_>>();

    Cli::from_args(&command_name, &args).unwrap_or_else(|e| {
        eprintln!("{}", e.output);
        std::process::exit(1);
    })
}
// endregion: cli

fn get_config() -> Config {
    let mut config = Config::new();

    let cli = get_cli();

    if let Some(value) = cli.host
        && !value.is_empty()
    {
        config.server.host = value;
    }

    if let Some(value) = cli.port {
        config.server.port = value;
    }

    if cli.no_static {
        config.server.static_dir = None;
    } else if let Some(value) = cli.static_dir {
        config.server.static_dir = Some(value);
    }

    if cli.no_cors {
        config.server.cors = vec![];
    } else {
        config.server.cors = cli.cors;
    }
    if config.server.cors.iter().any(|origin| origin == "*") {
        config.server.cors = vec!["*".to_string()];
    }

    if let Some(value) = cli.lock_timeout_millis {
        config.server.lock_timeout_millis = value;
    }

    if let Some(value) = cli.data_dir {
        config.storage.data_dir = value;
    }

    let writeable = match std::fs::metadata(&config.storage.data_dir) {
        Ok(meta) => meta.is_dir() && !meta.permissions().readonly(),
        Err(_) => false,
    };
    if !writeable {
        eprintln!(
            "data directory ({}) is not writeable",
            config.storage.data_dir.display()
        );
        std::process::exit(1);
    }

    if let Some(value) = cli.queue_size {
        config.storage.queue_size = value;
    }

    if let Some(value) = cli.wait_interval_seconds {
        config.storage.wait_interval_seconds = value;
    }

    if cli.deny_signup {
        config.security.allow_signup = false;
    } else if cli.allow_signup {
        config.security.allow_signup = true;
    }

    if let Some(value) = cli.jwt_secret
        && !value.is_empty()
    {
        config.security.jwt_secret = value;
    }

    if let Some(value) = cli.jwt_issuer
        && !value.is_empty()
    {
        config.security.jwt_issuer = value;
    }

    if let Some(value) = cli.jwt_expires_minutes {
        config.security.jwt_expires_minutes = value;
    }

    if cli.no_log {
        config.logging.level = None;
    } else if let Some(value) = cli.log_level {
        config.logging.level = Some(value);
    }

    config.security.jwt_expires_minutes =
        config.security.jwt_expires_minutes.clamp(1, 60 * 24 * 180);

    if let Some(dir) = &config.server.static_dir {
        if !dir.is_dir() {
            eprintln!("static directory ({}) is not a directory", dir.display());
            config.server.static_dir = None;
        }
    }

    config.server.lock_timeout_millis = config.server.lock_timeout_millis.clamp(1, 1000);
    config.storage.queue_size = config.storage.queue_size.clamp(3, 128);
    config.storage.wait_interval_seconds = config.storage.wait_interval_seconds.clamp(1, 300);

    config
}

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn config() -> &'static Config {
    CONFIG.get_or_init(get_config)
}
