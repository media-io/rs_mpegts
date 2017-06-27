
use mpegts::table_id::*;

pub fn get_table_id(table_id: TableId) -> u8 {
  match table_id {
    TableId::ProgramAssociation => 0x00,
    TableId::ConditionalAccess => 0x01,
    TableId::ProgramMap => 0x02,
    TableId::TransportStreamDescription => 0x03,
    TableId::IsoIec14496SceneDescription => 0x04,
    TableId::IsoIec14496ObjectDescription => 0x05,
    TableId::Metadata => 0x06,
    TableId::IsoIec1381811IpmpControlInformation => 0x07,

    TableId::IsoIec138186DsmCcMultiprotocolEncapsulated => 0x3a,
    TableId::IsoIec138186DsmCcUNMessages => 0x3b,
    TableId::IsoIec138186DsmCcDownloadDataMessages => 0x3c,
    TableId::IsoIec138186DsmCcStreamDescriptorList => 0x3d,
    TableId::IsoIec138186DsmCcPrivatelyDefined => 0x3e,
    TableId::IsoIec138186DsmCcAddressable => 0x3f,
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
