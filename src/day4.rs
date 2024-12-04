struct WordSearch {
    chars: Vec<Vec<char>>,
}

fn main() {
    assert_eq!(18, part1(&test_input()));
    assert_eq!(2557, part1(&input()));
    assert_eq!(9, part2(&test_input()));
    assert_eq!(1854, part2(&input()));
}

fn part1(input: &str) -> usize {
    let word_search = parse(input);

    let mut matches_found: usize = 0;

    let max_size = word_search.chars.len() - 3;
    for y in 0..word_search.chars.len() {
        for x in 0..word_search.chars[y].len() {
            // left to right
            if x < max_size
                && word_search.chars[y][x].eq(&'X')
                && word_search.chars[y][x + 1].eq(&'M')
                && word_search.chars[y][x + 2].eq(&'A')
                && word_search.chars[y][x + 3].eq(&'S')
            {
                matches_found += 1;
            }

            // right to left
            if x > 2
                && word_search.chars[y][x].eq(&'X')
                && word_search.chars[y][x - 1].eq(&'M')
                && word_search.chars[y][x - 2].eq(&'A')
                && word_search.chars[y][x - 3].eq(&'S')
            {
                matches_found += 1;
            }

            // top to bottom
            if y < max_size
                && word_search.chars[y][x].eq(&'X')
                && word_search.chars[y + 1][x].eq(&'M')
                && word_search.chars[y + 2][x].eq(&'A')
                && word_search.chars[y + 3][x].eq(&'S')
            {
                matches_found += 1;
            }

            // bottom to top
            if y > 2
                && word_search.chars[y][x].eq(&'X')
                && word_search.chars[y - 1][x].eq(&'M')
                && word_search.chars[y - 2][x].eq(&'A')
                && word_search.chars[y - 3][x].eq(&'S')
            {
                matches_found += 1;
            }

            // diagonal south-west to north-east
            if y < max_size
                && x < max_size
                && word_search.chars[y][x].eq(&'X')
                && word_search.chars[y + 1][x + 1].eq(&'M')
                && word_search.chars[y + 2][x + 2].eq(&'A')
                && word_search.chars[y + 3][x + 3].eq(&'S')
            {
                matches_found += 1;
            }

            // diagonal north-west to south-east
            if y > 2
                && x < max_size
                && word_search.chars[y][x].eq(&'X')
                && word_search.chars[y - 1][x + 1].eq(&'M')
                && word_search.chars[y - 2][x + 2].eq(&'A')
                && word_search.chars[y - 3][x + 3].eq(&'S')
            {
                matches_found += 1;
            }

            // diagonal north-east to south-west
            if y > 2
                && x > 2
                && word_search.chars[y][x].eq(&'X')
                && word_search.chars[y - 1][x - 1].eq(&'M')
                && word_search.chars[y - 2][x - 2].eq(&'A')
                && word_search.chars[y - 3][x - 3].eq(&'S')
            {
                matches_found += 1;
            }

            // diagonal south-east to north-west
            if y < max_size
                && x > 2
                && word_search.chars[y][x].eq(&'X')
                && word_search.chars[y + 1][x - 1].eq(&'M')
                && word_search.chars[y + 2][x - 2].eq(&'A')
                && word_search.chars[y + 3][x - 3].eq(&'S')
            {
                matches_found += 1;
            }
        }
    }

    matches_found
}

fn part2(input: &str) -> usize {
    let word_search = parse(input);

    let mut matches_found: usize = 0;

    for y in 1..word_search.chars.len() - 1 {
        for x in 1..word_search.chars[y].len() - 1 {
            // We'll only start checking chars that are the "middle of the x", which is an A
            if word_search.chars[y][x].ne(&'A') {
                continue;
            }

            // Check if there is at least a MAS diagonal from north-west to south-east or opposite
            if (word_search.chars[y - 1][x - 1].eq(&'M')
                && word_search.chars[y + 1][x + 1].eq(&'S'))
                || (word_search.chars[y + 1][x + 1].eq(&'M')
                    && word_search.chars[y - 1][x - 1].eq(&'S'))
            {
                // If this is the case, we continue to the next check.
            } else {
                // Otherwise, no need to do further checking.
                continue;
            }

            // Check if there is also a MAS diagonal from north-east to south-west or opposite
            if (word_search.chars[y - 1][x + 1].eq(&'M')
                && word_search.chars[y + 1][x - 1].eq(&'S'))
                || (word_search.chars[y + 1][x - 1].eq(&'M')
                    && word_search.chars[y - 1][x + 1].eq(&'S'))
            {
                matches_found += 1;
            }
        }
    }

    matches_found
}

fn parse(input: &str) -> WordSearch {
    let chars = input.lines().map(|line| line.chars().collect()).collect();

    WordSearch { chars }
}

#[allow(dead_code)]
fn test_input() -> String {
    "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
        .to_string()
}

