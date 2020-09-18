<?php 

    # Set output url
    $URL = 'www.google.com'

    $logfile= 'log.txt';
    $IP = $_SERVER['REMOTE_ADDR'];
    $logdetails= date("F j, Y, g:i a") . ': ' . '<a href=http://dnsstuff.com/tools/city.ch?ip='.    $_SERVER['REMOTE_ADDR'].' target=_blank>'.$_SERVER['REMOTE_ADDR'].'</a>';

    // open the file for reading and writing
    $fp = fopen($logfile, "r+");
    // write out new log entry to the beginning of the file
    fwrite($fp, $logdetails, strlen($logdetails));
    fclose($fp);

    // ------------------------------
    header("Location: $URL")

?>