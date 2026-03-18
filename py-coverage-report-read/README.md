This project is to be used to figure out how the `.coverage` file produced by
the python coverage tool is formatted. It looks like it's SQLite3 and so I
should be able to just poke around in the database.

The file present for the project was created by py-coverage-report-generator
project.

Looks like the .coverage sqlite database stores the lines that were executed
for each file. Then the "report" step compares the executed lines from the
database against the executable lines by reading the files in and parsing the
Abstract Syntax Tree. The database is not sufficient by itself to determine
what's needed for the report.

Figuring out the connection was done with the help of ChatGPT. Sounds like you
could use coverage itself though to parse and provide the data you'd want.
Documentation at
https://coverage.readthedocs.io/en/7.13.4/api_coveragedata.html sounds like
it's just used for reading the database. It doesn't fill in the missing "what
lines were executable" piece. I think I'd be on my own for that part still.
