pub enum Operator {
    Equal,
    Like,
    ILike,
    GreaterThan,
    LessThan,
    GreaterThanEqual,
    LessThanEqual,
}

impl Operator {
    pub fn as_str(&self) -> &'static str {
        match self {
            Operator::Equal => "=",
            Operator::Like => "LIKE",
            Operator::ILike => "ILIKE",
            Operator::GreaterThan => ">",
            Operator::LessThan => "<",
            Operator::GreaterThanEqual => ">=",
            Operator::LessThanEqual => "<=",
        }
    }

    pub fn where_clause(&self, field: &str, index: usize) -> String {
        format!("{} {} ${}", field, self.as_str(), index)
    }
}

pub const ASTERISK: &str = "*";

pub fn like_pattern(value: &str) -> String {
    format!("%{}%", value)
}

pub struct WhereMaker {
    count: usize,
}

impl WhereMaker {
    pub fn new() -> Self {
        Self::new_with_count(1)
    }

    pub fn new_with_count(count: usize) -> Self {
        WhereMaker { count }
    }

    pub fn next_index(&mut self) -> usize {
        let current = self.count;
        self.count += 1;
        current
    }

    pub fn where_clause(&mut self, operator: &Operator, field: &str) -> String {
        operator.where_clause(field, self.next_index())
    }
}
