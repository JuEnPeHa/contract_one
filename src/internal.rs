use crate::*;

//Evitamos colisión de la data generando un prefijo para cada colección de storage
//Avoiding data collisition generate a prefix for the storage collections
pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}