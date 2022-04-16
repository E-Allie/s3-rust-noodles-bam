//We Reiterate the generated types in order to add new Serde Tags
//In the future, it might be beneficial to add the tags during the build-phase in place.

//We EXPLICITLY specify serde to avoid scope errors/prosts serde impls.

//Some structs lack serde-derives, to work around generated HashMaps/unordered structures
//Which violate the trait bounds of serialize and deserialize

//TODO: Properly utilize type_attributes in the build.rs to impl serde.

/// A single CIGAR operation.
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct CigarUnit {
    #[prost(enumeration="cigar_unit::Operation", tag="1")]
    pub operation: i32,
    /// The number of genomic bases that the operation runs for. Required.
    #[prost(int64, tag="2")]
    pub operation_length: i64,
    /// `referenceSequence` is only used at mismatches
    /// (`SEQUENCE_MISMATCH`) and deletions (`DELETE`).
    /// Filling this field replaces SAM's MD tag. If the relevant information is
    /// not available, this field is unset.
    #[prost(string, tag="3")]
    pub reference_sequence: ::prost::alloc::string::String,
}
/// Nested message and enum types in `CigarUnit`.
pub mod cigar_unit {
    /// Describes the different types of CIGAR alignment operations that exist.
    /// Used wherever CIGAR alignments are used.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,  serde::Serialize, serde::Deserialize)]
    #[repr(i32)]
    pub enum Operation {
        Unspecified = 0,
        /// An alignment match indicates that a sequence can be aligned to the
        /// reference without evidence of an INDEL. Unlike the
        /// `SEQUENCE_MATCH` and `SEQUENCE_MISMATCH` operators,
        /// the `ALIGNMENT_MATCH` operator does not indicate whether the
        /// reference and read sequences are an exact match. This operator is
        /// equivalent to SAM's `M`.
        AlignmentMatch = 1,
        /// The insert operator indicates that the read contains evidence of bases
        /// being inserted into the reference. This operator is equivalent to SAM's
        /// `I`.
        Insert = 2,
        /// The delete operator indicates that the read contains evidence of bases
        /// being deleted from the reference. This operator is equivalent to SAM's
        /// `D`.
        Delete = 3,
        /// The skip operator indicates that this read skips a long segment of the
        /// reference, but the bases have not been deleted. This operator is commonly
        /// used when working with RNA-seq data, where reads may skip long segments
        /// of the reference between exons. This operator is equivalent to SAM's
        /// `N`.
        Skip = 4,
        /// The soft clip operator indicates that bases at the start/end of a read
        /// have not been considered during alignment. This may occur if the majority
        /// of a read maps, except for low quality bases at the start/end of a read.
        /// This operator is equivalent to SAM's `S`. Bases that are soft
        /// clipped will still be stored in the read.
        ClipSoft = 5,
        /// The hard clip operator indicates that bases at the start/end of a read
        /// have been omitted from this alignment. This may occur if this linear
        /// alignment is part of a chimeric alignment, or if the read has been
        /// trimmed (for example, during error correction or to trim poly-A tails for
        /// RNA-seq). This operator is equivalent to SAM's `H`.
        ClipHard = 6,
        /// The pad operator indicates that there is padding in an alignment. This
        /// operator is equivalent to SAM's `P`.
        Pad = 7,
        /// This operator indicates that this portion of the aligned sequence exactly
        /// matches the reference. This operator is equivalent to SAM's `=`.
        SequenceMatch = 8,
        /// This operator indicates that this portion of the aligned sequence is an
        /// alignment match to the reference, but a sequence mismatch. This can
        /// indicate a SNP or a read error. This operator is equivalent to SAM's
        /// `X`.
        SequenceMismatch = 9,
    }
}
/// An abstraction for referring to a genomic position, in relation to some
/// already known reference. For now, represents a genomic position as a
/// reference name, a base number on that reference (0-based), and a
/// determination of forward or reverse strand.
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct Position {
    /// The name of the reference in whatever reference set is being used.
    #[prost(string, tag="1")]
    pub reference_name: ::prost::alloc::string::String,
    /// The 0-based offset from the start of the forward strand for that reference.
    #[prost(int64, tag="2")]
    pub position: i64,
    /// Whether this position is on the reverse strand, as opposed to the forward
    /// strand.
    #[prost(bool, tag="3")]
    pub reverse_strand: bool,
}
/// A 0-based half-open genomic coordinate range for search requests.
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct Range {
    /// The reference sequence name, for example `chr1`,
    /// `1`, or `chrX`.
    #[prost(string, tag="1")]
    pub reference_name: ::prost::alloc::string::String,
    /// The start position of the range on the reference, 0-based inclusive.
    #[prost(int64, tag="2")]
    pub start: i64,
    /// The end position of the range on the reference, 0-based exclusive.
    #[prost(int64, tag="3")]
    pub end: i64,
}
// This file contains information about a reference genome assembly. It
// currently only defines a message to represent a single contig, but can be
// extended to capture information about an entire assembly as well (e.g.
// all contigs present on an assembly, the species, build, etc.).

