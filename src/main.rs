#[allow(dead_code)]
enum Mode {
    Indexer,
    Searcher,
}

const MODE: Mode = Mode::Searcher;

fn main() {
    match MODE {
        Mode::Indexer => {
            asgn1::create_index();
        }
        Mode::Searcher => {
            asgn1::start_search();
        }
    }
}
