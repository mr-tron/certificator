use crate::models::{
    X509Certificate, X509CertificateMeta, X509RequestMeta, X509Subject, KeyMeta
};
use rusqlite::Connection;
use crate::utils;

const SCHEMA1: &str = "
    CREATE TABLE IF NOT EXISTS certificates (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        thumbprint TEXT NOT NULL,
        name TEXT NOT NULL,
        raw BLOB,
        der BLOB
     );
    CREATE UNIQUE INDEX IF NOT EXISTS thumbprint ON certificates(thumbprint);

    CREATE TABLE IF NOT EXISTS keys (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        algo TEXT NOT NULL,
        option TEXT NOT NULL,
        public BLOB NOT NULL,
        private BLOB
    );
    CREATE UNIQUE INDEX IF NOT EXISTS keys_public ON keys(public);

    CREATE TABLE IF NOT EXISTS csrs (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        raw BLOB,
        der BLOB
    );";

pub struct X509RequestDB {
    pub name: String,
    pub raw: Option<Vec<u8>>,
    pub der: Option<Vec<u8>>,
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
        let thumbprint = utils::thumprint_repr(d.as_ref());
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

pub struct DB {
    conn: rusqlite::Connection,
}
impl DB {
    pub fn new() -> rusqlite::Result<DB> {
        let conn = Connection::open("/tmp/certificator.sqlite")?;
        // todo: versions
        conn.execute_batch(SCHEMA1)?;
        return Ok(DB { conn });
    }

    pub fn get_cert(&self, thumbprint: String) -> rusqlite::Result<X509Certificate> {
        let mut stmt = self.conn.prepare(
            "SELECT thumbprint, name, raw, der FROM certificates WHERE thumbprint = ?1;",
        )?;
        let cert = stmt.query_row(&[&thumbprint.as_str()], |row| {
            Ok(X509Certificate {
                meta: X509CertificateMeta {
                    thumbprint: row.get(0)?,
                    name: row.get(1)?,
                },
                subject: X509Subject {
                    common_name: "test.example.com".to_string(),
                    country: "US".to_string(),
                },
                raw: row.get(3)?,
            })
        })?;
        return Ok(cert);
    }

    pub fn get_certs(&self, offset: u32, limit: u32) -> rusqlite::Result<Vec<X509CertificateMeta>> {
        let mut stmt = self
            .conn
            .prepare("SELECT thumbprint, name FROM certificates ORDER BY id LIMIT ?1 OFFSET ?2;")?;
        let cert_list = stmt
            .query_map(&[&limit, &offset], |row| {
                Ok(X509CertificateMeta {
                    thumbprint: row.get(0)?,
                    name: row.get(1)?,
                })
            })
            .unwrap();
        let mut vec = Vec::new();
        for c in cert_list {
            let cert = c?;
            vec.push(cert)
        }
        return Ok(vec);
    }

    pub fn save_cert(&self, cert: X509CertificateDB) -> rusqlite::Result<String> {
        let thumbprint = cert.thumbprint;
        let result = self.conn.execute_named(
            "INSERT INTO certificates (name, thumbprint, raw, der)
                  VALUES (:name, :thumbprint, :raw, :der);",
            &[
                (":name", &cert.name.as_str()),
                (":thumbprint", &thumbprint.as_str()),
                (":raw", &cert.raw),
                (":der", &cert.der),
            ],
        )?;
        if result == 1 {
            return Ok(thumbprint);
        }
        return Err(rusqlite::Error::InvalidQuery);
    }

    pub fn get_csrs(&self, offset: u32, limit: u32) -> rusqlite::Result<Vec<X509RequestMeta>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name FROM csrs ORDER BY id LIMIT ?1 OFFSET ?2;")?;
        let cert_list = stmt
            .query_map(&[&limit, &offset], |row| {
                Ok(X509RequestMeta {
                    id: row.get(0)?,
                    name: row.get(1)?,
                })
            })
            .unwrap();
        let mut vec = Vec::new();
        for c in cert_list {
            let cert = c?;
            vec.push(cert)
        }
        return Ok(vec);
    }
    pub fn get_keys(&self, offset: u32, limit: u32) -> rusqlite::Result<Vec<KeyMeta>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, algo, option FROM keys ORDER BY id LIMIT ?1 OFFSET ?2;")?;
        let cert_list = stmt
            .query_map(&[&limit, &offset], |row| {
                Ok(KeyMeta {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    algo: row.get(2)?,
                    option: row.get(3)?
                })
            })
            .unwrap();
        let mut vec = Vec::new();
        for c in cert_list {
            let cert = c?;
            vec.push(cert)
        }
        return Ok(vec);
    }
}
