echo hello
pwd
ls
alias ll="ls -la"
ll
unalias ll
export TEST=hello
echo $TEST
unset TEST
echo hello | cat
printf "apple\nbanana\napple\n" | grep apple | wc -l
echo hello > /tmp/jodsh-warning-test.txt
cat /tmp/jodsh-warning-test.txt
true && echo success
false || echo fallback
echo *.txt
echo $(pwd)
printf "one\ntwo\nthree\n" | cat
printf "one\ntwo\nthree\n" | grep two
printf "one\ntwo\ntwo\nthree\n" | grep two | wc -l
