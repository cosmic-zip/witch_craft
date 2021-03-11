module FakePages 

    def gem_fake_page 

        pathkp = set('Set path to project: ["like ./my-fake-pages"]')
        title = set('Set page title: [without spaces or especial characters]:')
        content = set('Set contet text: [lorem]: ')
        output = set('Set output file:')
        loadline = set('Set articles numbers: [default 500]:')

        if content == nil 
            content = "0x00000000x00000000x00000000x00000000x00000000x00000000x00000000x00000000x00000000x00000000x00000000x00000000x00000000x0000000
            0x00000000x00000000x00000000x00000000x00000000x00000000x00000000x00000000x00000000x00000000x00000000x00000000x00000000x00000000x0000000
            0x00000000x00000000x00000000x00000000x00000000x00000000x00000000x00000000x00000000x00000000x00000000x00000000x00000000x00000000x0000000"
        end

        loadline == nil ? loadline = 500 : nil

        style = "<style>.ft{ font-size: 0px; }</style>"        
        header = "<html>\n<head>\n<title>#{title}</title>\n#{style}\n</head>\n<body>\n"
        footer = "<div>\n<hr>\n</div>\n</body>\n</html>"
        
        system "mkdir -p #{pathkp}/pages"
        system "echo '#{header}' >> #{pathkp}/#{output}.html"
        count = 1
        while count < loadline.to_i
            load = "<h1 class='ft'><a href='pages/#{title}#{count}.html'>load-number#{rand(100..999999)}-#{count}</a></h1>"
            system "cp database/lorem.html #{pathkp}/pages/#{content}#{count}.html"
            system "echo '#{load}' >> #{pathkp}/#{output}.html"
            count += 1
        end

        system "echo '#{footer}' >> #{pathkp}/#{output}.html"
        puts('Done')

    end

end
