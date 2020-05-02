pub mod core {
    extern crate curve25519_dalek;
    extern crate elgamal_ristretto;

    use curve25519_dalek::scalar::Scalar;
    use elgamal_ristretto::ciphertext::Ciphertext;
    use errors::errors::StorageError;

    /// A representation of the sinkhole database to query.
    ///
    /// Implementations of this trait contain the
    /// servers private data addition and retrieval.
    pub trait Storage {
        /// Adds content to the database.
        fn add(&self, content: Scalar, index: usize) -> Result<(), StorageError>;

        /// Returns the data for a given ID.
        fn retrieve(&self, query: Vec<Ciphertext>) -> Result<Ciphertext, StorageError>;
    }
}