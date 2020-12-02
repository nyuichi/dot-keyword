use dot_keyword::postfix_match;

fn inc(num_opt: Option<u32>) -> Option<u32> {
    postfix_match! {
        num_opt.match {
            Some(num) => Some(num + 1),
            None => None,
        }
    }
}

fn inc_inc(num_opt: Option<u32>) -> Option<u32> {
    postfix_match! {
        num_opt.match {
            Some(num) => Some(num + 1),
            None => None,
        }.match {
            Some(num) => Some(num + 1),
            None => None,
        }
    }
}

fn main() {
    assert_eq!(inc(Some(42)), Some(43));
    assert_eq!(inc_inc(Some(42)), Some(44));
}
