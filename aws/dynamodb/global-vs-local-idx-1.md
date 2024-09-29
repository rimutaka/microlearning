A large DynamoDB table has the following structure:
- `UserID` (PK)
- `ItemID` (SK)
- `ItemBarcode`
- other fields, an often changing list including Lists and Maps.

The number of reads exceeds the number of writes by an order of magnitude.

What is the most efficient way of fetching a single record by `UserID` and `ItemBarcode`?

# Perform a table Scan with `UserID` and `ItemBarcode` in the ProjectionExpression
Incorrect.

_ProjectionExpression_ specifies which attributes to return in the result.
The caller would have to paginate through the entire table until the right record is encountered,
because Scan reads and returns all items from the table.

https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Scan.html

# Perform a table Scan with `UserID` and `ItemBarcode` as the FilterExpression
Incorrect.

_FilterExpression_ specifies which items to return to the caller,
but it is still very inefficient way of retrieving a single item because _Scan_ would attempt to read all items in the table.

https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_Scan.html#API_Scan_RequestSyntax

# Query the table with `UserID` and `ItemBarcode` as keys
Incorrect.

Only Partition and Secondary Keys can be used as query keys.

`ItemBarcode` would have to go into the _FilterExpression_,
but that query would result in excessive RCUs because it would select all items matching the UserID
value, calculate the RCU and then filter out those that do not match the `ItemBarcode`.

https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_Query.html

# Create a Local Secondary Index for `ItemBarcode` and perform Query with `UserID` and `ItemBarcode` as keys
Correct.

You can create a Local Secondary Index (LSI) with `ItemBarcode` as the Sort Key and `ItemID` as the projected attribute.
1. A Query to the index with `UserID` and `ItemBarcode` would return an record that includes `ItemID`.
2. A subsequent Query to the table with `UserID` and `ItemID` would return the item with all its fields.

The two query process is necessary because Secondary Indexes cannot contain List and Map attributes,
so we project the table's Sort Key to the Secondary Index and use it to retrieve the full item from the table.

https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LSI.html


# Create a Global Secondary Index for `ItemBarcode`, project ALL or some of the other fields and perform Query with `UserID` and `ItemBarcode` as keys
May be correct.

A _Global Secondary Index_ (GSI) is like a replica of the main table with different _Partition_ and _Secondary_ keys.

It would be a good choice if we wanted to retrieve items by `ItemBarcode` and some other attribute from the _other fields_ list.
GSIs may contain some or all fields from the main table, so unlike the two-step query with the local index,
we could retrieve the record with a single query.

Consider the storage and read/write capacity when deciding between LSI and GSI in this type of scenario.
