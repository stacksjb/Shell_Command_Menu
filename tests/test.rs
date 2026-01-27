use shell_command_menu::{
    config::{CommandOption, Config, load_config, save_config, validate_json},
    csv::read_commands_from_csv,
    menu_edit::clear_all_commands,
    menu_main::{generate_menu, prompt_or_return},
    utils::get_version,
};

#[test]
fn config_roundtrip_save_load() {
    let temp = tempfile::NamedTempFile::new().expect("temp file");
    let path = temp.path().to_path_buf();

    let original = Config {
        commands: vec![CommandOption {
            display_name: "List".into(),
            command: "ls -la".into(),
        }],
        cmd_sound: Some("sound.mp3".into()),
        window_title_support: true,
        window_title: Some("CLI Menu".into()),
    };

    save_config(&path, &original);
    let loaded = load_config(&path).expect("load config");

    assert_eq!(loaded.commands.len(), 1);
    assert_eq!(loaded.commands[0].display_name, "List");
    assert_eq!(loaded.cmd_sound, original.cmd_sound);
    assert_eq!(loaded.window_title_support, original.window_title_support);
    assert_eq!(loaded.window_title, original.window_title);
}

#[test]
fn config_validate_default_json() {
    let config = Config::default();
    assert!(validate_json(&config));
}

#[test]
fn csv_read_commands_fixture() {
    let commands = read_commands_from_csv("tests/fixtures/commands.csv").expect("parse csv");
    assert_eq!(commands.len(), 2);
    assert_eq!(commands[0].display_name, "List Files");
    assert_eq!(commands[0].command, "ls -la");
}

#[test]
fn menu_generate_menu_strikes_selected() {
    let commands = vec![
        CommandOption {
            display_name: "One".into(),
            command: "echo 1".into(),
        },
        CommandOption {
            display_name: "Two".into(),
            command: "echo 2".into(),
        },
    ];

    let rendered = generate_menu(&commands, &[2]);
    assert_eq!(rendered.len(), 2);
    assert_eq!(rendered[0], "1. One");
    assert_eq!(rendered[1], "2. T\u{0336}w\u{0336}o\u{0336}");
}

#[test]
fn menu_prompt_or_return_ok() {
    let value = prompt_or_return(|| Ok::<_, inquire::error::InquireError>(123));
    assert_eq!(value, Some(123));
}

#[test]
fn menu_prompt_or_return_cancelled() {
    let value: Option<i32> =
        prompt_or_return(|| Err::<i32, _>(inquire::error::InquireError::OperationCanceled));
    assert_eq!(value, None);
}

#[test]
fn menu_edit_clear_all_commands() {
    let mut config = Config {
        commands: vec![
            CommandOption {
                display_name: "A".into(),
                command: "echo a".into(),
            },
            CommandOption {
                display_name: "B".into(),
                command: "echo b".into(),
            },
        ],
        ..Default::default()
    };
    let mut changed = false;

    clear_all_commands(&mut config, &mut changed);

    assert!(config.commands.is_empty());
    assert!(changed);
}

#[test]
fn utils_version_matches_package() {
    assert_eq!(get_version(), env!("CARGO_PKG_VERSION"));
}
