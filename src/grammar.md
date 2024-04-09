# Grammar for mark

```
expression     → assignment ;
assignment     → IDENTIFIER "=" expr 
               | equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")"
               | IDENTIFIER ;


program        → declaration* EOF ;
declaration    → varDecl 
               | statement;
statement      → exprStmt
               | printStmt
               | blockStmt ;

exprStmt       → expression ";" ;
printStmt      → "print" expression ";" ;
varDecl        → "var" IDENTIFIER "=" expr ;
blockStmt      → "{" declaration* "}" ;
```
