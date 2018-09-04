tui enable
layout split

# Black magic probe
target extended-remote /dev/ttyBmpGDB
monitor swdp_scan
attach 1

# print demangled symbols by default
set print asm-demangle on


load
compare-sections
tb main
run
focus CMD
