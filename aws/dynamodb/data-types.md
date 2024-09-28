Select all DynamoDB data types:

# String
Correct.

Strings are Unicode with UTF-8 binary encoding. 

- min length: 0
- max length is constrained by the maximum DynamoDB item size limit of 400 KB
- partition key: 1 - 2048 bytes
- sort key: 1 - 1024 bytes
- collation and comparison are done using the bytes of the underlying UTF-8 string encoding, e.g. `a` (0x61) is greater than `A` (0x41)

https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html

# Number
Correct.

Numbers can be positive, negative, or zero with up to 38 digits of precision.
Leading and trailing zeroes are trimmed.
Use strings for precision more than 38 digits.

# List
Correct.

A list is similar to a JSON array and can store an ordered collection of values, e.g. 
`FavoriteThings: ["Cookies", "Coffee", 3.14159]`.
The elements in a list do not have to be of the same type.
The values can be JSON objects up to 32 levels deep.

# Map
Correct.

A map is similar to a JSON object. 
You can work with individual elements within maps, even if those elements are deeply nested. The nesting is limited to 32 levels.

This example shows a map with a string, a number, and a nested list that contains another map:

```json
{
    Day: "Monday",
    UnreadEmails: 42,
    ItemsOnMyDesk: [
        "Coffee Cup",
        "Telephone",
        {
            Pens: { Quantity : 3},
            Pencils: { Quantity : 2},
            Erasers: { Quantity : 1}
        }
    ]
}
```


# DateTime
Incorrect.

There is no such data type in DynamoDB. You can use the string data type to represent a date or a timestamp. 

- String examples using ISO 8601: `2016-02-15`, `2015-12-21T17:42:34Z`, `20150311T122706Z`
- Timestamp using Number data type: `1727494394`

# Binary
Correct.

Binary data should be encoded using base-64 before sending the value to DynamoDB, e.g. `dGhpcyB0ZXh0IGlzIGJhc2U2NC1lbmNvZGVk`

It can be used for partition (1 - 2048 bytes max) and sort (1 - 1024 bytes max) keys.
DynamoDB decodes the values before calculating their length.


# Boolean
Correct.
A Boolean type attribute can store either true or false.

# Null
Correct.
Null represents an attribute with an unknown or undefined state.
It is similar to JSON Null data type.

# Array
Incorrect.
Array is a reserved keyword, but not a data type.
You probably meant List or Set, which are valid DynamoDB data types.

# Set
Correct.

DynamoDB supports types that represent sets of number, string, or binary values, for example, a _String Set_: `["Black", "Green", "Red"]`. 

- All the elements within a set must be of the same type
- Each value within a set must be unique
- The order of the values within a set is not preserved
- Empty sets are not supported

# Item
Incorrect.

Item is a DynamoDB term for a record, as in a relational database record.

# Document
Incorrect.

# JSON
Incorrect.
You probably meant Map, which allows for JSON-like data structures up to 32 levels deep.

# Blob
Incorrect.
Blob is a reserved keyword, but not a data type.
You probably meant Binary, which is a valid DynamoDB data type.
