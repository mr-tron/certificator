pub struct X509CertificateMeta {
    pub name: String,
    pub thumbprint: String,
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
