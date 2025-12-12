use asn1_codecs::PerCodecData;
use asn1_codecs::uper::UperCodec;

pub mod telco_sample_hampi {
    include!(concat!(env!("OUT_DIR"), "/telco_sample_hampi.rs"));
}

pub fn build_telco_sample_hampi() -> telco_sample_hampi::PDU {
    use crate::hampi_telco_helpers::telco_sample_hampi::*;

    // Create 20 records with varied data to increase message size
    let records: Vec<Record> = (0..20)
        .map(|i| {
            let attributes = if i % 3 == 0 {
                Some(Attributes {
                    flag1: AttributesFlag1(i % 2 == 0),
                    flag2: AttributesFlag2(true),
                    counter: AttributesCounter((i * 10) as u8),
                    qualifier: AttributesQualifier(((i * 50) % 1024) as u16),
                })
            } else {
                None
            };

            let metadata = if i % 2 == 0 {
                Some(RecordMetadata(vec![
                    0xAB, 0xCD, 0xEF, i as u8, 0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC,
                ]))
            } else {
                None
            };

            Record {
                record_id: RecordRecordId(i as u16),
                record_type: RecordType(match i % 4 {
                    0 => RecordType::DATA,
                    1 => RecordType::CONTROL,
                    2 => RecordType::STATUS,
                    _ => RecordType::CONFIG,
                }),
                value: RecordValue((i * 1000 + 12345) as u32),
                attributes,
                metadata,
            }
        })
        .collect();

    // Create a Request message with moderate complexity
    let request = Request {
        request_id: RequestRequestId(b"REQ-2024-12345-ABCDEF-001".to_vec()),
        priority: PriorityLevel(PriorityLevel::HIGH),
        timestamp: Some(RequestTimestamp(1702368000)),
        source_address: Some(RequestSourceAddress(
            b"sip:user1234@10.20.30.40:5060".to_vec(),
        )),
        destination_address: RequestDestinationAddress(b"sip:service@10.20.30.50:5060".to_vec()),
        payload: RequestPayload(vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B,
            0x1C, 0x1D,
        ]),
        records: RequestRecords(records),
    };

    PDU::Request(request)
}

pub fn encode_telco_sample_hampi(pdu: &telco_sample_hampi::PDU) -> Vec<u8> {
    let mut data = PerCodecData::new_uper();
    pdu.uper_encode(&mut data).unwrap();
    data.into_bytes()
}

pub fn decode_telco_sample_hampi(encoded: &[u8]) -> telco_sample_hampi::PDU {
    let mut d = PerCodecData::from_slice_uper(encoded);
    telco_sample_hampi::PDU::uper_decode(&mut d).unwrap()
}
