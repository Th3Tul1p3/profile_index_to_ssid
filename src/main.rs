use winreg::enums::*;
use winreg::RegKey;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: cmd <ProfilID> !");
        std::process::exit(0);
    }

    let mut is_ssid_found: bool = false;
    let profile_id: u32 = u32::from_str_radix(args.get(1).unwrap(), 10).unwrap();
    
    println!("--------------- profilID to SSID ---------------\n");
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let interfaces = hklm
        .open_subkey("Software\\Microsoft\\WlanSvc\\Interfaces")
        .unwrap();
    for interface in interfaces.enum_keys().map(|x| x.unwrap()) {
        let profiles = match interfaces.open_subkey(format!("{}\\Profiles", interface)) {
            Ok(profile) => profile,
            Err(_) => continue,
        };
        for profile_name in profiles.enum_keys().map(|x| x.unwrap()) {
            let profile = profiles.open_subkey(profile_name).unwrap();
            let get_profile_index: u32 = profile.get_value("ProfileIndex").unwrap();
            if get_profile_index == profile_id {
                let metadata = profile.open_subkey("MetaData").unwrap();
                let channel_hints = metadata.get_raw_value("Channel Hints").unwrap();
                let end_pos: usize = channel_hints.bytes[4..]
                    .iter()
                    .position(|x| *x == 0)
                    .unwrap();
                let ssid: String =
                    String::from_utf8(channel_hints.bytes[4..end_pos + 4].to_vec()).unwrap();
                println!("Le SSID correspondant au profileID: {}", ssid);
                is_ssid_found = get_profile_index == profile_id;
            }
        }
    }

    if !is_ssid_found {
        println!("Le profileID ne correspond a aucun SSID. ")
    }
}
