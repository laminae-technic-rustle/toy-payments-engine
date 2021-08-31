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
