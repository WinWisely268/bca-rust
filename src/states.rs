// Global AppState
#[derive(Copy, Clone, Debug)]
pub struct AppState {
    pub is_logged_in: bool,
}

impl AppState {
    pub fn new() -> Self {
        AppState{
            // is logged in to klikbca individual
            is_logged_in: false,
        }
    }

    pub fn toggle_login(&mut self) -> Self {
        self.is_logged_in = !self.is_logged_in;
        *self
    }

}


