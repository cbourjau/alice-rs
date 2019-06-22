#[macro_use]
extern crate failure;
extern crate dirs;
extern crate glob;
extern crate reqwest;

use failure::Error;
use reqwest::Certificate;
use reqwest::Url;
use std::fs::{DirBuilder, File};
use std::io::copy;
use std::io::Read;
use std::path::PathBuf;

/// Download the given file to the local collection
pub fn download(base_dir: PathBuf, url: Url) -> Result<u64, Error> {
    let mut dest = base_dir;
    let mut sub_dir = url.path().to_owned();
    // Remove the leading "\" from the original path
    sub_dir.remove(0);
    dest.push(sub_dir);
    // Do not re-download if the file already exists
    if dest.exists() {
        return Ok(0);
    }
    // Make sure the dir exists
    if let Some(dir) = dest.parent() {
        DirBuilder::new().recursive(true).create(dir)?;
    }
    let mut resp = download_with_https(url)?;
    let mut f = File::create(dest)?;
    Ok(copy(&mut resp, &mut f)?)
}

/// Base path to the local ALICE open data directory
pub fn data_dir() -> Result<PathBuf, Error> {
    let mut dir = dirs::home_dir().ok_or_else(|| format_err!("No home directory"))?;
    dir.push("lhc_open_data");
    Ok(dir)
}

/// Hardcoded path to a specific file. Useful for testing.
/// That file should be the the first to be downloaded automatically.
pub fn test_file() -> Result<PathBuf, Error> {
    let mut dir = data_dir()?;
    dir.push("eos/opendata/alice/2010/LHC10h/000139038/ESD/0001/AliESDs.root");
    Ok(dir)
}

/// Path to all files of `LHC10h`
pub fn all_files_10h() -> Result<Vec<PathBuf>, Error> {
    let mut search_dir = data_dir()?;
    search_dir.push("**/AliESDs.root");
    let files: Vec<_> = glob::glob(search_dir.to_str().unwrap())
        .expect("Can't resolve glob")
        .map(|path| path.unwrap())
        .collect();
    Ok(files)
}

pub fn get_file_list(run: u32) -> Result<Vec<Url>, Error> {
    let uri = "http://opendata.cern.ch/record/".to_owned()
        + match run {
            139_038 => "1102/files/ALICE_LHC10h_PbPb_ESD_139038_file_index.txt",
            139_173 => "1103/files/ALICE_LHC10h_PbPb_ESD_139173_file_index.txt",
            139_437 => "1104/files/ALICE_LHC10h_PbPb_ESD_139437_file_index.txt",
            139_438 => "1105/files/ALICE_LHC10h_PbPb_ESD_139438_file_index.txt",
            139_465 => "1106/files/ALICE_LHC10h_PbPb_ESD_139465_file_index.txt",
            _ => return Err(format_err!("Invalid run number")),
        };
    let mut resp = reqwest::get(uri.as_str())?;
    if resp.status().is_success() {
        let mut content = String::new();
        resp.read_to_string(&mut content)?;
        Ok(content
            .lines()
            .map(|l| format!("http://eospublichttp.cern.ch/{}", &l[26..]))
            .map(|l| l.parse::<Url>().expect("Invalid file URI"))
            .collect())
    } else {
        Err(format_err!("Could not download list of files"))
    }
}

fn download_with_https(uri: Url) -> Result<reqwest::Response, Error> {
    let client = reqwest::Client::builder()
        .add_root_certificate(Certificate::from_pem(ROOT).unwrap())
        .add_root_certificate(Certificate::from_pem(GRID).unwrap())
        .build()
        .unwrap();
    Ok(client.get(uri).send()?)
}

#[cfg(test)]
mod tests {
    use std::{env, fs};
    #[test]
    fn test_get_file_lists() {
        let runs = [139_038, 139_173, 139_437, 139_438, 139_465];
        for run in runs.iter() {
            println!("Testing run {}", run);
            super::get_file_list(*run).unwrap();
        }
    }
    #[test]
    fn test_download_file() {
        let uris = &super::get_file_list(139038).unwrap();
        let resp = super::download_with_https(uris[0].clone()).unwrap();
        println!("{:?}", resp);
        println!("{:?}", uris[0].path());
    }
    #[test]
    fn test_download_file_high_level() {
        let uri = super::get_file_list(139038).unwrap()[0].clone();
        {
            // Remobe old stuff:
            let mut dir = env::temp_dir();
            dir.push("eos");
            if dir.exists() {
                fs::remove_dir_all(dir).unwrap();
            }
        }
        let base_dir = env::temp_dir();
        // Download if file does not exist
        assert_eq!(
            super::download(base_dir.clone(), uri.clone()).unwrap(),
            14283265
        );
        // Don't download twice
        assert_eq!(super::download(base_dir.clone(), uri.clone()).unwrap(), 0);
    }
}

