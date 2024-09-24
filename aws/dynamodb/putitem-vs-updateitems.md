You need to upsert records (items) into a DynamoDB table with three fields:
- `user_id` (PK)
- `name`
- `photo_uri`

Lambda A adds or updates `name` and Lambda B adds or updates `photo_uri` using `user_id` as the key.
Neither of the two lambdas know the value for the other field.

What is the MOST EFFICIENT way of performing an upsert in both lambdas?

# PutItem
Incorrect.
_PutItem_ will overwrite all fields, so that if the item exists the value for the other field will be removed.
E.g. Lambda A will drop `photo_uri` because it has no value for that field.
- https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_PutItem.html

# GetItem -> merge values -> PutItem
Incorrect.
This requires more units to perform the same operation than UpdateItem.
Also, it can lead to inconsistencies if some other code performs a concurrent update on the same item.
- https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/on-demand-capacity-mode.html

# PutItem with a condition expression
Incorrect.
_PutItem_ with a _condition expression_ will prevent the addition of the record if the condition is true.
- https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ConditionExpressions.html

# UpdateItem
Correct.
_UpdateItem_ behaves like _PutItem_ if the item does not exist,
but will only update the fields specified in the update expression if the item does exists.
It is a cheaper, faster and eventually consistent way of upserting records into DynamoDB tables.
- https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_UpdateItem.html