
use mpegts::table_id::*;

pub fn get_table_id(table_id: TableId) -> u8 {
  match table_id {
    TableId::ProgramAssociation => 0x00,
    TableId::ConditionalAccess => 0x01,
    TableId::ProgramMap => 0x02,
    TableId::TransportStreamDescription => 0x03,
    TableId::IsoIec_14496_SceneDescription => 0x04,
    TableId::IsoIec_14496_ObjectDescription => 0x05,
    TableId::Metadata => 0x06,
    TableId::IsoIec_13818_11_IpmpControlInformation => 0x07,

    TableId::IsoIec_13818_6_DsmCc_MultiprotocolEncapsulated => 0x3a,
    TableId::IsoIec_13818_6_DsmCc_UNMessages => 0x3b,
    TableId::IsoIec_13818_6_DsmCc_DownloadDataMessages => 0x3c,
    TableId::IsoIec_13818_6_DsmCc_StreamDescriptorList => 0x3d,
    TableId::IsoIec_13818_6_DsmCc_PrivatelyDefined => 0x3e,
    TableId::IsoIec_13818_6_DsmCc_Addressable => 0x3f,
    TableId::Forbidden => 0xff,
    _ => unimplemented!()
    //    _ => {
    //   if (table_id >= 0x08) && (table_id <= 0x39) {
    //     TableId::Reserved
    //   } else {
    //     TableId::Other
    //   }
    // }
  }
}
