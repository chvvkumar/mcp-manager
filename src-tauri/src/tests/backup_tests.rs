#[cfg(test)]
mod tests {
    use crate::backup::BackupManager;
    use std::io::Write;

    #[test]
    fn test_backup_and_restore() {
        let dir = tempfile::tempdir().unwrap();
        let backup_dir = dir.path().join("backups");
        let config_path = dir.path().join("test_config.json");
        let mut file = std::fs::File::create(&config_path).unwrap();
        write!(file, r#"{{"mcpServers": {{"s1": {{"command": "cmd1"}}}}}}"#).unwrap();
        let manager = BackupManager::new(backup_dir.clone());
        let backup_path = manager.backup("test-tool", &config_path).unwrap();
        assert!(backup_path.exists());
        std::fs::write(&config_path, r#"{"mcpServers": {}}"#).unwrap();
        manager.restore(&backup_path, &config_path).unwrap();
        let content = std::fs::read_to_string(&config_path).unwrap();
        assert!(content.contains("cmd1"));
    }

    #[test]
    fn test_list_backups() {
        let dir = tempfile::tempdir().unwrap();
        let backup_dir = dir.path().join("backups");
        let config_path = dir.path().join("test_config.json");
        std::fs::write(&config_path, "{}").unwrap();
        let manager = BackupManager::new(backup_dir);
        manager.backup("test-tool", &config_path).unwrap();
        manager.backup("test-tool", &config_path).unwrap();
        let backups = manager.list_backups("test-tool").unwrap();
        assert_eq!(backups.len(), 2);
    }
}
