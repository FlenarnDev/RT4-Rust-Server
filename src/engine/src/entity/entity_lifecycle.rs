#[repr(u8)]
#[derive(Eq, Hash, PartialEq)]
pub enum EntityLifeCycle {
    Forever = 0, // Never respawns or despawns, always in the world.
    Respawn = 1, // Entity added from engine that respawns later.
    Despawn = 2, // Entity added from script that despawns later,
}