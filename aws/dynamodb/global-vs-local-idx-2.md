A large DynamoDB table has the following structure:
- `UserID` (PK)
- `ItemID` (SK)
- `ItemBarcode`
- other fields

What is the most efficient way of fetching a list of users (`UserID`) by `ItemBarcode`?

# Perform a table Scan with `ItemBarcode` as the FilterExpression
Incorrect.

It is a very inefficient way of retrieving a single item because _Scan_ would attempt to read all items in the table and then exclude those that do not match the `ItemBarcode`.

https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_Scan.html#API_Scan_RequestSyntax

# Create a Local Secondary Index for `ItemBarcode` and perform Query with `ItemBarcode` as the key
Incorrect.

Local Secondary Indexes provide an alternative option for the _Sort Key_ and the query would still require a User ID for the _Partition Key_ value.

https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LSI.html


# Create a Global Secondary Index for `ItemBarcode` and perform Query with `ItemBarcode` as the only key
Correct.

A _Global Secondary Index_ (GSI) is like a replica of the main table with different _Partition_ and _Secondary_ keys.

GSI items include the table Partition Key by default. All or some other attributes can be added as needed.

GSI's incur additional RCUs for maintenance.

https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/GSI.html
