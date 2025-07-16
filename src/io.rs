pub mod io {
    use crate::game::game::*;
    use crate::read_kdl::read_kdl::*;

    pub fn get_data(path: String) -> Reality {
        let curr_path = String::from(
            std::env::current_exe()
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap()
                .split("exploration.exe")
                .into_iter()
                .collect::<Vec<&str>>()[0],
        );
        dbg!(curr_path);
        reality_from_kdl(std::fs::read_to_string(&path).expect("FAILED TO LOAD DATA FILE"))
            .expect("FAILED TO LOAD DATA FILE")
    }
}
