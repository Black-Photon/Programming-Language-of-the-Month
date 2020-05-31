(defmacro run (prog)
    "Prints a statement to say it is about to start before running a program"
    (write-line "Starting Sequence")
    (eval prog)
    (write-line "Sequence Complete")
)
(run 
    (progn
        (setq a 5)
        (setq b 6)
        (write (+ b a)) (terpri)
    )
)