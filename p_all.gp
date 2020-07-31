set term png
set output "averages_100000000.png"
in = "averages_100000000.dat"
plot in using 1:2 w lines title "no blocks", in using 1:3 w lines title "blocks"
set output "medians_100000000.png"
in = "medians_100000000.dat"
plot in using 1:2 w lines title "no blocks", in using 1:3 w lines title "blocks"
