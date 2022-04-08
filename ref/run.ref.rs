pub fn run() {
    // create_index();
    let now = Instant::now();
    let index = index::load_index();
    let elapsed = now.elapsed();
    println!("Load index elapsed: {:.5?}", elapsed.as_secs_f64());

    let mut mini = MiniIndex {
        btree: BTreeMap::new(),
    };

    mini.create_mini_index(index.btree);

    let now = Instant::now();
    let mini_index_loaded = load_mini_index();
    let elapsed = now.elapsed();
    println!("Load Mini elapsed: {:.5?}", elapsed.as_secs_f64());

    // println!("Enter search term");
    // let mut s = String::new();
    // stdin()
    //     .read_line(&mut s)
    //     .expect("Did not enter a correct string");

    // println!("Searching for: {}", s);

    // let now = Instant::now();
    // index.search(s.trim());
    // let elapsed = now.elapsed();
    // println!("Search elapsed: {:.5?}", elapsed.as_secs_f64());
}