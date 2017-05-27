extern crate genetic_algorithms;
extern crate rand;

use rand::{Rng, XorShiftRng, SeedableRng};
use genetic_algorithms::solution::{PString, Solution};
use genetic_algorithms::population::Population;
use genetic_algorithms::normalizer::{normalize, decode};

fn main() {
    let seed = 1;
    //let strings = generate_rangom_strings(100, 10);
    let strings = vec![
"CCCBCABAB".to_string(),
"ACAACBBBC".to_string(),
"BBBCABAAB".to_string(),
"CCCAABCBA".to_string(),
"ABBBAABBB".to_string(),
"CBBABBACB".to_string(),
"AABAAAABC".to_string(),
"BBCBAACAC".to_string(),
"AABACCCBA".to_string(),
"AAABBCACB".to_string(),
"CBBCBAACB".to_string(),
"BACBAABCC".to_string(),
"BCACCBABB".to_string(),
"AABCBBCBC".to_string(),
"CCCABABBC".to_string(),
"BBBCABBBB".to_string(),
"CACAABCCB".to_string(),
"AABBCCABB".to_string(),
"CBBACBBAC".to_string(),
"ABBBBBBCC".to_string(),
"CCBBABACB".to_string(),
"ACAABAAAB".to_string(),
"CCCABBCBB".to_string(),
"ABBAAACAB".to_string(),
"BCABCACCB".to_string(),
"ACBBABABC".to_string(),
"BAABBBAAC".to_string(),
"CCBCCBACC".to_string(),
"ACBCCAAAC".to_string(),
"BAAABCCCC".to_string(),
"BBBBCCCAA".to_string(),
"BBAAACCAB".to_string(),
"BAABBBAAB".to_string(),
"ABCBBCCBA".to_string(),
"ABCAAACBA".to_string(),
"ABBCACABB".to_string(),
"BAAABBCCC".to_string(),
"CCABABCCA".to_string(),
"CCABACAAB".to_string(),
"BABABBBAC".to_string(),
"CBABABACB".to_string(),
"BCBACCABC".to_string(),
"BCBCBBBBA".to_string(),
"BCABCCBCC".to_string(),
"CABACBCCA".to_string(),
"CCBCACAAC".to_string(),
"CBACCABCB".to_string(),
"CACABABBA".to_string(),
"ABCACCBBC".to_string(),
"CABABAABC".to_string(),
"CABACCBCB".to_string(),
"AACBBBBBC".to_string(),
"BCBACBABC".to_string(),
"CABACCCAB".to_string(),
"BACCBAABB".to_string(),
"ABBACBCBC".to_string(),
"CCAACBCAC".to_string(),
"BBABBABAA".to_string(),
"CBCAACCBA".to_string(),
"ACCCBABCB".to_string(),
"CBCAACBCA".to_string(),
"ABCCAAABA".to_string(),
"CCBACAAAA".to_string(),
"CCACCABAB".to_string(),
"AABBCBACB".to_string(),
"CBCCBABBA".to_string(),
"ACCCBCACC".to_string(),
"AABCCCBCA".to_string(),
"ABABACCCB".to_string(),
"ABBACBBCC".to_string(),
"AACCBCAAC".to_string(),
"BACCACBBC".to_string(),
"ACBCAAABA".to_string(),
"AACCBABCC".to_string(),
"CBCCBCBBC".to_string(),
"AACACCBCC".to_string(),
"CBABAAACB".to_string(),
"BABABCCCB".to_string(),
"CAABBBCCB".to_string(),
"ABCABBCAC".to_string(),
"CAACBCBAB".to_string(),
"CCACABAAA".to_string(),
"ABCBABBCA".to_string(),
"CABBBABAB".to_string(),
"CCBCCCCAA".to_string(),
"CBAAABBBA".to_string(),
"BCCCBABAA".to_string(),
"AABCACAAA".to_string(),
"BCACBCCBA".to_string(),
"BBCBBCAAA".to_string(),
"ABACBCCCB".to_string(),
"BCACAABCA".to_string(),
"AACCCCCAB".to_string(),
"BBBBBBABC".to_string(),
"BBCBBBBCC".to_string(),
"ACABAAACA".to_string(),
"AAAACBBAC".to_string(),
"ABBCAAAAA".to_string(),
"BCBBACBCB".to_string(),
"CBAACBACA".to_string(),
"CACABCBAC".to_string(),
"ABBBBCBAA".to_string(),
"BBBBCCAAC".to_string(),
"CBCCCABAA".to_string(),
"ACCABAABC".to_string(),
"ACBAABBAB".to_string(),
"CCACBABBB".to_string(),
"ABACBBCCB".to_string(),
"CBCCACBBA".to_string(),
"AABABCCBB".to_string(),
"ACCABBBCA".to_string(),
"BCABCACBC".to_string(),
"CBAACCCAB".to_string(),
"AACCACBAC".to_string(),
"BBBCCBACA".to_string(),
"AAAACCCBB".to_string(),
"CAABABBAA".to_string(),
"CBABBAACA".to_string(),
"CCCBCCAAC".to_string(),
"CBBBBACCC".to_string(),
"CBBABABAA".to_string(),
"BCAABCCCA".to_string(),
"BCBACCCBA".to_string(),
"AABBBCBCB".to_string(),
"ACAACBCBA".to_string(),
"CAACAAAAA".to_string(),
"BBBCBBBAC".to_string(),
"BBCCABACB".to_string(),
"CCBCABACC".to_string(),
"CBBBBBBAA".to_string(),
"ACCCABBAC".to_string(),
"BBAACAACC".to_string(),
"CCABACBCA".to_string(),
"AAABABABA".to_string(),
"CCBBBACBC".to_string(),
"CBCCABCBB".to_string(),
"AAACABAAA".to_string(),
"ABABCCABA".to_string(),
"BAACCACAA".to_string(),
"ABACBBABA".to_string(),
"ACACACBCA".to_string(),
"BCCCBCBAA".to_string(),
"CCBABBCBB".to_string(),
"CCCAACACA".to_string(),
"ABCAABABB".to_string(),
"CBBAAAAAB".to_string(),
"BAACBABCC".to_string(),
"BBCAABCCA".to_string(),
"ABAAAAABC".to_string(),
"CABCCCAAB".to_string(),
"AACAAACBB".to_string(),
"CCACAACCC".to_string(),
"AACACCBBA".to_string(),
"AABABBCAC".to_string(),
"BCBCCCAAC".to_string(),
"ACBBCACBB".to_string(),
"CBCCCCCAA".to_string(),
"ABCACACAB".to_string(),
"CBBBABBBC".to_string(),
"BBACCCBCA".to_string(),
"CBCCBACCA".to_string(),
"ABCAAABAB".to_string(),
"CBBCBCCAB".to_string(),
"AAABBCAAB".to_string(),
"CACBCCCCB".to_string(),
"BCBABBCCB".to_string(),
"CBABABBBA".to_string(),
"ABBCBBBBB".to_string(),
"BCACBCBCC".to_string(),
"CAABCBACA".to_string(),
"ACBACCBBB".to_string(),
"BCBBBBCCA".to_string(),
"BABCCABAA".to_string(),
"BBCCABACC".to_string(),
"ACBACBACA".to_string(),
"AACBBBBAC".to_string(),
"BCBCCBBCB".to_string(),
"BCBCACCCA".to_string(),
"BBBBACBAA".to_string(),
"CCAACACBC".to_string(),
"AABABCACC".to_string(),
"ACBACCCAB".to_string(),
"ACBBBBCCC".to_string(),
"CAAABAACA".to_string(),
"CBBAABABA".to_string(),
"BCACACACB".to_string(),
"ACCCCACBB".to_string(),
"AAACCAAAC".to_string(),
"CACBCCCCC".to_string(),
"AAACCCABB".to_string(),
"BABACBCAC".to_string(),
"BBBCBBBAA".to_string(),
"CACBCCCAA".to_string(),
"CAABCAACB".to_string(),
"ACCABBBAB".to_string(),
"AABCCBBAB".to_string(),
"ACACBACCC".to_string(),
"CBBCACBBC".to_string(),
"CABAAACCB".to_string(),
"CCCCACBAC".to_string(),
"CAABCCABC".to_string(),
"AAACAACCA".to_string(),
"CCBCCCCCB".to_string(),
"BACACCCBB".to_string(),
"ABBAACCAA".to_string(),
"BABBCAAAA".to_string(),
"BBAAAABBA".to_string(),
"ACCACCBCB".to_string(),
"ABBBCACAC".to_string(),
"CBCBAABAB".to_string(),
"AACABACAC".to_string(),
"ABBABBCBB".to_string(),
"BCACCCABA".to_string(),
"AABCCACAB".to_string(),
"BABCACCAA".to_string(),
"BCABACACC".to_string(),
"ACBCCCBCB".to_string(),
"ACCAACCCA".to_string(),
"BABABAAAA".to_string(),
"BBACBBCBC".to_string(),
"AACACCABC".to_string(),
"BABCAABAC".to_string(),
"ACCCCBACB".to_string(),
"CAACCCAAC".to_string(),
"ABABBABAC".to_string(),
"ABCCCABCB".to_string(),
"BABCAACCA".to_string(),
"BAAACBCAB".to_string(),
"CABBBBACA".to_string(),
"ACCBABBBB".to_string(),
"ACCBCCAAC".to_string(),
"ACACBBCBB".to_string(),
"BBABBABCC".to_string(),
"ABCBBCACB".to_string(),
"CBAABAACB".to_string(),
"CCCACACAA".to_string(),
"BCBBCBCAA".to_string(),
"AABCABAAB".to_string(),
"CBCBCAACA".to_string(),
"BCBBABBAA".to_string(),
"CABABBCCB".to_string(),
"CABCCCBBC".to_string(),
"AAACCACAC".to_string(),
"CAABCBAAB".to_string(),
"BABBCCCCB".to_string(),
"AACCBCCBC".to_string(),
"CBACCCABC".to_string(),
"ACCBCBBAC".to_string(),
"CCABBAACB".to_string(),
"ABBAABBCA".to_string(),
"BAACAAACC".to_string(),
"BCCCAABAA".to_string(),
"CBBABABBB".to_string(),
"AABBCAACA".to_string(),
"BCCBABCBB".to_string(),
"BABCCCACC".to_string(),
"ACABBBBCB".to_string(),
"CCBABBABA".to_string(),
"AACCBCABB".to_string(),
"ACBCBCBAA".to_string(),
"BCBBABCBC".to_string(),
"CACCAABCA".to_string(),
"CBBCCCCAA".to_string(),
"BBBCCCACA".to_string(),
"BBBCACACB".to_string(),
"BCCCCBBAC".to_string(),
"ABBCCBBCB".to_string(),
"BACBCACAA".to_string(),
"CACCCACCC".to_string(),
"CBAACAAAA".to_string(),
"BAABCACAC".to_string(),
"CBAACCCAC".to_string(),
"CCAACBCCA".to_string(),
"BAABAAAAC".to_string(),
"ACCAACABA".to_string(),
"CCABBBACA".to_string(),
"AACACCBAA".to_string(),
"BCCACCCBA".to_string(),
"ACCBABCCC".to_string(),
"ABCABABCC".to_string(),
"CBAACBBCA".to_string(),
"BABCABACB".to_string(),
"ABCABABAC".to_string(),
"AAABBAACB".to_string(),
"CCAAACAAB".to_string(),
"AABAABCBB".to_string(),
"CACCBCBBA".to_string(),
"CBAABBBAB".to_string(),
"BACCCCBAA".to_string(),
"CCCBABAAA".to_string(),
"BABBBCABA".to_string(),
"ABBAACAAC".to_string(),
"BCAAAACBB".to_string(),
"CACCABCBA".to_string(),
"ACBCACAAB".to_string(),
"BAAAAABAB".to_string(),
"ACACCBBBC".to_string(),
"CCACABCBA".to_string(),
"CAACCBBBB".to_string(),
"CBACABACA".to_string(),
"CBACBABCC".to_string(),
"AAAABCBCC".to_string(),
"CAACBACCC".to_string(),
"BCABABCCC".to_string(),
"CBAABCBAA".to_string(),
"AAABACACC".to_string(),
"BBBAAACBA".to_string(),
"CAAACAACC".to_string(),
"CBBBCAAAA".to_string(),
"CCBBCAAAA".to_string(),
"ABCBBACBC".to_string(),
"CABCBBCCA".to_string(),
"BACABABBA".to_string(),
"CCAAACCBC".to_string(),
"ACBCABBAB".to_string(),
"CBCAABAAA".to_string(),
"AAACBABBA".to_string(),
"CCCABBAAA".to_string(),
"BCABAABCB".to_string(),
"CACCABAAB".to_string(),
"CBBCBBCAB".to_string(),
"ACBBAAACA".to_string(),
"CCABACBAB".to_string(),
"ACBBBBBBC".to_string(),
"BAAAACCAA".to_string(),
"BBACCBCAB".to_string(),
"ACBBABCBA".to_string(),
"BCBAACBAB".to_string(),
"CBBCCBCCC".to_string(),
"BCBCACBBC".to_string(),
"AAAACCAAA".to_string(),
"AACCCBCAA".to_string(),
"ABBAAAACA".to_string(),
"BCCCACACA".to_string(),
"BCBBBBBCC".to_string(),
"CBAAACABA".to_string(),
"AACAACBAA".to_string(),
"BBBBAABAC".to_string(),
"CCCBBCCBB".to_string(),
"ACBABCCBA".to_string(),
"BABABCABC".to_string(),
"ACABCAABA".to_string(),
"CBCACBABB".to_string(),
"CBBABCCAB".to_string(),
"CCBCCBAAA".to_string(),
"ABCAAAABC".to_string(),
"BBBBBCACC".to_string(),
"CAAABABAB".to_string(),
"ABACAACBC".to_string(),
"CBBBABCBA".to_string(),
"BACBABCCC".to_string(),
"BCCAACCBC".to_string(),
"CACCBBBAA".to_string(),
"CBBAACBBB".to_string(),
"CBABAABAA".to_string(),
"BCACCAACC".to_string(),
"BCBCCCCBC".to_string(),
"AAABAACBB".to_string(),
"BBCACAABC".to_string(),
"ABCBBAAAB".to_string(),
"AAABABBBA".to_string(),
"ABABCAABC".to_string(),
"BAAACCACA".to_string(),
"CACABBCCA".to_string(),
"CCBBCABBC".to_string(),
"ABAAABCAB".to_string(),
"CCACCCBCC".to_string(),
"AAACBACAC".to_string(),
"BCABBBBBA".to_string(),
"CAAABABAA".to_string(),
"BABCAABBC".to_string(),
"CAABBBCCA".to_string(),
"ACAABBBAB".to_string(),
"BCBCCCBAB".to_string(),
"CBBCCBCCA".to_string(),
"ACCBAABCC".to_string(),
"CCCCBCCBA".to_string(),
"ABCBCCBAC".to_string(),
"CCACBACAC".to_string(),
"AABBABBCA".to_string(),
"CCBABBABC".to_string(),
"BCCABCBCC".to_string(),
"ABCBABBCC".to_string(),
"BCCABACAC".to_string(),
"BABABABCA".to_string(),
"BAAAABAAC".to_string(),
"ACCCBCBAC".to_string(),
"ACACBACAC".to_string(),
"BABBCBCCC".to_string(),
"CAABCBABA".to_string(),
"ACAAABAAA".to_string(),
"BCACBCCBB".to_string(),
"CAABCBBAB".to_string(),
"ABBBACCBC".to_string(),
"CACBBCCAC".to_string(),
"CACCACBCB".to_string(),
"ACBBCAACC".to_string(),
"CBCBBCBCC".to_string(),
"AACACBCCA".to_string(),
"ACCACBBAB".to_string(),
"BBBAAABAB".to_string(),
"BBBACABCA".to_string(),
"ACBCACBCA".to_string(),
"BBAABCABA".to_string(),
"ABBCAABAA".to_string(),
"CCABABCCC".to_string(),
"CBAACABBB".to_string(),
"BABACABCC".to_string(),
"CBCACCACA".to_string(),
"ABCBBAABC".to_string(),
"BCBCCAACC".to_string(),
"CAAACCAAC".to_string(),
"BACBCBBAC".to_string(),
"CACCBCABB".to_string(),
"CACBBBBAB".to_string(),
"AAAABACAA".to_string(),
"AABBBCCCB".to_string(),
"BBBBCCCCA".to_string(),
"ACBBCAACB".to_string(),
"BACBCBAAA".to_string(),
"AABBBCABA".to_string(),
"ACBBACCCC".to_string(),
"CCACBAABB".to_string(),
"CCCBBCACB".to_string(),
"BCCBBCBBC".to_string(),
"CCBCAACAA".to_string(),
"CACBBCAAB".to_string(),
"BCCBACCBA".to_string(),
"CBABCACAB".to_string(),
"BABBCCCAA".to_string(),
"AAACCCCCA".to_string(),
"ABBBABACA".to_string(),
"CCBACACAB".to_string(),
"CCBCACBAC".to_string(),
"ACAACAACC".to_string(),
"ACCAAABCA".to_string(),
"BCCACCBBB".to_string(),
"AACCBCBCA".to_string(),
"BCBAACCCB".to_string(),
"CCABCCCBC".to_string(),
"BBCBACCCA".to_string(),
"AAACCCACC".to_string(),
"BBABBABCB".to_string(),
"AAABCBBCA".to_string(),
"BBABABAAA".to_string(),
"ABBCBABBC".to_string(),
"BCACABAAA".to_string(),
"AABAABABB".to_string(),
"ABBCCACCA".to_string(),
"BCBACACCA".to_string(),
"CBABAABAC".to_string(),
"BAABCBCBB".to_string(),
"BAACCCABC".to_string(),
"AAAABAABA".to_string(),
"ABBABCBBA".to_string(),
"CBCBACCBA".to_string(),
"CCCCBBBCA".to_string(),
"BACBABCCA".to_string(),
"BCACCACAA".to_string(),
"ACCCAABBC".to_string(),
"BABACBAAB".to_string(),
"AACBCCABB".to_string(),
"CCCACCCCA".to_string(),
"ABABAABBC".to_string(),
"AAABCBBCC".to_string(),
"AACABAABA".to_string(),
"CBCBAAACB".to_string(),
"BAABBBBCB".to_string(),
"BBCCBAABB".to_string(),
"BCACABBCB".to_string(),
"CBBABCCBB".to_string(),
"BCAACBCAB".to_string(),
"BABACCBCB".to_string(),
"BBBBCCCCB".to_string(),
"CCBBCABCC".to_string(),
"ABACBCBAA".to_string(),
"CAABAACBC".to_string(),
"CAABBACAB".to_string(),
"BABBCACCA".to_string(),
"BBAAAABAB".to_string(),
"AAACCCCAC".to_string(),
"BAAACBBAC".to_string(),
"CCCCCBBBB".to_string(),
"ABCBABCAB".to_string(),
"BBAACBAAB".to_string(),
"AABAAAABA".to_string(),
"CACACBBAA".to_string(),
"ABCACBAAA".to_string(),
"CACBACCAC".to_string(),
"CCBBBBACA".to_string(),
"ABCACCCAA".to_string(),
"AAAABABBC".to_string(),
"BCABAAAAA".to_string(),
"CBCCBCAAA".to_string(),
"CCCACBCCA".to_string(),
"AACABABBA".to_string(),
"AAAABBBBC".to_string(),
"CCACABBCB".to_string(),
"ACBCABACB".to_string(),
"BBCBACCCB".to_string(),
"BCBAABACC".to_string(),
"CCBAABBCB".to_string(),
"ACBBBAABA".to_string(),
"CBBAAACAA".to_string(),
"BAACBBCCC".to_string(),
"CCCCAABAA".to_string(),
"BCCBCAACB".to_string(),
"AAACBCCBC".to_string(),
"BABCBBABB".to_string(),
"BCCCCCCAC".to_string(),
"AACBBAAAB".to_string(),
"ABCCBBCBB".to_string()
    ];
    //let strings = vec![
    //    "ACROSS".to_string(),
    //    "ALMOST".to_string(),
    //    "AMOUNT".to_string(),
    //    "ANIMAL".to_string(),
    //    "ANSWER".to_string(),
    //    "ATTACK".to_string(),
    //    "BASKET".to_string(),
    //    "BEFORE".to_string(),
    //    "BELIEF".to_string(),
    //    "BITTER".to_string(),
    //    "BOTTLE".to_string(),
    //    "BRANCH".to_string(),
    //    "BREATH".to_string(),
    //    "BRIDGE".to_string(),
    //    "BRIGHT".to_string(),
    //    "BROKEN".to_string(),
    //    "BUCKET".to_string(),
    //    "BUTTER".to_string(),
    //    "BUTTON".to_string(),
    //    "CAMERA".to_string(),
    //    "CANVAS".to_string(),
    //    "CHANCE".to_string(),
    //    "CHANGE".to_string(),
    //    "CHEESE".to_string(),
    //    "CHURCH".to_string(),
    //    "CIRCLE".to_string(),
    //    "COLLAR".to_string(),
    //    "COLOUR".to_string(),
    //    "COMMON".to_string(),
    //    "COPPER".to_string(),
    //    "COTTON".to_string(),
    //    "CREDIT".to_string(),
    //    "DAMAGE".to_string(),
    //    "DANGER".to_string(),
    //    "DEGREE".to_string(),
    //    "DESIGN".to_string(),
    //    "DESIRE".to_string(),
    //    "DETAIL".to_string(),
    //    "DRAWER".to_string(),
    //    "EFFECT".to_string(),
    //    "ENGINE".to_string(),
    //    "ENOUGH".to_string(),
    //    "EXPERT".to_string(),
    //    "FAMILY".to_string(),
    //    "FATHER".to_string(),
    //    "FEEBLE".to_string(),
    //    "FEMALE".to_string(),
    //    "FINGER".to_string(),
    //    "FLIGHT".to_string(),
    //    "FLOWER".to_string(),
    //    "FRIEND".to_string(),
    //    "FUTURE".to_string(),
    //    "GARDEN".to_string(),
    //    "GROWTH".to_string(),
    //    "HAMMER".to_string(),
    //    "HOLLOW".to_string(),
    //    "HUMOUR".to_string(),
    //    "INSECT".to_string(),
    //    "ISLAND".to_string(),
    //    "KETTLE".to_string(),
    //    "LETTER".to_string(),
    //    "LIQUID".to_string(),
    //    "LITTLE".to_string(),
    //    "LIVING".to_string(),
    //    "MARKET".to_string(),
    //    "MEMORY".to_string(),
    //    "MIDDLE".to_string(),
    //    "MINUTE".to_string(),
    //    "MONKEY".to_string(),
    //    "MOTHER".to_string(),
    //    "MOTION".to_string(),
    //    "MUSCLE".to_string(),
    //    "NARROW".to_string(),
    //    "NATION".to_string(),
    //    "NEEDLE".to_string(),
    //    "NORMAL".to_string(),
    //    "NUMBER".to_string(),
    //    "OFFICE".to_string(),
    //    "ORANGE".to_string(),
    //    "PARCEL".to_string(),
    //    "PENCIL".to_string(),
    //    "PERSON".to_string(),
    //    "PLEASE".to_string(),
    //    "PLOUGH".to_string(),
    //    "POCKET".to_string(),
    //    "POISON".to_string(),
    //    "POLISH".to_string(),
    //    "PORTER".to_string(),
    //    "POTATO".to_string(),
    //    "POWDER".to_string(),
    //    "PRISON".to_string(),
    //    "PROFIT".to_string(),
    //    "PUBLIC".to_string(),
    //    "REASON".to_string(),
    //    "RECORD".to_string(),
    //    "REGRET".to_string(),
    //    "REWARD".to_string(),
    //    "RHYTHM".to_string(),
    //    "SCHOOL".to_string(),
    //    "SECOND".to_string(),
    //    "SECRET".to_string(),
    //    "SILVER".to_string(),
    //    "SIMPLE".to_string(),
    //    "SISTER".to_string(),
    //    "SMOOTH".to_string(),
    //    "SNEEZE".to_string(),
    //    "SPONGE".to_string(),
    //    "SPRING".to_string(),
    //    "SQUARE".to_string(),
    //    "STICKY".to_string(),
    //    "STITCH".to_string(),
    //    "STREET".to_string(),
    //    "STRONG".to_string(),
    //    "SUDDEN".to_string(),
    //    "SUMMER".to_string(),
    //    "SYSTEM".to_string(),
    //    "THEORY".to_string(),
    //    "THREAD".to_string(),
    //    "THROAT".to_string(),
    //    "TICKET".to_string(),
    //    "TONGUE".to_string(),
    //    "VESSEL".to_string(),
    //    "WEIGHT".to_string(),
    //    "WINDOW".to_string(),
    //    "WINTER".to_string(),
    //    "YELLOW".to_string(),
    //];
    let (normalized, key) = normalize(strings.clone());
    //let mut pop = Population::new_from_init(500, &normalized, key.clone(), [seed, seed*3, seed*7, seed*11]);
    let mut pop = Population::new_random(500, &normalized, key.clone(), [seed, seed*3, seed*7, seed*11]);

    //Lord forgive me but it's time to go back to tha old me
    let all_nines = generate_every_single_motherfucking_abc_string(&normalized);
    for sol in all_nines {
        println!("{:?}", sol);
    }


    //for i in 0..2000 {
    //    println!("{}", i);
    //    println!("{:#?}", pop.population);
    //    println!("avg_fitness: {:#?}", pop.avg_fitness());
    //    println!("best_solution: {:#?}", pop.best_solution);
    //    pop.next_generation();
    //}
    let string = decode(&pop.best_solution.pstr, &key);
    //println!("best solution: {:?}", pop.best_solution);
    //println!("{}", string);
    //println!("{:#?}", strings);
    //println!("{}", minimum_pairwise_distance(&normalized));
    //println!("{:#?}", pop.key);
}

