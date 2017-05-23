use std::collections::HashMap;
use solution::PString;

// Alias para el tipo de dato que permite convertir una String en una PString
pub type Key = Vec<HashMap<char, usize>>;
// Alias para el tipo de dato que permite convertir una PString en una String;
pub type ReverseKey = Vec<Vec<char>>;

pub fn normalize(strings: Vec<String>) -> (Vec<PString>, ReverseKey) {
    let len = strings[0].len();
    let mut occ = Vec::with_capacity(len);          // Vector de vectores de ocurrencias de caracteres por columna. En otras palabras,
                                                // occ[i] es un vector con la cuenta de
                                                // cada caracter ocurrido en la columna i
                                                // de la matriz representada por `strings`
    for i in 0..len {
        occ.push(HashMap::new());
    }

    for string in &strings {
        let str_vec: Vec<char> = string.chars().collect();
        for i in 0..(str_vec.len()) {
            *occ[i].entry(str_vec[i]).or_insert(0) += 1;
        }
    }
    println!("{:#?}", occ);

    let mut char_to_index = Vec::with_capacity(len);
    let mut index_to_char = Vec::with_capacity(len);

    for hm in &occ {
        let mut vec: Vec<(&char, &u32)> = hm.iter().collect();
        vec.sort_by(|a,b| b.1.cmp(a.1));
        let mut i_to_c = vec!['0'; vec.len()];
        let mut c_to_i = HashMap::new();
        
        for i in 0..vec.len() {
            i_to_c[i] = *vec[i].0;
        }

        for i in 0..i_to_c.len() {
            c_to_i.insert(i_to_c[i], i);
        }

        println!("{:?}", i_to_c);
        println!("{:?}", c_to_i);
        
        index_to_char.push(i_to_c);
        char_to_index.push(c_to_i);
    }

    let mut normalized = Vec::with_capacity(strings.len());
    
    for string in strings {
        normalized.push(encode(&string, &char_to_index));
    }
    
    println!("{:#?}", char_to_index);
    (normalized, index_to_char)
}

fn encode(string: &String, key: &Key) -> PString {
    let vec_str: Vec<char> = string.chars().collect();
    let mut vec_pst = vec![0; vec_str.len()];

    for i in 0..vec_str.len() {
        vec_pst[i] = key[i][&vec_str[i]];    
    }

    PString(vec![])
}
