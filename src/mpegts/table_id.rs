
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum TableId {
  ProgramAssociation,
  ConditionalAccess,
  ProgramMap,
  TransportStreamDescription,
  IsoIec_14496_SceneDescription,
  IsoIec_14496_ObjectDescription,
  Metadata,
  IsoIec_13818_11_IpmpControlInformation,
  IsoIec_13818_6_DsmCc_MultiprotocolEncapsulated,
  IsoIec_13818_6_DsmCc_UNMessages,
  IsoIec_13818_6_DsmCc_DownloadDataMessages,
  IsoIec_13818_6_DsmCc_StreamDescriptorList,
  IsoIec_13818_6_DsmCc_PrivatelyDefined,
  IsoIec_13818_6_DsmCc_Addressable,
  Other,
  Reserved,
  Forbidden,
}
