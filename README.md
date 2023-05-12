# the Cool Programming language

## lexical rules

The lexical units of Cool are integers, type identifiers, object identifiers, special notation, strings, keywords, and white space.

Some base regex expr defination is:

    Digits = [0-9]
    Lowercase = [a-z]
    Uppercase = [A-Z]
    Letters = {Lowercase}|{Uppercase}
    Asciis = [ -~]

| Token Class        | Description                                                  | regex expr                                                   |
| ------------------ | ------------------------------------------------------------ | ------------------------------------------------------------ |
| integers           | Integers are non-empty strings of digits 0-9.                | {Digits}{Digits}*                                            |
| type identifiers   | Identifiers are strings (other than keywords) consisting of letters, digits, and the underscore character.Type identifiers begin with a capital letter. | {Uppercase}({Letters}\|{Digits}\|_)*                         |
| object identifiers | Object identifiers begin with a lower e letter.              | {Lowercase}({Letters}\|{Digits}\|_)*                         |
| strings            | Strings are enclosed in double quotes "...". Within a string, a sequence ‘\c’ denotes the character ‘c’, with the exception of the following:A string may not contain EOF. A string may not contain the null (character \0). Any other character may be included in a string. Strings cannot cross file boundaries. | "{Asciis}*"                                                  |
| keywords           | class, else, false, fi, if, in, inherits, isvoid, let, loop, pool, then, while, case, esac, new, of, not, true. Except for the constants true and false, keywords are case insensitive.To conform to the rules for other objects, the first letter of true and false must be lowercase; the trailing letters may be upper or lower case. | class\|else\|false\|fi\|if\|in\|inherits\|isvoid\|let\|loop\|pool\|then\|while\|case\|esac\|new\|of\|not\|true |
| white space        | White space consists of any sequence of the characters: blank (ascii 32), \n (newline, ascii 10), \f (form feed, ascii 12), \r (carriage return, ascii 13), \t (tab, ascii 9), \v (vertical tab, ascii 11). | ( \|\n\|\r\|\f\|\t\|\v)*                                     |
| comments           | There are two forms of comments in Cool. Any characters between two dashes “--” and the next newline (or EOF, if there is no next newline) are treated as comments. Comments may also be written by enclosing text in (∗ . . . ∗). The latter form of comment may be nested. Comments cannot cross file boundaries. | (--{Asciis}*\n)\|\(\*{Asciis}*\*\)                           |
| special notation   | ['(', ')', '{', '}', '.', ':', '<-', ',', ';', '+', '-', '*', '/', '<', '<=', '=', '>=', '>', '~', '&', '\|'] | \\(\|\\)\|{\|}\|\\.\|:\|<-\|,\|;\|+\|-\|\\*\|/\|<\|<=\|=\|>=\|>\|~\|&\|\\ |
|                    |                                                              |                                                              |



