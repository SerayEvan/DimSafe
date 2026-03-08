use super::*;

#[derive(Clone)]
pub struct ErrorCollector {
    errors: Vec<Error>,
}

impl ErrorCollector {

    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn get_errors(&self) -> &Vec<Error> {
        &self.errors
    }

    pub fn clear_errors(&mut self) {
        self.errors.clear();
    }

    pub fn raise<M: ErrorMessage + Send + Sync + 'static>(&mut self, error: M) {
        self.errors.push(Error::new(error));
    }

    pub fn add_error(&mut self, error: Error) {
        self.errors.push(error);
    }

    pub fn set_loc_range(&mut self, loc_range: &RangeIndex) {
        for error in self.errors.iter_mut() {
            error.set_loc_range(loc_range);
        }
    }

    pub fn into_string(&self) -> String {
        self.errors.iter().map(|error| error.get_message()).collect::<Vec<String>>().join("\n")
    }
}