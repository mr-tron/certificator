use crate::models::{X509Certificate, X509CertificateMeta, X509Subject};
use rusqlite::{Connection, NO_PARAMS};

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

    pub fn save_cert(&self, cert: X509Certificate) -> rusqlite::Result<bool> {
        let result = self.conn.execute_named(
            "INSERT INTO certificates (name, thumbprint)
                  VALUES (:name, :thumbprint)",
            &[
                (":name", &cert.meta.name.as_str()),
                (":thumbprint", &cert.meta.thumbprint.as_str()),
            ],
        )?;
        let succes = result == 1;
        return Ok(succes);
    }
}
