target extended-remote :3333
monitor arm semihosting enable
load
monitor reset halt
break main
