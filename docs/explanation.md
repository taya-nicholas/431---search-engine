# What is it?

This is a basic search engine built from scratch in rust.
The program parses an XML file (collection of documents) to build an inverted index.
It can then find relevant documents based on a query input.

# How does it work?

A basic search engine scans through some series of documents (in my case a 500mb Wall street journal xml collection).
Every word the parser finds is added to an index, along with which document it found it from.
So each unique word in the collection points to a postings list. Each posting is a tuple of document_id (where the word was found), and term frequency (how many times did the word show up in this document).

The main challenge is in reducing what you have to load for each search. If each search required a full index load to memory, then it would be impossible to get less than 1 second search times.
