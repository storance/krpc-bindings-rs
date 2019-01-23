use crate::codec::*;
use crate::*;

use crate::client::schema::ProcedureCall;

use std::collections::BTreeMap;

remote_type!(
/// A server side expression.
object SpaceCenter.Expression {
    static_methods: {
        {
            /// A constant value of double precision floating point type.
            fn constant_double(value: f64) -> Expression<'a> {
                ConstantDouble(value)
            }
        }
        {
            /// A constant value of single precision floating point type.
            fn constant_float(value: f32) -> Expression<'a> {
                ConstantFloat(value)
            }
        }
        {
            /// A constant value of integer type.
            fn constant_int(value: i32) -> Expression<'a> {
                ConstantInt(value)
            }
        }
        {
            /// A constant value of boolean type.
            fn constant_bool(value: bool) -> Expression<'a> {
                ConstantBool(value)
            }
        }
        {
            /// A constant value of boolean type.
            fn constant_string(value: &str) -> Expression<'a> {
                ConstantString(value)
            }
        }
        {
            /// An RPC call.
            fn call(call: &ProcedureCall) -> Expression<'a> {
                Call(call)
            }
        }
        {
            /// Equality comparison.
            fn equal(left: &Expression, right: &Expression) -> Expression<'a> {
                Equal(left, right)
            }
        }
        {
            /// Inequality comparison.
            fn not_equal(left: &Expression, right: &Expression) -> Expression<'a> {
                NotEqual(left, right)
            }
        }
        {
            /// Greater than numerical comparison.
            fn greater_than(left: &Expression, right: &Expression) -> Expression<'a> {
                GreaterThan(left, right)
            }
        }
        {
            /// Greater than or equal numerical comparison.
            fn greater_than_or_equal(left: &Expression, right: &Expression) -> Expression<'a> {
                GreaterThanOrEqual(left, right)
            }
        }
        {
            /// Less than numerical comparison.
            fn less_than(left: &Expression, right: &Expression) -> Expression<'a> {
                LessThan(left, right)
            }
        }
        {
            /// Less than or equal numerical comparison.
            fn less_than_or_equal(left: &Expression, right: &Expression) -> Expression<'a> {
                LessThanOrEqual(left, right)
            }
        }
        {
            /// Boolean and operator.
            fn and(left: &Expression, right: &Expression) -> Expression<'a> {
                And(left, right)
            }
        }
        {
            /// Boolean or operator.
            fn or(left: &Expression, right: &Expression) -> Expression<'a> {
                Or(left, right)
            }
        }
        {
            /// Boolean exclusive or operator.
            fn exclusive_or(left: &Expression, right: &Expression) -> Expression<'a> {
                ExclusiveOr(left, right)
            }
        }
        {
            /// Boolean negation or operator.
            fn not(expr: &Expression) -> Expression<'a> {
                Not(expr)
            }
        }
        {
            /// Numerical addition.
            fn add(left: &Expression, right: &Expression) -> Expression<'a> {
                Add(left, right)
            }
        }
        {
            /// Numerical subtraction.
            fn subtract(left: &Expression, right: &Expression) -> Expression<'a> {
                Subract(left, right)
            }
        }
        {
            /// Numerical multiplication.
            fn multiply(left: &Expression, right: &Expression) -> Expression<'a> {
                Multiply(left, right)
            }
        }
        {
            /// Numerical division.
            fn divide(left: &Expression, right: &Expression) -> Expression<'a> {
                Divide(left, right)
            }
        }
        {
            /// Numerical modulo operator.
            fn modulo(left: &Expression, right: &Expression) -> Expression<'a> {
                Modulo(left, right)
            }
        }
        {
            /// Numerical power operator.
            fn power(left: &Expression, right: &Expression) -> Expression<'a> {
                Power(left, right)
            }
        }
        {
            /// Bitwise left shift.
            fn left_shift(left: &Expression, right: &Expression) -> Expression<'a> {
                LeftShift(left, right)
            }
        }
        {
            /// Bitwise right shift.
            fn right_shift(left: &Expression, right: &Expression) -> Expression<'a> {
                RightShift(left, right)
            }
        }
        {
            /// Perform a cast to the given type.
            ///
            /// # Arguments
            /// * `expr` - The expression to cast.
            /// * `cast_type` – Type to cast the `expr` to.
            fn cast(expr: &Expression, cast_type: &Type) -> Expression<'a> {
                Cast(expr, cast_type)
            }
        }
        {
            /// A named parameter of the given type.
            ///
            /// # Arguments
            /// * `name` - The name of the parameter.
            /// * `parameter_type` – The type of the parameter.
            fn parameter(name: &str, parameter_type: &Type) -> Expression<'a> {
                Parameter(name, parameter_type)
            }
        }
        {
            /// A function.
            ///
            /// # Arguments
            /// * `parameters` – The parameters of the function.
            /// * `body` – The body of the function.
            fn function(parameters: &[Expression], body: &Expression) -> Expression<'a> {
                Function(parameters, body)
            }
        }
        {
            /// A function call.
            ///
            /// # Arguments
            /// * `function` – The function to call.
            /// * `args` – The arguments to call the function with.
            fn invoke(function: Expression, args: &BTreeMap<String, Expression>) -> Expression<'a> {
                Invoke(function, args)
            }
        }
        {
            /// Construct a tuple.
            fn create_tuple(elements: &[Expression]) -> Expression<'a> {
                CreateTuple(elements)
            }
        }
        {
            /// Construct a list.
            fn create_list(elements: &[Expression]) -> Expression<'a> {
                CreateList(elements)
            }
        }
        {
            /// Construct a set.
            fn create_set(elements: &[Expression]) -> Expression<'a> {
                CreateSet(elements)
            }
        }
        {
            /// Construct a dictionary.
            fn create_dictionary(keys: &[Expression], values: &[Expression]) -> Expression<'a> {
                CreateDictionary(keys, values)
            }
        }
        {
            /// Convert a collection to a list.
            fn to_list(expr: &Expression) -> Expression<'a> {
                ToList(expr)
            }
        }
        {
            /// Convert a collection to a set.
            fn to_set(expr: &Expression) -> Expression<'a> {
                ToSet(expr)
            }
        }
        {
            /// Number of elements in a collection.
            fn count(collection: &Expression) -> Expression<'a> {
                Count(collection)
            }
        }
        {
            /// Sum all elements of a collection.
            fn sum(collection: &Expression) -> Expression<'a> {
                Sum(collection)
            }
        }
        {
            /// Maximum of all elements in a collection.
            fn max(collection: &Expression) -> Expression<'a> {
                Max(collection)
            }
        }
        {
            /// Minimum of all elements in a collection.
            fn min(collection: &Expression) -> Expression<'a> {
                Min(collection)
            }
        }
        {
            /// Average of all elements in a collection.
            fn average(collection: &Expression) -> Expression<'a> {
                Average(collection)
            }
        }
        {
            /// Run a function on every element in the collection.
            fn select(collection: &Expression, func: &Expression) -> Expression<'a> {
                Select(collection, func)
            }
        }
        {
            /// Run a function on every element in the collection.
            fn where_(collection: &Expression, func: &Expression) -> Expression<'a> {
                Where(collection, func)
            }
        }
        {
            /// Determine if a collection contains a value.
            fn contains(collection: &Expression, value: &Expression) -> Expression<'a> {
                Contains(collection, value)
            }
        }
        {
            /// Applies an accumulator function over a sequence.
            fn aggregate(collection: &Expression, func: &Expression) -> Expression<'a> {
                Aggregate(collection, func)
            }
        }
        {
            /// Applies an accumulator function over a sequence, with a given seed.
            fn aggregate_with_seed(collection: &Expression, seed: &Expression, func: &Expression) -> Expression<'a> {
                AggregateWithSeed(collection, seed, func)
            }
        }
        {
            /// Concatenate two sequences.
            fn concat(left: &Expression, right: &Expression) -> Expression<'a> {
                Concat(left, right)
            }
        }
        {
            /// Order a collection using a key function.
            fn order_by(collection: &Expression, key_func: &Expression) -> Expression<'a> {
                OrderBy(collection, key_func)
            }
        }
        {
            /// Determine whether all items in a collection satisfy a boolean predicate.
            fn all(collection: &Expression, predicate: &Expression) -> Expression<'a> {
                All(collection, predicate)
            }
        }
        {
            /// Determine whether any items in a collection satisfy a boolean predicate.
            fn any(collection: &Expression, predicate: &Expression) -> Expression<'a> {
                Any(collection, predicate)
            }
        }
    }
});

remote_type!(
/// Server side type.
object SpaceCenter.Type {
    static_methods: {
        {
            /// Double type.
            fn double() -> Type {
                Double()
            }
        }
        {
            /// Float type.
            fn float() -> Type {
                Float()
            }
        }
        {
            /// Int type.
            fn int() -> Type {
                Int()
            }
        }
        {
            /// Bool type.
            fn bool() -> Type {
                Bool()
            }
        }
        {
            /// String type.
            fn string() -> Type {
                String()
            }
        }
    }
});
