module NumberMenu

    def menu
        prYellow "[0]    help                [16]   cls                 "
        prYellow "[1]    rsh --cow           [17]   sdown               "
        prYellow "[2]    plugin --fpage      [18]   rm                  "
        prYellow "[3]    gem --g rg          [19]   cp                  "
        prYellow "[4]    gem --g cpf         [20]   kl --init           "
        prYellow "[5]    gem --g name        [21]   kl --cover          "
        prYellow "[6]    gem --g fakedb      [22]   kl --compress       "
        prYellow "[7]    vs --v banner       [23]   kl --extract        "
        prYellow "[8]    vs --v dns          [24]   kl ---ginfo         "
        prYellow "[9]    vs --v lfile        [25]   kl --dns --brute    "
        prYellow "[10]   vs --v lfolder      [26]   kl --dns --scan     "
        prYellow "[11]   vs --v wfile        [27]   kl --dir --scan     "
        prYellow "[12]   vs --v luss         [28]   kl --smap           "
        prYellow "[13]   vs --v tsearch      [29]   kl --maclookup      "
        prYellow "[14]   vs --v talt         [30]   kl --myip           "
        prYellow "[15]   ls                  \n\n"
    end

    def help
        prCyan 'help                    | Help'
        prCyan 'cow                     | Reset shell view'
        puts ''
        prCyan 'plugin --fpage          | Generate fake website for bypass monetization systems  '
        puts ''
        prCyan 'gem --g rg              | Generate fake Brazilian rg'
        prCyan 'gem --g cpf             | Generate fake Brazilian'
        prCyan 'gem --g name            | Generate fake Brazilian/Portuquese/Spanish name'
        prCyan 'gem --g fakedb          | Generate fake database '
        puts ''
        prCyan 'vs --v banner           | Show Banner'
        prCyan 'vs --v dns              | Show Web dns search engines'
        prCyan 'vs --v lfile            | Show useful linux file '
        prCyan 'vs --v lfolder          | Show useful linux folders'
        prCyan 'vs --v wfile            | Show useful Windows files'
        prCyan 'vs --v luss             | Show linux useful information '
        prCyan 'vs --v tsearch          | Show Tor search engines'
        prCyan 'vs --v talt             | Show Tor alternatives'
        puts ''
        prCyan 'kl --init               | Set global target variable   '
        prCyan 'kl --cover              | Cover your logs  '
        prCyan 'kl --compress           | Compress files       '
        prCyan 'kl --extract            | Extract files       '
        prCyan 'kl ---ginfo             | Grep banners and domain information'
        prCyan 'kl --dns --brute        | Dns brute force           '
        prCyan 'kl --dns --scan         | Dns scanner      '
        prCyan 'kl --dir --scan         | Dir scanner       '
        prCyan 'kl --smap               | Simple map'
        prCyan 'kl --maclookup          | Mac address lookup'
        prCyan 'kl --myip               | Show your ip'
        puts "\n\n"
    end
 
end 