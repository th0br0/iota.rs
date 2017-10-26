use curl::{Curl, ProofOfWork};
use copy::*;
use trytes::*;
use search::*;

pub struct CpuPoW;

impl ProofOfWork<Trit> for CpuPoW {
    fn search<C: Curl<Trit>, CB: Curl<BCTrit>>(
        weight: u8,
        offset: usize,
        length: usize,
        tcurl: &mut C,
        bcurl: &mut CB,
    ) -> Option<usize> {
        search_prepare_trits(tcurl, bcurl, offset);

        search_cpu(
            &mut tcurl.state_mut()[..HASH_LENGTH],
            bcurl,
            offset,
            length,
            0,
            move |t: &[BCTrit]| {
                let mut probe = usize::max_value();
                let wt: usize = weight as usize;
                let start = t.len() - wt;
                for e in t[start..].iter() {
                    probe &= !(e.0 ^ e.1);
                    if probe == 0 {
                        return None;
                    }
                }
                Some(probe.trailing_zeros() as usize)
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cpucurl::*;
    use curl_tests;

    #[test]
    pub fn run_testsuite() {
        curl_tests::run_search::<CpuPoW, CpuCurl<Trit>, CpuCurl<BCTrit>>();
    }

}
