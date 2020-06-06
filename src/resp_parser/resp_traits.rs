use tui::widgets::TableState;

pub trait TuiTableCreator {
    fn to_tui_table(&self) -> TuiTable;
}

pub trait TuiListCreator {
    fn to_tui_list(&self) -> TuiList;
}

#[derive(Clone)]
pub struct TuiTable {
    pub state: TableState,
    pub items: Vec<Vec<String>>,
}

impl TuiTable {
    pub fn new(items: Vec<Vec<String>>) -> TuiTable {
        TuiTable {
            state: TableState::default(),
            items,
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

#[derive(Clone)]
pub struct TuiList {
    pub items: Vec::<String>,
}

impl TuiList {
    pub fn new() -> TuiList {
        TuiList {
            items: Vec::new(),
        }
    }

    pub fn with_items(items: Vec<String>) -> TuiList {
        TuiList {
            items,
        }
    }
}


