//// [compoundArithmeticAssignmentWithInvalidOperands.ts]
var E;
!function(E) {
    E[E.a = 0] = "a", E[E.b = 1] = "b";
}(E || (E = {})), E.a, E.a, E.a, E.a;
