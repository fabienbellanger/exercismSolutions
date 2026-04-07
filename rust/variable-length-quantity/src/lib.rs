/// ## Encodage VLQ (Variable Length Quantity)
///
/// L’idée est de **stocker un entier avec le minimum d’octets** : les petits nombres tiennent sur 1 octet, les grands en utilisent plusieurs.
///
/// ### Règles
///
/// 1. **7 bits utiles par octet** (les bits 0 à 6), alignés comme des “chiffres” en base 128.
/// 2. **Bit 7 (bit de continuation)**  
/// - **1** : ce n’est pas le dernier octet, il en suit d’autres.  
/// - **0** : c’est le **dernier** octet de ce nombre.
/// 3. **Ordre dans le flux** : on envoie d’abord les morceaux **les plus significatifs**, puis les moins significatifs (comme en “gros-boutiste” pour les groupes de 7 bits).
///
/// Pour **reconstruire** la valeur : on part du premier octet et, tant que le bit 7 vaut 1, on enchaîne avec l’octet suivant. À chaque fois on fait en gros :  
/// `valeur = valeur * 128 + (octet & 0x7F)`.
///
/// ---
///
/// ### Exemples
///
/// **0**  
/// - Un seul groupe : `00000000` → octet **`0x00`**.
///
/// **127**  
/// - Tient sur 7 bits, dernier octet → bit 7 à 0 : **`0x7F`**.
///
/// **128**  
/// - En binaire utile : `10000000` (8 bits) → deux groupes de 7 bits : `1` puis `0000000`.  
/// - Premier octet (suite) : `1 | 0x80` = **`0x81`**  
/// - Dernier octet : **`0x00`**  
/// - Flux : **`81 00`** → \(1 \times 128 + 0 = 128\).
///
/// **255**  
/// - \(255 = 1 \times 128 + 127\) → **`81 7F`**.
///
/// **16384** ( \(= 128^2\) )  
/// - Trois octets : **`81 80 00`** (même logique : chunks de 7 bits avec continuation).
///
/// **Petit nombre en un octet**  
/// - **42** → **`0x2A`** (un seul octet, bit 7 à 0).

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    todo!("Convert the values {values:?} to a list of bytes")
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    todo!("Convert the list of bytes {bytes:?} to a list of numbers")
}
