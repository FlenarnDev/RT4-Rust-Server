#[repr(u8)]
#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub enum EntityLifeCycle {
    FOREVER = 0, // Never respawns or despawns, always in the world.
    RESPAWN = 1, // Entity added from engine that respawns later.
    DESPAWN = 2, // Entity added from script that despawns later,
}