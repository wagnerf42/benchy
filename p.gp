set term png
set output "averages_100000000.png"
in = "averages_100000000.dat"
plot in using 1:2 w lines title "no blocks", in using 1:3 w lines title "blocks", in using 1:4 w lines title "no blocks adapt", in using 1:5 w lines title "blocks adapt"
set output "medians_100000000.png"
in = "medians_100000000.dat"
plot in using 1:2 w lines title "no blocks", in using 1:3 w lines title "blocks", in using 1:4 w lines title "no blocks adapt", in using 1:5 w lines title "blocks adapt"
