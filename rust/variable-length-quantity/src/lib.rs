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
/// #### Décodage
///
/// ```no_test
/// résultat = 0
///
/// pour chaque octet du flux :
///     résultat = (résultat << 7) | (octet & 0x7F)
///
///     si (octet & 0x80) == 0 :   // bit 7 = 0
///         → terminé, retourner résultat
/// ```
///
/// Le `<< 7` fait de la place pour les 7 nouveaux bits, et le `| (octet & 0x7F)` les insère.
///
/// ---
///
/// ### Exemples
///
/// Encodage VLQ de 16383, étape par étape
///
/// ## 1. Convertir en binaire
///
/// 16383 = 2^14 - 1 = `11111111111111` (14 bits, que des 1)
///
/// ## 2. Découper en groupes de 7 bits (en partant de la droite)
///
/// ```no_test
/// 11111111111111 (14 bits)
///
/// Bits 0–6   : 1111111 = 127
/// Bits 7–13  : 1111111 = 127
/// ```
///
/// **2 groupes** suffisent (14 bits = 2×7 exactement).
///
/// ## 3. Ajouter le bit de continuation
///
/// ```no_test
/// Groupe 1 : 1111111 → bit 7 = 1 (continuation) → 1_1111111 = 0xFF
/// Groupe 0 : 1111111 → bit 7 = 0 (fin)          → 0_1111111 = 0x7F
/// ```
///
/// ## 4. Résultat
///
/// ```no_test
/// 0xFF  0x7F
/// ```
///
/// ## 5. Vérification (décodage)
///
/// ```no_test
/// 0xFF → bit 7 = 1 (continuation), valeur = 0xFF & 0x7F = 0x7F = 127
/// 0x7F → bit 7 = 0 (fin),          valeur = 0x7F & 0x7F = 0x7F = 127
///
/// Résultat = (127 << 7) | 127
///          = 127 × 128  + 127
///          = 16256      + 127
///          = 16383 ✓
/// ```
///
/// ## Résumé visuel
///
/// ```no_test
/// 16383 →  11111111111111 (binaire, 14 bits)
///       →  [1111111] [1111111]       (2 groupes de 7 bits)
///       →  [1|1111111] [0|1111111]   (ajout bit continuation)
///       →  0xFF  0x7F
/// ```

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

pub fn value_to_bytes(value: u32) -> Vec<u8> {
    let mut chunks = format!("{:b}", value)
        .as_bytes()
        .rchunks(7)
        .map(|chunk| {
            let s = std::str::from_utf8(chunk).unwrap();
            format!("{:0>7}", s) // padding de zéros à gauche
        })
        .collect::<Vec<_>>();

    for (i, chunk) in chunks.iter_mut().enumerate() {
        if i > 0 {
            chunk.insert(0, '1');
        }
    }

    chunks
        .iter()
        .rev()
        .map(|chunk| u8::from_str_radix(chunk, 2).unwrap())
        .collect()
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut result = Vec::new();
    for &value in values {
        result.extend(value_to_bytes(value));
    }
    result
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut result = Vec::new();
    let mut value = 0u32;
    let mut last = false;

    for &byte in bytes {
        value = (value << 7) | ((byte & 0x7F) as u32);

        if byte & 0x80 == 0 {
            result.push(value);
            value = 0;
            last = true;
        } else {
            last = false;
        }
    }

    if !last {
        return Err(Error::IncompleteNumber);
    }

    Ok(result)
}
