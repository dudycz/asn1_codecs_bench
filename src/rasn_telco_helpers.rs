use rasn::uper;

pub mod telco_sample_rasn {
    include!(concat!(env!("OUT_DIR"), "/telco_sample_rasn.rs"));
}

pub fn build_telco_sample_rasn() -> telco_sample_rasn::telecom_protocol::PDU {
    use crate::rasn_telco_helpers::telco_sample_rasn::telecom_protocol::*;

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
                Some(
                    vec![
                        0xAB, 0xCD, 0xEF, i as u8, 0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC,
                    ]
                    .into(),
                )
            } else {
                None
            };

            Record {
                record_id: i as u16,
                record_type: match i % 4 {
                    0 => RecordType::data,
                    1 => RecordType::control,
                    2 => RecordType::status,
                    _ => RecordType::config,
                },
                value: (i * 1000 + 12345) as u32,
                attributes,
                metadata,
            }
        })
        .collect();

    // Create a Request message with moderate complexity
    let request = Request {
        request_id: b"REQ-2024-12345-ABCDEF-001".to_vec().into(),
        priority: PriorityLevel::high,
        timestamp: Some(1702368000),
        source_address: Some(b"sip:user1234@10.20.30.40:5060".to_vec().into()),
        destination_address: b"sip:service@10.20.30.50:5060".to_vec().into(),
        payload: vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B,
            0x1C, 0x1D,
        ]
        .into(),
        records,
    };

    PDU::request(request)
}

pub fn encode_telco_sample_rasn(pdu: &telco_sample_rasn::telecom_protocol::PDU) -> Vec<u8> {
    uper::encode(&pdu).unwrap()
}

pub fn decode_telco_sample_rasn(encoded: &[u8]) -> telco_sample_rasn::telecom_protocol::PDU {
    uper::decode::<telco_sample_rasn::telecom_protocol::PDU>(encoded).unwrap()
}
