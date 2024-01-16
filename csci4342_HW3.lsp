;Description: This program takes two user inputs, a start and an end, and 
; iterates from the start value to the end value and determines if each is
; a multiple of 3 or 5. If 3, it prints "Fizz"; if 5, it prints "Buzz"; 
; if both, it prints "Fizz Buzz"
;Assume user inputs valid range with start < end
;https://www.geeksforgeeks.org/introduction-to-lisp/ used for reference

(defun fizzbuzz (start end)
    (cond
        ((and(= 0 (mod start 3))(= 0 (mod start 5)))(print "Fizz Buzz"))
        ((= 0 (mod start 3))(print "Fizz"))
        ((= 0 (mod start 5))(print "Buzz"))
    )
    (setq start (+ start 1))
    (if (not(> start end))(fizzbuzz start end))
)