## Import

- [ ] Scan trusted stores and import all CA certificate (on startup)
- [ ] Scan all FS and find all certificates, private keys and CSRs (manual)
- [x] Window for copypaste 
  - [ ] with autodetection copypaste content
- [ ] File selector
- [ ] Parent CA via CA Issuers field in cert (optional)
- [ ] OCSP

### Support formats

- [x] pem
- [ ] der
- [ ] pkcs12

## Display

### CSR 

- [ ] Subject
- [ ] Key info
- [ ] EKU

### Certificate

- [ ] Subject
- [ ] Key info
- [ ] Issuer
- [ ] Dates
- [ ] EKU

### Key

- [ ] Public in pem
- [ ] Private (optional)
- [ ] Key info (algo, size, curve)

## Storage

- [ ] Move to something encrypted (may be only for private keys)

## Issuing

- [ ] Generate csr manually
- [ ] Create csr from over csr or request
(form for manual creating but filled with values from csr/certificate)
- [ ] Sign any csr with any imported key

## Private keys

- [ ] Generating RSA with different key lengths
- [ ] Generating ECDSA with different curves
- [ ] ed25519

## Validating 

- [ ] Signature
- [ ] Usage for web
- [ ] Chain

## Relations

- [ ] Find parent CA for cert
- [ ] Find key for cert
- [ ] Find key for csr
- [ ] Find all certs and csr with same key
- [ ] Find all certs signed by CA
- [ ] And subchildren

## Information

- [ ] Curves decription
- [ ] EKU descriptions
- [ ] Most popular CA commentaries