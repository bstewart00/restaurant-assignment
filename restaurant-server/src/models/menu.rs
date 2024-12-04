#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Deserialize)]
pub struct MenuItemId(pub i32);
impl std::fmt::Display for MenuItemId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq)]
pub struct MenuItem {
    pub id: MenuItemId,
    pub name: String,
    pub description: String, // details, ingredients etc
}

// for simplicity, assume all ids are valid rather than pulling from some kind of list
pub fn get_menu_item(id: &MenuItemId) -> MenuItem {
    return MenuItem {
        id: id.clone(),
        name: format!("menu item {}", &id),
        description: format!("menu item desc {}", &id)
    };
}
