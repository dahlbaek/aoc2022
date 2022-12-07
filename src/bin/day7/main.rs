use std::collections::HashMap;

fn prepare(s: &str) -> HashMap<String, u64> {
    let mut current_folder = vec![String::from("")];
    let mut blobs = HashMap::<String, u64>::new();
    for line in s.lines() {
        let mut args = line.split(" ");
        match (args.next().unwrap(), args.next().unwrap(), args.next()) {
            ("$", "cd", Some("/")) => current_folder.truncate(1),
            ("$", "cd", Some("..")) => current_folder.truncate(current_folder.len() - 1),
            ("$", "cd", Some(dir)) => current_folder.push(dir.to_string()),
            ("$", "ls", None) => (),
            ("dir", _, None) => (),
            (size, file_name, None) => {
                blobs.insert(
                    format!("{}/{}", current_folder.join("/"), file_name),
                    size.parse::<u64>().unwrap(),
                );
            }
            _ => panic!("Unrecognized input {}", line),
        }
    }

    let mut folders = HashMap::<String, u64>::new();
    for (key, size) in blobs {
        let mut folder = key.rsplit_once('/').unwrap().0.to_string();
        *folders.entry(folder.clone().to_string()).or_default() += size;
        while let Some((inner_folder, _)) = folder.clone().rsplit_once('/') {
            folder = inner_folder.clone().to_string();
            *folders.entry(folder.clone()).or_default() += size;
        }
    }

    folders
}

fn small_folder_total_size(folders: HashMap<String, u64>) -> u64 {
    folders.into_values().filter(|&v| v <= 100000).sum()
}

fn small_folder_large_enough(folders: HashMap<String, u64>) -> u64 {
    let to_delete = folders.get(&String::from("")).unwrap() - 40000000;
    folders
        .into_values()
        .filter(|&v| v >= to_delete)
        .min()
        .unwrap()
}

fn main() {
    let input = include_str!("input");
    println!("{}", small_folder_total_size(prepare(input)));
    println!("{}", small_folder_large_enough(prepare(input)));
}

#[cfg(test)]
mod tests {
    use crate::{prepare, small_folder_large_enough, small_folder_total_size};

    const INPUT: &'static str = include_str!("test-input");

    #[test]
    fn it_works_1() {
        assert_eq!(small_folder_total_size(prepare(INPUT)), 95437);
    }

    #[test]
    fn it_works_2() {
        assert_eq!(small_folder_large_enough(prepare(INPUT)), 24933642);
    }
}
