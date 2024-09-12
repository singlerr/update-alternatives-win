use std::path::PathBuf;
use windows::core::GUID;
use windows::Win32::UI::Shell::{FOLDERID_Profile, FOLDERID_ProgramFiles, FOLDERID_ProgramFilesX86, FOLDERID_RoamingAppData, FOLDERID_UserProfiles, FOLDERID_UsersFiles, SHGetFolderPathA, SHGetKnownFolderPath, CSIDL_APPDATA, KF_FLAG_DEFAULT};

#[macro_export]
macro_rules! jdk_vendor{
    ($name: literal, $display_name: literal) => {
        JDKVendor(
            stringify!($name),
            stringify!($display_name)
        )
    };
}

macro_rules! search_candidate {
    ($guid: expr, $path: literal) => {
        SearchCandidate(
            $guid,
            stringify!($path)
        )
    };
}
/// JDK Vendors: Amazon Coretto, Zulu, OpenJDK...
/// JDK Vendor candidates got from Intellij IDEA
struct JDKVendor(&'static str, &'static str);

struct SearchCandidate(GUID,&'static str);

pub struct JDK{
    version: &'static str,
    path: PathBuf
}
const ADOPT_OPENJDK_HS: JDKVendor = jdk_vendor!("adopt", "AdoptOpenJDK (HotSpot)");
const ADOPT_OPENJDK_J9: JDKVendor = jdk_vendor!("adopt-j9", "AdoptOpenJDK (OpenJ9)");
const TEMURIN: JDKVendor = jdk_vendor!("temurin","Eclipse Temurin");
const SEMERU: JDKVendor = jdk_vendor!("semeru", "IBM Semeru");
const CORRETTO: JDKVendor = jdk_vendor!("corretto","Amazon Corretto");
const GRAALVM_CE: JDKVendor = jdk_vendor!("graalvm-ce","GraalVM CE");
const GRAALVM: JDKVendor = jdk_vendor!("graalvm","GraalVM");
const IBM: JDKVendor = jdk_vendor!("ibm", "IBM JDK");
const JBR: JDKVendor = jdk_vendor!("jbr", "JetBrains Runtime");
const LIBERICA: JDKVendor = jdk_vendor!("liberica", "BellSoft Liberica");
const ORACLE: JDKVendor = jdk_vendor!("jdk", "Oracle OpenJDK");
const SAP_MACHINE: JDKVendor = jdk_vendor!("sap", "SAP SapMachine");
const ZULU: JDKVendor = jdk_vendor!("zulu", "Azul Zulu");
const UNKNOWN: JDKVendor = jdk_vendor!("", "");

// const VENDORS: Vec<JDKVendor> = vec![ADOPT_OPENJDK_HS, ADOPT_OPENJDK_J9, TEMURIN, SEMERU, CORRETTO, GRAALVM_CE, GRAALVM, IBM, JBR, LIBERICA, ORACLE, SEMERU, ZULU, UNKNOWN];

const PROFILE_PATH: SearchCandidate = search_candidate!(FOLDERID_Profile, ".jdks");
const PROGRAMFILES_PATH: SearchCandidate = search_candidate!(FOLDERID_ProgramFiles, ".");
const PROGRAMFILES_X86_PATH: SearchCandidate = search_candidate!(FOLDERID_ProgramFilesX86, ".");


// const SEARCH_CANDIDATES: Vec<SearchCandidate> = vec![PROFILE_PATH, PROGRAMFILES_PATH, PROGRAMFILES_X86_PATH];

/// Candidates: FOLDERID_Profile/.jdks, FOLDERID_ProgramFiles, FOLDERID_ProgramFilesX86

pub unsafe fn get_search_path_candidates() -> Vec<PathBuf>{
    let candidates = vec![PROFILE_PATH, PROGRAMFILES_PATH, PROGRAMFILES_X86_PATH];
    let mut result:Vec<PathBuf> = Vec::new();

    for SearchCandidate(guid, inner_path) in candidates {
        let path = SHGetKnownFolderPath(&guid, KF_FLAG_DEFAULT, None);

        if let Ok(path) = path{
            let jdk_path = PathBuf::from_iter(vec![path.to_string().unwrap().as_str(), inner_path]);
            &result.push(jdk_path);
        }
    }
    result
}

pub fn get_jdks() -> Vec<JDK>{
    let path_candidates = unsafe { get_search_path_candidates() };
    let mut jdks: Vec<JDK> = Vec::new();

    for path in path_candidates {

    }

    jdks
}



