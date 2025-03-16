/// These priorities are important for cases where the content developer wants to be aware
/// of the bandwidth implications their script may run into and how it impacts the player experience.
#[derive(Debug, PartialEq)]
pub enum ServerProtocolPriority {
    /// Counted as part of the [buffer_full] command
    /// Alternate names: [LOW], [CONTENT]
    BUFFERED,
    
    /// Not counted as part of the [buffer_full] command
    /// Alternate names: [HIGH], [ESSENTIAL], [ENGINE]
    IMMEDIATE
}