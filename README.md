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
5. Allow the add method to handle different delimiters

    To change the delimiter, the beginning of the input will contain a separate line that looks like this:

//[delimiter]\n[numbers]

    “//;\n1;3” should return “4”
    “//|\n1|2|3” should return “6”
    “//sep\n2sep5” should return “7”
    “//|\n1|2,3” is invalid and should return an error (or throw an exception) with the message “‘|’ expected but ‘,’ found at position 3.”

STOP HERE if you are a beginner. Continue if you could finish the steps (1-5.) within 30 minutes.

6. Calling add with negative numbers will return the message “Negative number(s) not allowed: <negativeNumbers>”

    “1,-2” is invalid and should return the message “Negative number(s) not allowed: -2”
    
    “2,-4,-9” is invalid and should return the message “Negative number(s) not allowed: -4, -9”