const ROOT: &[u8] = b"\
-----BEGIN CERTIFICATE-----
MIIGqTCCBJGgAwIBAgIQAojDcLlcbrhBX0qrEka4mzANBgkqhkiG9w0BAQ0FADBK
MQswCQYDVQQGEwJjaDENMAsGA1UEChMEQ0VSTjEsMCoGA1UEAxMjQ0VSTiBSb290
IENlcnRpZmljYXRpb24gQXV0aG9yaXR5IDIwHhcNMTMwMzE5MTI1NTM2WhcNMzMw
MzE5MTMwNTM0WjBKMQswCQYDVQQGEwJjaDENMAsGA1UEChMEQ0VSTjEsMCoGA1UE
AxMjQ0VSTiBSb290IENlcnRpZmljYXRpb24gQXV0aG9yaXR5IDIwggIiMA0GCSqG
SIb3DQEBAQUAA4ICDwAwggIKAoICAQDxqYPFW2qVVi3Rw1NKlEf7x70xF+6a8uE/
Tu4ZVQF/K2RXI95QLkYfKItZvy9Az3ib/VlUho5f8fBaqy4n70uwC7+qd3Aq1/xQ
ysykPCbBBAsOSQQpTlhrMD2V5Ya9zrirphOhutddiqV96zBCyMM+Gz5uYv9u+cm4
tg1EOmAMGh2UNxfTFNVmXKkk7eFTSC1+zgb28H6nd3xzV27sn9bfOfGh//ZPy5gm
Qx0Oh/tc6WMreWzRZBQm5SJiK0QOzPv09p5WmdY2WxZoqNTFBDACQO7ysFOktc74
fPVFX/lmt4jFNSZRIOvvaACI/qlEaAJTR4FHIY9uSMsV8DrtzhI1Ucyv3kqlQpbF
jDouq44IryA/np4s/124bW+x8+n/v+at/AxPjvHBLiGhB+J38Z6KcJogoDnGzIXR
S+YUr/vGz34jOmkRuDN5STuuAXzyCKFXaoAm0AwjTziIv3E0jxC1taw6FpKevnd1
CLsTLAEUiEjzStFkDhd/Hpipc57zmMFY8VYet2wVqSFjnt2REWOVbZlbCiMHmSeD
u5EuZLiU8xlkiaCfn4A5XZ6X0qprbgDviGJtwxzNvTg7Hn0ziW5/ELryfQXCwZJ+
FVne8Zu8sbgy/sDkX+pyFuyB4XgiM0eMNkoexIXJaRdlMWDIL5ysiIXQKjhynAv5
KLHbRjciVwIDAQABo4IBiTCCAYUwCwYDVR0PBAQDAgGGMA8GA1UdEwEB/wQFMAMB
Af8wHQYDVR0OBBYEFPp7+96bDaPyUrds7VsPC6KmpvgEMBAGCSsGAQQBgjcVAQQD
AgEAMIIBMgYDVR0gBIIBKTCCASUwggEhBgorBgEEAWAKBAEBMIIBETCBwgYIKwYB
BQUHAgIwgbUegbIAQwBFAFIATgAgAFIAbwBvAHQAIABDAGUAcgB0AGkAZgBpAGMA
YQB0AGkAbwBuACAAQQB1AHQAaABvAHIAaQB0AHkAIAAyACAAQwBlAHIAdABpAGYA
aQBjAGEAdABlACAAUABvAGwAaQBjAHkAIABhAG4AZAAgAEMAZQByAHQAaQBmAGkA
YwBhAHQAZQAgAFAAcgBhAGMAdABpAGMAZQAgAFMAdABhAHQAZQBtAGUAbgB0MEoG
CCsGAQUFBwIBFj5odHRwOi8vY2FmaWxlcy5jZXJuLmNoL2NhZmlsZXMvY3AtY3Bz
L2Nlcm4tcm9vdC1jYTItY3AtY3BzLnBkZjANBgkqhkiG9w0BAQ0FAAOCAgEAo0Px
l4CZ6C6bDH+b6jV5uUO0NIHtvLuVgQLMdKVHtQ2UaxeIrWwD+Kz1FyJCHTRXrCvE
OFOca9SEYK2XrbqZGvRKdDRsq+XYts6aCampXj5ahh6r4oQJ8U7aLVfziKTK13Gy
dYFoAUeUrlNklICt3v2wWBaa1tg2oSlU2g4iCg9kYpRnIW3VKSrVsdVk2lUa4EXs
nTEJ30OS7rqX3SdqZp8G+awtBEReh2XPhRgJ6w3xiScP/UdWYUam2LflCGX3RibB
/DZhgGHRRoE4/D0kQMP2XTz6cClbNklECTlp0qZIbiaf350HbcDEFzYRSSIi0emv
kRGcMgsi8yTTU87q8Cr4hETxAF3ZbSVNC0ZaTZ8RBbM9BXguhYzKkVBgG/cMpUjs
B6tY2HMZbAZ3TKQRb/bRyUigM9DniKWeXkeL/0Nsno+XbcpAqLjtVIRwCg6jTLUi
1NRsl3BP6C824dVaoI8Ry7m+o6O+mtocw4BMhHfTcoWCO8CWjT0ME67JzaAYa5eM
+OqoWtgbgweBlfO0/3GMnVGMAmI4FlhH2oWKWQgWdgr0Wgh9K05VcxSpJ87/zjhb
MQn/bEojWmp6eUppPaqNFcELvud41qoe6hLsOYQVUQ1sHi7n6ouhg4BAbwS2iyD2
uiA6FHTCeLreFGUzs5osPKiz3GE5D6V9she9xIQ=
-----END CERTIFICATE-----";

