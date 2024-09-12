use core::fmt;
use std::ffi::OsString;
use std::fmt::Display;
use std::fmt::Formatter;
use std::path::{PathBuf};
use windows::core::GUID;
use windows::Win32::UI::Shell::{FOLDERID_Profile, FOLDERID_ProgramFiles, FOLDERID_ProgramFilesX86, FOLDERID_RoamingAppData, FOLDERID_UserProfiles, FOLDERID_UsersFiles, SHGetFolderPathA, SHGetKnownFolderPath, CSIDL_APPDATA, KF_FLAG_DEFAULT};

/// JDK Vendors: Amazon Coretto, Zulu, OpenJDK...
/// JDK Vendor candidates got from Intellij IDEA
#[derive(Debug)]
struct JDKVendor(&'static str,&'static str);
struct SearchCandidate(GUID, &'static str);

impl JDKVendor {
    pub fn new(folder_name: &'static str, display_name: &'static str) -> JDKVendor{
        JDKVendor(folder_name, display_name)
    }
}

impl Display for JDKVendor{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "JDKVendor({}, {})", self.0, self.1)
    }
}

impl SearchCandidate{
    pub fn new(guid: GUID, inner_path: &'static str) -> SearchCandidate{
        SearchCandidate(guid, inner_path)
    }
}

#[derive(Debug)]
pub struct JDK{
    version: &'static str,
    path: PathBuf
}


impl Display for JDK{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "JDK(version:{}, path:{:?})", self.version, self.path)
    }
}

impl JDK{
    pub fn new(version: &'static str, path: PathBuf) -> JDK{
        JDK{
            version,
            path
        }
    }
}

const ADOPT_OPENJDK_HS: JDKVendor = JDKVendor("adopt", "AdoptOpenJDK (HotSpot)");
const ADOPT_OPENJDK_J9: JDKVendor = JDKVendor("adopt-j9", "AdoptOpenJDK (OpenJ9)");
const TEMURIN: JDKVendor = JDKVendor("temurin","Eclipse Temurin");
const SEMERU: JDKVendor = JDKVendor("semeru", "IBM Semeru");
const CORRETTO: JDKVendor = JDKVendor("Amazon Corretto","Amazon Corretto");
const GRAALVM_CE: JDKVendor = JDKVendor("graalvm-ce","GraalVM CE");
const GRAALVM: JDKVendor = JDKVendor("graalvm","GraalVM");
const IBM: JDKVendor = JDKVendor("ibm", "IBM JDK");
const JBR: JDKVendor = JDKVendor("jbr", "JetBrains Runtime");
const LIBERICA: JDKVendor = JDKVendor("liberica", "BellSoft Liberica");
const ORACLE: JDKVendor = JDKVendor("Java", "Oracle OpenJDK");
const SAP_MACHINE: JDKVendor = JDKVendor("sap", "SAP SapMachine");
const ZULU: JDKVendor = JDKVendor("Zulu", "Azul Zulu");
const UNKNOWN: JDKVendor = JDKVendor("Java", "");

// const VENDORS: Vec<JDKVendor> = vec![ADOPT_OPENJDK_HS, ADOPT_OPENJDK_J9, TEMURIN, SEMERU, CORRETTO, GRAALVM_CE, GRAALVM, IBM, JBR, LIBERICA, ORACLE, SEMERU, ZULU, UNKNOWN];

const PROFILE_PATH: SearchCandidate = SearchCandidate(FOLDERID_Profile, ".jdks");
const PROGRAMFILES_PATH: SearchCandidate = SearchCandidate(FOLDERID_ProgramFiles, ".");
const PROGRAMFILES_X86_PATH: SearchCandidate = SearchCandidate(FOLDERID_ProgramFilesX86, ".");


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
    let vendors = vec![ADOPT_OPENJDK_HS, ADOPT_OPENJDK_J9, TEMURIN, SEMERU, CORRETTO, GRAALVM_CE, GRAALVM, IBM, JBR, LIBERICA, ORACLE, SEMERU, ZULU, UNKNOWN];
    for path in path_candidates {
        if let Ok(dirs) = path.read_dir(){
            for entry in dirs {
                if let Ok(dir) = entry{
                    if ! dir.path().is_dir(){
                        continue
                    }

                    let folder_name = dir.file_name();

                    if let Some(vendor) = get_jdk_vendor(&vendors, &folder_name){
                        let folder_name = &folder_name.to_str().unwrap();
                        &jdks.push(JDK::new(folder_name, dir.path()));
                    }
                }
            }
        }
    }

    jdks
}

fn get_jdk_vendor<'h>(vendors: &'h Vec<JDKVendor>, folder_name: &OsString) -> Option<&'h JDKVendor>{
    for vendor in vendors {
        if vendor.0 == folder_name{
            return Some(vendor)
        }
    }

    None
}



