# Rust Lox

A rust implementation of lox

Todo:

- [ ] Current locals keys do not contain whole token, make sure they do
- [ ] Write tests for methods unattached from their classes

```c
class Toad {
  tongue() {
    return true;
  }

  fry() {
    return 42;
  }
}

let weapon = Toad().tongue;
print weapon(); // true
```
