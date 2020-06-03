pub struct Date {
}

pub enum Request {
    Balance(Option<i64>),
    Edit(String),
    Insertion(i64, String, Date, String),
    Listing(Date, Date, String),
    Retrieval(String),
}

pub struct Transaction {
    amount: i64,
    date: Date,
    data: String,
}

impl Transaction {
    pub fn new(amount: i64, date: Date, data: String) -> Transaction {
        Transaction {
            amount,
            date,
            data,
        }
    }
    
    pub fn read_from_file(filename: &String) {
        
    }
    
    pub fn edit_date(&mut self, new_date: Date) {
        self.date = new_date;
    }
    
    pub fn edit_amount(&mut self, new_amount: i64) {
        self.amount = new_amount;
    }
}
