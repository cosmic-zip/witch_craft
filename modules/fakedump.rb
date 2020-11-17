#-------------------------------------------------------------
#
#                     Linux Evil Toolkit
# 
#                          By v0id
#
#
#-------------------------------------------------------------

module FakeDump

    def password()
        letters = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ'
        numbers = '0123456789'
        symbols = '`~!@#$%^&*()_-+=[{]}|;:,.<>/?'
        characters = letters + numbers + symbols
        temp = nil
        for (var x = 0; x < 12; x++)
            randomNumber = rand(0..9999)
            randomFloatingIndex = randomNumber * characters.length
            randomIndex = Math.floor(randomFloatingIndex)
            randomCharacter = characters.charAt(randomIndex)
            temp += randomCharacter
        end 
        return temp
    end

    def cpf()
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
        puts cpf
    end

    wom = ['Alice', 'Júlia', 'Isabella', 'Manuela', 'Maria', 'Eduarda', 'Lara', 'Lorena', 'Yasmin', 'Nicole', 'Cecília', 'Emanuelly', 'Clara', 'Rebeca', 'Maria', 'Catarina', 'Valentina', 'Beatriz', 'Lara', 'Júlia', 'Heloísa', 'Lívia', 'Bianca', 'Ana', 'Lorena', 'Isadora', 'Yasmin', 'Bárbara', 'Abigail', 'Maria', 'Bruna', 'Nicole', 'Mariana', 'Sara', 'Olívia', 'Laila', 'Rebeca', 'Cloe', 'Ariela', 'Catarina', 'Micaela', 'Fernanda', 'Emanuela', 'Eva', 'Yara', 'Juliana', 'Ingrid', 'Angelina', 'Milene', 'Adriana']

    men = ['Zoe', 'Davi', 'Pedro', 'Bernardo', 'Heitor', 'Samuel', 'Theo', 'Pietro', 'Benjamin', 'Lucca', 'Caio', "João 'Miguel'," 'Francisco', 'Antônio', "Enzo 'Gabriel'," "Davi 'Lucca'," 'Thiago', 'Thomas', 'Enrico', 'Artur', 'Enzo', 'Lorenzo', 'Theo', 'Guilherme', 'Pedro', 'Henrique', 'Nicolas', 'Rafael', 'Caio', 'Joaquim', 'Leonardo', 'Bruno', 'Vicente', 'Diego', 'Fernando', 'Kelvin', 'Renan', 'Francisco', 'Valentim', 'Manuel', 'Paulo', 'Santiago', 'Wilian', 'Abraão', 'Renato', 'Edgar', 'Juliano', 'Maurício', 'Caetano', 'Anderson']

    wom_name = ['Sofia', 'Isabella', 'Camila', 'Valentina', 'Valeria', 'Mariana', 'Luciana', 'Daniela', 'Gabriela', 'Victoria', 'Martina', 'Lucia', 'Ximena/Jimena', 'Sara', 'Samantha', 'Maria José', 'Emma', 'Catalina', 'Julieta', 'Mía', 'Antonella', 'Renata', 'Emilia', 'Natalia', 'Zoe', 'Nicole', 'Paula', 'Amanda', 'María Fernanda', 'Emily', 'Antonia', 'Alejandra', 'Juana', 'Andrea', 'Manuela', 'Ana Sofia', 'Guadalupe', 'Agustina', 'Elena', 'María', 'Bianca', 'Ariana', 'Ivanna', 'Abril', 'Florencia', 'Carolina', 'Maite', 'Rafaela', 'Regina', 'Adriana', 'Michelle', 'Alma', 'Violeta', 'Salomé', 'Abigail', 'Juliana', 'Valery', 'Isabel', 'Montserrat', 'Allison', 'Jazmín', 'Julia', 'Lola', 'Luna', 'Ana', 'Delfina', 'Alessandra', 'Ashley', 'Olivia', 'Constanza', 'Paulina', 'Rebeca', 'Carla', 'María Paula', 'Micaela', 'Fabiana', 'Miranda', 'Josefina', 'Laura', 'Alexa', 'María Alejandra', 'Luana', 'Fátima', 'Sara Sofía', 'Isidora', 'Malena', 'Romina', 'Ana Paula', 'Mariangel', 'Amelia', 'Elizabeth', 'Aitana', 'Ariadna', 'María Camila', 'Irene', 'Silvana', 'Clara', 'Magdalena', 'Sophie', 'Josefa']

    men_name = ['Santiago', 'Sebastián', 'Matías', 'Mateo', 'Nicolás', 'Alejandro', 'Diego', 'Samuel', 'Benjamín', 'Daniel', 'Joaquín', 'Lucas', 'Tomas', 'Gabriel', 'Martín', 'David', 'Emiliano', 'Jerónimo', 'Emmanuel', 'Agustín', 'Juan Pablo', 'Juan José', 'Andrés', 'Thiago', 'Leonardo', 'Felipe', 'Ángel', 'Maximiliano', 'Christopher', 'Juan Diego', 'Adrián', 'Pablo', 'Miguel Ángel', 'Rodrigo', 'Alexander', 'Ignacio', 'Emilio', 'Dylan', 'Bruno', 'Carlos', 'Vicente', 'Valentino', 'Santino', 'Julián', 'Juan Sebastián', 'Aarón', 'Lautaro', 'Axel', 'Fernando', 'Ian', 'Christian', 'Javier', 'Manuel', 'Luciano', 'Francisco', 'Juan David', 'Iker', 'Facundo', 'Rafael', 'Alex', 'Franco', 'Antonio', 'Luis', 'Isaac', 'Máximo', 'Pedro', 'Ricardo', 'Sergio', 'Eduardo', 'Bautista', 'Miguel', 'Cristóbal', 'Kevin', 'Jorge', 'Alonso', 'Anthony', 'Simón', 'Juan', 'Joshua', 'Diego Alejandro', 'Juan Manuel', 'Mario', 'Alan', 'Josué', 'Gael', 'Hugo', 'Matthew', 'Ivan', 'Damián', 'Lorenzo', 'Juan Martín', 'Esteban', 'Álvaro', 'Valentín', 'Dante', 'Jacobo', 'Jesús', 'Camilo', 'Juan Esteban', 'Elías']

    dub_name = ['D’Ávila', 'da Aldeia', 'da Bandeira', 'da Barra', 'da Barranca', 'da Conceição', 'da Costa', 'da Cruz', 'da Cunha', 'da Fonseca', 'da Fontoura' 'da Fronteira', 'da Gama', 'da Luz', 'da Madureira', 'da Maia', 'da Mata', 'da Mota', 'da Nóbrega', 'da Paz', 'da Penha', 'da Rocha', 'da Rosa', 'da Silva', 'da Silveira', 'da Terra', 'da Veiga', 'Dantas', 'das Neves', 'Datena', 'de Assunção', 'de Borba', 'de Lara', 'de Lins', 'de Oliveira', 'de Quadros', 'de Sá', 'Dias', 'Dieckmann', 'Diegues', 'do Prado'] 
 
    last_name = ['Abravanel', 'Albuquerque', 'Alencar', 'Almada', 'Álvares', 'Alves', 'Amoreira', 'Andrade', 'Annenberg', 'Antena', 'Antunes', 'Aparício', 'Arantes', 'Aroeira', 'Arriaga', 'Assumpção', 'Assunção', 'Banheira', 'Barata', 'Barcellos', 'Barcelos', 'Barreira', 'Barreto', 'Barroso', 'Bastos', 'Beltrão', 'Bentes', 'Bernardes', 'Bernardi', 'Bial', 'Bittencourt', 'Bittencourt', 'Boaventura', 'Bonfim', 'Bongiovanni', 'Bonner', 'Bouças', 'Brites', 'Brum', 'Bulhões', 'Cabreira', 'Cachoeira', 'Caldas', 'Caldeira', 'Camacho', 'Campos', 'Carneiro', 'Carreira', 'Carvalheira', 'Carvalho', 'Caseira', 'Casqueira', 'Castanheira', 'Castanho', 'Castelo', 'Castiel', 'Cerqueira', 'Chapelin', 'Chaves', 'Close', 'Coqueiro', 'Corte', 'Coutinho', 'Crespo', 'Dolabella', 'Domingues', 'Dorneles', 'dos', 'Reis', 'Drummond', 'Duarte', 'Duarte', 'Espinheira', 'Espinhosa', 'Esteves', 'Evelyn', 'Fagundes', 'Falabella', 'Fernandes', 'Ferraço', 'Ferrari', 'Ferraz', 'Ferreira', 'Figueira', 'Filgueira', 'Fischer', 'Fogaça', 'Fontenelle', 'Fontes', 'Fontinhas', 'Gabeira', 'Galego', 'Galvão', 'Ganzarolli', 'Garcês', 'Garcez', 'Gentil', 'Geraldes', 'Giácomo', 'Gimenez', 'Godinho', 'Godins', 'Gomes', 'Gomes', 'Gomide', 'Gonçalves', 'Goulart', 'Grotas', 'Guedes', 'Guterres', 'Henriques', 'Hermingues', 'Hernandes', 'Hickmann', 'Jaques', 'Junqueira', 'Kannenberg', 'Lafaiete', 'Lambertini', 'Laranjeira', 'Leiria', 'Lessa', 'Lessa', 'Liberato', 'Limeira', 'Lins', 'Lobos', 'Lombardi', 'Longuinho', 'Lopes', 'Louzada', 'Macieira', 'Madeira', 'Maldonado', 'Mangueira', 'Marcondes', 'Marinho', 'Marins', 'Marques', 'Marques', 'Martim', 'Martins', 'Matoso', 'Mazzaropi', 'Medeiros', 'Meira', 'Meireles', 'Mendanha', 'Mendes', 'Meneghel', 'Menendes', 'Modesto', 'Moniz', 'Monteira', 'Moraes', 'Monteiro', 'Monteiro', 'Montenegro', 'Moraes', 'Morais', 'Moreira', 'Moreno', 'Moscovis', 'Müller', 'Munhoz', 'Muniz', 'Muniz', 'Negrão', 'Nogueira', 'Noronha', 'Novaes', 'Nunes', 'Ordonhes', 'Ornelas', 'Ouriques', 'Paes', 'Palhares', 'Palmeira', 'Parreira', 'Passarinho', 'Pedroso', 'Peixoto', 'Pereira', 'Peres', 'Pimenta', 'Pinheira', 'Porteira', 'Porto', 'Quaresma', 'Quarteira', 'Raia',- 'Ramalho', 'Rameira', 'Ramires', 'Ramos', 'Rebouças', 'Rêgo', 'Regueira', 'Resende', 'Reymond', 'Rezende', 'Ribas', 'Ribas', 'Ribeira', 'Ribeiro', 'Riche', 'Rios', 'Rios', 'Rodrigues', 'Rolim', 'Roriz', 'Roriz', 'Roseira', 'Sais', 'Sales', 'Sampaio', 'Sanches', 'Sanches', 'Santana', 'Schmütz', 'Schoemberger', 'Schumacher', 'Sheherazade', 'Silveira', 'Silverstone', 'Simão', 'Simões', 'Siqueira', 'Siqueiro', 'Soares', 'Soeira', 'Solimões', 'Steves', 'Tavares', 'Taveira', 'Teixeira', 'Teles', 'Timberg', 'Torres', 'Trindade', 'Vargas', 'Vasconcelos', 'Vasques', 'Velasques', 'Veles', 'Vidal', 'Videira', 'Vieira', 'Vilela', 'Vimaranes', 'Vitti', 'Viveiros', 'Werneck', 'Ximenes']

