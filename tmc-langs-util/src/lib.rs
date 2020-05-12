pub mod tar;
pub mod task_executor;

use tmc_langs_framework::plugin::LanguagePlugin;
use tmc_langs_python3::Python3Plugin;

const PLUGINS: [&dyn LanguagePlugin; 1] = [&Python3Plugin::new()];
