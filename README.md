# Allgemein
Dieser Rust-Parser für arithmetische Ausdrücke basiert auf dem im 2. Semester verwendeten Programm, das in C++ geschrieben wurde. Er erfüllt die Anforderungen, um gegebene arithmetische Ausdrücke korrekt zu formatieren und darzustellen.


# main.rs
## main
Die **main**-Methode ist der Einstiegspunkt des Programms und verweist lediglich auf die **test_parser**-Methode.

## test_parser
Die **test_parser**-Methode ruft die **test_parser_good**-Methode auf, um die Tests durchzuführen.

## test_parser_good
Die **test_parser_good**-Methode ruft die **parse_expression**-Methode für verschiedene Werte von gültigen arithmetischen Ausdrücken auf. Dies dient dazu, sicherzustellen, dass der Parser korrekt arbeitet.

## parse_expressions
Die **parse_expression**-Methode nimmt eine Zeichenkette als Argument, erstellt einen Parser für diese Zeichenkette und ruft die **display**-Methode mit dem Ergebnis auf.

## display
Die **display**-Methode bekommt eine Option von Ast-Ausdrücken **(Optional<Exp>)** als Argument und gibt den analysierten Ausdruck auf der Konsole aus. Falls die Option jedoch "none" ist, wird "nothing" ausgegeben.

# tokenizer.rs
## Token
Das Enum definiert die verschiedenen Arten von Tokens, die vom Tokenizer erkannt werden können. Folgende Tokens gibt es:

  **EOS:** repräsentiert das Ende der Eingabe
  **ZERO, ONE, TWO:** repräsentieren die Ziffern 0, 1 und 2
  **OPEN** und **CLOSE:** repräsentieren offene und geschlossene Klammern
  **PLUS** und **MULT:** repräsentieren die Operatoren Plus und Multiplikation
  **DEFAULT:** repräsentiert alle anderen unbekannten Symbole
  
## Tokenizer
Die **Tokenizer**-Struktur ist verantwortlich für das Aufteilen der Eingabezeichenkette in Tokens. Sie enthält folgende Felder:

  **position:** Speichert die aktuelle Position beim Durchlauf
  **expression:** Speichert die gesamte Eingabe
  **token:** Speichert das aktuelle Token. Dieses muss auf "pub" gesetzt werden, um in der **parse.rs** verwendet werden zu können.
  
## new
Die **new**-Methode dient hier als Konstruktor für die oben definierte Struktur. Sie bekommt eine Referenz auf den eingegebenen arithmetischen Ausdruck und erstellt hieraus eine neue Instanz des Tokenizers. Anschließend wird die **next**-Methode aufgerufen, um das erste Zeichen der Eingabe zu lesen.
### next
Die **next**-Methode ist die Hauptmethode des Tokenizers und wird verwendet, um das nächste Token in der Zeichenkette zu erkennen. Sie wird aufgerufen, um das Token an der aktuellen Position zu identifizieren und die Position entsprechend zu aktualisieren. Die Methode verwendet eine Schleife und ein **match**-Statement, um das passende Token basierend auf dem aktuellen Zeichen zu bestimmen.
### next_token
Die **next_token**-Methode ist eine Hilfsmethode, die verwendet wird, um das nächste Token zu erkennen und es im token-Feld zu speichern. Sie ruft die **next**-Methode auf, um das Token zu erhalten und zu setzen.

# ast.rs
## Exp
Das Enum definiert die verschiedenen Varianten von Ausdrücken:

  **Int:** Eine einfache ganze Zahl
  **Plus:** Eine Addition von zwei Ausdrücken e1 und e2
  **Mult:** Eine Multiplikation von zwei Ausdrücken e1 und e2
  
## eval
Die **eval**-Methode wird verwendet, um die Variable b auf set_to zu setzen für die Fälle Int und Mult.

## pretty
Die **pretty**-Methode gibt eine formatierte Zeichenkette zurück. Für den Fall Int wird die Zahl zurückgegeben. Für den Fall Plus wird der Ausdruck in Klammern zurückgegeben, wenn b true ist, andernfalls ohne Klammern.

# parser.rs
### Parser
Die **Parser**-Struktur wird verwendet, um die Texteingabe zu analysieren und in einen abstrakten Syntaxbaum umzuwandeln. Sie enthält ein Feld t vom Typ Tokenizer.

## new
Die **new**-Methode dient als Konstruktor, um eine neue Parser-Instanz zu erstellen. Als Argument nimmt sie eine Zeichenkette, die den Ausdruck enthält, der vom Parser analysiert werden soll. Die Tokenizer-Instanz der Struktur wird initialisiert.

## parse
Die **parse**-Methode startet den Prozess zur Analyse, um die Eingabe in einen abstrakten Syntaxbaum (AST) umzuwandeln. Falls dieser Vorgang nicht erfolgreich war, wird "none" zurückgegeben.

## parse_e
Die **parse_e**-Methode analysiert einen Teil des Ausdrucks. Bei Erfolg wird eine Option<Exp> zurückgegeben, andernfalls "none".

## parse_e2
Die **parse_e2**-Methode dient dazu, Ausdrücke zu analysieren, die mit einem Pluszeichen beginnen.

## parse_t
Die **parse_t**-Methode analysiert einen Teil des Ausdrucks. Bei Erfolg wird eine Option<Exp> zurückgegeben, andernfalls "none".

## parse_t2
Die **parse_t2**-Methode dient dazu, Ausdrücke zu analysieren, die mit einem Multiplikationszeichen beginnen.

## parse_f
Die **parse_f**-Methode dient dazu, F-Ausdrücke zu analysieren. Diese repräsentieren die atomaren Bestandteile des gegebenen Ausdrucks.
