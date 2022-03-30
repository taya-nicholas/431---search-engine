# Current status

## Indexer

Index stored in HashMap(String, Vec(u32)). Where String is the word, and Vec is a list of the document that it occurs in.

Struct of HashMap is then converted into bincode, which is stored as .bin file.

### Performance

Basic index: 20 seconds.
Encoding and storing file: 10 minutes.
File size: 349 MB.

Current problems:

- Words are truncated on punctuation. So [don't] becomes [don] and [t].
- File size is too large. Perhaps a different data structure would be more appropriate.
