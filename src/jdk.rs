use core::fmt;
use std::ffi::OsString;
use std::fmt::Display;
use std::fmt::Formatter;
use std::path::PathBuf;
use windows::core::GUID;
use windows::Win32::UI::Shell::{
    FOLDERID_Profile, FOLDERID_ProgramFiles, FOLDERID_ProgramFilesX86, SHGetKnownFolderPath,
    KF_FLAG_DEFAULT,
};

/// JDK Vendors: Amazon Coretto, Zulu, OpenJDK...
/// JDK Vendor candidates got from Intellij IDEA
#[derive(Debug)]
struct JDKVendor(&'static str, &'static str);
struct SearchCandidate(GUID, &'static str);

impl JDKVendor {
    pub fn new(folder_name: &'static str, display_name: &'static str) -> JDKVendor {
        JDKVendor(folder_name, display_name)
    }
}

impl Display for JDKVendor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "JDKVendor({}, {})", self.0, self.1)
    }
}

impl SearchCandidate {
    pub fn new(guid: GUID, inner_path: &'static str) -> SearchCandidate {
        SearchCandidate(guid, inner_path)
    }
}

#[derive(Debug)]
pub struct JDK {
    pub version: String,
    pub path: PathBuf,
}

impl Display for JDK {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "JDK(version:{}, path:{:?})", self.version, self.path)
    }
}

impl JDK {
    pub fn new(version: String, path: PathBuf) -> JDK {
        JDK { version, path }
    }
}

const ADOPT_OPENJDK_HS: JDKVendor = JDKVendor("adopt", "AdoptOpenJDK (HotSpot)");
const ADOPT_OPENJDK_J9: JDKVendor = JDKVendor("adopt-j9", "AdoptOpenJDK (OpenJ9)");
const TEMURIN: JDKVendor = JDKVendor("temurin", "Eclipse Temurin");
const SEMERU: JDKVendor = JDKVendor("semeru", "IBM Semeru");
const CORRETTO: JDKVendor = JDKVendor("Amazon Corretto", "Amazon Corretto");
const GRAALVM_CE: JDKVendor = JDKVendor("graalvm-ce", "GraalVM CE");
const GRAALVM: JDKVendor = JDKVendor("graalvm", "GraalVM");
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

// Candidates: FOLDERID_Profile/.jdks, FOLDERID_ProgramFiles, FOLDERID_ProgramFilesX86
unsafe fn get_search_path_candidates() -> Vec<PathBuf> {
    let candidates = vec![PROFILE_PATH, PROGRAMFILES_PATH, PROGRAMFILES_X86_PATH];
    let mut result: Vec<PathBuf> = Vec::new();

    for SearchCandidate(guid, inner_path) in candidates {
        let path = SHGetKnownFolderPath(&guid, KF_FLAG_DEFAULT, None);

        if let Ok(path) = path {
            let jdk_path = PathBuf::from_iter(vec![path.to_string().unwrap().as_str(), inner_path]);
            &result.push(jdk_path);
        }
    }
    result
}

pub fn get_jdks() -> Vec<JDK> {
    let path_candidates = unsafe { get_search_path_candidates() };
    let mut jdks: Vec<JDK> = Vec::new();
    let vendors = vec![
        ADOPT_OPENJDK_HS,
        ADOPT_OPENJDK_J9,
        TEMURIN,
        SEMERU,
        CORRETTO,
        GRAALVM_CE,
        GRAALVM,
        IBM,
        JBR,
        LIBERICA,
        ORACLE,
        SEMERU,
        ZULU,
        UNKNOWN,
    ];
    for path in path_candidates {
        if let Ok(dirs) = path.read_dir() {
            for entry in dirs {
                if let Ok(dir) = entry {
                    if !dir.path().is_dir() {
                        continue;
                    }

                    let folder_name = dir.file_name();
                    if let Some(_) = get_jdk_vendor(&vendors, &folder_name) {
                        for jdk in get_jdk_versions(&dir.path()) {
                            &jdks.push(jdk);
                        }
                    }
                }
            }
        }
    }

    jdks
}

fn get_jdk_versions(root_dir: &PathBuf) -> Vec<JDK> {
    let mut versions = Vec::new();
    if let Ok(dir) = root_dir.read_dir() {
        for entry in dir {
            match entry {
                Ok(entry) => {
                    if is_jdk(entry.path()) {
                        versions.push(JDK::new(
                            entry.file_name().into_string().unwrap(),
                            entry.path(),
                        ));
                    }
                }
                Err(_) => continue,
            }
        }
    }

    versions
}
// We don't want to mutate path buf that has origins
fn is_jdk(mut jdk_path: PathBuf) -> bool {
    let mut is_jdk = true;

    // Check it has bin folder
    jdk_path.push("bin");
    is_jdk &= jdk_path.exists();
    jdk_path.pop();

    // Check it has include folder
    jdk_path.push("include");
    is_jdk &= jdk_path.exists();
    jdk_path.pop();

    // Check it has lib folder
    jdk_path.push("lib");
    is_jdk &= jdk_path.exists();
    jdk_path.pop();

    is_jdk
}

fn get_jdk_vendor<'h>(
    vendors: &'h Vec<JDKVendor>,
    folder_name: &OsString,
) -> Option<&'h JDKVendor> {
    for vendor in vendors {
        if vendor.0 == folder_name {
            return Some(vendor);
        }
    }

    None
}
