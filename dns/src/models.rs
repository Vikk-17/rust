enum OpCode {
    QUERY,
    IQUERY,
    STATUS,
    UNKNOWN(u8),
}

enum ResponseCode {
    NOERR,
    FORMERR,
    SERVFAIL,
    NXERR,
    NOTIMP,
    REFUSED,
}

pub struct DnsHeader {
    pub id: u16,
    pub qr: bool,
    pub opcode: OpCode,
    pub authoritativie_answer: bool,
    pub truncated_message: bool,
    pub recursion_desired: bool,
    pub recursion_available: bool,
    pub z: u8,
    pub response_code: ResponseCode,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}
