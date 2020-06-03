use std::env;
use std::process;
use spent::Request;
use spent::Date;
extern crate exitcode;

fn cli(args: Vec<String>) -> Request {
    process::exit(exitcode::USAGE);
}

fn set_balance(amt: i64) {
    
}

fn get_balance() {
    
}

fn create_transaction(amt: i64, dir: String, date: Date, msg: String) {
    
}

fn read_transaction(fname: String) {

}

fn write_transaction(fname: String) {

}

fn list_transactions(start: Date, end: Date, dir: String) {

}

fn main() {
    let command = cli(env::args().collect());
    match command {
        Request::Balance(opt) => {
            match opt {
                None => get_balance(),
                Some(amount) => set_balance(amount),
            }
        }
        Request::Edit(tx) => {
            write_transaction(tx)
        }
        Request::Insertion(amount,category,date,message) => {
            create_transaction(amount,category,date,message)
        }
        Request::Listing(since,until,category) => {
            list_transactions(since,until,category)
        }
        Request::Retrieval(tx) => {
            read_transaction(tx)
        }
    };
}
