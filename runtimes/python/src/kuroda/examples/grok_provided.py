class Symbol:
    def __init__(self, name, is_terminal=False):
        self.name = name
        self.is_terminal = is_terminal

class Production:
    def __init__(self, left, right):
        self.left = left  # List of Symbols
        self.right = right  # List of Symbols

class KurodaGrammar:
    def __init__(self, non_terminals, terminals, productions, start):
        self.non_terminals = set(non_terminals)
        self.terminals = set(terminals)
        self.productions = productions  # List of Production
        self.start = start
        self.validate_kuroda()

    def validate_kuroda(self):
        for prod in self.productions:
            lhs_len = len(prod.left)
            rhs_len = len(prod.right)
            if lhs_len > rhs_len or rhs_len == 0:
                raise ValueError("Non-contracting violation")
            if lhs_len == 1 and rhs_len == 1:  # A → a
                if not (prod.left[0] in self.non_terminals and prod.right[0] in self.terminals):
                    raise ValueError("Invalid A → a")
            elif lhs_len == 1 and rhs_len == 2:  # A → BC
                if not all(s in self.non_terminals for s in [prod.left[0], *prod.right]):
                    raise ValueError("Invalid A → BC")
            elif lhs_len == 2 and rhs_len == 2:  # AB → CD
                if not all(s in self.non_terminals for s in [*prod.left, *prod.right]):
                    raise ValueError("Invalid AB → CD")
            else:
                raise ValueError("Invalid rule type")

    def derive(self, string, steps=100):  # Simple non-deterministic deriver (pseudo)
        current = [self.start]
        for _ in range(steps):
            for i in range(len(current)):
                for prod in self.productions:
                    if current[i:i+len(prod.left)] == prod.left:
                        new = current[:i] + prod.right + current[i+len(prod.left):]
                        if all(s.is_terminal for s in new):  # Terminal string?
                            return ''.join(s.name for s in new)
                        # Else continue deriving (in full impl, use queue/tree)
        return None  # No derivation found

# Example usage
S, A, B, C = [Symbol(n) for n in 'SABC']
a, b = Symbol('a', True), Symbol('b', True)
prods = [
    Production([S], [A, B]),  # S → AB
    Production([A, B], [A, C]),  # AB → AC
    Production([A], [a]),  # A → a
    Production([B], [b]),  # B → b (add more for full grammar)
]
grammar = KurodaGrammar([S, A, B, C], [a, b], prods, S)
print(grammar.derive([]))  # Pseudo derive start