Simple exmple to show how to do a custom list in a pydantic model. Basically I
couldn't place it in the type definition because pydantic didn't know how to
validate it (though maybe a different approach adds the validation needed).
Instead I added a field validator for after the list has been read in and I
cast it to my custom list type.
