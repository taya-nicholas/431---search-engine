# Current status

## Indexer

#### Iteration 3

Doc_ids are stored as d-gaps.

#### Iteration 2

Index stored as HashMap(String, Vec((u32, u32))). In other words, a list of tupes (doc_id, word_count).

#### Iteration 1

Index stored in HashMap(String, Vec(u32)). Where String is the word, and Vec is a list of the document that it occurs in.

Struct of HashMap is then converted into bincode, which is stored as .bin file.

### Performance

#### Iteratoin 3

Basic Index: 29 seconds.

- Parse words: 3 seconds.
- Load words into index: 26 seconds.
  Encoding and storing file: 5.2 minutes.
  File size: 92 MB.
  Loading file: 2.5 minutes.

#### Iteration 2

Basic index: 20 seconds.
Encoding and storing file: 7.6 minutes.
File size: 214 MB.
Loading file: 3 minutes.

#### Iteration 1

Basic index: 20 seconds.
Encoding and storing file: 10 minutes.
File size: 349 MB.

Current problems:

- File size is too large. Perhaps a different data structure would be more appropriate.
