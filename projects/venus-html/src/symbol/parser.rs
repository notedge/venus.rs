use super::*;
use pex::{ParseResult, ParseState, StopBecause};
use ucd_trie::TrieSetSlice;
use crate::RulerResult;




impl FromStr for Symbol {
    type Err = RulerError;

    fn from_str(s: &str) -> RulerResult<Self> {
        match Symbol::parse(ParseState::new(s)) {
            ParseResult::Pending(state, compound) if state.is_empty() => Ok(compound),
            ParseResult::Pending(state, ..) => Err(RulerError::syntax_error("Except end of symbol", state.start_offset, state.start_offset + 1)),
            ParseResult::Stop(e) => Err(RulerError::syntax_error(e, 0, 1)),
        }
    }
}

impl Symbol {
    /// Parse new symbol from parse state
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        // no string, avoid accidental memory expansion allocations
        let mut chars = input.rest_text.chars();
        let mut offset = 0;
        match chars.next() {
            Some(c) if XID_START.contains_char(c) => {
                offset += c.len_utf8();
            }
            _ => {
                StopBecause::must_be("XID_START", input.start_offset)?
            }
        }
        for ch in chars {
            if XID_CONTINUE.contains_char(ch) {
                offset += ch.len_utf8();
            } else {
                break;
            }
        }
        let (state, str) = input.advance_view(offset)?;
        state.finish(Symbol::new_unchecked(str))
    }
}


