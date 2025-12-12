use asn1rs::prelude::*;

pub mod telco_sample_asn1rs {
    include!(concat!(env!("OUT_DIR"), "/telecom_protocol.rs"));
}

pub fn build_telco_sample_asn1rs() -> telco_sample_asn1rs::Pdu {
    use crate::asn1rs_telco_helpers::telco_sample_asn1rs::*;

    // Create 20 records with varied data to increase message size
    let records: Vec<Record> = (0..20)
        .map(|i| {
            let attributes = if i % 3 == 0 {
                Some(Attributes {
                    flag1: i % 2 == 0,
                    flag2: true,
                    counter: (i * 10) as u8,
                    qualifier: ((i * 50) % 1024) as u16,
                })
            } else {
                None
            };

            let metadata = if i % 2 == 0 {
                Some(vec![
                    0xAB, 0xCD, 0xEF, i as u8, 0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC,
                ])
            } else {
                None
            };

            Record {
                record_id: i as u16,
                record_type: match i % 4 {
                    0 => RecordType::Data,
                    1 => RecordType::Control,
                    2 => RecordType::Status,
                    _ => RecordType::Config,
                },
                value: (i * 1000 + 12345) as u32,
                attributes,
                metadata,
            }
        })
        .collect();

    // Create a Request message with moderate complexity
    let request = Request {
        request_id: b"REQ-2024-12345-ABCDEF-001".to_vec(),
        priority: PriorityLevel::High,
        timestamp: Some(1702368000),
        source_address: Some(b"sip:user1234@10.20.30.40:5060".to_vec()),
        destination_address: b"sip:service@10.20.30.50:5060".to_vec(),
        payload: vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B,
            0x1C, 0x1D,
        ],
        records,
    };

    Pdu::Request(request)
}

pub fn encode_telco_sample_asn1rs(pdu: &telco_sample_asn1rs::Pdu) -> Vec<u8> {
    let mut writer = UperWriter::default();
    writer.write(pdu).unwrap();
    writer.into_bytes_vec()
}

pub fn decode_telco_sample_asn1rs(encoded: &[u8]) -> telco_sample_asn1rs::Pdu {
    let mut reader = UperReader::from(encoded);
    reader.read::<telco_sample_asn1rs::Pdu>().unwrap()
}
