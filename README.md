## Parsing a BAM header with Noodles

This fork takes first steps towards deconstructing the BAM file type into a more universally utilizable format.

Temporarily, and easily reversibly, AWS has been gutted in favor of local build tests.

Currently, [ProtoBufs from Google's nucleus](https://github.com/google/nucleus/tree/v0.6.0/nucleus/protos) are utilized to generate SamHeader structures.

A test bam file is included, [sourced from tiny-test-data](https://github.com/brainstorm/tiny-test-data), however, a more comprehensive test structure is forthcoming.