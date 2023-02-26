# MozJs runner

This is a project attempting to run spidermonkey similar to how node js runs v8. 


# What do we have?

There is a program which creates a run time and then runs a small script inline and returns a value.

# How to test?

```
$ cargo build

```

# Building
```
$ cargo build
```

# Running

```
$ ./target/debug/mozjs-runner "`cat jsExp/utcDate.js`"
Input script -> "let d = new Date()\nd.toUTCString()"
Sun, 26 Feb 2023 08:41:15 GMT
```

```
$ ./target/debug/mozjs-runner "1000 * 5"
Input script -> "1000 * 5"
5000
```

# TODO

* Get a script to output Date.now()
  >> Done
* Figure out how to get character out the string length.
  >> Done
* Figure out how to get character out of the spider monkey heap.
  >> Done
* Write a function to convert JSString to rust string.

* Accept javascript input as command line argument.

* Get a script new Date(Date.now()) to work

