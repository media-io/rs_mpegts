
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum StreamId {
  Reserved,
  IsoIec111722Mpeg1Video,
  IsoIec138182Mpeg2Video,
  IsoIec111723Mpeg1Audio,
  IsoIec138183Mpeg2Audio,
  IsoIec138181PrivateSection,
  IsoIec138181Pes,
  IsoIec13522Mheg,
  ItuTH2220AnnexADsmCc,
  ItuTH2221,
  IsoIec138186DsmCcTypeA,
  IsoIec138186DsmCcTypeB,
  IsoIec138186DsmCcTypeC,
  IsoIec138186DsmCcTypeD,
  IsoIec138181Auxiliary,
  IsoIec138187AacAudio,
  IsoIec144962Mpeg4Video,
  IsoIec144963AacLatmAudio,
  ItuTH264Video,
  ItuTH265Video,
  Vc1Video,
  DiracVideo,
  Ac3Audio,
  DtsAudio,
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
