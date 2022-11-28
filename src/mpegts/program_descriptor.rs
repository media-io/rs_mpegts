use std::fmt;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum ProgramDescriptor {
    Forbidden,
    Video_Stream,
    Audio_Stream,
    Hierarchy,
    Registration,
    Data_Stream_Alignment,
    Target_Background_Grid,
    Video_Window,
    CA_Descriptor,
    ISO_639_Language,
    System_Clock,
    Multiplex_Buffer_Utilization,
    Copyright,
    Maximum_Bitrate,
    Private_Data_Indicator,
    Smoothing_Buffer,
    STD,
    IBP,
    MPEG4_Video,
    MPEG4_Audio,
    IOD,
    SL,
    FMC,
    External_ES_ID,
    MuxCode,
    FmxBufferSize,
    MultiplexBuffer,
    Content_Labeling,
    Metadata_Pointer,
    Metadata,
    Metadata_STD,
    AVC_Video,
    IPMP,
    AVC_Timing_And_HRD,
    MPEG2_AAC_Audio,
    FlexMuxTiming,
    MPEG4_Text,
    MPEG4_Audio_Extension,
    Auxiliary_Video_Stream,
    SVC_Extension,
    MVC_Extension,
    J2K_Video,
    MVC_Operation_Point,
    MPEG2_Stereoscopic_Video_Format,
    Stereoscopic_Program_Info,
    Stereoscopic_Video_Info,
    Transport_Profile,
    HEVC_Video,
    Extension,
    Reserved,
    UserPrivate,
}

impl fmt::Display for ProgramDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
