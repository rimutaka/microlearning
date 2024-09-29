Which two options that require the least amount of RCUs?

# Scan with FilterExpression and ProjectionExpression
Incorrect.

Scan reads all records in a table or index and evaluates them one by one against the _FilterExpression_.
The RCUs are calculated for every read before _FilterExpression_ and _ProjectionExpression_ are applied.
It is usually the most expensive option or retrieving data.

# Query with Partition and Secondary Keys
Correct.

Using both keys results in least number of reads and least number of RCUs.
Neither _ProjectionExpression_ nor _FilterExpression_ affect the RCUs.

https://aws.amazon.com/dynamodb/pricing/on-demand/

# Query with with Partition and Secondary Keys, ProjectionExpression and FilterExpression
Correct.

_ProjectionExpression_ specifies which attributes to return to the caller.

_FilterExpression_ discards items after DynamoDB read them and before they are returned to the caller.

Both _ProjectionExpression_ and _FilterExpression_ reduce the amount of data returned to the caller, but they have no impact on RCUs.

The RCUs are calculated for the number of items matching the PK and SK part of the query.

https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_Query.html

# Query with ConsistentRead set to True
Incorrect.

_Strongly Consistent Reads_ require more RCUs compared to the default option of _Eventually Consistent Reads_.

https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.ReadConsistency.html