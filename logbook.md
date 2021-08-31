# Initial Commit
The initial commit contains some scaffolding to parse the CSV file given a path
from the CSV.
I've decided to code in a certain monetary precision, set to 4 decimals in a
const. With the help of some serializer / deserializer functions, we can
instantly parse the amount for a transaction to an integer, so we won't ever
have to deal with floating point rounding errors.
Assumption here is that a transaction fits within a f64 (and thus in a u64).

From there, we get either an error (reading the file, or parsing), or a list
of transactions. I'm currently working under the assumption that every
transaction is a well-formed one. I might change this later on to return a tuple
of successfully parsed transactions and unsuccessfully parsed ones, where the
unsuccessfull ones could be parsed later on. But this would lead to the ledger
being only eventually consistent. That's kind of a pain (but I'm sure a
real-life scenario).

My thinking is that from here-on-out, it's quite a simple state machine.
Given the account and a transaction;
Deposit -> Simply add the monetary
Withdraw -> If enough; withdraw; otherwise; ignore (?)
Dispute -> Find transaction, pull amount from available to held
Resolve -> Find transaction, pull amount from held to available

The last two sort of imply we need to keep a map of in memory. Possibly the
(tx_id, amount) tuple is enough. 
