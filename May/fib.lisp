(
    defun fib
    (
        i; The index of the fibonacci to find
    )
    "Inefficient Fibonacci Finder"
    (
        cond
        ((= i 0) (return-from fib 1))
        ((= i 1) (return-from fib 1))
        (t
            (return-from fib (+ (fib (- i 1)) (fib (- i 2))))
        )
    )
)


(
    defun fib2
    (
        i; The index of the fibonacci to find
    )
    "More Efficient Fibonacci Finder"
    (
        cond
        ((= i 1) (return-from fib2 (values 1 0)))
        (t
            (setf (values last1 last2) (fib2 (- i 1)))
            (return-from fib2 (values (+ last1 last2) last1))
        )
    )
)

(
    write-line (write-to-string (fib2 100))
)