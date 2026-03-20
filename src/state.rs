#![allow(dead_code)]

struct Node<S> {
    state: S,
    // more state for the node will stay here!
}

struct Leader {
    // carries leader metadata,
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

impl From<Node<Candidate>> for Node<Leader> {
    fn from(_value: Node<Candidate>) -> Self {
        Node { state: Leader {} }
    }
}

impl From<Node<Leader>> for Node<Follower> {
    fn from(_value: Node<Leader>) -> Self {
        Node { state: Follower }
    }
}

impl From<Node<Candidate>> for Node<Follower> {
    fn from(_value: Node<Candidate>) -> Self {
        Node { state: Follower }
    }
}

impl From<Node<Follower>> for Node<Candidate> {
    fn from(_value: Node<Follower>) -> Self {
        Node { state: Candidate }
    }
}
