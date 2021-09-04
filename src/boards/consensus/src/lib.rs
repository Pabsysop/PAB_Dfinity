use std::collections::HashMap;

type Calculator = fn () -> ();

pub enum  Consensus {
    POW,
    POA,
    POS
}

struct POW<T> {
    work: HashMap<T,Calculator>,
}

impl Consensus {
    fn validate() -> bool {
        todo!()
    }

    fn finalize() -> Consensus {
        todo!()
    }
}
