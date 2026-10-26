pub struct Model_Task {
    pub id: u32,
    pub date: String,
    pub title: String,
    pub description: String,
    pub completed: u32,
    pub completed_at: Option<String>,
}
