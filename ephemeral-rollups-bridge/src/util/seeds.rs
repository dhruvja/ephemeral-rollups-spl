pub struct SeedsSigner {}

pub fn seeds_signer_for_pda<'a>(
    pda_seeds: &'a [&'a [u8]],
    pda_bump_box: &'a [u8],
) -> Vec<&'a [u8]> {
    [pda_seeds, &[&pda_bump_box]].concat()
}
