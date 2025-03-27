use engine::engine::Engine;
use engine::util::cache::obj_unpacker::unpack_objs;

fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    unpack_objs();
    
    //Engine::init();

    //Engine::get().start(true);
}