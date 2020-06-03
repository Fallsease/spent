# Spent
A spending tracker

## Requirements

- Store date, amount, and category of transaction
- Allow extra comments (for vendor, description, etc.)
- Retrieve all transactions in a given month
- Retrieve all transactions in a given category
- Be extensible

### Bonus features

- Use date/time of entry unless other date is specified
- Be readable with a text editor
- Perform currency conversion / accept multiple currencies
- Keep track of balance
- Integrate with git
- Auto-complete for transaction categories

## High-Level Design

### Storage

- Each transaction is its own file
- File path goes like category/subcategory/datetime.tx$
    - File directory is the category
    - Filename is the datetime of transaction
    - File extension is tx$
- First line of transaction file contains amount
    - Lines beyond that are extra comments
    - Extra lines can be used as data for extensions

### Usage

The application runs in the command line. I haven't decided on a name for the application, but for now, I'm calling it `spent`.

#### To Insert Transaction

`spent [OPTIONS] AMOUNT CATEGORY`

AMOUNT should be just a number. Units can be added through an extension option.

CATEGORY will be a relative directory from the $HOME/.spent directory. If a new transaction is inserted with an unknown category, it will create a directory with that new category label. Auto-complete will be added.

OPTIONS:
- -d or --date refers to the date and time of the transaction. By default, it will be *now*. It will use the `date` command to parse the date. Future dates are not permitted.
- -i or --income says that the transaction is not an expense, which is assumed, but is instead income. 
- -m or --message allows you to add additional info about the transaction in the second line of the file.

#### To Retrieve Transactions

`spent [OPTIONS] list [CATEGORY]`

`spent [OPTIONS] ls [CATEGORY]`

OPTIONS:
- --since or --after only shows transactions after a certain date. The default is the beginning of the month in local time.
- --until or --before only shows transactions before a certain date. The default is *now*. 
- --income only shows income transactions. The default is to show both income and expenditures. Including a category usually eliminates the need for this option, since it is unlikely that income and expense categories would have similar names.
- --expense only shows expenditure transactions. The default is to show both income and expenditures. Including a category usually eliminates the need for this option, since it is unlikely that income and expense categories would have similar names.
- -v or --verbose gives the entire transaction file contents, rather than just the date, amount, and category.

#### To Retrieve Balances

`spent [OPTIONS] [CATEGORY]`

Returns a single number totaling all of the transactions in the given category. By default, it only includes transactions since the start of the month.

OPTIONS:
- --since or --after only shows transactions after a certain date. The default is the beginning of the month in local time.
- --until or --before only shows transactions before a certain date. The default is *now*. 
- --income only shows income transactions. The default is to show both income and expenditures. Including a category usually eliminates the need for this option, since it is unlikely that income and expense categories would have similar names.
- --expense only shows expenditure transactions. The default is to show both income and expenditures. Including a category usually eliminates the need for this option, since it is unlikely that income and expense categories would have similar names.

`spent balance`

Your balance, tracked since initialization of the repository, and stored in the .balance file. I don't expect it to be used often, since it will get more and more inaccurate as time goes on, simply because sometimes the user will forget to include the transaction. However, theoretically, the user can manually edit the balance by running `spent balance reset`.

#### To Edit Transactions

Sometimes we make mistakes and type in a transaction incorrectly. We would want to edit a transaction to fix it. Unfortunately, this requires that we figure out which transaction we are referring to. Assuming they have managed to do so, to edit the transaction, they can run 

`spent [OPTIONS] edit CATEGORY/DATE`

This will bring up the default text editor and allow them to manually edit the file. Note that the date must be human-readable so that a person could run that kind of command. 

Note that editing any transaction could end up changing the balance, and so the application must account for that. 

OPTIONS:
- `spent edit --date NEWDATE CATEGORY/OLDDATE` allows you to change the date. It will do a simple file rename.
- `spent mv OLDCATEGORY/DATE NEWCATEGORY/` allows you to change the category. 
