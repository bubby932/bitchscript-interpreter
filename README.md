# A terrible interpreted language, initially concieved as a Computer Science project but since expanded out of spite.

BitchScript is designed with absolutely 0 goals in mind, other than spite.

Syntax example:

```
let a = 0
let b = "String Literal"
let c = true

if c {
  print "First loop iteration."
  let c = false
  goto 12
} else {
  print "Second loop iteration - exiting."
}

print "Program exit."
```


# Reserved keywords

* `let`
* `true`
* `false`
* `if`
* `else`

# Syntax Rules

* Variables must start with an alphabetic [aA-zZ] character. Variable names can contain numbers, but numbers may not be the first character.
* Variables cannot be set using C-Style `Identifier = Value` syntax, they must be wholly redefined using `let`.


# Todo List

- [ ] Add expression support using ().  
- [ ] Add label support using `Identifier` followed by a colon.  
- [ ] Write package manager.  
- [ ] Add `import` statement.  