/// This record type records information about a contig. This is used both in
/// VCF header parsing and by GenomeReference objects for querying references.
/// Due to its generality, this message is also used by the FastaReader to
/// provide detailed information on the description line of a FASTA record even
/// in cases where the record does not correspond to a reference genome contig.
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct ContigInfo {
    /// Required. The name of the contig. Canonically this is the first
    /// non-whitespace-containing string after the > marker in a FASTA file.
    /// For example, the line:
    ///      >chr1 more info here
    /// has a name of "chr1" and a description of "more info here"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Ideally this record is filled in as described above, but not all FASTA
    /// readers capture the description information after the name. Since a
    /// description is not required by the FASTA spec, we cannot distinguish cases
    /// where a description was not present and where a parser ignored it.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// The length of this contig in basepairs.
    #[prost(int64, tag="3")]
    pub n_bases: i64,
    /// Additional information used when reading and writing VCF headers. An
    /// example map of key-value extra fields would transform an input line
    /// containing 'assembly=B36,taxonomy=x,species="Homo sapiens"' to a map with
    /// "assembly" -> "B36", "taxonomy" -> "x", "species" -> "Homo sapiens". We
    /// never use this information internally, other than reading it in so we can
    /// write the contig out again.
    #[prost(map="string, string", tag="5")]
    pub extra: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// The position of this contig in the src_fasta file. The first contig would
    /// have position 0.
    /// TODO(mdepristo): rename to something more generic.
    #[prost(int32, tag="4")]
    pub pos_in_fasta: i32,
}
/// A full, or partial, sequence of bases from a contig in a reference genome.
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct ReferenceSequence {
    /// The location on the genome this sequence of bases comes from.
    #[prost(message, optional, tag="1")]
    pub region: ::core::option::Option<Range>,
    /// The bases of this part of the reference genome.
    #[prost(string, tag="2")]
    pub bases: ::prost::alloc::string::String,
}
/// `Struct` represents a structured data value, consisting of fields
/// which map to dynamically typed values. In some languages, `Struct`
/// might be supported by a native representation. For example, in
/// scripting languages like JS a struct is represented as an
/// object. The details of that representation are described together
/// with the proto support for the language.
///
/// The JSON representation for `Struct` is JSON object.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Struct {
    /// Unordered map of dynamically typed values.
    #[prost(map="string, message", tag="1")]
    pub fields: ::std::collections::HashMap<::prost::alloc::string::String, Value>,
}
/// `Value` represents a dynamically typed value which can be either
/// null, a number, a string, a boolean, a recursive struct value, or a
/// list of values. A producer of value is expected to set one of that
/// variants, absence of any variant indicates an error.
///
/// The JSON representation for `Value` is JSON value.
/// TODO: Serde work
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    /// The kind of value.
    #[prost(oneof="value::Kind", tags="1, 2, 7, 3, 4, 5, 6")]
    pub kind: ::core::option::Option<value::Kind>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    /// The kind of value.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        /// Represents a null value.
        #[prost(enumeration="super::NullValue", tag="1")]
        NullValue(i32),
        /// Represents a double value.
        #[prost(double, tag="2")]
        NumberValue(f64),
        /// Represents an integer value.
        #[prost(int32, tag="7")]
        IntValue(i32),
        /// Represents a string value.
        #[prost(string, tag="3")]
        StringValue(::prost::alloc::string::String),
        /// Represents a boolean value.
        #[prost(bool, tag="4")]
        BoolValue(bool),
        /// Represents a structured value.
        #[prost(message, tag="5")]
        StructValue(super::Struct),
        /// Represents a repeated `Value`.
        #[prost(message, tag="6")]
        ListValue(super::ListValue),
    }
}
/// `ListValue` is a wrapper around a repeated field of values.
///
/// The JSON representation for `ListValue` is JSON array.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListValue {
    /// Repeated field of dynamically typed values.
    #[prost(message, repeated, tag="1")]
    pub values: ::prost::alloc::vec::Vec<Value>,
}
/// `NullValue` is a singleton enumeration to represent the null value for the
/// `Value` type union.
///
///  The JSON representation for `NullValue` is JSON `null`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum NullValue {
    /// Null value.
    NullValue = 0,
}
/// A linear alignment can be represented by one CIGAR string. Describes the
/// mapped position and local alignment of the read to the reference.
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct LinearAlignment {
    /// The position of this alignment.
    #[prost(message, optional, tag="1")]
    pub position: ::core::option::Option<Position>,
    /// The mapping quality of this alignment. Represents how likely
    /// the read maps to this position as opposed to other locations.
    ///
    /// Specifically, this is -10 log10 Pr(mapping position is wrong), rounded to
    /// the nearest integer.
    #[prost(int32, tag="2")]
    pub mapping_quality: i32,
    /// Represents the local alignment of this sequence (alignment matches, indels,
    /// etc) against the reference.
    #[prost(message, repeated, tag="3")]
    pub cigar: ::prost::alloc::vec::Vec<CigarUnit>,
}
/// A read alignment describes a linear alignment of a string of DNA to a
/// [reference sequence]\[learning.genomics.v1.Reference\], in addition to metadata
/// about the fragment (the molecule of DNA sequenced) and the read (the bases
/// which were read by the sequencer). A read is equivalent to a line in a SAM
/// file. A read belongs to exactly one read group and exactly one
/// [read group set]\[learning.genomics.v1.ReadGroupSet\].
///
/// For more genomics resource definitions, see [Fundamentals of Google
/// Genomics](<https://cloud.google.com/genomics/fundamentals-of-google-genomics>)
///
/// ### Reverse-stranded reads
///
/// Mapped reads (reads having a non-null `alignment`) can be aligned to either
/// the forward or the reverse strand of their associated reference. Strandedness
/// of a mapped read is encoded by `alignment.position.reverseStrand`.
///
/// If we consider the reference to be a forward-stranded coordinate space of
/// `[0, reference.length)` with `0` as the left-most position and
/// `reference.length` as the right-most position, reads are always aligned left
/// to right. That is, `alignment.position.position` always refers to the
/// left-most reference coordinate and `alignment.cigar` describes the alignment
/// of this read to the reference from left to right. All per-base fields such as
/// `alignedSequence` and `alignedQuality` share this same left-to-right
/// orientation; this is true of reads which are aligned to either strand. For
/// reverse-stranded reads, this means that `alignedSequence` is the reverse
/// complement of the bases that were originally reported by the sequencing
/// machine.
///
/// ### Generating a reference-aligned sequence string
///
/// When interacting with mapped reads, it's often useful to produce a string
/// representing the local alignment of the read to reference. The following
/// pseudocode demonstrates one way of doing this:
///
///     out = ""
///     offset = 0
///     for c in read.alignment.cigar {
///       switch c.operation {
///       case "ALIGNMENT_MATCH", "SEQUENCE_MATCH", "SEQUENCE_MISMATCH":
///         out += read.alignedSequence\[offset:offset+c.operationLength\]
///         offset += c.operationLength
///         break
///       case "CLIP_SOFT", "INSERT":
///         offset += c.operationLength
///         break
///       case "PAD":
///         out += repeat("*", c.operationLength)
///         break
///       case "DELETE":
///         out += repeat("-", c.operationLength)
///         break
///       case "SKIP":
///         out += repeat(" ", c.operationLength)
///         break
///       case "CLIP_HARD":
///         break
///       }
///     }
///     return out
///
/// ### Converting to SAM's CIGAR string
///
/// The following pseudocode generates a SAM CIGAR string from the
/// `cigar` field. Note that this is a lossy conversion
/// (`cigar.referenceSequence` is lost).
///
///     cigarMap = {
///       "ALIGNMENT_MATCH": "M",
///       "INSERT": "I",
///       "DELETE": "D",
///       "SKIP": "N",
///       "CLIP_SOFT": "S",
///       "CLIP_HARD": "H",
///       "PAD": "P",
///       "SEQUENCE_MATCH": "=",
///       "SEQUENCE_MISMATCH": "X",
///     }
///     cigarStr = ""
///     for c in read.alignment.cigar {
///       cigarStr += c.operationLength + cigarMap\[c.operation\]
///     }
///     return cigarStr
///
/// (== resource_for v1.reads ==)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Read {
    /// The server-generated read ID, unique across all reads. This is different
    /// from the `fragmentName`.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The ID of the read group this read belongs to. A read belongs to exactly
    /// one read group. This is a server-generated ID which is distinct from SAM's
    /// RG tag (for that value, see
    /// \[ReadGroup.name][learning.genomics.v1.ReadGroup.name\]).
    #[prost(string, tag="2")]
    pub read_group_id: ::prost::alloc::string::String,
    /// The ID of the read group set this read belongs to. A read belongs to
    /// exactly one read group set.
    #[prost(string, tag="3")]
    pub read_group_set_id: ::prost::alloc::string::String,
    /// The fragment name. Equivalent to QNAME (query template name) in SAM.
    #[prost(string, tag="4")]
    pub fragment_name: ::prost::alloc::string::String,
    /// The orientation and the distance between reads from the fragment are
    /// consistent with the sequencing protocol (SAM flag 0x2).
    #[prost(bool, tag="5")]
    pub proper_placement: bool,
    /// The fragment is a PCR or optical duplicate (SAM flag 0x400).
    #[prost(bool, tag="6")]
    pub duplicate_fragment: bool,
    /// The observed length of the fragment, equivalent to TLEN in SAM.
    #[prost(int32, tag="7")]
    pub fragment_length: i32,
    /// The read number in sequencing. 0-based and less than numberReads. This
    /// field replaces SAM flag 0x40 and 0x80.
    #[prost(int32, tag="8")]
    pub read_number: i32,
    /// The number of reads in the fragment (extension to SAM flag 0x1).
    #[prost(int32, tag="9")]
    pub number_reads: i32,
    /// Whether this read did not pass filters, such as platform or vendor quality
    /// controls (SAM flag 0x200).
    #[prost(bool, tag="10")]
    pub failed_vendor_quality_checks: bool,
    /// The linear alignment for this alignment record. This field is null for
    /// unmapped reads.
    #[prost(message, optional, tag="11")]
    pub alignment: ::core::option::Option<LinearAlignment>,
    /// Whether this alignment is secondary. Equivalent to SAM flag 0x100.
    /// A secondary alignment represents an alternative to the primary alignment
    /// for this read. Aligners may return secondary alignments if a read can map
    /// ambiguously to multiple coordinates in the genome. By convention, each read
    /// has one and only one alignment where both `secondaryAlignment`
    /// and `supplementaryAlignment` are false.
    #[prost(bool, tag="12")]
    pub secondary_alignment: bool,
    /// Whether this alignment is supplementary. Equivalent to SAM flag 0x800.
    /// Supplementary alignments are used in the representation of a chimeric
    /// alignment. In a chimeric alignment, a read is split into multiple
    /// linear alignments that map to different reference contigs. The first
    /// linear alignment in the read will be designated as the representative
    /// alignment; the remaining linear alignments will be designated as
    /// supplementary alignments. These alignments may have different mapping
    /// quality scores. In each linear alignment in a chimeric alignment, the read
    /// will be hard clipped. The `alignedSequence` and
    /// `alignedQuality` fields in the alignment record will only
    /// represent the bases for its respective linear alignment.
    #[prost(bool, tag="13")]
    pub supplementary_alignment: bool,
    /// The bases of the read sequence contained in this alignment record,
    /// **without CIGAR operations applied** (equivalent to SEQ in SAM).
    /// `alignedSequence` and `alignedQuality` may be
    /// shorter than the full read sequence and quality. This will occur if the
    /// alignment is part of a chimeric alignment, or if the read was trimmed. When
    /// this occurs, the CIGAR for this read will begin/end with a hard clip
    /// operator that will indicate the length of the excised sequence.
    #[prost(string, tag="14")]
    pub aligned_sequence: ::prost::alloc::string::String,
    /// The quality of the read sequence contained in this alignment record
    /// (equivalent to QUAL in SAM). Optionally can be read from OQ tag. See
    /// `SamReaderOptions` proto for more details.
    /// `alignedSequence` and `alignedQuality` may be shorter than the full read
    /// sequence and quality. This will occur if the alignment is part of a
    /// chimeric alignment, or if the read was trimmed. When this occurs, the CIGAR
    /// for this read will begin/end with a hard clip operator that will indicate
    /// the length of the excised sequence.
    #[prost(int32, repeated, tag="15")]
    pub aligned_quality: ::prost::alloc::vec::Vec<i32>,
    /// The mapping of the primary alignment of the
    /// `(readNumber+1)%numberReads` read in the fragment. It replaces
    /// mate position and mate strand in SAM.
    #[prost(message, optional, tag="16")]
    pub next_mate_position: ::core::option::Option<Position>,
    /// A map of additional read alignment information. This must be of the form
    /// map<string, string[]> (string key mapping to a list of string values).
    #[prost(map="string, message", tag="17")]
    pub info: ::std::collections::HashMap<::prost::alloc::string::String, ListValue>,
}
/// The SamHeader message represents the metadata present in the header of a
/// SAM/BAM file.
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct SamHeader {
    /// The VN field from the HD line.  Empty if not present (valid formats
    /// will match /^\[0-9]+\.[0-9\]+$/).
    #[prost(string, tag="1")]
    pub format_version: ::prost::alloc::string::String,
    #[prost(enumeration="sam_header::SortingOrder", tag="2")]
    pub sorting_order: i32,
    #[prost(enumeration="sam_header::AlignmentGrouping", tag="3")]
    pub alignment_grouping: i32,
    /// @SQ header field in SAM spec.
    /// The order of the contigs defines the sorting order.
    #[prost(message, repeated, tag="4")]
    pub contigs: ::prost::alloc::vec::Vec<ContigInfo>,
    /// @RG header field in SAM spec.
    /// Read groups.
    #[prost(message, repeated, tag="5")]
    pub read_groups: ::prost::alloc::vec::Vec<ReadGroup>,
    /// @PG header field in SAM spec.
    /// A program run to generate the alignment data.
    #[prost(message, repeated, tag="6")]
    pub programs: ::prost::alloc::vec::Vec<Program>,
    /// @CO header field in SAM spec.
    /// One-line text comments.
    #[prost(string, repeated, tag="7")]
    pub comments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `SamHeader`.
pub mod sam_header {
    /// The SO field from the HD line.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration, serde::Serialize, serde::Deserialize)]
    #[repr(i32)]
    pub enum SortingOrder {
        Unknown = 0,
        Unsorted = 1,
        Queryname = 2,
        Coordinate = 3,
    }
    /// The GO field from the HD line.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration, serde::Serialize, serde::Deserialize)]
    #[repr(i32)]
    pub enum AlignmentGrouping {
        None = 0,
        Query = 1,
        Reference = 2,
    }
}
/// A read group is all the data that's processed the same way by the sequencer.
/// This is a sub-message of SamHeader, at the same scope to reduce verbosity.
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct ReadGroup {
    /// RG@ ID field in SAM spec.
    /// The read group name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// RG@ CN field in SAM spec.
    /// The name of the sequencing center producing the read.
    #[prost(string, tag="2")]
    pub sequencing_center: ::prost::alloc::string::String,
    /// @RG DS field in SAM spec.
    /// A free-form text description of this read group.
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    /// @RG DT field in SAM spec.
    #[prost(string, tag="4")]
    pub date: ::prost::alloc::string::String,
    /// @RG FO field in SAM spec.
    #[prost(string, tag="5")]
    pub flow_order: ::prost::alloc::string::String,
    /// @RG KS field in SAM spec.
    #[prost(string, tag="6")]
    pub key_sequence: ::prost::alloc::string::String,
    /// @RG LB field in SAM spec.
    /// A library is a collection of DNA fragments which have been prepared for
    /// sequencing from a sample. This field is important for quality control as
    /// error or bias can be introduced during sample preparation.
    #[prost(string, tag="7")]
    pub library_id: ::prost::alloc::string::String,
    /// @RG PG field in SAM spec.
    #[prost(string, repeated, tag="8")]
    pub program_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// @RG PI field in SAM spec.
    /// The predicted insert size of this read group. The insert size is the length
    /// of the sequenced DNA fragment from end-to-end, not including the adapters.
    #[prost(int32, tag="9")]
    pub predicted_insert_size: i32,
    /// @RG PL field in SAM spec.
    /// The platform/technology used to produce the reads.
    #[prost(string, tag="10")]
    pub platform: ::prost::alloc::string::String,
    /// @RG PM field in SAM spec.
    /// The platform model used as part of this run.
    #[prost(string, tag="11")]
    pub platform_model: ::prost::alloc::string::String,
    /// @RG PU field in SAM spec.
    /// The platform unit used as part of this experiment, for example
    /// flowcell-barcode.lane for Illumina or slide for SOLiD. A unique identifier.
    #[prost(string, tag="12")]
    pub platform_unit: ::prost::alloc::string::String,
    /// @RG SM field in SAM spec.
    /// A client-supplied sample identifier for the reads in this read group.
    #[prost(string, tag="13")]
    pub sample_id: ::prost::alloc::string::String,
}
/// A Program is used in the SAM header to track how alignment data is generated.
/// This is a sub-message of SamHeader, at the same scope to reduce verbosity.
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct Program {
    /// @PG ID field in SAM spec.
    /// The locally unique ID of the program. Used along with
    /// `prev_program_id` to define an ordering between programs.
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
    /// @PG PN field in SAM spec.
    /// The display name of the program. This is typically the colloquial name of
    /// the tool used, for example 'bwa' or 'picard'.
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    /// @PG CL field in SAM spec.
    /// The command line used to run this program.
    #[prost(string, tag="1")]
    pub command_line: ::prost::alloc::string::String,
    /// @PG PP field in SAM spec.
    /// The ID of the program run before this one.
    #[prost(string, tag="4")]
    pub prev_program_id: ::prost::alloc::string::String,
    /// @PG DS field in SAM spec.
    /// The description of the program.
    #[prost(string, tag="6")]
    pub description: ::prost::alloc::string::String,
    /// @PG VN field in SAM spec.
    /// The version of the program run.
    #[prost(string, tag="5")]
    pub version: ::prost::alloc::string::String,
}
///////////////////////////////////////////////////////////////////////////////
// I/O-related messages.
///////////////////////////////////////////////////////////////////////////////

/// The SamReaderOptions message is used to alter the properties of a SamReader.
/// It enables reads to be omitted from parsing based on their attributes, as
/// well as more fine-grained handling of particular fields within the SAM
/// records.
/// Next ID: 12.
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct SamReaderOptions {
    /// Read requirements that must be satisfied before our reader will return
    /// a read to use.
    #[prost(message, optional, tag="1")]
    pub read_requirements: ::core::option::Option<ReadRequirements>,
    #[prost(enumeration="sam_reader_options::AuxFieldHandling", tag="3")]
    pub aux_field_handling: i32,
    /// Block size to use in htslib, in reading the SAM/BAM. Value <=0 will use the
    /// default htslib block size.
    #[prost(int64, tag="4")]
    pub hts_block_size: i64,
    /// Controls if, and at what rate, we discard reads from the input stream.
    ///
    /// This option allows the user to efficiently remove a random fraction of
    /// reads from the source SAM/BAM file. The reads are discarded on the fly
    /// before being parsed into protos, so the downsampling is reasonably
    /// efficient.
    ///
    /// If 0.0 (the default protobuf value), this field is ignored. If != 0.0, then
    /// this must be a value between (0.0, 1.0] indicating the probability p that a
    /// read should be kept, or equivalently (1 - p) that a read will be kept. For
    /// example, if downsample_fraction is 0.25, then each read has a 25% chance of
    /// being included in the output reads.
    #[prost(float, tag="5")]
    pub downsample_fraction: f32,
    /// Random seed to use with downsampling fraction.
    #[prost(int64, tag="6")]
    pub random_seed: i64,
    /// By default aligned_quality field is read from QUAL in SAM. If flag is set,
    /// aligned_quality field is read from OQ tag in SAM.
    #[prost(bool, tag="10")]
    pub use_original_base_quality_scores: bool,
    /// By default, this field is empty. If empty, we keep all aux fields if they
    /// are parsed. If set, we only keep the aux fields with the names in this
    /// list.
    #[prost(string, repeated, tag="11")]
    pub aux_fields_to_keep: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `SamReaderOptions`.
pub mod sam_reader_options {
    /// How should we handle the aux fields in the SAM record?
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration, serde::Serialize, serde::Deserialize)]
    #[repr(i32)]
    pub enum AuxFieldHandling {
        Unspecified = 0,
        SkipAuxFields = 1,
        ParseAllAuxFields = 2,
    }
}
/// Describes requirements for a read for it to be returned by a SamReader.
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct ReadRequirements {
    /// By default, duplicate reads will not be kept. Set this flag to keep them.
    #[prost(bool, tag="1")]
    pub keep_duplicates: bool,
    /// By default, reads that failed the vendor quality checks will not be kept.
    /// Set this flag to keep them.
    #[prost(bool, tag="2")]
    pub keep_failed_vendor_quality_checks: bool,
    /// By default, reads that are marked as secondary alignments will not be kept.
    /// Set this flag to keep them.
    #[prost(bool, tag="3")]
    pub keep_secondary_alignments: bool,
    /// By default, reads that are marked as supplementary alignments will not be
    /// kept. Set this flag to keep them.
    #[prost(bool, tag="4")]
    pub keep_supplementary_alignments: bool,
    /// By default, reads that aren't aligned are not kept. Set this flag to keep
    /// them.
    #[prost(bool, tag="5")]
    pub keep_unaligned: bool,
    /// Paired (or greater) reads that are improperly placed are not kept by
    /// default. Set this flag to keep them. We define improperly placed to mean
    /// reads whose (next) mate is mapped to a different contig.
    #[prost(bool, tag="6")]
    pub keep_improperly_placed: bool,
    /// By default, reads with any mapping quality are kept. Setting this field
    /// to a positive integer i will only keep reads that have a MAPQ >= i. Note
    /// this only applies to aligned reads. If keep_unaligned is set, unaligned
    /// reads, which by definition do not have a mapping quality, will still be
    /// kept.
    #[prost(int32, tag="7")]
    pub min_mapping_quality: i32,
    /// Minimum base quality. This field indicates that we are enforcing a minimum
    /// base quality score for a read to be used. How this field is enforced,
    /// though, depends on the enum field min_base_quality_mode, as there are
    /// multiple ways for this requirement to be interpreted.
    #[prost(int32, tag="8")]
    pub min_base_quality: i32,
    #[prost(enumeration="read_requirements::MinBaseQualityMode", tag="9")]
    pub min_base_quality_mode: i32,
}
/// Nested message and enum types in `ReadRequirements`.
pub mod read_requirements {
    /// How should we enforce the min_base_quality requirement?
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration, serde::Serialize, serde::Deserialize)]
    #[repr(i32)]
    pub enum MinBaseQualityMode {
        /// If UNSPECIFIED, there are no guarantees on whether and how
        /// min_base_quality would be enforced. By default we recommend
        /// implementations ignore min_base_quality if this is set to UNSPECIFIED.
        Unspecified = 0,
        /// The min_base_quality requirement is being enforced not by the reader but
        /// by the client itself. This is commonly used when the algorithm for
        /// computing whether a read satisfying the min_base_quality requirement is
        /// too complex or too specific for the reader.
        EnforcedByClient = 1,
    }
}
