tui enable
layout split

# Black magic probe
target extended-remote /dev/ttyBmpGDB
monitor swdp_scan
attach 1
set mem inaccessible-by-default off

# print demangled symbols by default
set print asm-demangle on


load
compare-sections
tb main
run
focus CMD
