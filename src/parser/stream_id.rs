
use mpegts::stream_id::*;

pub fn get_stream_id(stream_id: u8) -> StreamId {
  match stream_id {
    0x00 => StreamId::Reserved,
    0x01 => StreamId::IsoIec_11172_2_Mpeg1_Video,
    0x02 => StreamId::IsoIec_13818_2_Mpeg2_Video,
    0x03 => StreamId::IsoIec_11172_3_Mpeg1_Audio,
    0x04 => StreamId::IsoIec_13818_3_Mpeg2_Audio,
    0x05 => StreamId::IsoIec_13818_1_PrivateSection,
    0x06 => StreamId::IsoIec_13818_1_Pes,
    0x07 => StreamId::IsoIec_13522_Mheg,
    0x08 => StreamId::Itu_T_H222_0_Annex_A_Dsm_Cc,
    0x09 => StreamId::Itu_T_H222_1,
    0x0a => StreamId::IsoIec_13818_6_Dsm_Cc_Type_A,
    0x0b => StreamId::IsoIec_13818_6_Dsm_Cc_Type_B,
    0x0c => StreamId::IsoIec_13818_6_Dsm_Cc_Type_C,
    0x0d => StreamId::IsoIec_13818_6_Dsm_Cc_Type_D,
    0x0e => StreamId::IsoIec_13818_1_Auxiliary,
    0x0f => StreamId::IsoIec_13818_7_AAC_Audio,
    0x10 => StreamId::IsoIec_14496_2_Mpeg4_Video,
    0x11 => StreamId::IsoIec_14496_3_AAC_Latm_Audio,
    0x1b => StreamId::Itu_T_H264_Video,
    0x24 => StreamId::Itu_T_H265_Video,
    0xea => StreamId::Vc1_Video,
    0xd1 => StreamId::Dirac_Video,
    0x81 => StreamId::Ac3_Audio,
    0x8a => StreamId::Dts_Audio,
    0xbd => StreamId::NonMpegAudioSubpictures,
    0xbe => StreamId::PaddingStream,
    0xbf => StreamId::NavigationData,
       _ => {
      if (0xc0..=0xdf).contains(&stream_id) {
        StreamId::AudioStream{id: stream_id}
      } else if (0xe0..=0xef).contains(&stream_id) {
        StreamId::VideoStream{id: stream_id}
      } else {
        println!("Unknown Stream ID {:?}", stream_id);
        StreamId::Unknown
      }
    }
  }
}
