# rubicx

A generalized (on-dev) building block for kvs, indexes and spaces (metric space or non metric space).

Currently, it still in a single crates (so sed) and still working in creating abstractions
on the physical level.

- Space >< SegmentPool ~ SegmentFile ~ Grid(lz4) ~ Metric (append & merge & minify & compress)
- SegmentPool::minify() -> for all x, x < SegmentFile, do x::minify
- Segment::signal(Minify)
- Graveyard ~ Tombstone >< SegmentPool ~ SegmentFile ~ Sequence(lz4)
- Index ~ Cluster ~ SegmentPool ~ SegmentFile ~ Grid(lz4) ~ (append & merge & minify & compress)

Note: if you interested in the project, please contact me :).

# Links


# References:

(man, I'm too lazy to put most references in here since it's a lot... =__=)


## I/O

## Embedded Storage Impl

- [tiledb](https://github.com/TileDB-Inc/TileDB)


## Space

- [nmslib](https://github.com/nmslib/nmslib)
- [GNAT](www.vldb.org/conf/1995/P574.PDF)
- [Presentation - Similarity Search in Metric Space](http://www.nmis.isti.cnr.it/amato/similarity-search-book/SAC-07-tutorial.pdf)
