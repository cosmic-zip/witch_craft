# CONFIG FILE FOR AUTOMATION 
# Set time
$time = Time.now.strftime("%d-%m-%Y_%H-%M")
# Write results in file?
$documentation = false
# Enable proxy
$proxy         = false
# Set target. Ex: www.google.com
$target        = 'www.google.com'
# Set target ip. Ex: 192.0.0.1
$ip            = '1.1.1.1'
# automatically generate new ip. [true| false]
$silent_mode   = false
# Change mac [true| false]
$change_mac = false
#Set network interface name: [wlan0, wlp2s0]: "
$interface = false