const GRID: &[u8] = b"\
-----BEGIN CERTIFICATE-----
MIIJdjCCB16gAwIBAgIKYZhqPwAAAAAAAzANBgkqhkiG9w0BAQ0FADBKMQswCQYD
VQQGEwJjaDENMAsGA1UEChMEQ0VSTjEsMCoGA1UEAxMjQ0VSTiBSb290IENlcnRp
ZmljYXRpb24gQXV0aG9yaXR5IDIwHhcNMTMwNDIyMTExMDE2WhcNMjMwNDIyMTEy
MDE2WjBWMRIwEAYKCZImiZPyLGQBGRYCY2gxFDASBgoJkiaJk/IsZAEZFgRjZXJu
MSowKAYDVQQDEyFDRVJOIEdyaWQgQ2VydGlmaWNhdGlvbiBBdXRob3JpdHkwggIi
MA0GCSqGSIb3DQEBAQUAA4ICDwAwggIKAoICAQDS9Ypy1csm0aZA4/QnWe2oaiQI
LqfeekV8kSSvOhW2peo5cLNIKbXATOo1l2iwIbCWV8SRU2TLKxHIL8fAOJud5n9K
mEKBew7nzubl1wG93B4dY0KREdb3/QB/7OkG8ZZvLqrvQZVGT1CgJ+NFFUiJ315D
FWkKctZv27LjQamzCxpX+gZSsmwZmSReY67cnm6P7z+/3xVNhwb+4Z+1Ww4vHhMc
dh1Dsrkv9vXU01UN752QtQ6l56uQLYEB2+vaHB6IpyC9zAQ/33GulCq8Gbj7ykPd
9AcRVBeJAErSK+oMHThtdLD7mhTkZivakaNe4O1EhPFH0rWwV45IFN7ipELA5qDx
djdzo6JtLJQMaSV/TV+amEf2CaKlD0giqGhjfSNiOX5HCmpqV14kbl+7Qho6ykZy
b1DGpf70yILnX+AUtdpd8lulTu1yg1Bg5cFQskUIk5+s4nsC1VpmeNxYaeFEcYZj
Ph2mdD7zLo889MtF7kZv7+6J6p4NBL3fQ9Os8/h8XVlfDatzbpVH4jYKKAd4nwJb
knJaKPE0LzLzVfJBwnDxqe8hb64gI8Frludp+jaOYzvMqlzAe9z4a9971iXIWaaG
unbAoEkXj69y7MsvCjWXB7o9HdBaS9FL+ZtXTKCyXl+XLFseYQoQburKr+eTcRed
KLJNj4tRF1799PO69wIDAQABo4IEUDCCBEwwEAYJKwYBBAGCNxUBBAMCAQAwHQYD
VR0OBBYEFKWg/WZY/bndeuGynZ+j0eVQGJTnMIIBLQYDVR0gBIIBJDCCASAwggEc
BgorBgEEAWAKBAEBMIIBDDCBvgYIKwYBBQUHAgIwgbEega4AQwBFAFIATgAgAEcA
cgBpAGQAIABDAGUAcgB0AGkAZgBpAGMAYQB0AGkAbwBuACAAQQB1AHQAaABvAHIA
aQB0AHkAIABDAGUAcgB0AGkAZgBpAGMAYQB0AGUAIABQAG8AbABpAGMAeQAgAGEA
bgBkACAAQwBlAHIAdABpAGYAaQBjAGEAdABlACAAUAByAGEAYwB0AGkAYwBlACAA
UwB0AGEAdABlAG0AZQBuAHQwSQYIKwYBBQUHAgEWPWh0dHA6Ly9jYWZpbGVzLmNl
cm4uY2gvY2FmaWxlcy9jcC1jcHMvY2Vybi1ncmlkLWNhLWNwLWNwcy5wZGYwGQYJ
KwYBBAGCNxQCBAweCgBTAHUAYgBDAEEwCwYDVR0PBAQDAgGGMA8GA1UdEwEB/wQF
MAMBAf8wHwYDVR0jBBgwFoAU+nv73psNo/JSt2ztWw8Loqam+AQwggFEBgNVHR8E
ggE7MIIBNzCCATOgggEvoIIBK4ZSaHR0cDovL2NhZmlsZXMuY2Vybi5jaC9jYWZp
bGVzL2NybC9DRVJOJTIwUm9vdCUyMENlcnRpZmljYXRpb24lMjBBdXRob3JpdHkl
MjAyLmNybIaB1GxkYXA6Ly8vQ049Q0VSTiUyMFJvb3QlMjBDZXJ0aWZpY2F0aW9u
JTIwQXV0aG9yaXR5JTIwMixDTj1DRVJOUEtJUk9PVDAyLENOPUNEUCxDTj1QdWJs
aWMlMjBLZXklMjBTZXJ2aWNlcyxDTj1TZXJ2aWNlcyxDTj1Db25maWd1cmF0aW9u
LERDPWNlcm4sREM9Y2g/Y2VydGlmaWNhdGVSZXZvY2F0aW9uTGlzdD9iYXNlP29i
amVjdENsYXNzPWNSTERpc3RyaWJ1dGlvblBvaW50MIIBRAYIKwYBBQUHAQEEggE2
MIIBMjBnBggrBgEFBQcwAoZbaHR0cDovL2NhZmlsZXMuY2Vybi5jaC9jYWZpbGVz
L2NlcnRpZmljYXRlcy9DRVJOJTIwUm9vdCUyMENlcnRpZmljYXRpb24lMjBBdXRo
b3JpdHklMjAyLmNydDCBxgYIKwYBBQUHMAKGgblsZGFwOi8vL0NOPUNFUk4lMjBS
b290JTIwQ2VydGlmaWNhdGlvbiUyMEF1dGhvcml0eSUyMDIsQ049QUlBLENOPVB1
YmxpYyUyMEtleSUyMFNlcnZpY2VzLENOPVNlcnZpY2VzLENOPUNvbmZpZ3VyYXRp
b24sREM9Y2VybixEQz1jaD9jQUNlcnRpZmljYXRlP2Jhc2U/b2JqZWN0Q2xhc3M9
Y2VydGlmaWNhdGlvbkF1dGhvcml0eTANBgkqhkiG9w0BAQ0FAAOCAgEAQjzXhTV8
d+6HaLqSnp7k9whxK6E75BZQJNR2Q/rslhhwijs6nekBjb+JPgmM6M0a7ra+D1Oi
4wKaWiCvU9yleZZSqfEkRl7WK9trRYXHqkqVSnmwNJNsediqioBBDHn/ZMnyc25Z
OLbM+99Z+awvoMbyPy0moUrR7ZqKi3C02N2mkiidO0m3bYnXKwxDUvka5n06oLnI
YSZfwFNJ7IEvSSF4mEzdDeQI+A+87+deb5XOTXee8i1ZUyI08Cg6tuZ8W6NdvY7t
+5iNxRmZJ6DBVwrvXutz0JSqklBCw267osEpX0AKGSL9fE2yGlWBX8WfDLB43lVE
z/HP7kQwYEmsfnfT2yTLzkMJrHSeR0Zymm/oB3amZziKex4kGk+/v7yV1pSYKJce
9QDZE+LYio/ndz01sejMPS87prYJqnII5hDYUjg9F1CoaejhjOlpmCU/10wyEVN0
nhSP9Wc5z0+lhzU5C1A9r1gXQMuqCA2e7Cv5wv+r9dS+12Uly52jwmYf8mm6H0ZY
LZQbvMayHebD4WCnB7HNdp2Va4z5JrLvwG3J1EXfTjWiPhqOweevOg0rc6t2yhkM
iB9RXMlFoFzbsuE/4Z4Hd0GQcDijcnWJ/VbT15OD2C16yyBiLvu88nXX1gKuOzxL
vu4cw9FOuQZo147y9KPelpUT/SO+nrePzVs=
-----END CERTIFICATE-----";
