<?php 

    # Set output url
    $URL = 'www.google.com'

    $logfile = 'log.txt';
    $date = date("F j, Y, g:i a");
    $ip = $_SERVER['REMOTE_ADDR'];
    $dnsstuff = "<a href=http://dnsstuff.com/tools/city.ch?ip=$iptarget=_blank>$ip</a>";

    $logdetails = "DATE: $date | USER: $ip | ";

    // open the file for reading and writing
    $fp = fopen($logfile, "r+");
    // write out new log entry to the beginning of the file
    fwrite($fp, $logdetails, strlen($logdetails));
    fclose($fp);

    // ------------------------------
    header("Location: $URL")

?>