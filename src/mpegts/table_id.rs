
#[derive(Debug, Clone)]
pub enum TableId {
  ProgramAssociation,
  ConditionalAccess,
  ProgramMap,
  TransportStreamDescription,
  IsoIec14496SceneDescription,
  IsoIec14496ObjectDescription,
  Metadata,
  IsoIec1381811IpmpControlInformation,
  IsoIec138186DsmCcMultiprotocolEncapsulated,
  IsoIec138186DsmCcUNMessages,
  IsoIec138186DsmCcDownloadDataMessages,
  IsoIec138186DsmCcStreamDescriptorList,
  IsoIec138186DsmCcPrivatelyDefined,
  IsoIec138186DsmCcAddressable,
  Other,
  Reserved,
  Forbidden,
}
