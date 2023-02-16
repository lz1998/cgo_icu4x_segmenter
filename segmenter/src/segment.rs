use alloc::vec::Vec;

use aho_corasick::AhoCorasick;
use lazy_static::lazy_static;

lazy_static! {
    // https://github.com/unicode-org/cldr/blob/90aaf561d1827ce06b1e9c4173a70c018665c7d2/common/segments/en.xml
    static ref ABBREVIATIONS: AhoCorasick = AhoCorasick::new([
        "L.P.", "Alt.", "Approx.", "E.G.", "O.", "Maj.", "Misc.", "P.O.", "J.D.", "Jam.", "Card.",
        "Dec.", "Sept.", "MR.", "Long.", "Hat.", "G.", "Link.", "DC.", "D.C.", "M.T.", "Hz.",
        "Mrs.", "By.", "Act.", "Var.", "N.V.", "Aug.", "B.", "S.A.", "Up.", "Job.", "Num.",
        "M.I.T.", "Ok.", "Org.", "Ex.", "Cont.", "U.", "Mart.", "Fn.", "Abs.", "Lt.", "OK.", "Z.",
        "E.", "Kb.", "Est.", "A.M.", "L.A.", "Prof.", "U.S.", "Nov.", "Ph.D.", "Mar.", "I.T.",
        "exec.", "Jan.", "N.Y.", "X.", "Md.", "Op.", "vs.", "D.A.", "A.D.", "R.L.", "P.M.", "Or.",
        "M.R.", "Cap.", "PC.", "Feb.", "Exec.", "I.e.", "Sep.", "Gb.", "K.", "U.S.C.", "Mt.", "S.",
        "A.S.", "C.O.D.", "Capt.", "Col.", "In.", "C.F.", "Adj.", "AD.", "I.D.", "Mgr.", "R.T.",
        "B.V.", "M.", "Conn.", "Yr.", "Rev.", "Phys.", "pp.", "Ms.", "To.", "Sgt.", "J.K.", "Nr.",
        "Jun.", "Fri.", "S.A.R.", "Lev.", "Lt.Cdr.", "Def.", "F.", "Do.", "Joe.", "Id.", "Mr.",
        "Dept.", "Is.", "Pvt.", "Diff.", "Hon.B.A.", "Q.", "Mb.", "On.", "Min.", "J.B.", "Ed.",
        "AB.", "A.", "S.p.A.", "I.", "a.ahc.", "Comm.", "Go.", "VS.", "L.", "All.", "PP.", "P.V.",
        "T.", "K.R.", "Etc.", "D.", "Adv.", "Lib.", "E.g.", "Pro.", "U.S.A.", "S.E.", "AA.",
        "Rep.", "Sq.", "As.", "Dr.",
    ]);
}

#[cfg(feature = "word")]
pub fn get_word_breakpoints(input: &str) -> Vec<usize> {
    icu::segmenter::WordSegmenter::try_new_unstable(&icu_testdata::unstable())
        .expect("Data exists")
        .segment_str(input)
        .collect()
}

#[cfg(feature = "sentence")]
pub fn get_sentence_breakpoints(input: &str) -> Vec<usize> {
    if input.is_empty() {
        return Vec::new();
    }
    let end_preserved: alloc::collections::BTreeSet<usize> = ABBREVIATIONS
        .find_overlapping_iter(input)
        .map(|m| (m.end()))
        .collect();
    let icu_points: Vec<_> =
        icu::segmenter::SentenceSegmenter::try_new_unstable(&icu_testdata::unstable())
            .expect("Data exists")
            .segment_str(input)
            .collect();
    let mut breakpoints = alloc::vec![0];
    let mut breakpoint;

    for i in 0..icu_points.len() - 1 {
        let start = icu_points[i];
        let mut end = icu_points[i + 1];
        breakpoint = end;
        while end > start && input.as_bytes()[end - 1].is_ascii_whitespace() {
            end -= 1;
        }
        if !end_preserved.contains(&end) {
            breakpoints.push(breakpoint);
            continue;
        }
        if i == icu_points.len() - 2 {
            breakpoints.push(breakpoint);
            break;
        }
    }
    breakpoints
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_segment<F>(breaker: F, input: &str)
    where
        F: FnOnce(&str) -> Vec<usize>,
    {
        let segments = breaker(input);
        println!("===========");
        for range in segments.windows(2) {
            println!(
                "start: {}, end: {}, text: {:?}",
                range[0],
                range[1],
                &input[range[0]..range[1]]
            )
        }
    }

    #[test]
    fn test_break_words() {
        test_segment(get_word_breakpoints, "Welcome龟山岛龟山岛Welcome");
        test_segment(get_word_breakpoints, "うなぎうなじ");
        test_segment(get_word_breakpoints, "It would be great to not have to worry about low-level implementation details. Who could possibly care how much space the empty tuple occupies? Sadly, it sometimes matters and we need to worry about it. The most common reason developers start to care about implementation details is performance, but more importantly, these details can become a matter of correctness when interfacing directly with hardware, operating systems, or other languages.");
    }

    #[test]
    fn test_break_sentences() {
        test_segment(get_sentence_breakpoints, "如果不用担心低级别的实现细节，那就太好了。谁会关心空元组占用了多少空间呢？可悲的是，它有时很重要，我们需要担心这个问题。开发人员开始关心实现细节的最常见的原因是性能，但更重要的是，当与硬件、操作系统或其他语言直接对接时，这些细节会成为正确性的问题。");
        test_segment(
            get_sentence_breakpoints,
            "Mr. Jones comes home. Dr. Smith Ph.D. is out. In the U.S.A. it is hot.",
        );
        test_segment(get_sentence_breakpoints, "    ");
    }
}
