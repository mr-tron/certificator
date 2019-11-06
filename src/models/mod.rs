use pem;
use ring;
use x509_parser;
pub struct X509CertificateMeta {
    pub name: String,
    pub thumbprint: String,
}

pub struct X509CertificateDB {
    pub id: u32,
    pub thumbprint: String,
    pub name: String,
    pub raw: Option<Vec<u8>>,
    pub der: Option<Vec<u8>>,
}

impl X509CertificateDB {
    pub fn from_pem(s: &str) -> X509CertificateDB {
        let b: String = s.into();
        let b1: String = s.into();
        let pem = pem::parse(b).unwrap();

        let (_, cert) = match x509_parser::parse_x509_der(pem.contents.as_ref()) {
            Ok(cert) => cert,
            Err(e) => panic!(e),
        };
        let d = ring::digest::digest(&ring::digest::SHA1_FOR_LEGACY_USE_ONLY, b1.as_bytes());
        let thumbprint = thumprint_repr(d.as_ref());
        let name = format!("{}", cert.tbs_certificate.subject);
        X509CertificateDB {
            raw: Some(b1.into_bytes()),
            thumbprint,
            name,
            der: Some(pem.contents),
            id: 0,
        }
    }
}

// todo: move somewhere else
// todo: add colons between bytes
fn thumprint_repr(slice: &[u8]) -> String {
    let mut buf = "".to_string();
    for b in slice {
        fn hex_from_digit(num: u8) -> char {
            if num < 10 {
                (b'0' + num) as char
            } else {
                (b'A' + num - 10) as char
            }
        }
        buf.push(hex_from_digit(b / 16));
        buf.push(hex_from_digit(b % 16));
    }
    return buf;
}

impl X509Certificate {
    pub fn repr(&self) -> String {
        return format!(
            "Version: 3 (0x2)
Serial Number:
    {}
Signature Algorithm: sha256WithRSAEncryption
Issuer: CN = Venafi QA
Validity
    Not Before: Oct  9 10:19:26 2019 GMT
    Not After : Oct  7 10:19:26 2027 GMT
Subject: CN = {}
Subject Public Key Info:
    Public Key Algorithm: rsaEncryption
        RSA Public-Key: (2048 bit)
",
            self.meta.thumbprint, self.subject.common_name
        );
    }
}

pub struct X509Subject {
    pub common_name: String,
    pub country: String,
}

pub struct X509Certificate {
    pub meta: X509CertificateMeta,
    pub raw: Option<Vec<u8>>,
    pub subject: X509Subject,
}

pub struct X509RequestMeta {
    pub id: u32,
    pub name: String,
}

pub struct X509Request {
    pub meta: X509RequestMeta,
    pub subject: X509Subject,
}

pub struct KeyMeta {
    pub id: u32,
    pub name: String,
    pub algo: String,
    pub option: String,
}
pub struct Key {
    pub meta: KeyMeta,
    pub public: Vec<u8>,
    pub private: Option<Vec<u8>>,
}
