# Current status

## Indexer

#### Iteration 2

Index stored as HashMap(String, Vec((u32, u32))). In other words, a list of tupes (doc_id, word_count).

#### Iteration 1

Index stored in HashMap(String, Vec(u32)). Where String is the word, and Vec is a list of the document that it occurs in.

Struct of HashMap is then converted into bincode, which is stored as .bin file.

### Performance

#### Iteratoin 3

Basic Index: ?

- Parse words: 3 seconds.
- Load words into index: ?
  Encoding and storing file: ?
  File size: ?
  Loading file: ?

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
