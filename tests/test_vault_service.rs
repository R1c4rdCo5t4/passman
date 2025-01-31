#[cfg(test)]
mod mocks {
    pub mod mock_vault_manager;
}

#[cfg(test)]
mod tests {
    use passman::domain::app::state::AppState;
    use passman::services::vault_service::VaultService;
    use secrecy::SecretBox;
    use passman::domain::cli::field::Field;
    use crate::mocks::mock_vault_manager::MockVaultManager;

    fn setup() -> (AppState, VaultService<MockVaultManager>, SecretBox<String>) {
        let state = AppState { session: None };
        let service = VaultService::new(MockVaultManager);
        let secret = SecretBox::new(Box::new("test_password".to_string()));
        (state, service, secret)
    }

    #[test]
    fn test_create_and_open_vault() {
        let (mut state, service, secret) = setup();
        let vault = "test_vault";

        // create vault
        service.create(vault, &secret);

        // open vault
        service.open(vault, &secret, &mut state).unwrap();

        // check result
        assert!(state.session.is_some());
        assert_eq!(state.session.as_ref().unwrap().name, vault);
    }

    #[test]
    fn test_vault_close() {
        let (mut state, service, secret) = setup();
        let vault = "test_vault";

        // setup
        service.create(vault, &secret);
        service.open(vault, &secret, &mut state).unwrap();

        // close vault
        service.close(&mut state);
        assert!(state.session.is_none());
    }

    #[test]
    fn test_add_entry() {
        let (mut state, service, secret) = setup();
        let vault = "test_vault";
        let entry = "test_entry";

        // setup
        service.create(vault, &secret);
        service.open(vault, &secret, &mut state).unwrap();

        // add entry
        service.add_entry(entry, "user", "pass", &mut state);

        // check result
        let result = service.show(Some(entry.to_string()), false, &mut state);
        assert!(result.is_ok());
        assert_eq!(state.session.as_ref().unwrap().vault.entries.len(), 1);
    }

    #[test]
    fn test_update_entry() {
        let (mut state, service, secret) = setup();
        let vault = "test_vault";
        let entry = "test_entry";

        // setup
        service.create(vault, &secret);
        service.open(vault, &secret, &mut state).unwrap();
        service.add_entry(entry, "user", "pass", &mut state);

        // update entry
        let _ = service.update_entry(entry, &Field::Username, "new_user", &mut state);

        // check result
        let session = state.session.as_ref().unwrap();
        let entry = session.vault.entries.iter()
            .find(|e| e.name == entry)
            .unwrap();
        assert_eq!(entry.username, "new_user");
    }

    #[test]
    fn test_delete_vault() {
        let (mut state, service, secret) = setup();
        let vault = "test_vault";
        let entry = "test_entry";

        // setup
        service.create(vault, &secret);
        service.open(vault, &secret, &mut state).unwrap();

        // delete vault
        service.delete(&mut state);

        // check result
        let result = service.show(Some(entry.to_string()), false, &mut state);
        assert!(result.is_err());
    }

    #[test]
    fn test_list_vaults() {
        let (_, service, secret) = setup();
        let vault1 = "test_vault1";
        let vault2 = "test_vault2";

        // setup
        service.create(vault1, &secret);
        service.create(vault2, &secret);

        // list vaults
        let result = service.list();

        // check result
        assert_eq!(result.split("\n").count() - 1, 2);
        assert!(result.contains(vault1));
        assert!(result.contains(vault2));
    }

    #[test]
    fn test_vault_security() {
        let (mut state, service, secret) = setup();
        let vault1 = "test_vault1";
        let wrong_secret = SecretBox::new(Box::new("wrong_pass".to_string()));

        // setup
        service.create(vault1, &secret);

        // attempt to unlock with wrong password
        let result = service.open(vault1, &wrong_secret, &mut state);

        // check result
        assert!(result.is_err());
        assert!(state.session.is_none());
    }
}