# TDD String Calculator

## [TDD Manifesto Excercise 2](https://tddmanifesto.com/exercises/)

Create a simple calculator that takes a String and returns an integer

Signature (pseudo code):

int Add(string numbers)

### Requirements

1. The method can take up to two numbers, separated by commas, and will return their sum as a result. So the inputs can be: “”, “1”, “1,2”. For an empty string, it will return 0.
2. Allow the add method to handle an unknown number of arguments
3. Allow the add method to handle newlines as separators, instead of comas
4. Add validation to not to allow a separator at the end
    * For example “1,2,” should return an error (or throw an exception)