#[rustfmt::skip]
const XID_START: TrieSetSlice<'static> = TrieSetSlice {
    tree1_level1: &[
        0, 576460743847706622, 297241973452963840, 18410715276682199039, 18446744073709551615, 18446744073709551615, 18446744073709551615, 18446744073709551615,
        18446744073709551615, 18446744073709551615, 18446744073709551615, 88094074470339, 0, 13321366222785216512, 18446744056529672000, 18428729675200069631,
        18446744073709551615, 18446744073709551615, 18446744073709550595, 18446744073709551615, 18446462598732840959, 18446744069456527359, 511,
        2119858418286592, 18446744069414584320, 18446392229988665343, 18446744073709551615, 11241196188469297151, 281474976514048, 18446744073709543424,
        563224831328255, 301749971126844416,
    ],
    tree2_level1: &[
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 23, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
        35, 35, 35, 35, 36, 37, 38, 39, 40, 41, 42, 43, 35, 35, 35, 35, 35, 35, 35, 35, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 3,
        58, 59, 60, 30, 61, 62, 63, 64, 65, 66, 67, 68, 35, 35, 35, 30, 35, 35, 35, 35, 69, 70, 71, 72, 30, 73, 74, 30, 75, 76, 77, 30, 30, 30, 30,
        30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
        30, 30, 35, 35, 35, 78, 79, 80, 81, 82, 30, 30, 30, 30, 30, 30, 30, 30, 83, 43, 84, 85, 86, 35, 87, 88, 30, 30, 30, 30, 30, 30, 30, 30, 35,
        35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35,
        35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35,
        35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 30, 35, 35,
        35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35,
        35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35,
        35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35,
        35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35,
        35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35,
        35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35,
        35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35,
        35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35,
        35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35,
        35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 89, 90, 35, 35, 35, 35,
        91, 92, 93, 94, 95, 35, 96, 97, 98, 49, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 35, 111, 35, 35, 35, 35, 35, 35, 35, 35,
        35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35,
        35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35,
        35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35,
        35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35,
        35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 112, 113, 30, 30, 30, 30, 30, 30,
        30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
        30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
        30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
        30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 35, 35, 35, 35, 35, 114, 35, 115, 116, 117, 118, 119,
        35, 120, 35, 35, 121, 122, 123, 124, 30, 125, 35, 126, 127, 128, 129, 130,
    ],
    tree2_level2: &[
        1168302407679, 18446471390564450303, 18446744069414616831, 1023, 2594073385365405680, 18446181140919287808, 2577745637692514273, 1153765945374687232,
        247132830528276448, 7881300924956672, 2589004636761079776, 144115200960823296, 2589004636760940512, 562965791113216, 288167810662516712,
        65536, 2594071186342010848, 13539213312, 2589567586714640353, 1688864355778560, 2882303761516978160, 18158513712597581824, 3457638613854978016,
        127, 1688849860263934, 2307531515476572118, 4026531935, 1, 35184372088575, 7936, 0, 9223380832947798015, 18438229877581611008, 18446744069414600707,
        17870283321406070975, 18446744073709551615, 18446744070446333439, 9168765891372858879, 18446744073701162813, 18446744073696837631, 134217727,
        18446744069414649855, 4557642822898941951, 18446744073709551614, 18446638520593285119, 18446744069548802046, 144053615424700415, 1125897759621119,
        527761286627327, 4503599627370495, 276824064, 18446744069414584320, 144115188075855871, 18446469195802607615, 18014398509481983, 2147483647,
        8796093022142464, 18446480190918885375, 18446744069422972927, 2097151, 549755813888, 4503599627370464, 8160, 18158724812380307448, 274877906943,
        68719476735, 4611686018360336384, 16717361816799216127, 319718190147960832, 18446744070475743231, 4611686017001275199, 6908521828386340863,
        2295745090394464220, 9223934986808197120, 536805376, 17582049991377026180, 18446744069414601696, 511, 3509778554814463, 18446498607738650623,
        141836999983103, 9187201948305063935, 2139062143, 2251241253188403424, 18446744068886102015, 17870283321406128127, 18446462598732840928,
        18446744069414617087, 18446462598732840960, 8191, 4611686018427322368, 13198434443263, 9223512774343131135, 18446744070488326143, 281474976710655,
        18446744060816261120, 18446744073709550079, 18445618173868443647, 34359736251, 4503599627370492, 7564921474075590656, 18446462873610746880,
        2305843004918726783, 2251799813685232, 8935422993945886720, 2199023255551, 14159317224157876215, 4495436853045886975, 7890092085477381, 18446602782178705022,
        18446466996645134335, 34359738367, 18446462667452317695, 1152921504606845055, 18446532967477018623, 67108863, 6881498030004502655, 18446744073709551579,
        1125899906842623, 18446744073709027328, 18446744006063816703, 4611686018427387903, 18446744073709486080, 18446744073709355007, 287948901175001343,
        12288634533233819648, 2305843009213693951, 576460743713488896, 18446743798965862398, 9223372033633550335, 486341884,
    ],
    tree3_level1: &[
        0, 1, 2, 3, 4, 5, 6, 7, 8, 5, 9, 10, 5, 11, 12, 5, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 13, 14, 15, 7, 16, 17, 7, 18, 19, 5, 5, 5, 5, 5, 5, 5, 5,
        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    ],
    tree3_level2: &[
        0, 1, 2, 3, 4, 5, 4, 4, 4, 4, 6, 7, 8, 9, 10, 11, 2, 2, 12, 13, 14, 15, 16, 4, 2, 2, 2, 2, 17, 18, 19, 4, 20, 21, 22, 23, 24, 4, 25, 4, 26,
        27, 28, 29, 30, 31, 32, 4, 2, 33, 34, 34, 35, 4, 4, 4, 4, 4, 36, 4, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 22, 52, 53,
        4, 4, 5, 54, 55, 56, 4, 4, 57, 58, 55, 59, 60, 4, 61, 62, 4, 4, 63, 4, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 4, 4, 4, 4, 74, 75, 76, 4,
        77, 78, 79, 4, 4, 4, 4, 80, 81, 4, 82, 4, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 83, 4, 2, 57, 2, 2, 2, 84, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 85, 86, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 55, 87, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 2, 2, 2, 2, 2, 2, 2, 2, 2, 62, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 2, 2, 2, 2, 2, 2, 2, 2, 73, 88, 89, 90, 55, 91, 76,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 2, 4, 4, 2, 92, 93, 94, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 95, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 96, 33, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 97, 2, 2, 2, 2, 98, 99, 2, 2, 2, 2, 2, 100, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 2, 101, 102, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 2, 103, 104, 105, 106, 107, 2, 2, 2, 2, 108, 109, 110, 111, 112, 113, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 114, 4, 4, 4, 115, 116, 4, 4, 117, 118, 4, 4, 4, 4, 90, 63, 4, 4, 4,
        4, 4, 4, 4, 119, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 120, 2, 2, 2, 121, 2, 122, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 123, 124,
        125, 4, 4, 4, 4, 4, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 126, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 127, 2, 2, 2, 10, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 128, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 129, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 2, 2, 2, 2, 2, 2, 2, 2, 130, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 131, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 55, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    ],
    tree3_level3: &[
        13258596753222922239, 1073692671, 18446744073709551615, 576460752303423487, 0, 9007199254740991, 18446744069951455231, 131071, 18446708893632430079,
        18014398509418495, 18446744070488326143, 4128527, 18446462599806582783, 1152921504591118335, 18446463698244468735, 17870001915148894207,
        2016486715970549759, 36028797018963967, 1095220854783, 575897802350002111, 10502394331027995967, 36028792728190975, 2147483647, 15762594400829440,
        288230371860938751, 13907115649320091647, 18014398491590657, 2305843004918726656, 536870911, 137438953215, 18014398509481983, 2251795522912255,
        262143, 511, 2251799813685247, 68719476735, 848822976643071, 18446463149025525759, 18446462598732841023, 18446462598732840963, 36028792723996703,
        72057594037927928, 10696049115004928, 281474976710648, 2199023190016, 549755813880, 20266198323101840, 2251799813685240, 335544350, 9223389629040558079,
        1, 18446464796682337663, 2589004636760940512, 16643063808, 15032387456, 281474976710655, 176, 140737488355327, 251658240, 16, 72066390130950143,
        134217727, 127, 17592186044415, 18446744069414584320, 9223372041149743103, 9223653511822045823, 2, 18446740770879700992, 42949804031, 290482175965394945,
        18446744073441181696, 18446462599269712895, 144115188075855871, 140737488354815, 18445618173802708993, 65535, 562949953420159, 18446741595513421888,
        16778239, 2251795518717952, 4503599627239412, 281474976710656, 67108863, 15, 18446744073709486080, 562949953421311, 126, 18446462600880324607,
        9223372036854775807, 70368744112128, 16212958624174047247, 67583, 4294443008, 47244640256, 72057594037927935, 4194303, 8065665457643847680,
        1125934266580991, 18446463629527547904, 1152921504606846975, 2305570330330005503, 67043839, 18446744073707454463, 17005555242810474495, 18446744073709551599,
        8935141660164089791, 18446744073709419615, 18446743249075830783, 17870283321271910397, 18437736874452713471, 18446603336221163519, 18446741874686295551,
        4087, 8660801552383, 18446462598732840960, 70368744177663, 4575692405780512767, 16384, 17592185978880, 9223213153129594880, 31, 2063, 790380184120328175,
        6843210385291930244, 1152917029519358975, 4294967295, 288230376151711743, 18446462615912710143, 8589934591, 1073741823, 18446744073709488127,
    ],
};
#[rustfmt::skip]
const XID_CONTINUE: TrieSetSlice<'static> = TrieSetSlice {
    tree1_level1: &[
        287948901175001088, 576460745995190270, 333270770471927808, 18410715276682199039, 18446744073709551615, 18446744073709551615, 18446744073709551615,
        18446744073709551615, 18446744073709551615, 18446744073709551615, 18446744073709551615, 88094074470339, 18446744073709551615, 13321647697761927167,
        18446744056529672128, 18428729675200069631, 18446744073709551615, 18446744073709551615, 18446744073709550843, 18446744073709551615, 18446462598732840959,
        18446744069456527359, 13835058055282033151, 2119858418286774, 18446744069548736512, 18446678103011885055, 18446744073709551615, 11529212845433552895,
        18446744073709486080, 18446744073709545471, 1125899906842623, 2612087783874887679,
    ],
    tree2_level1: &[
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 4, 32, 33, 34, 4, 4,
        4, 4, 4, 35, 36, 37, 38, 39, 40, 41, 42, 4, 4, 4, 4, 4, 4, 4, 4, 43, 44, 45, 46, 47, 4, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60,
        4, 61, 4, 62, 63, 64, 65, 66, 4, 4, 4, 4, 4, 4, 4, 4, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78,
        78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 4, 4, 4, 79,
        80, 81, 82, 83, 78, 78, 78, 78, 78, 78, 78, 78, 84, 42, 85, 86, 87, 4, 88, 89, 78, 78, 78, 78, 78, 78, 78, 78, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 78, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 90, 91, 4, 4, 4, 4, 92, 93, 4, 94, 95, 4, 96, 97, 98, 62,
        4, 99, 100, 101, 4, 102, 103, 104, 4, 105, 106, 107, 4, 108, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 109, 110, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78,
        78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78,
        78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78,
        78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78,
        78, 78, 4, 4, 4, 4, 4, 100, 4, 111, 112, 113, 94, 114, 4, 115, 4, 4, 116, 117, 118, 119, 120, 121, 4, 122, 123, 124, 125, 126,
    ],
    tree2_level2: &[
        70368744177663, 18446471390799331327, 18446744073692806911, 18446744056529682431, 18446744073709551615, 18446462392574410751, 17565725197581524975,
        5765733215448889759, 15235112390417287150, 18014125208779143, 17576984196650090478, 18302910150157089727, 17576984196649951214, 844217444219295,
        14123225865944680428, 281200107273671, 17582050746231021567, 281265183931871, 17577547146603651055, 4221915814182367, 18446744073709412351,
        18158794964244397535, 3457638613854978030, 3658904103781503, 576460752303423486, 67076095, 4611685674830002134, 4093607775, 14024213633433600001,
        18446216308128218879, 2305843009196916703, 64, 18446744073709487103, 18446744070488326143, 17870283321406070975, 18446744070446333439, 9168765891372858879,
        18446744073701162813, 18446744073696837631, 1123704775901183, 18446744069414649855, 4557642822898941951, 18446744073709551614, 18446638520593285119,
        18446744069548802046, 144053615424700415, 9007197111451647, 3905461007941631, 4394566287359, 18446744069481674752, 144115188075855871, 18446471394825863167,
        18014398509481983, 1152657619668697087, 8796093022207936, 18446480190918885375, 134153215, 18446744069683019775, 11529215043920986111, 13834777130128311295,
        32767, 4494803601399807, 4503599627370495, 72057594037927935, 4611686018427380735, 16717361816799216127, 576460752302833664, 18446744070475743231,
        4611686017001275199, 6908521828386340863, 2295745090394464220, 9223372036854775808, 9223934986809245697, 536805376, 562821641207808, 17582049991377026180,
        18446744069414601696, 511, 0, 4494940973301759, 18446498607738650623, 9223513873854758911, 9187201948305063935, 18446744071553646463, 2251518330118602976,
        18446744068986765311, 17870283321406128127, 18446462598732840928, 18446744069414617087, 18446462598732840960, 8191, 4611686018427322368,
        17592185987071, 13830835930631503871, 1125899906842623, 18446744060816261120, 18446744073709550079, 18445618173868443647, 18691697672191,
        16789419406609285183, 18446532967477018623, 2305843004919775231, 9223372032626884609, 36028797018963967, 18194542490348896255, 35184368733388807,
        18446602782178705022, 18446466996645134335, 288010473826156543, 18446462667452317695, 1152921504606845055, 67108863, 6881498031078244479,
        18446744073709551579, 18446744073709027328, 18446744006063816703, 4611686018427387903, 18446744073709486080, 18446744073709355007, 287948901175001343,
        7036870122864639, 12288634533233876992, 2305843009213693951, 9799832780635308032, 18446743798965862398, 9223372036854775807, 486341884,
    ],
    tree3_level1: &[
        0, 1, 2, 3, 4, 5, 6, 7, 8, 5, 9, 10, 11, 12, 13, 14, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 15, 16, 17, 7, 18, 19, 7, 20, 21, 5, 5, 5, 5, 5, 5, 5,
        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 22, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    ],
    tree3_level2: &[
        0, 1, 2, 3, 4, 5, 4, 6, 4, 4, 7, 8, 9, 10, 11, 12, 2, 2, 13, 14, 15, 16, 17, 4, 2, 2, 2, 2, 18, 19, 20, 4, 21, 22, 23, 24, 25, 4, 26, 4,
        27, 28, 29, 30, 31, 32, 33, 4, 2, 34, 35, 35, 36, 4, 4, 4, 4, 4, 37, 38, 39, 40, 41, 42, 2, 43, 3, 44, 45, 46, 2, 47, 48, 49, 50, 51, 52,
        53, 4, 4, 2, 54, 2, 55, 4, 4, 56, 57, 2, 58, 59, 60, 61, 62, 4, 4, 3, 4, 63, 64, 65, 66, 67, 68, 69, 70, 71, 59, 4, 4, 4, 4, 72, 73, 74,
        4, 75, 76, 77, 4, 4, 4, 4, 78, 79, 80, 81, 4, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 82, 4, 2, 83, 2, 2, 2, 84, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 85, 86, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 87, 88, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 2, 2, 2, 2, 2, 2, 2, 2, 2, 62, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 2, 2, 2, 2, 2, 2, 2, 2, 59, 89, 69, 90, 18, 91,
        92, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 2, 4, 4, 2, 93, 94, 95, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 96, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 88, 34, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 97, 2, 2, 2, 2, 98, 99, 2, 2, 2, 2, 2, 100, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 2, 101, 102, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 103, 62, 4, 4, 4, 4, 4, 4, 4, 104, 105, 4, 4, 106, 4, 4, 4, 4, 4, 4, 2, 107, 108, 109,
        110, 111, 2, 2, 2, 2, 112, 113, 114, 115, 116, 117, 4, 4, 4, 4, 4, 4, 4, 4, 118, 119, 120, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 121, 4, 4, 4, 122, 123, 124, 4, 125, 126, 4, 4, 4, 4, 127, 128, 4, 4, 4, 4, 4, 4, 4, 129, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 130, 2, 2,
        2, 131, 2, 132, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 133, 134, 135, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 136, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 137, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 128, 2, 2, 2, 11, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 138, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 139, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 2, 2, 2, 2, 2, 2, 2, 2, 140, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 141, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 87, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 2, 2, 2, 87, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    ],
    tree3_level3: &[
        13258596753222922239, 1073692671, 18446744073709551615, 576460752303423487, 0, 9007199254740991, 2305843009213693952, 18446744069951455231,
        4295098367, 18446708893632430079, 576460752303359999, 18446744070488326143, 4128527, 18446466993558126591, 1152921504591118335, 18446463698244468735,
        17870001915148894207, 2016486715970549759, 36028797018963967, 1095220854783, 575897802350002111, 10502394331027995967, 36028792728190975,
        2147483647, 15762594400829440, 288230371860938751, 13907115649320091647, 9745789593611923567, 2305843004918726656, 536870911, 549755813631,
        18014398509481983, 2251795522912255, 262143, 511, 2251799813685247, 287950000686628863, 875211255709695, 16140901064495857664, 18446463149025525759,
        18446462598732972031, 18446462598732841023, 36028792723996703, 9241386160486350975, 287951100198191108, 18437736874454810623, 22517998136787184,
        402644511, 13907115649319829503, 3, 18446464796682337663, 287957697268023295, 18153444948953374703, 8760701963286943, 16173172735, 67043519,
        18392700878181105663, 1056964609, 67043345, 144115188075855871, 1023, 287966492958392319, 127, 18446744069414584320, 9223376434901286911,
        17996384110963061375, 67043343, 18446740770879700992, 120208752639, 9223372036854775807, 18446744073709486208, 18446462599336820735, 18410715276690587135,
        18445618173869752321, 36027697507139583, 13006395723845991295, 18446741595580465407, 4393784803327, 36028792723996672, 14411518807585456127,
        67043335, 281474976710656, 67108863, 140737488355327, 15, 18446744073709486080, 562949953421311, 281474976710655, 4194303, 18446466994631868415,
        8796093022143487, 16212958624241090575, 65535, 18446744073709520895, 4294934783, 844540894248960, 72057594037927935, 8065665457643847680,
        1125934266580991, 18446463629527547904, 1152921504606846975, 2305570330330005503, 1677656575, 18446532967477018623, 17872504197455282176,
        65970697670631, 28, 18446744073707454463, 17005555242810474495, 18446744073709551599, 8935141660164089791, 18446744073709419615, 18446743249075830783,
        17870283321271910397, 18437736874452713471, 18446603336221163519, 18446741874686295551, 18446744073709539319, 17906312118425092095, 9042383626829823,
        281470547525648, 8660801552383, 18446471240106377087, 70368744177663, 32768, 4611439727822766079, 17407, 140737488289792, 288230376151711743,
        288230376151646208, 9223213153129594880, 8323103, 67047423, 790380184120328175, 6843210385291930244, 1152917029519358975, 287948901175001088,
        4294967295, 18446462615912710143, 8589934591, 1073741823, 18446744073709488127,
    ],
};