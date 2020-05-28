(defun main ()
    (write-string "Enter First Number: ")
    (setq a (parse-integer (read-line)))
    (write-string "Enter Second Number: ")
    (setq b (parse-integer (read-line)))
    (format t "The Answer is ~D~%" (+ a b) )
)

(defun try-main ()
   (restart-case
        (main)
        (continue () nil)
   )
)

(loop
    (handler-bind
        (
            (parse-error
                #'(lambda (x)
                    (format t "The string you input is invalid~%")
                    (invoke-restart 'continue)
                )
            )
        )
        (try-main)
    )
)