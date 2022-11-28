use mpegts::stream_id::*;

pub fn get_stream_id(stream_id: StreamId) -> u8 {
    match stream_id {
        StreamId::Reserved => 0x00,
        StreamId::IsoIec_11172_2_Mpeg1_Video => 0x01,
        StreamId::IsoIec_13818_2_Mpeg2_Video => 0x02,
        StreamId::IsoIec_11172_3_Mpeg1_Audio => 0x03,
        StreamId::IsoIec_13818_3_Mpeg2_Audio => 0x04,
        StreamId::IsoIec_13818_1_PrivateSection => 0x05,
        StreamId::IsoIec_13818_1_Pes => 0x06,
        StreamId::IsoIec_13522_Mheg => 0x07,
        StreamId::Itu_T_H222_0_Annex_A_Dsm_Cc => 0x08,
        StreamId::Itu_T_H222_1 => 0x09,
        StreamId::IsoIec_13818_6_Dsm_Cc_Type_A => 0x0a,
        StreamId::IsoIec_13818_6_Dsm_Cc_Type_B => 0x0b,
        StreamId::IsoIec_13818_6_Dsm_Cc_Type_C => 0x0c,
        StreamId::IsoIec_13818_6_Dsm_Cc_Type_D => 0x0d,
        StreamId::IsoIec_13818_1_Auxiliary => 0x0e,
        StreamId::IsoIec_13818_7_AAC_Audio => 0x0f,
        StreamId::IsoIec_14496_2_Mpeg4_Video => 0x10,
        StreamId::IsoIec_14496_3_AAC_Latm_Audio => 0x11,
        StreamId::Itu_T_H264_Video => 0x1b,
        StreamId::Itu_T_H265_Video => 0x24,
        StreamId::Vc1_Video => 0xea,
        StreamId::Dirac_Video => 0xd1,
        StreamId::Ac3_Audio => 0x81,
        StreamId::Dts_Audio => 0x8a,
        StreamId::NonMpegAudioSubpictures => 0xbd,
        StreamId::PaddingStream => 0xbe,
        StreamId::NavigationData => 0xbf,
        StreamId::VideoStream { id } => id,
        StreamId::AudioStream { id } => id,
        _ => unimplemented!(), // {
                               //   if (stream_id >= 0xc0) && (stream_id <= 0xdf) {
                               //     StreamId::AudioStream
                               //   } else {
                               //     if (stream_id >= 0xe0) && (stream_id <= 0xef) {
                               //       StreamId::VideoStream
                               //     } else {
                               //       println!("Unknown Stream ID {:?}", stream_id);
                               //       StreamId::Unknown
                               //     }
                               //   }
                               // }
    }
}