#[allow(dead_code)]
fn input() -> String {
    "XMSSMXMSMXASAMMXMXMSSSMMSMMMSXMAXXMSMSMXMAXSXSXSMMXSXMSSXMAMXMMXXSXSAMXASXMXXAXSXSXXMMSAMSSSMSMSAMXXXSXSMSSXSXMSAXMMMSMXSAMXMXSAMXAMAMXMXXXX
XSAAXAMMAXSAMSMSMSXAXAAXSAAAAMSSMMMAMAMXXMAMAMMSASXMASAXXSMXSSSMMSASAXXMSAMASXMMMSMXSAXAXMAMAAMMMSSMMSAAAMAASAMXSAAAXAMXMMMXSAMASXSSMSMMMXMM
MMSSMSAMXMAAXXAAAXMASXMMSSMXSMAXMASASXSMMXSMAMAMAMXXMMMAXAMXXAAXAMMMMMMXSAMAXAAXAXAAMMSMMMAMMMSMAAAAAMMMMMMMMAMAMSSMSMSASAMXMMSMMAASAAAAAAAA
XAAMAMSXMASMMMSMSMAMXAMMMASXAMAMMMSXSMMAXAMXSMSMSMMMAAXSAMXSMSMMSXMAAAXASAMAMSSMMSMMSXMMAMAMAAAMMSSMMSAXAAXMXMMMMAMAMXAASXSSXXAAMMMMSMXXSSSS
MMXSAMMMMXMXAXXAAAXASXMAMAMSXMSASASASASXMXSAMXMAXAXSXMSXXMAXXXMAMASXSSMMMXMXMMAXXAAXXMASXMAMSMMSXMAXASMSMSSSXXAXMXSMAXXXMAMAXSSSMSXMXSSMXMAX
AXMSMSAASAMSXMMSMSXMSAAMMSXMMMASMAMASMMXAAMXSAMXMSMSASXMAMSXSASXMMMMAMMMMSXMASXMMSSXSMMMAMSXXAAXMXAMXSASAMAMMSMSMAXMAXSAMAMMXMAAAXXXAXXAXXAM
ASMMXSMXXAMMAMAMAXMASMMMAMAAASMMMSMMMMSMSMSASXMXAAAMMMASXMAAMAAXXAXMASAAASAMAAASXXMXXAAMXMMASMMMSMAXMMAMMMASAAXAMMSMSSXASAMXAMSMMMSMMMMSMMSS
MXAMMMMSMSASMXSSXXMXSXSMXSSSMXXSAXXXAAAAASXMSAAXSMSMASAMAMMSMAMMMMMSAMXSXSAMXSAMMAMMSXMMAXMAMAMAXMXMAMAMXSASMSSSXMAAXMXMMMSSSXMAXAAMAAAAAXAM
MSSMAXAAXXMXAMMXMXXXMASAXAAXAMSMMSMSSSMSMSXXSMSXMAXMXMAXSMAMXSAAAAAMASAMXSAMAMAASXSAMASXMXMASXMMSMMAMSASMMASAMAMASMSMSMSAMMMXMMMMMSSSSSMSMAS
XAXSMMMSSSMMSMSASMXAMMMXMMSMSMSAXMAAMAMXXMMMMAMXMAMSMSSMMXXSAXXSSMMSMAAMASAMXSSXMXMMSAMASMSAMAAAAAASXSASXMAMXMAMMMAAXAXMAMXSAMSSSMXMAMMXXMMS
MMXAXSMMMAAXAXSAAASAMXAXMXMAMASXSMMMXXMXAXAMMAMAASMSAAXXSAMMXSMMAMMMAMSMXSAMXMAXMAMAMASAMXMXSSMSMXSAXMAMMMXSSSMSAMSMSMSSSMMMXSAAAAXXSXSMSMSS
SSMSMMAASXMMMXMSMXXAXMMXMAMXMXMASAXMASXMSSMXSASXSMAMMMSAMASAMXXSAMXXMXMMXXXXAXAAMAMAMXMXSXMMAMAMXMAMXMXMASAXXAASAMXXMXXAMXMSMMMXMMMAMAAAXXAS
MAAMASXMMXAAXAAMXMMMXXAAXAMAMAMSXMAMAXAAMXXXSXSXXMSMMAXMMMMMSMMMAMMXXAASXMASMMSXMAXXSMMMMAXMASMSMXAMXMXSAMXSMMMMAXXMSMMAMMAAAAAASXMAMSMMMMMS
SMMMAMXAAXSMMMMMAMAMMMSMMAMASXSAAMASXMMMMAMMMMSXMAXXSXXMAAAXAAMSSMAMSSXMAXAAAAAXSSSMAASASMSAMXAAMSMMXAAXASASXMASXMXAAASAMMSXSMSXSAMXMAMAAXAS
SAMMSMMMSAMXMAMSMSAAAAAMSMXMSXSMASASAMAMMAMXAAXXXXMAMXASMMXSSXMAAMXMAAASMMSSMMSMXAAMSMMAAAAMXMMMMAASMMMXAMXXAMXSSSMMSXMXXXMXMAXMSAMXMAXSXSXS
SAMSAMXAXMASMAMAAXMSMSSMASAAXAMXMAXMXMAXMASXMSSMMXMAXXXMMSMMMSMMSMMMSXMMAAXXMAXXMSMMMXMSMSMMSXSXMMXMAAXAXMSMSMSXAXAMXAMXSAASXMSXSAMXSMMXAMXS
SMMSMSMMSMAMXMMMSMAMAAMMASMSMMMMMMAMASXXSASXMMAXAXMMSMAMAAAAAXXAMMSAMASXXSXAXSXMAXMAXAXXMMMMAASMXSASXMMSAAMAMXSMMMSMXAXAXMMXAXXMMAMAAAAMASAM
XAAXMXAXXMASAMAAAMAMXMSMXMXAMXAAAXMSXXAXMXMASXMMMXMAAXAMSSSMSSMXSAMASAMSMMMSMAAXSXMMMSMAXAMMMXMAAAAXXAAXMAMSMMSAMAASMMMMMXAMMMSXSAMXMMMSAMAS
XMMSSSSMMXAMASMSSSMSMAXMAMXMSXMXSSXMAMSMSAMXMAMSMSMSSSXMMAMXXXASMAMMMXMAXMAMASXMMMXMAXSMMXXAXAXSMMAMMMSSMAMAAASAMMSXMXAXAMXSMASAXSXSMMXMASXM
XAAXXAAASMSXMMMAMAAAMMMMMMSXMASXMMAMMMMASAMSSSMAAAAMMMMMXAXMAMMMSXMAAMSMSXAXMXMAXAXMAXAXMMMSXXXAXXAXAMXXMASMMMSXMXMXMMXMXSAAMAMAMMMSAMSSMMAM
MMMSMMMMMMMASXMMSMMMMAXXAXAAMAMMASXMAAMAMAMAAMXMMMMMAAMSSSMMAAMXXXMMSMAMSXMSMMSSSSSMSSSMMAXMASXSMXSSSMSSSMXMAMSXXAMAMSMAAXXXMASAMXASAMAAXSXS
SASAAAMASASAMAAAAAMXMAXXASMXMAMSAMASXXXAMASMSMSSMSAXXMSMMMASMSSMMXSAAMAMMAXAAMAXMASAXXMAMSAMXMAMAAXAMAAAAMMSAMMMMSSXSAMSAMXXMASAXMMMAMSSMSAA
MXSMSXSAMMMASXMSSMMSMAXXAMXMSSMMMSXMASMXSAMXMAAAASMSSMSAAXXMXMAXAAMSMSMSSMMSXMXSMMMXMASAMMMMSMMMMMMAMMMXMMAXXSMAXAAMSXMXAMXMMMXXXMSSXMXMXMAM
MMXXAMMAMXSXMAAXAASMMAXAAXMASMXAMAXMXMAMMMMAMMMMMMASAASMMSXMASXMMMMAXAMAASAXMSMMMXMMSAMASMXAXMXXSMMSMMSSSMMSAMXSMMSMMASMMMAMSSSMMXAAXSMMMAMX
XSAMXXXAMAXAXMMMSMMAMAXSAMMSMXSMMAXMMMXMAASAMXSAXASXMMMAXAASXSMASXXXSSSSMMSMXAAAXASXMASAMMMSSMSAMXMMAAAXAAAMAMAMXXAXSXMAXSASXAAAAMMSMSAAMMSM
AMAMSASXSMASXAXXXMSSMMMXAXXXXMAAMSXSAAXMSMXXSAXXXSAMXASXMSXMXSASMMMXXAAAXSMXSXSMSXSASAMAMAAMSAMXMASXMMSSMMMSMMXSMXMASASMMSSSMSMMXSAMASMMSAAA
MSAMMMSAAMAXMMMSMXAXAMSSMMSXMASXMXASMSMAAAMSMXMXMAMSXMXAMXASAXXAAAMASMSMMXMAXAAAMMXXMASXSMXSMMMSSMSXASAMMAAAXMXMASXMSAMMASASAMAAAMAMXMASMMSS
XSXSAAMMMMMSAXAAXMASAMXXMAMAMXMXSMMMAMASMSMAASMSXMASASXMSSXMASXXSASXSAAXAAMAMSMMMMXSMMXMAMXMASAXMASMMMASXMSXSXMMMSXMMAMMSMAMXSAMXMAMASXMAMMM
MXXXMSSSXMASMMSSMSAMXXXMASMXMAMXXAXMAMMMXXMMXMAAAMASAMAAXMAMAMXXMMAMMMMSSMMXXXAMXSAMASXSAMXSAMMSMAMAXSMMXAMASMMAASAMMSXSAMXMXSMMSSMSXSXMXSAM
SXSXSAAMAMMSXMAAXMASMMSAAXMAMAXASXMMSSMXSASXSMSMMMASASMMMSMMASAAMSAXSXMXXSMMMSAMAMASMMASXMXMXSMAXSSSMSMMMSMAMASXSXMMAMMSASXMAMAAXAAMAMAMAMAX
MAMMMSSSXMXMMMMXXXAMXXAMXXMMSSSMSAAXAAXAMXSAAAXAXMXXAXAAAAXASMMMAMMSMXMSMAMAMXMMASXMXMAMASXMXMMMXMAAAXAAAAMXSXMMAMSMMSASAMAMXSMMMAMMAMAMMMSM
MAMAAMMMAXSXAXMASMMSSXSSSMMMMMAASASXSMMSMAMXMMSSMSASMSSMSSSMAXAMASXMXAXAAXMXSAXXAMXMAMASXMAXXMASXMSMMMSMSSSXMASMMMXAAMMSMSSMMMMMSMMMAMXSXAXS
SSSMXSASAMASXXSAMAMXAAAAXMXAASMMMAXAAXSAMAXXXAAXASAMXAXAAAAMMMXMAMAASMSMSASAMAXSASASAXMSMSXMXSAXXAXMSAAAXAMMMAXMSMSMMSAMXXMASAAXAMSMXSMXMAXA
XXXAASASAXMASXMASXMSSSMMMSSSXMASMMMSMMSASXSMMMSSMMAMMXMASMXMAAAMXXMMAAAAMMAAMAMSASMXSASMXSSXAXXSMXMXMXSXMMXAMMSMAMSAASXSMMSAMXSMSMSAASXMASAM
SAMMMSMMMMXXAXMAMXAAAXXXAMAMASAMAXAAXXSXMXAXXAXMMAXXMXSMXMASMSMMSAXSMMMSMSSXMXMMAMXAXMAXAMMMMMSAMXSASAXASXMASMAMSSSMMSAXAAMASAXXMAMXMMAMAMAX
SXMMAXMXSSSMASMSSMMMSMSMMXAMAMXMSMSXSMMSSSXSMMMSXMMSXAXMASAMXXAASAAMSMSXMMMMSXMMAMMMSXSMSMMAMSXMASXAMAXMMMXSSMSSMAXMSXXSMMSAMXSAMAMAXXXMASMM
SAXSMSMSMAAMMMAXXXAXXMMAASMMSXMAXAXAMXAAMAAXXAASMSASMSMSAMXMSMMMSAMXAXMAMMSASAASASAXMAMAAASXSMMMMMMMMMMAAASAMXMAMXMXXAXXMAMASXXXSASMSMSSXSXA
MAMAXAMAMSMSAMXMMXSXMASMMSXAMAMAMAMSMMMSSMSMSMMSAMXSAAMMMSAMXAMAXMASMMMAMXMAXXMAMSMSXXMMMXMMAMASASAAASXMMSMASMSXMAMMMMMMMXSXMAXMXXSAAAAAAMMM
SAMMSXSAXAXSXSXAXMMAAXAMXXMMSAMSMMMAXSMMXXAAXXXMXMAMMMSAASXSXSASMXMMASXMMXMAMMAMXSMMMMMXSXSAMSMMASAMXXSAMXXASXMASXMAMXAXAXXMXSAXXAMMMXMMMMAX
SMSXAASMSMXSMMMSXAMSMMASAMAXMXMXAXSASAMXMSSSMMXSXSXXAXSXXSAMXMAMMMSMAMASMSMAXSAMAMAAAASMSASMXSXMXMMSSMSMMXMMMASXMMSSSMSSMSAMXAAASXSAMXSSMSSS
XAAMMXMAAXASASAMMSMAASAMMMAMMAMXXMASMXXSXAXAAXXXAMXMXMXSMMXMMMAMAAAMASMAAAMAMSAMSAXSXXXAMAMSAMAXSAAXMASXXSMMMAMXMAMMAXMAMMAMMMMMAAAASMMAAAAX
MMMMSMMSMSASXMASAXMXMMASAMAMMAXSAXXXXMXAMXSMSMMMMMAMXAAXXAXMSAASMSSSMSXMMMMSXSXMXAMXXSMSMXMAMSAMMMMSMAMMSMAAMSMSMSSSSMSMMSAMAASXMMSXMASMMMSM
MSAAAAAAAAMXASAMXSMSSXXMAMAXMASMXMMMAXSMMASAAMXAAXAXAMSMMMSXAXMAXAAMMMAXXSMMASMMMSMSAXAAXSMXXMASXAAXMASXAMMMXXAAXXMAAXSAASXMSMSXXMAXMMMMMXXA
ASMSXXMMSMXSXMASXMAAAXSSSMASXXSXXMMMMMAXMAMXMASMSMSAMXMAAAXMMMSXMMSMMSSMASAMAMAXSAAMMMSMMMSMASXMXMMMXAMMXXSMMMSMSSMSMMSMMMXAAMMMMXMXMAXMMAXS
AXXAXSAMMXAAAMSSMMMMSMAAXMAMXASAXSAASAMXMXMMXAXXXAAXAMXMMXSAAAXAXAXAAAAAASXMASMMMMMMXMXSXAASMMXAAXASXSSXSASAAXXMAXMAMAMXAMSSSXSAMAMMSSSXMXSX
MAMAMSAMASMSMMMXAMMXMAMSMSMXMXMAMSSSSSXSMSSSMMSXMXMXMASXSXSXXSSSMMSSMSSMMSMSXMMAXAAMMMAMMXMSXAMSMSMSAAAAMAMXMXAMXXXMMAXMXMAAXMMMSASAAMXXXMXM
SXMSXSXMASXXMAXXAMXAXMAXAXXXSXMXMXMMMMMAAAAXXMAXAAMXXSAXMAMXMXXMAXXXAAXXSXAXAMSXSMXSAMASAAMXMMXAAAXMMMMMMMMASMSMMSSSSSSMMMMSMSSXSMSMSSMXMMAA
MXMMASXMASMMSASXXASMSSMMSMMASXMXMASAAASMMMSMMXSMSXSAMXXAMAMAMXSMSMXMMMMSAMXMSMSASAXSASXSMMMAMXSMSMMXMXXMASXMXAASAAAMXMAMASMMAXMASAMAMAXXAMSM
AMXSAMMMMSMAMAXXMXMAAAAAMAMSXAMXSASXSMSXSXAXSAXAXAMMSSSMSASASXAMAAXSSXMXXXSXMAMAMAMSAMXSAMXSAMSXAXSMXMXSASAAMSMMMMSMMSXMXMAMSMMAMAMSMAMSSMXX
SAAMAMAMXMASMSMASMSMSSMMSXMMSAMMMXSAMXMAXMAAMAMXMSMAAXAAAASASXMSSSMAMAMXMXMAMSMMMMMMAMXMAMXMASMMMMAXAAAMASMMMMAXXAMMAAASMSMMXAMMSXMAMAMSMXSA
AMSSSMXSAMXMAMAXAAAAAAAXMMAXMXSXXASMMXMAMMMSMSMXXMMMSXMMMXMXXAXAMXAASAMMMAMXMAAXXXXXSXMSAMXSMMMAAAMSXSXMXMXSASMMMSSMMXMMAAXSXSAXSMMMSMXXAAMM
MAMAAASMMMXXXXXMMSMMSSMSMSSXMSAMXMMAMMMSSXAMAMMSMXMAMXXAXXAASMMMSASASAMASASASMSMMMMMMAMMMXMAXASMXSXSAMXSXMXXAXAAAXXXSMAMSMMXAXXXMAXXXMSMMMSX
XSMMMXSAASMSMMSMXXXMAMXSAAAASXMXSXSXMMAAXMXMAMXAAXMAXAMXXSMMAAXXMAMASXMXSAXXSAAMAAMAMXMAMSASXXSAAMAMAMAAAMSMMSMMXXMASXMMMXAMXMMMSXMASMAAAXMX
XXAXSAMMMAAXAAAXMXAMXXAMMMSMMAXAMAMMSMMXMSXMASMMSSMSSSSSMXSMXMMMMXXMMMMMMMSAMXMMSSMAMSASMSAMMAMMMMAMAMSSMMAAXSXSSSMASAMMSMMMMAXAMAMAASMSMSSX
ASMMMXSXMMXMMSXSMSSMSMSSSMXASXMSMAMAMMXSAAXSASAXAXAAXAAAXAXXMXMSASMXMAMAAMMMSSXMXXMASMAXXMAMMAMASMXMXAAXASXXMMAMAXMAXAMASXAASAMASXMXMMXMMSAS
MXMAMXSAMSMSMXAAAAXAAAAAAASAMAAMMMXSAAAMXMXMASMMASMSMMSMMASMAAXXASAAMSMMMSAAAAASASMXMMAMMMMMXAXMMMMMMMMMXMMXSXMMMMSASXMAMMMMSAAMMXMMXMAMXSAM
XAMASAMXMAASAMSMMMMSMSMSMMMXMMXMASMMMMMSAXXMXMXMMSXAAXAAMAXAXASMMMMMXXAAASXMMSMMASMSXMASAAASMMSSMSAXSASASXSAMAMXXAMAAXMSSXSAXMXAXAXAAMMSAMAM
SXSSMMSAMMSMAAMMASXMAXXMAXMXSXXSASASMMAMMSAMXAXMAMXMSSSSMMXSAAMAAAXXASMMMSASXMXMXMASMMASMSMAASAMASAXSAXAXAMMSSMSASMMMMAMAAMAMSSMSMMSMXXMASMM
MMMXAAAAMSXXMMMSASAMXMASXMXAXMAMXMXMAXSSXMASXSMMMSSXAAAAAMAXMMSMMSSMAXXAMXAMAXSSMMAMXSAXAXXSXMASAXXXMAMSMMMXMASAXMXSSXAMMMMXMAAXAMAMMMXXASAS
MAAMSMMSMMAXMAMMMXXMAXMSAMMMMMMMAMSASAMXASAMXMMXXAAMMSMSMMMXXMAMSXXMASXMMMXMSMASAMXSAMASXMAXXSMMMSMXMAMXAASASMMMMMXMMSMSSMMSMSSMMMAXAASMXSAM
SMSMXAAXMMXMMASMSMAXMXXXAMSAMMAMAXMAMMSSXMAMSXSSMXSAMMAAXAXMXMAMMASAMAMSASAAMMXMSXXMASAMAMXMAAXAXAAAMAMMSMSASXAAMAMAAMXAAAAXMMMMXMSSSMXSAMAM
XAXXSMMSAMSXSMXAASAASMSSSMMAMMAMSSMAMAMMASAMAAAXMAXAAAMMSSSMXASXSAMAMAASASMSMSMMMMMSXMASAMMMAMXMMSMSMMMMMXMXMMMXMAMMXMMSSMMMMXAXXMAAASAMXSXM
MXMXXAXSAMAAXAMSMMSSMAXAMXSAMSSMMAMXMASAAMAXMMMAMMSAMXXMXMAMSXXAMXXXSMMMXMAMMMAMXAAAMSXSXSAMSXSSXMAAMSAMSSMMSSSMSXSASXMMXAAAMMMMMMMSMMXSXMMS
XAMXSXMSAMMMMXMAAXXXMMMSMAMAMAAASXMASXSMMSSMXSSSSMSMSMMMASAMXMMMMAMXXMSMMMSMXXSXXMMSASMMMMASMAAXAMXMASASAAAAASAAAAMAMAASXSSXSAAAXMAMXMMMAMAM
MSSMMAASAMAAMXSMSMMMMSAMMSSXMMSMMAMXMMMXMAXMXMAMXAXAAAASMMASXAMASMXXASAAAMAAXSASXXMAMXAASXMMMMMMXMAAMSXMAXMMXMMMMXMASMMMMMAMSXSMMMASAXASAMAS
SAAAMMMXMXSMSAAXMAXAAMAMAMMMSAMXXMXAXAMAMXSAMMASMMMMMSMSASAMMSMAXXASMMSSMMMSMMAAMMSSMXSMSAMXXAXAMSMSMSMSSSMSSMSSMXSAXMAAMMMMXAMASXMSMSXSAMAA
MSXMXSMMSAAAMMSSSMMMMSXMMMAAMAMXMASXSMXMSAAAXXAMAXAMAMXSAMXXAMMXSMXMSAMXMXSAXMSMSXAAAXXXSAMXMMXSAAAXXXAAAAMAAAAAXXMASXSMXSXSMSMAMAAXASMMMSMM
MAXXASAAMMMMSAAMXAAMMMAMAXSSMSSSMAAAXXASMMSSMMSSMMSMMSAXXMMSSSSMAMAMMMMAMXAAXXXXAXAMXMSASAMXAAAXMMSMMMMMSMMSXMXMMSMXMXXXMXAMAXMMSMMMXMASAAAX
MSSMASAMXAASMMXSSSMSAMAMMXMAMAAXMMMXMMMSAXMAXAAAAAXASMMSASAAMAAAMMAXAAXAXAMXMXMMMXMXMAMXMAMSMSMSAMXAAXAXMXMMAMMSMMMMSAMAXMAMSMAXXXXMASXMXSXM
SAXMAMAMMSMSAAAMXAXSASMXSASAMMSMXAMASXMSMMSMMMSSMMSSMAASAMMSSXMMMSXXMXSMSMXAAMSAMASAMSSMSSMXXXASXMSXMSMSMSAMXMAAAAAXMASAMXAMAXMMSMMSASAAMXMX
AMMMXXXMXXSSMMXSAMXSAMMASASXSXXMSMSAXAAXAAXXXMAMAAXXMMMMMMMAMASXXXMASMMMAMSMMAXASASXSASMAXXXMMMMAMXMXAMMASXMAMXSSMMXSAMAXSASMMSAAMAMXXMMAMXM
MXMXMASXXMASXXXMXMAMMMMAMMMASAMAAAMASMMSMMSMXMASMMXSMSXSAXMASAMAXMMAAAXSAXAAXSSMMMSXMAMMAMMSAAASASAMSAMXAMASXSAMMAAAMAMAMXAMAAMSSSSSSXMMAXAA
XMMAAXMASMAMMSAMXMXSAXMASAAXSAMSMSMAXAXXXAAXAMXSASMAAAAXASXMMMSSMXMASXMAMMMSMMAMAAXAMAXMAAXSXSXXAMAXSAMMASXMAMAMMMMMSAMMMMSSMXMAMAXAAAXSXSXS
XSSSSSXMXMXSAMAXAXAXXXMAXMSMMMMAAMMXSSXMASMSSSMSAXSMMMSMAMXAXAAAAXMMXMAXXMXXXMASXSSXSAXXAXXSAXASXSSMSXMXXMMMMMASXMSASAMSAAXAXSMAXSMMMSMSAMXM
AXAAAAXXAXMMMSMMMMMSMSMSMSAAAMSMSMMAXMAMAAXSXAXMMMMMAAXMAMSMMMSSSSSMSMXXAMXMAMXSAAAASAMSSMMMAMAMAAXAMAMMSAMAMMMSAAMASAMMMMSMMSSSMXXXAXMMAMAM
MMMMMMMSMMAAMAXMAAMAAXAAXMSSMXAXXAMMSSMMSSMMSAMXSAXXMSSSXMMMAXXAXXAASAASXMAXMXAMMMMXMAMAMMXMXMAMMMMXXAMXSAMMSAAMMMMAMAMMSXMXMMAXAXMMMSAMXSMS
MAXXXMASXMXXSASMMSSMSMMMMXAXXSXMSAMAAAXAAAAXSXSAMXSAMXMASMMSXSMXMAXMMXMSASXSXMASAXXSSMMAXMAXSMXSXXXXXXXMSXMASMSMAXMASXMMSAMSXMAMMMSAMXAAAAMA
MXMXXMASAXSAMMMAMXAXXXMASMAMMXMAXXMMXXMMSSMMMMMXMASMMMXAMMAXMAMXXMXSXXXSASAAXAMXMAMAASMMSSSSSMMSXMXMASMMMAMXMMMMAXXAXAMAXAMAXMASAASXMMXMXMSM
SSSMXMASAMMAMXMXMSAMXAAAMMMMXXAMXSSSSXSAAMXMAAAAMXMAAAMXMXAXSAMXSMASXSAMAMXMXXMASXMAAMSAMXMAMAAMAMAAMAASMXMXSAMXMAMMMSMMSSMMMSAMMXSXMXMSMXMM
XAAXXMXMXMMAMXASASXXSMMXSASAXMSMAMAAMAMMXSASXSSMSMSSXSMMSMSMSASXAMASAXAMXMXXMASAMXMMSMXMSXMAMMMXAMMSMSAMXMSSMMSAMSASAAAXAAMXXMMXSAMASAAAAAXA
MSMMXSAMMMMSSMXSAMXXMASMAAMXMXAAAMMMMAMAASASAXMMAXMAMAXMAAMAMAMMAMXSMSMMMMMSSMMMMSXAXAAMSMSXSASMMXAXAAMAMXMAXAXAMXAMMSMMSSMSMMMAMXSAMMSMMXSM
MAXAXXAMXSAAAXAMAMXXMAMAMMMXMSASMSMMSASAXMAMXMAMMMMASAASMSMSMSAMXAXSXAXAAAAMAAASASMSSMMMSAAAMAXAXMXSSMSAMASMMMSSMMXMAMAMXAASASMSXXMASAMXSMMM
SMMSSSMMAMMSMMXMMMXMMASXXAMAMAXMAXMASXXXXMXMSMMMSASXMMMXAAAAXXXMXXXMASMSSMSSMMMMASAMXMMAMMMSMXXMMSAAAXSASASXXXAMXXSMMSAMSMMSSMAMMMSAMAMAAAAA
XAAAAAAMAMMXXXSAMXAMSASASAMASMSMSMMMMMXMAMAMXMAAXMSXSAXMSMMMMXXSASMSMMAMAMXXXAXMAMAMSMMAXXMMMAAMAMMSMMSMMASAXMXSMMMAASAMXXAMXMAMAASASAMSSSMM
MMMMXXMSMSMMMASAMMMMMASAXXSMMXXAAAAAAMAAASASASMSSXSAMXSXXASXSMAMMSAAXAAXASXMSASMASAMXXSSSSXAMSSMASAAXMMMMMMMMMASAASMMMXXMASAMXXSMMMMMXMAAXMM
XSSMXSAMAAMASASAXSAAMMMXMMXMSSMSMSMSMMXXMMASXSAMAMMSMMMMXMMAXMAMXMXMMSMMMSAMSSMMMSASAMXAAXMXMAMMXMMSSXMAAXAXAMASXMMMAMSMSAMASMMMASXXAMXMSMSA
MAAAAMXMSMMXMASXMSMMXAMXXMAMAAAXAXAXMSMSXSAMAMXMMSAXMSAAXSMMMSMSAASXMASMMSAMSAMXMSAMMMMMMMMSSSSSXSAMAASXSSMSMSASAMXMAMMAMXSAMAASMMXSAMXXMAMX
AMMMMMAAMMMSMMMAXXASAMXSASAMMMXMMMSMMAAAAMAMSMSSXMASASXXMAAMAAAXMXSASASMAMXMSAMXXMMMMXMAAAMMAMAMAXSMSMMAMXAAMMMSXMAMMXMAMXMXMMMXAAXSMMAMMSMM
SXMSSSSSMMAAAASXMMAMAMAMMMASXAXMAMAASMMMSMSMMAXAXMAMXMASXSMMSMSXMAMAMASMXXAMXAXXMASMMAMXSSXMSMMMMMMXMXMMMMXMSMMMXMXMMAMAXMSAXSSXMMMSMMXXAAAX
AAMAAXAAMMSSSMXMAAMSXMASXSXMAAMXASMMMASAMXAAMAMMMMASMMMXAAXMXAXXMSSSMMMASMSMMXMMMXMXSAMXAMMMMAAMAASASXMXXMSMSAAMMXMASMSXSAMXMXAASAMXASXMXSMM
SMMMSMSAMXMAAMXMSMMMAMXMMAAXSMXSASMXSAMXSSXSMAAXAXMXXAMMSMMMMSMAXAAMAMMXMAXAMSMSMAAAMAMSAMAAAMMSSXSASAMAMMAASXMSMSMAXMAAMXMAMMXMMASMMMAXAMXX
XXXSXMXXAMMSMMAMAAAMXMXMASMMXAXMMMMXMXSSXMAAXXSMSSSMSASXMASXAAMAMMXXAMMXXAMXMXAAXMXMMAMSAMSSMSXXMMMXMAMASMMMMASXAXMXMMMMMASMSXMMSAMASXSMSSSS
MMMSAMXSMMMAASMSSSMSAXXSAAXXMSMMSAMAMXMXAXMASXXAXAXAMXAAMSXMXMSMMXMXSSMSMXMASMSMSSMXSAMMAMXAASAMXAAAMMMASAMASXMMMMSAAAXASXSAAXMAMAXMMAMAXAAA
XAXMAMMSAAXSXMAAAMASMMMASAMXMXAAXASASAXMXMXAMMMXMASXMSSSMMXSMMAXXAASXAAAASAXMAXAXAAAMXSSSMMMMMASMSSMXSAMMASASMMMAAMASMSXSASMMSAASAMSMMMSMMMM
SSSSXMASMMXMAMMMSMAMAXXAXXXAMMMMSASASMSMSMMXSAMAMXMAMMMMAXAAMSAMSXSAMMMMXSAMXSMSMMMMSAMAMXMMASAMXMAMAXAMSXMMSASMMMSAMMSAMXMAAXMXMAXXAAAAMSXM
SAAXSMXXMMASXMAMAMASMMMXMAMMSXXAAMMAMXAAAXSMSASMSSSSMAAMXMXMAMAXXMMMMSAXMSAMAXAAAXAAMMSAMAXSXSXSXSAMXSMMXAAMSAMMAAMASAXAMAMXMAXMSSSSSMSSXXAS
MMMMMMSXMMASAMASASMSXAXMMSMSAMMSSXMAMSMSMXSAMXMAAAAMSMMSXMSXMSSMSMAAXMAXMMAMXMMMSSMXSASASAXSXMAMAMMMMXMASAMXMAMMSMMMMMMMSXSSMMXXAAMAAAAXMSMM
SXAAAAXMSMMMXSASASASMXSXXAMXASAAMMSSXSXAXXMXMAMMMMMMAMSAMXAAMAMXXMMMMXMASXXMAMMSMMMMMMMMMMXMASXMSMMSAMMAAAXMSMMMAXAAAAXXAMXAAMAMXSMSMMMSAAAA
ASXSMXSAXXAAXMXMAMAMMMMXMASMMMMXMAAMXMASMXMAMXSAXXXXAXXMAXMMMASMXXMASXSXSAMSSSMAAMSMSASXMASMXMASAAXAMSMSSSMAXMASMSSSSXSAMSSSMMAAAMMXXMASXSMM
XSXMAAMXMSMSSMXMAMSMMAAXSAMXXSXSMMSSMMMMSMXAMXAMAMXSSSSSSMSASXMASXAAAAMAMXMAXAMSSMAAXAASXMXXMASXMSMMXMAAAAMSMSMXXMMAAAMXXXXAMSMSSSMSAMXMXMAM
XSAMMMMAMAMMXMAMAXMASMXXAXXAMXAMXAAMMSAMASMSXSASMSAAXAAAAAAAMXMASXSAMXMAMXMMSSMMAXMSMMMXMASXMMXXMAMXSMMMSMMAAAAXSAMMMXMASMSMMAAXAAXXMXMMAMAM
SMAMMAMASAXMAMSSSSSXMMMSMSMSSMMMMMMSAMASAMXAAMAMMMMMXMMSMMMAMAMMMAXMSASXMXAAAXASAMXAXAASXMXASASASMSASMAMAMXMMMSASMXXMAXAAAAAXMSMSMMXMAAMXSAM
MSMMSAMXSASXSMAAMXMXMXXAXMMAAAXSAMXMXMAMMXMXMMMMSMSXXXXXMXMASASAMSMASMSAMSMMXSXMMXSMSXXMAXXXMASAMAMXSSXXASXSAAAMXAXAMMMSSMSSMAMAAMAAXXXAASAM
XMMASMSMMAMAAXMXMASMMASMSSMSSMMSASAMSSSMAAXMMAMXMASMMXMASMSASAMXMAMXMAMXXMASASXAAXAXXMASMMMSMXMMMAMXMAXSAMASMSMXMMMMSMAMAXAAMAMMMSSMSMXMXMSS
MSMASXAXMXMSMMXASMMAAAXXAAAAAAAXMSAMXAAMSXSAXASAMXMAMXMXMMMASASXSMSSMAMAASAMAXSXMXSMMSMAMAAAMXAMMAMMMAMMAMXMMXXXAXAXAMASXMSSMSSXMAMMAMSMSMAM
AAMASXSSXSMMXAXMXMXAMSMMMXMMXMMSXSMMMSMMAMAXSMMASXSSMMSMMAMXMASAAXAXSAXXXAMMSMSMSMMMAAXASMSXSSMMSMSMMSSSMSMSXMMSSSMXMXASMAMAMAMMMASMAMSAAMAM
SMSASAMSAAXAXSXSASMSAMASXMSSXAXMXMAMAXAMXSAMMXMAMXAAMAAAXMASMAMMSMSMSMXMXAXAMAXMAAMMSSSXSXMXAAXXAAAMSMAXAAAMMMAAMAMMMASXMASAMASAMXMMXSMSMSXS
XXAXMASMXMMASMAMASMMASAMAMAASMMSAXMMASAMMMASXSASXMMMMSSSMXXXMXMSXAXAMMASXMMSMAXSMSMAAAMASAMMSMMSMSMMAMSMSMMMAMXMXAMMMXMASASXMMMMSMXSMMAMXXAA
XAMXSSMMMXMXAMAMXMASXMASXMSXMSSSXSSMMMAMASXMASMXAMXXXXMMAMXMXSMMMSMSMSASAAAXMXMXAXMXSXMAMSMAXSMXAAMSSSMAMASMXSXMMASAMASAMXSASXAAXAMXAMAMAMAM
XMXMXMAASXMSAXMSXMAMXMXMXAXXAXMXAAXAXSSMMMSMXMXSAMXMSAMXSMAMASAAAMXXAMASAMXMSSXMAMMAXAMXXXMSSMSMMMXMXAMAMXMMAMAASASASMSAMXMAAMMMXXMSSMASAMAA
MMAMXSXMSAXSAMSAXMXSAXASMXMMAMMMSMSSMMXAXAXMASASMMAAMSMAMMXSASMMMSSMAMMMXSXSASXMASXMMXMSAMXXXAAXAMXMSXMMSSMMAMXMMASXMXXMMSMMMSSXSAMAXMASASXS
ASASASAMMMMMMMMAMSASMSMSAAAAASAAAXAAAMXMMXMAAMASXSXXSAMXSSMMASXMXAAMMSMSAMXMASXXAMAMXSAMAMMMMSMSMSAMXXMASAASMSMXMMMMMSMMAXASXXAASXMXSMXSAMXX
XSASASXMAAAAAXMMMMASXAMMXXSSMXMSXMXSMMASASMMSSMMXSAMSAMAMXASAMXSMXXMAAMMASMMMMAXXSASAMXSAMAAAAAAASASXXMMMMMAAXAMMAAXAAAMASAMSMMMMMMASAAMXSXS
XMMMMMMSMSSSSMXMAMXMMSSMSXMAXMMXASMMAXXMASXAAAAMXMAMSXMAXMXMAXAXSMAMSSMXAMXMMMSMMAMMAMASXXXXSXSMMMASMMASASXSMSSSXSMMSSXMAMAMXXAAAAMMSMXMASAS
MSMSAAXMXXAAXAMMAXSMSMAMSASAMMASXMASAMXMSMMMSSMMAMMMMXSMSSSSSMSAMXAMAMMMSMSSXMASAAMSXMXMMSSMMMMMXMMMASASMSAMXAAMAMXMAXAMXXXSMXSXSXSAMAMSAMAM
AAASXSXSSMMMMSSXSSMMAMXMSAMMSMAXSAXSMSAXAXAMXXMMSXSASXMSAAAAXAXAASASASXMMAAMAMAMSAXXXMAAAAXAAXSAAMAXXMASMMMMMMSMAMAAAMXMAMXMXMMAXAMASAMMMMSM
MMMMAXAXSXAAAXXAMAXAASMAMAMAAMMSMSMMXSAMMXSMMASAMXSASASMSMMMMSMSXSASMSAAMMMMMSAMXMASASXMMSSSMMMSSSXMSMXMASAXXMXMASMMSMAXASASAMXAMASAMMSMSAXS
XXASAMXMMSSSSSMSMAASMSMASAMMXSXAAAXXAMXMSAXASXMAMAMMMMMMXSXXXMAAAXXXXSMMXMXXXMASXXAXMXXMAMAMAMAAMAMASAMXMMMASXSMAAAAXMAXASASAMMASXMMXSAMMASX
MMXXASXSMAXMAAAAMXMXAXXAMASMMMMMSMXMXSSMXASAMXXAMAMXAAAMXMMMMMXMXMXMAXXMXSMMMSXMXMMSASXSSXMASMMMSAXXXXXXXMXMAAAAMMMSMSMXMMXMAMSMMMSAXSAXMMMX
XMMSAMAMMMSSSMMMSMXMASMSSSMXAAAXAMXMAMAMMMMAMMSMSXSSMMMXAMMAAXAMAMMMXXXSAMAAAMAAXAAMAMAAXXXXXAXMMMXXMMSXMMAMMSMMMXMAAAMMMAAMAMAAAAMMAMAMSAMM
AAAMMMAMMMAAAAXAAMXMMMAMXMASMSMMAMXMASAMAXMMXMAMAMXAAAMSMXASMSMSASXSMMMMMSSMMSSMXSAMMMMMMSMMSXMAAXSAAASASMXSAMXXMXMMSMSASXMSASMSMSSXAMSMSASX
SMXSASMSMMMSMMMSSSXXAMXMAMAMMAXSMSMSAMXSMXXMASMSXSXMSMXASXAMXAASXSAMXAAAXMASAXAMAXAXSXMSAXAAMAMSMAASMMSAMXAMAXMASXMAMMSMSAMXASAAXMAMXSAASAMA
XXAMAMAAXAAMXAAAAMMSXSMSXSAXXAXSAAAMMMMMAMSSMSASAXXMAXXAMXMSMMMMMMMMSXSMMSAMXSAMAMMMSAAMSSMMSAMAMXMMXAMAMMXMXAXSXMMASXMXMAAMAMMMMXMMMMMMMSMM
SMSMSMSMSMSSMMXXSAMSASASAMXSMSSMSSSMSASMAMAAXMAMMMXMASMMMXMAMMAMAAAXMAMAMXSAMXXMASAASMMMAMMAMXSXMMXMSMSMMASXSMSMXAXASMMMASAXMASMXXAAAXMAAXMX
XAXAMAXAAAAXXAXSMXSMAMSMAMAXXMAAAMAASXSSMSSMMMMMMXXSAMXAMAMAMSASMXSAMAMAMMMMXMSAMAMMMAAMXXAXSAXMXAAAXMAXSAXMMSAMSSMASAMAAXSXMASAASMMMSSMXSAM
MSMAMSMSMSSSMMMXASXMXMXXXMASMSMMMSMMMMXAMAXXAXAAMMXMXMXASAXAMSASAAMMMXSXMSAXSAMXXMMSSSMSMSXMAXMSASMMSSMXMSSSXXMMAAMAMXMMSXMMSAMMMMSAMXXMSAMX
SMSXMAMXXAAMASAMMSAMXAMXXMSAMXXMASMAAMMMMMSSMMSASXMMASMXSMMSAMXMMMMAMMMMAMXMAASAMXXAXMAXAAXSAMSXMAAAAAMSAMXMAMXMSSMASAMXMASAMASXMASMMXAXMASA
XAAMSMSSMSMSAMXSASAMAAXMSMMMSMMMAXMSXSAXAXXASAXXXMASASMMMMAMASASXSMMMAAASXMMSMMXAAMMSMMMSMMMAMAMXSMMSSMMXSAMAMXMAAMXSMSAXAMAMAMAMASXMXMASXMM
MXMAAXMAAAMMXSSMMSAMSMXMASAAAASMSSXMASMSMMSAMXSSMSXMAMMAMMXXMMMMAMASMSSXMASAMAXXMSSXAASXXASMMMASMMAAAAAAASASASXMXSMXXAMAMSSMMSSSMAXAXAMXMSSS
ASMSSSMMSMXMAMXAXMMMAAMSASMMSSMAAAMSASXSAAAAXAXAXAMMSMSSSMSMXAXMAMAMAMXASXMASXMASAXXSSMASAMASXSMASMMSSMMMSXSXSXSAXAMMMSSXMAXAXAMMSSSMSXXMAMX
SXAAXAXMMXMASASMMMSSMSMMASAAMAMMMSMXASASMMXXMXMASMSMMAMAXAAAMMSXSSMMXMXXMASAMAXMXXSAMAMXMXMMMXASAMAAXXAXXMXSAMAMAMSAAXAXMXMMXMSMMAAAAXAXMXSM
XMMMSAMXMMSAMAXMASMSXAXMMMXMSAMMXAXXXMXMASMXMXSAXMAXMAMAMMMSAASMMAXMSSXMMAMMSMAMAAMMXAMXMAMAMSMMXSMSSSSMASAMAMMMXMASMMSXSASAMMAMMMSMSMMMSMAM
MAAAAAAXXAAAMMMSMSAMSAMSAMXXSAMXSMMXSAXMAAXSAMMMSSSSSMMXSXAXMMMMSAMXAAXMMSXMAXAMAMSSSSSSSMXAMMSMAXAXAAMSMMASAMXMMAMAMXMASASAXSASXXAXXAAAXMAM
SSMSSMMMMXMAMXAMMMMMMMASAXXAXMMXXAAASXSMSMSASXAXXAXXAXMASMSMXMAAMXMMMSSXAMASMSMSMMAAAAAAAASMSMAMMMMMMMMAXXAMXSAMSMMMSAMAMXSMMSXSXXMXSMMXSMSS
MXMXMMASXSXAMAXMSMAXXSAMMSASAMSSSMMMSAMXMXAMXSMMMMMSAMMXXAXXXMASMMMAXAMMASAMAAAAMMMMMMMMMMXAAXXSAXMAXASXXSAMASMXAAAAXXSAMXXMASXMMMMMSXXAMAXA
MAMXMSMSAMXSMMXMAAAXXMASAMXXAXAAMXMAMAMASMMXXAMXAAAMXASMMSMSMMAAASXAMASAMMXMXXMASXSAMXXXSMMSMMMSASMXMAXAASXMAMMXSSMSMASAXXAMASAXAAAAXAMSSSMA
SMSAAAXXXMAMAMASMSXXAMXMMSSSMMMSMAMASAMASMXMSMMSSMSXSSMAXAAASMXSAMMMSAXMAMXMASMXXAXAXXSXAMMXMXXSAMAMMSMMXMSMSXSAAXAXMASMMSXMXSMMSMXXMXMAAASX
AASMSMSXAMXSASAMXAASAMAMXAAAXAAAXASASAMASXAAAAMXMAAMSAMSSMSASMAMAXSAMXSMXSAMMXMMMMSMMMMSMSMAMSMMAMAXAXAXSAMXMAMMSMMMMASAAXAXAXXAMMSMXAMMSMAX
MMMXAAXMAMXSAMXSMXMSASASXMSMMSSSSXSASAMXSXMSSSMAMMMMSAMMSXMASMXSXMSAMXMAASASAAAAAXAXAXAAXAMMSAASAMXMASAMXSMAMAMAAAASMASMMSXMMSMMSAAASXSAAXMS
XSXSMSMXSMXMAMSAMMMSMSXSXXMAXAMMMASAMXMASMMAMXXSMXSAMXSMSXMAMMMAMXMXXMASMSAMMSSSMSAMXSSSSXSMSAMSSSXMAMXAMXSXSXMSSSMSAMXMXMASXXMSMMSMMMMXSSXM".to_string()
}