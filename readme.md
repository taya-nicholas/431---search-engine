# Docs

See docs/explanation.md for a description of the program and how it works.

# To do

- Allow multi-word queries, using some posting multi-way merge
- Allow score to work in multi-word queries

# Current status

## Indexer

#### Iteration 4

Small root node points to the specific block node to search. Block node contains disk size, and disk offset to postings.bin file.
When displaying docno tags, doc_id (index) is converted to byte offset to seek to doc_offset.bin file. This retreives offset to the originial document collection.

#### Iteration 3

Doc_ids are stored as d-gaps. File read through BuffReader.

#### Iteration 2

Index stored as BTreeMap(String, Vec((u32, u32))). In other words, a list of tupes (doc_id, word_count).

#### Iteration 1

Index stored in BTreeMap(String, Vec(u32)). Where String is the word, and Vec is a list of the document that it occurs in.

Struct of BTreeMap is then converted into bincode, which is stored as .bin file.

### Performance

#### Iteration 4

Parsing and indexing: 70 seconds.

Searching:

- Load root node: 0.00024 seconds.
- Load block node: 0.00027 seconds.
- Search block node: 0.00000 seconds.
- Seek read postings: 0.00008 seconds.
- Decode d-gap: 0.00000 seconds.
- Display postings: (around 0.0001 seconds per document).

File sizes:

- root.tree = 5kb
- nodes = around 10kb
- postings.bin = 90 MB
- doc_lengths/doc_offsets.bin = 677kb

#### Iteration 3

Basic Index: 29 seconds.

- Parse words: 3 seconds.
- Load words into index: 26 seconds.
  Encoding and storing file: 5.2 minutes.
  File size: 92 MB.
  Loading file: 0.6 seconds.

#### Iteration 2

Basic index: 20 seconds.
Encoding and storing file: 7.6 minutes.
File size: 214 MB.
Loading file: 3 minutes.

#### Iteration 1

Basic index: 20 seconds.
Encoding and storing file: 10 minutes.
File size: 349 MB.
