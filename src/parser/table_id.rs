
use mpegts::table_id::*;

pub fn get_table_id(table_id: u8) -> TableId {
  match table_id {
    0x00 => TableId::ProgramAssociation,
    0x01 => TableId::ConditionalAccess,
    0x02 => TableId::ProgramMap,
    0x03 => TableId::TransportStreamDescription,
    0x04 => TableId::IsoIec14496SceneDescription,
    0x05 => TableId::IsoIec14496ObjectDescription,
    0x06 => TableId::Metadata,
    0x07 => TableId::IsoIec1381811IpmpControlInformation,

    0x3a => TableId::IsoIec138186DsmCcMultiprotocolEncapsulated,
    0x3b => TableId::IsoIec138186DsmCcUNMessages,
    0x3c => TableId::IsoIec138186DsmCcDownloadDataMessages,
    0x3d => TableId::IsoIec138186DsmCcStreamDescriptorList,
    0x3e => TableId::IsoIec138186DsmCcPrivatelyDefined,
    0x3f => TableId::IsoIec138186DsmCcAddressable,
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
