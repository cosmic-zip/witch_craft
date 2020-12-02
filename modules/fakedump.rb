#-------------------------------------------------------------
#
#                     Linux Evil Toolkit
# 
#                          By v0id
#
#
#-------------------------------------------------------------

module FakeDump

    def fakeEmail 
        server = ['gmail', 'hotmail', 'outlook', 'protonmail', 'tutonata', 'yahoo', 'simpleMail']
    end

    def cpf 
        value = 0; total = 0; value_segundo = 0; total_segundo = 0
        digito_1 = [10,9,8,7,6,5,4,3,2]
        cpf = Array.new(9) {| i | i = rand(10)}
        9.times do |val|
            total = digito_1[val] * cpf[val]
            value += total
        end
        primeiro_digito = value%11
        if primeiro_digito < 2
            primeiro_digito = 0
        else
            primeiro_digito = 11 - primeiro_digito
        end
        digito_1.push(11).sort!.reverse!
        cpf.push(primeiro_digito)
        10.times do |value|
            total_segundo = digito_1[value] * cpf[value]
            value_segundo += total_segundo
        end
        segundo_digito = value_segundo%11
        if segundo_digito < 2
            segundo_digito = 0
        else
            segundo_digito = 11 - segundo_digito
        end
        cpf.pop
        cpf = "#{cpf.join("")}#{primeiro_digito}#{segundo_digito}"
        return cpf
    end

    def gem(number = 1)
        count = 0 
        while count <= number 
            
            # types
            # 1 name 1 middle 1 final | 30% 0..2 
            # 1 name 1 middle 1 final | 30% 3..5
            # 2 name 2 middle 1 final | 20% 6..8
            # 1 name 2 middle         | 10% 8..9
            # 2 name 2 middle         | 10% 9..10
                   
            first_name,middle_name,last_name,final_name = nil
            
            sex = rand(0..1)
            if sex == 0 #Men
                first_name = $men[rand(0..$men.length)]
            else #Women
                first_name = $wom[rand(0..$wom.length)]
            end
            middle_name = $middle[rand(0..$middle.length)]
            last_name = rand(0..10)
            if last_name < 6 # Add last name
                last_name = $middle[rand(0..$middle.length)]
            else
                last_name = nil
            end
            final_name = rand(0..10)
            if final_name < 9 # Add Final name
                final_name = $final[rand(0..$final.length)]
            else
                final_name = nil
            end
            count += 1
            return "#{first_name} #{middle_name} #{last_name} #{final_name}"
        end
    end

    def idhash(string = 'monkey')
        key = '23942849023423480392948394032984230489'
        digest = OpenSSL::Digest.new('sha256')
        return OpenSSL::HMAC.hexdigest(digest, key, string)
    end
      
    def simple_dump(size)
        count = 0 
        while count < size
            id = "id:#{rand(123..999) + count}"
            data = "data:00/00/00"
            time = "time:00:00"
            string = "#{id} | #{data} | #{time} | #{gem()} | #{cpf()}"
            system "echo '#{string}' >> fakedump.txt"
        end
    end

    def dump_xml(size = 1)
        #person
        count = 0
        while count < size
            xml = Builder::XmlMarkup.new( :indent => 2 )
            xml.instruct! :xml, :encoding => "UTF-8"
            xml.database do |p|
                p.id idhash(gem(1))
                p.insert_date "NULL"
                p.update_date "NULL"
                p.cpf cpf()
                p.name gem(1)
                p.password idhash(rand(9999..999999))
            end
            system "echo '#{xml.database}' >> fakedump.xml"
        end
    end      
    
    def simple_call() 
        print "Set output mothod: [xml|txt]: "
        met = gets.chomp.to_s
        print "Set output size (lines in file): "
        siz = gets.chomp.to_i
        if met == 'xml'
            dump_xml(siz)
        else
            simple_dump(siz)
        end
    end

end
