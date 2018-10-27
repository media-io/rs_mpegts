
use bitstream_io::{BigEndian, BitReader};
use mpegts::descriptor::hevc::*;

pub fn parse_descriptor(mut stream: &mut BitReader<BigEndian>) -> Hevc {
  let _descriptor_id = stream.read::<u8>(8).unwrap();
  let descriptor_length = stream.read::<u8>(8).unwrap();
  let profile_space = stream.read::<u8>(2).unwrap();
  let tier_flag = stream.read_bit().unwrap();
  let profile_idc = stream.read::<u8>(5).unwrap();
  let profile_compatibility_indication = stream.read::<u32>(32).unwrap();
  let progressive_source_flag = stream.read_bit().unwrap();
  let interlaced_source_flag = stream.read_bit().unwrap();
  let non_packed_constraint_flag = stream.read_bit().unwrap();
  let frame_only_constraint_flag = stream.read_bit().unwrap();
  let profile_idc_description = stream.read::<u64>(44).unwrap();
  let level_idc = stream.read::<u8>(8).unwrap();
  let temporal_layer_subset_flag = stream.read_bit().unwrap();
  let hevc_still_present_flag = stream.read_bit().unwrap();
  let hevc_24hr_picture_present_flag = stream.read_bit().unwrap();
  let reserved = stream.read::<u8>(5).unwrap();

  println!("HEVC descriptor");
  println!("profile_space {:?}", profile_space);
  println!("tier_flag {:?}", tier_flag);
  println!("profile_idc {:?}", profile_idc);
  println!("profile_compatibility_indication {:b}", profile_compatibility_indication);
  println!("progressive_source_flag {:?}", progressive_source_flag);
  println!("interlaced_source_flag {:?}", interlaced_source_flag);
  println!("non_packed_constraint_flag {:?}", non_packed_constraint_flag);
  println!("frame_only_constraint_flag {:?}", frame_only_constraint_flag);
  println!("temporal_layer_subset_flag {:?}", temporal_layer_subset_flag);

  if temporal_layer_subset_flag {
    let _reserved = stream.read::<u8>(5).unwrap();
    let temporal_id_min = stream.read::<u8>(3).unwrap();
    let _reserved = stream.read::<u8>(5).unwrap();
    let temporal_id_max = stream.read::<u8>(3).unwrap();

    println!("temporal_id_min {:?}", temporal_id_min);
    println!("temporal_id_max {:?}", temporal_id_max);
  }

  Hevc{
    profile_space: profile_space,
    // tier_flag: tier_flag,
    // profile_idc: profile_idc,
    // profile_compatibility_indication: profile_compatibility_indication,
    // progressive_source_flag: progressive_source_flag,
    // interlaced_source_flag: interlaced_source_flag,
    // non_packed_constraint_flag: non_packed_constraint_flag,
    // frame_only_constraint_flag: frame_only_constraint_flag,
    // profile_idc_description: profile_idc_description,
    // level_idc: level_idc,
    // temporal_layer_subset_flag: temporal_layer_subset_flag,
    // hevc_still_present_flag: hevc_still_present_flag,
    // hevc_24hr_picture_present_flag: hevc_24hr_picture_present_flag,
  }
}