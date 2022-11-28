use mpegts::program_descriptor::*;

pub fn get_descriptor_id(descriptor_id: u8) -> ProgramDescriptor {
    match descriptor_id {
        00 => ProgramDescriptor::Reserved,
        01 => ProgramDescriptor::Forbidden,
        02 => ProgramDescriptor::Video_Stream,
        03 => ProgramDescriptor::Audio_Stream,
        04 => ProgramDescriptor::Hierarchy,
        05 => ProgramDescriptor::Registration,
        06 => ProgramDescriptor::Data_Stream_Alignment,
        07 => ProgramDescriptor::Target_Background_Grid,
        08 => ProgramDescriptor::Video_Window,
        09 => ProgramDescriptor::CA_Descriptor,
        10 => ProgramDescriptor::ISO_639_Language,
        11 => ProgramDescriptor::System_Clock,
        12 => ProgramDescriptor::Multiplex_Buffer_Utilization,
        13 => ProgramDescriptor::Copyright,
        14 => ProgramDescriptor::Maximum_Bitrate,
        15 => ProgramDescriptor::Private_Data_Indicator,
        16 => ProgramDescriptor::Smoothing_Buffer,
        17 => ProgramDescriptor::STD,
        18 => ProgramDescriptor::IBP,

        27 => ProgramDescriptor::MPEG4_Video,
        28 => ProgramDescriptor::MPEG4_Audio,
        29 => ProgramDescriptor::IOD,
        30 => ProgramDescriptor::SL,
        31 => ProgramDescriptor::FMC,
        32 => ProgramDescriptor::External_ES_ID,
        33 => ProgramDescriptor::MuxCode,
        34 => ProgramDescriptor::FmxBufferSize,
        35 => ProgramDescriptor::MultiplexBuffer,
        36 => ProgramDescriptor::Content_Labeling,
        37 => ProgramDescriptor::Metadata_Pointer,
        38 => ProgramDescriptor::Metadata,
        39 => ProgramDescriptor::Metadata_STD,
        40 => ProgramDescriptor::AVC_Video,
        41 => ProgramDescriptor::IPMP,
        42 => ProgramDescriptor::AVC_Timing_And_HRD,
        43 => ProgramDescriptor::MPEG2_AAC_Audio,
        44 => ProgramDescriptor::FlexMuxTiming,
        45 => ProgramDescriptor::MPEG4_Text,
        46 => ProgramDescriptor::MPEG4_Audio_Extension,
        47 => ProgramDescriptor::Auxiliary_Video_Stream,
        48 => ProgramDescriptor::SVC_Extension,
        49 => ProgramDescriptor::MVC_Extension,
        50 => ProgramDescriptor::J2K_Video,
        51 => ProgramDescriptor::MVC_Operation_Point,
        52 => ProgramDescriptor::MPEG2_Stereoscopic_Video_Format,
        53 => ProgramDescriptor::Stereoscopic_Program_Info,
        54 => ProgramDescriptor::Stereoscopic_Video_Info,
        55 => ProgramDescriptor::Transport_Profile,
        56 => ProgramDescriptor::HEVC_Video,
        63 => ProgramDescriptor::Extension,
        _ => {
            if descriptor_id >= 64 {
                return ProgramDescriptor::UserPrivate;
            }

            if descriptor_id >= 57 {
                return ProgramDescriptor::Reserved;
            }
            unimplemented!();
        }
    }
}
