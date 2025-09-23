# when updating the test suite make sure to `<c-v>` `r0gvg<c-a>`

echo -e 00 'foo' | bin/ltrep 'a' >> test.act
echo -e 01 'bar' | bin/ltrep 'a' >> test.act
echo -e 02 'foo' | bin/ltrep 'a' - >> test.act
echo -e 03 'bar' | bin/ltrep 'a' - >> test.act

echo -e 04 'foo' | bin/ltrep -i 'a' >> test.act
echo -e 05 'bar' | bin/ltrep -i 'a' >> test.act
echo -e 06 'BAZ' | bin/ltrep -i 'a' >> test.act
echo -e 07 'foo' | bin/ltrep -v 'a' >> test.act
echo -e 08 'bar' | bin/ltrep -v 'a' >> test.act
echo -e 09 'foo' | bin/ltrep -vi 'a' >> test.act
echo -e 10 'bar' | bin/ltrep -vi 'a' >> test.act
echo -e 11 'BAZ' | bin/ltrep -vi 'a' >> test.act
echo -e 12 'BAZ' | bin/ltrep -v -i 'a' >> test.act
echo -e 13 'foo' | bin/ltrep -vxi '.*a' >> test.act
echo -e 14 'bar' | bin/ltrep -vxi '.*a' >> test.act
echo -e 15 'BAZ' | bin/ltrep -vxi '.*a' >> test.act
echo -e 16 'aa' | bin/ltrep -vxi '.*a' >> test.act
echo -e 17 'AA' | bin/ltrep -vxi '.*a' >> test.act
echo -e 18 'AA' | bin/ltrep -v -xi '.*a' >> test.act

echo -e 19 'foo' | bin/ltrep -S 'a' >> test.act
echo -e 20 'bar' | bin/ltrep -S 'a' >> test.act
echo -e 21 'BAZ' | bin/ltrep -S 'a' >> test.act
echo -e 22 'foo' | bin/ltrep -S 'A' >> test.act
echo -e 23 'bar' | bin/ltrep -S 'A' >> test.act
echo -e 24 'BAZ' | bin/ltrep -S 'A' >> test.act

echo -e 25 '-W' | bin/ltrep '\-W' >> test.act
echo -e 26 '-W' | bin/ltrep -F -- '-W' >> test.act
echo -e 27 '-W' | bin/ltrep -Fv -- '-W' >> test.act
echo -e 28 'a+' | bin/ltrep -Fi 'a+' >> test.act
echo -e 29 'A+' | bin/ltrep -Fi 'a+' >> test.act

echo -e 30 'ab\ncd' | bin/ltrep -n '' >> test.act
echo -e 31 'ab\ncd' | bin/ltrep -Hn '' >> test.act
echo -e 32 'ab\ncd\ne' | bin/ltrep -c '' >> test.act
echo -e 33 'ab\nac\ne' | bin/ltrep -c 'a' >> test.act
echo -e 34 'ab\ncd\ne' | bin/ltrep -nc '' >> test.act
echo -e 35 'ab\ncd\ne' | bin/ltrep -Hc '' >> test.act
echo -e 36 'ab\ncd\ne' | bin/ltrep -vc '' >> test.act
echo -e 37 'diff\t' | bin/ltrep 'diff\s' test.sh >> test.act
echo -e 38 'diff\t' | bin/ltrep 'diff\s' - test.sh >> test.act
echo -e 39 'diff\t' | bin/ltrep 'diff\s' test.sh - >> test.act
echo -e 40 'diff\t' | bin/ltrep -c 'diff\s' test.sh - >> test.act
echo -e 41 'diff\t' | bin/ltrep -Hn 'diff\s' test.sh - >> test.act

echo -e 42 'bar' | bin/ltrep -xp 'a' >> test.act
echo -e 43 'bar' | bin/ltrep -px 'a' >> test.act
echo -e 44 'BAR' | bin/ltrep -is 'a' >> test.act
echo -e 45 'BAR' | bin/ltrep -si 'a' >> test.act
echo -e 46 'BAR' | bin/ltrep -Ss 'a' >> test.act
echo -e 47 'BAR' | bin/ltrep -sS 'a' >> test.act
echo -e 48 'bar' | bin/ltrep -Si 'A' >> test.act
echo -e 49 'bar' | bin/ltrep -iS 'A' >> test.act
echo -e 50 'bar' | bin/ltrep -nN 'a' >> test.act
echo -e 51 'bar' | bin/ltrep -Nn 'a' >> test.act
echo -e 52 'bar' | bin/ltrep -HI 'a' >> test.act
echo -e 53 'bar' | bin/ltrep -IH 'a' >> test.act

diff test.exp test.act
# cp test.act test.exp # for updating the test suite
rm test.act
