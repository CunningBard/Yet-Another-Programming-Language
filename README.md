# **OBSOLOTE** Yet-Another-Programming-Language
A language that's written in rust, that temporarily compiles to python

# NOTE: this is now obsolete, a re write is on a different repo(https://github.com/CunningBard/YAPL_rewrite) 

# Road Map
### Basics
- [x] Basic Variable Definition
- [ ] Basic Function Definition
- [ ] Basic Function Call (what works: calling sys defined functions)

### Full
- [ ] Full Variable Definition
- [ ] Full Function Definition
- [ ] Full Function Call

### Others
- [ ] Variable Definition
- [ ] Variable Definition 


### Meaning
Basic Variable Definition
- cant define a variable from function returns
- works `var name > int = 123;`
- doesnt works `var name > int = name();`

Full Variable Definition
- can define a variable from function returns
- works `var name > int = 123;`
- also works `var name > int = name();`

# Base
```
var name_var > int = 32;

func name(arg: int) > int {
  return arg;
}

var name_var_2 > int = name(32);
print(name_var_2);
/
\\ this is a comment
\\ c equivalent


int name_var = 32;

int name(int arg) {
  return arg
}

int name_var_2 = name(32);
printf("%i", name_var_2);
/
```
# temporary?
```
\ python:print(a) \ 
\the line above will compile to python as 'print(a)' instead of nothing\

\-------------------\

var name_var > int = 32;
\python:print(name_var) \

\-------------------\
\
# in python
name_var: int = 32
print(name_var)

\

```
