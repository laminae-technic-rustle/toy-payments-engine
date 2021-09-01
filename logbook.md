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

# Part Two
Manage to work on it another hour or so. My earlier idea was pretty much right
in that it's a fairly straight forward state machine. I did feel the need for
correctness, so I added a Currency module, with a Currency type and some
functions to add and safely subtract elements. It uses a u64 under the hood, and
is parsed from the f64 strings that come in. They are built to be immutable, so
operations can be done safely, aside from mutation the inner structure. 

I've been thinking on how to make this faster in the meanwhile. I'm fairly
certain that my approach, while correct, will not be the fastest out there. I'm
favouring readability, simplicity, and above all, correctness over performance
for now. There are some optimisations, especially when streaming things in, but
the problem comes from consistency, and how to deal with double spent problems
in a distributed system (which I'm sure I won't solve right away).

Some ideas;
- We should split out processing, and updating. The processors should put every
    processed item on a queue.
- The queue could be modeled in a distributed fashion, so multiple streams of
    input could put elements on the queue from different places, all on a single
    queue, but then the queue could take it from there (many-to-one-to-many if
    you will)
- Then, instead of saving the accounts as a static record, we would save every
    incarnation of them in a circular doubly linked list. That way, if the 
    queue does encounter an element where the head of the list has a newer 
    timestamp than this one, it can traverse the list, insert it where it needs, 
    assert the validity of the entire chain, and if it at some point is invalid, 
    return a different account for the new 'head'. It needs to be circular so we
    can easily jump to the start ot the account and re-create the current state
    from scratch. Sort of like an event-stream, but with accounts.

Then this could be the 'source of thruth', and should be fairly efficient. We
could have multiple streams put data on the queue, and as many actors as we want
updating the data in the end. The risk of clashing whilst writing data is
extremely small, as it would only happen if there are two transactions, where
they both happened before the current head of the list, and are adjecent, and
are updated at the same time. Then there is a potential clash. There would also
be a potential clash when the chain is being rebuild and / or rechecked. But if
so, we could implement a mutex as I would assume those cases would be relatively
rare.

# Last stretch
## Type System Guarantee's & Serde
In serde, [enum representations](https://serde.rs/enum-representations.html) can
be deserialized by checking an internal tag. Unfortunately, this doesn't work
for the CSV. This is kind of tricky, as this means we cannot guarantee at
compile time that the Enum's do their 'correct' thing.
This means we either have to implement a fully custom deserializer and do it
properly (check the tag, and deserialize accordingly), or do one of two things
wrong:
1. Use Optional Values. This means disputes / resolves / chargebacks won't have
   an amount value attributed with them. 
2. Deserialize 'null' values to 0.
Out of both, the second one is the lesser of two evil's for now. While it
bothers me that this isn't 'correct' at the type level, using option 1 would
imply there isn't a currency available for withrawals or for deposits. Which
isn't right, just as no. 2 isn't right.
Since, for intenger addition and subtraction, 0 is the 'identity' element, we
can safely do this too. In case it's ever used in a dispute, resolve, or
chargeback, given the current API of the Currency type, this means it will be a
no-op.

## Disputes
This is an interesting one. Because what whould happen if there is a dispute on
a:
- Withdrawal: I've disputing that some money taken from account X, should not
    have been taken from account X. This in term means that subtracting amount Y
    from account X makes no sense, as I'm expecting the withdrawal to be
    reversed, aka - getting money deposited. As such, my withdrawal will be
    deposited on the other side, and the dispute should happen on the deposit
    side.
- Deposit: This makes sense. But, this can put the account into overdraw,
    getting it below 0. Since withdrawals only go up to 0, this shouldn't
    matter. But this does mean my underlying representation had to change from
    u64 to i64.
- Dispute / Resolve / Chargeback: I could dispute these. But that would be 
    hard to resolve automatically.

So - I've one ahead and only implemented disputes for deposits, allowing for an
overdraft when this happens.

# Leftovers
## Time
I've spent about 4-5 hours on this in total. A bit more than I expected, but
this could easily take more if done more properly. There are still some things
left out.

## Ledger.rs
There were edge-cases. A bit more than I anticipated. While the logic seems
relatively straight forward, and it could be done in a rather unsound way
without much hassle, doing it properly, and catching all the edge-cases, makes
for more code which is less concise. I would:
- Put more effort into properly deserialising the data, so instead of:
  `{tx_type: Deposit, ...}` we have `Deposit({..})`, then we can immediatly
  deconstruct all values in the pattern match, and remove a lot of crud.
- Break out each match arm to it's own file

## Files / Modules
While reasonably structured, the file structure is completely flat. I would
start pushing things into their own folders / `mod.rs`, also keep the main
function a lot smaller.

## Writer
While I abstracted the CSV reading bit, I did not do the same for the writer.
This is something I would do given more time.

## Testing
I would write some more tests. Especially for the **Currency**. I did write some
to verify my ledger functionality was working as I would expect it.

## Performance
I have done 0 performance testing, and I think the code as-is won't be
particularly performant (I also don't think it will be horrible). It currently
parses everything before pushing it into the ledger. I've specified some ideas
above for speeding this up. The queue approach is still pretty good imho. I
think the underlying 'end' datastructure also depends a lot on where it will be
stored. I'm fairly sure this won't lay around in rust memory, and also won't be
outputted to plain CSV's, but there will be some distributed DB. That DB should
be able to handle concurrent writes and deal with conflicts so we don't have
too.

## Reader
The reader now fails if it encounters a single malformed row. We could push the
rows that do work to stderr and parse the rows that will work. Unfortunately,
this would have a cascading effect, because potential deposits could be dropped,
which in turn would cause failures for withdrawals...

# Concluding
A fun little test. I will take 15 min to brush some things up, and read through
the comments, but will otherwise leave it as is.
