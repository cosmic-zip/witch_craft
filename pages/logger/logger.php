<?php 

    function logger($props = ''){
        # Set output url
        if($props != ''){
            $props = " $props |";
        }
        $logfile = 'log.txt';
        $date = date("| Y/m/d | H:i:s");
        $ip = $_SERVER['REMOTE_ADDR'];
        $logdetails = "$date | $ip |$props\n";

        // open the file for reading and writing
        $fp = fopen($logfile, "a");
        // write out new log entry to the beginning of the file
        fwrite($fp, $logdetails);
        fclose($fp);

        // ------------------------------
    }

?>