fn minimum_pairwise_distance(pstrs: &Vec<PString>) -> u32 {
    let mut min = pstrs[0].distance(&pstrs[1]);
    for i in 0..pstrs.len(){
        for j in (i+1)..pstrs.len() {
            let dist = pstrs[i].distance(&pstrs[j]);
            min = if dist < min {
                dist
            } else {
                min
            };
        }
    }
    min.unwrap()
}


fn generate_every_single_motherfucking_abc_string(strs: &Vec<PString>) -> Vec<Solution> {
    let mut vec = Vec::new();
    for a in 0..3 {
    for b in 0..3 {
    for c in 0..3 {
    for d in 0..3 {
    for e in 0..3 {
    for f in 0..3 {
    for g in 0..3 {
    for h in 0..3 {
    for i in 0..3 {
        vec.push(Solution::new(PString(vec![a,b,c,d,e,f,g,h,i]), strs));
    }
    }
    }
    }
    }
    }
    }
    }
    }
    vec
}

fn generate_rangom_strings(n: usize, L: usize) -> Vec<String> {
    let mut rng: XorShiftRng = SeedableRng::from_seed([4,3,2,1]);
    let mut vec: Vec<String> = Vec::with_capacity(n);
    
    for _ in 0..n {
        let mut s_vec = Vec::with_capacity(L);
        for _ in 0..L {
            s_vec.push(rng.gen_range(b'A', b'E') as char);
        }
        vec.push(s_vec.into_iter().collect());
    }
    vec
}


