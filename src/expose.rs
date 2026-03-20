#![allow(dead_code)]

// The structure backing the storage that raft operates on!!!

pub trait Store<K, V>
where
    Self: Sized + Send + Sync,
    K: Send + 'static,
    V: Send + 'static,
{
    type Command: Command;
    type Error;

    // We take a &self because we want the underlying data structure to be concurrently modified
    // across thread boundaries and therefore we would need interior mutability for it!
    async fn apply_command(&self, command: Self::Command) -> Result<Option<V>, Self::Error>;
}

pub trait Command {}

// --------------------------------------------------------------------------------------------

// Example of how the structure can be used!

pub enum Commands<K, V> {
    Get(K),
    Set((K, V)),
    Remove(K),
}

impl<K, V> Command for Commands<K, V>
where
    K: Send + 'static,
    V: Send + 'static,
{
}

use std::collections::HashMap;
use std::hash::Hash;
use tokio::sync::Mutex;

impl<K, V> Store<K, V> for Mutex<HashMap<K, V>>
where
    K: Hash + Eq + Send + 'static,
    V: Clone + Send + 'static,
{
    type Command = Commands<K, V>;
    type Error = std::io::Error;

    async fn apply_command(&self, command: Self::Command) -> Result<Option<V>, Self::Error> {
        // It is the responsibility of the caller to implement the Store trait in such a way that
        // the Commands return a future that resolves to the Result<Option<V>, Self::Error>. This
        // is not a safety issue as not doing it will lead to the code getting rejected at compile
        // time!
        //
        // If the underlying structure is not I/O bound we can directly use it in the following
        // way!
        match command {
            Commands::Get(k) => {
                let guard = self.lock().await;
                // Call the appropriate method here
                if let Some(v) = guard.get(&k) {
                    Ok(Some(v.clone()))
                } else {
                    Ok(None)
                }
            }
            Commands::Set((k, v)) => {
                let mut guard = self.lock().await;
                // Call the appropriate method here
                guard.insert(k, v);
                Ok(None)
            }
            Commands::Remove(k) => {
                let mut guard = self.lock().await;
                // Call the appropriate method here
                if let Some(v) = guard.remove(&k) {
                    Ok(Some(v))
                } else {
                    Ok(None)
                }
            }
        }
    }
}
