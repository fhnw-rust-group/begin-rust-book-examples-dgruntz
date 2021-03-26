# 05: Conditionals

## Dangling Else

    if (a == 1)
      if (b == 1)
        a = 42;
    else
      b = 42;

In Rust kein Problem da die Blöcke zwingend sind.

Andere Lösungsvarianten:
- Einrückung (Python)
- ENDIF / fi


## if-expression

Die Anweisung
    let e = if a == 15 { 1; } ;
hat den Typ `unit` da der Body mit einer leeren Anweisung endet.

Die Anweisung

    let e = if a == 15 { 1 }

führt zur Fehlermeldung

		  |
		  |     let e = if a == 15 { 1 };
		  |             ^^^^^^^^^^^^^-^^
		  |             |            |
		  |             |            found here
		  |             expected `()`, found integer
		  |
		  = note: `if` expressions without `else` evaluate to `()`
		  = help: consider adding an `else` block that evaluates to the expected type

Falls man einen expliziten Typ hinzufügt mit

    let e: () = if a == 15 { 1 }

dann wird eine andere Fehlermeldung ausgegeben:

		  |
		  |     let e: () = if a == 15 { 1 };
		  |                 -------------^--- help: consider using a semicolon here
		  |                 |            |
		  |                 |            expected `()`, found integer
		  |                 expected this to be `()`

