
use mpegts::table_id::*;

pub fn get_table_id(table_id: u8) -> TableId {
  match table_id {
    0x00 => TableId::ProgramAssociation,
    0x01 => TableId::ConditionalAccess,
    0x02 => TableId::ProgramMap,
    0x03 => TableId::TransportStreamDescription,
    0x04 => TableId::IsoIec_14496_SceneDescription,
    0x05 => TableId::IsoIec_14496_ObjectDescription,
    0x06 => TableId::Metadata,
    0x07 => TableId::IsoIec_13818_11_IpmpControlInformation,

    0x3a => TableId::IsoIec_13818_6_DsmCc_MultiprotocolEncapsulated,
    0x3b => TableId::IsoIec_13818_6_DsmCc_UNMessages,
    0x3c => TableId::IsoIec_13818_6_DsmCc_DownloadDataMessages,
    0x3d => TableId::IsoIec_13818_6_DsmCc_StreamDescriptorList,
    0x3e => TableId::IsoIec_13818_6_DsmCc_PrivatelyDefined,
    0x3f => TableId::IsoIec_13818_6_DsmCc_Addressable,
    0xff => TableId::Forbidden,
       _ => {
      if (table_id >= 0x08) && (table_id <= 0x39) {
        TableId::Reserved
      } else {
        TableId::Other
      }
    }
  }
}
