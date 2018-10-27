
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
#[allow(non_camel_case_types)]
pub enum StreamId {
  Reserved,
  IsoIec_11172_2_Mpeg1_Video,
  IsoIec_13818_2_Mpeg2_Video,
  IsoIec_11172_3_Mpeg1_Audio,
  IsoIec_13818_3_Mpeg2_Audio,
  IsoIec_13818_1_PrivateSection,
  IsoIec_13818_1_Pes,
  IsoIec_13522_Mheg,
  Itu_T_H222_0_Annex_A_Dsm_Cc,
  Itu_T_H222_1,
  IsoIec_13818_6_Dsm_Cc_Type_A,
  IsoIec_13818_6_Dsm_Cc_Type_B,
  IsoIec_13818_6_Dsm_Cc_Type_C,
  IsoIec_13818_6_Dsm_Cc_Type_D,
  IsoIec_13818_1_Auxiliary,
  IsoIec_13818_7_AAC_Audio,
  IsoIec_14496_2_Mpeg4_Video,
  IsoIec_14496_3_AAC_Latm_Audio,
  Itu_T_H264_Video,
  Itu_T_H265_Video,
  Vc1_Video,
  Dirac_Video,
  Ac3_Audio,
  Dts_Audio,
  NonMpegAudioSubpictures,
  PaddingStream,
  NavigationData,
  AudioStream{id: u8},
  VideoStream{id: u8},
  Unknown,
}

impl fmt::Display for StreamId {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}
