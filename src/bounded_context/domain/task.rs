pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
}

impl Task {
    pub fn new(id: u32, title: String, description: String) -> Task {
        Task {
            id,
            title,
            description,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_constructor() {
        // Valores de entrada
        let id = 1;
        let title = "Sample Task".to_string();
        let description = "This is a sample task".to_string();

        // Resultado esperado
        let expected_task = Task {
            id,
            title: title.clone(),
            description: description.clone(),
        };

        // Llamada al constructor
        let task = Task::new(id, title, description);

        // Verificación
        assert_eq!(task.id, expected_task.id);
        assert_eq!(task.title, expected_task.title);
        assert_eq!(task.description, expected_task.description);
    }
}
