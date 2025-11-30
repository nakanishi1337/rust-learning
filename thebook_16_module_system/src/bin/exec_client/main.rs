mod config_server;

use thebook_16_module_system::modules::server_module;

fn main() {
    config_server::config_test();
    server_module::test_access();
}