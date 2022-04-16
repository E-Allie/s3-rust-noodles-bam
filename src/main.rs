mod proto_types;

use crate::proto_types::*;

extern crate jemallocator;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use bytes::Bytes;
use std::io::Cursor;
use serde_json::{json, Value};


use noodles::bam;
use noodles::sam;
use noodles::sam::header;
use noodles::sam::header::header::SortOrder;
use noodles::sam::header::Programs;
use crate::header::header::GroupOrder;
use crate::sam::header::ParseError;

//A Local Version of the example file, simply so I do not accidentally incur S3 overtures

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let func = bam_header_as_json().await?;
    println!("{}",func);
    Ok(())
}

/// Turns header string into a JSON
async fn bam_header_as_json() -> std::io::Result<Value> {

    //TODO: Harvest more test Bam files.

    let bam_file= Bytes::from(std::fs::read("mt.bam")?);
    let header = read_bam_header(bam_file).await.unwrap();

    let final_header = create_sam_header(header);


    Ok(json!({ "message": format!("{:?}", final_header) }))
}

/// Reads BAM object header
async fn read_bam_header(bam_bytes: Bytes) -> Result<sam::Header, ParseError> {
    let mut obj_buffer = Cursor::new(bam_bytes.to_vec());
    // Rewind buffer Cursor after writing, so that next reader can consume header data...
    obj_buffer.set_position(0);

    // ... and read the heade
    let mut reader = bam::Reader::new(obj_buffer);
    reader.read_header().unwrap().parse()//?.parse::<sam::Header>()

}

//Given an input noodles::sam header, try to construct a more parsable protobufs SamHeader
pub fn create_sam_header(header: header::Header) -> SamHeader {
    let def_h_h = header::header::Header::default();

    //A noodles::header does have its own header, or a header header.
    let header_header = header.header().unwrap_or(&def_h_h);

    let sam_header = proto_types::SamHeader {
        format_version : header_header.version().to_string(),
        sorting_order: match header_header.sort_order() {
            None => 0,
            Some(sorted_order) => match sorted_order {
                SortOrder::Unknown => 0,
                SortOrder::Unsorted => 1,
                SortOrder::QueryName => 2,
                SortOrder::Coordinate => 3
            }
        },
        alignment_grouping: match header_header.group_order() {
            None => 0,
            Some(grouped_order) => match grouped_order {
                GroupOrder::None => 0,
                GroupOrder::Query => 1,
                GroupOrder::Reference => 2
            }
        },
        contigs: vec![],
        read_groups: create_read_group(header.read_groups()),
        programs: create_programs(header.programs()),
        comments: Vec::from(header.comments())
    };
    sam_header
}

//A noodles_sam Programs is of type IndexMap<String, noodles::Program>
//This converts it to a vector of protobuf Programs

pub fn create_programs(n_programs: &Programs) -> Vec<proto_types::Program> {
    let mut sam_programs: Vec<proto_types::Program> = Vec::with_capacity(n_programs.len());

    n_programs
        .iter()
        //Where x is (String, Program)
        .for_each(|x| sam_programs.push(proto_types::Program {
            id: x.1.id().to_string(),
            name: x.1.name().unwrap_or("").to_string(),
            command_line: x.1.command_line().unwrap_or("").to_string(),
            prev_program_id: x.1.previous_id().unwrap_or("").to_string(),
            description: x.1.description().unwrap_or("").to_string(),
            version: x.1.version().unwrap_or("").to_string()
        }));

    sam_programs
}


//A noodles_sam read groups is of type IndexMap<String, noodles::::ReaderGroup>
//This converts it to a vector of protobuf ReadGroups
pub fn create_read_group(read_groups: &header::ReadGroups) -> Vec<proto_types::ReadGroup> {
    let mut sam_groups: Vec<proto_types::ReadGroup> = Vec::with_capacity(read_groups.len());

    read_groups
        .iter()
        //Where x is (String, ReaderGroup)
        .for_each(|x| sam_groups.push(proto_types::ReadGroup {
            name : x.0.to_string(),
            sequencing_center: x.1.sequencing_center().unwrap_or("").to_string(),
            description: x.1.description().unwrap_or("").to_string(),
            date: x.1.produced_at().unwrap_or("").to_string(),
            flow_order: x.1.flow_order().unwrap_or("").to_string(),
            key_sequence: x.1.key_sequence().unwrap_or("").to_string(),
            library_id: x.1.library().unwrap_or("").to_string(),

            //TODO, properly vectorize program id's
            program_ids: vec![x.1.program().unwrap_or("").to_string()],

            predicted_insert_size: x.1.predicted_median_insert_size().unwrap_or(0),

            //We break the "pattern" here as platform is it's own type and would not type-check
            platform: match x.1.platform() {
                None => "".to_string(),
                Some(inner) => inner.to_string()
            },

            platform_model: x.1.platform_model().unwrap_or("").to_string(),
            platform_unit: x.1.platform_unit().unwrap_or("").to_string(),
            sample_id: x.1.sample().unwrap_or("").to_string(),
        }) );

    sam_groups
}
