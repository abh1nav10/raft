#![allow(dead_code)]

use tokio::fs::File;

struct Node<S> {
    state: S,
    // Volatile state for all nodes!
    commit_index: u64,
    last_applied: u64,
    // Persistent state for all nodes!
    persistent: DiskState,
    // Keeping the metadata from disk in the RAM to avoid having to read from the disk when not
    // required to!
    disk_on_ram: DiskOnRam,
}

struct DiskOnRam {
    current_term: u64,
    voted_for: NodeId,
}

struct DiskState {
    log_file: File,
    meta_file: File,
}

struct Leader {
    next_index: u64,
    match_index: u64,
}

struct Follower;

struct Candidate;

impl Node<Leader> {
    // Methods for the leader
}

impl Node<Follower> {
    // Methods for the follower
}

impl Node<Candidate> {
    // Methods for the candidate
}

//impl From<Node<Candidate>> for Node<Leader> {
//    fn from(_value: Node<Candidate>) -> Self {
//        Node { state: Leader {} }
//    }
//}
//
//impl From<Node<Leader>> for Node<Follower> {
//    fn from(_value: Node<Leader>) -> Self {
//        Node { state: Follower }
//    }
//}
//
//impl From<Node<Candidate>> for Node<Follower> {
//    fn from(_value: Node<Candidate>) -> Self {
//        Node { state: Follower }
//    }
//}
//
//impl From<Node<Follower>> for Node<Candidate> {
//    fn from(_value: Node<Follower>) -> Self {
//        Node { state: Candidate }
//    }
//}
