# linux-evil-toolkit

    Linux evil toolkit e um framework de pentest que tem como objetivo 
    centralizar, padronizar e simplificar a utilização de varias ferramentas 
    por proficionais de segurança. LITK (Linux evil toolkit) dispôem de varios
    comandos comuns,como, por exemplo o dnsenum, nmap, netcat, telnet, ssh, 
    ferramentas para clonar sites, e um repositorio de paginas utilizada para
    engenharia social.

    LITK e melhor que o setoolkit? Sim, e Não, são frameworks que servem para
    a mesma coisa e fazem isso de forma semelhante, porem o LITK e feito para ser
    um alias com funções a mais, e não uma suite que até passa café.

    Dispenso script kiddies.

## Modulos

    Shell       Prove uma interface terminal para a utilização do LETK
    Core        Modulo que contem funções de pentest de fato
    Visual      Modulo de funções visuais, como banner, e funções de help

## Requisitos

    LETK pode ser usado em qualquer distribuição linux, porem as funções 
    de instalação de depedencias só estão disponiveis no Arch Linux

    O proxychains deve ser devidamente configurado.

## Comandos

    server(port)                                Criar um servidor em localhost
    ngrok(port)                                 Criar um servidor web externo
    metasploit()                                Instala o metasploit
    install                                     Instala todas as ferramentas no Arch Linux
    ssh(user, ip)                               Faz uso de ssh
    telnet(ip)                                  Faz uso de telnet
    dumptcp(pl, file)                           Faz dump de pacote da rede atual
    mac(pl)                                     Altera o endereço mac
    footprint(opt = '', host, proxy = nil)      Faz o ping scan    
    scanner(hst, proxy = nil)                   Faz Varredura de pportas, whois, encontra flahas,
                                                pequisa dns, e brute force de dns
    nmap(host, opt, proxy = nil)                Faz uso de nmap com alias TCP SYN, UDP, e bypass 
                                                de firewall
                                                                    

## Função INIT()

    Não implementado 



## Funções backend

    getErro(msg = '', command = nil)        Exibe uma mensagem em caso de erro
    proxy(props)                            Faz uso de proxy, props != null
    interpreter(command)                    Interpreta comandos do LETK e do shell

##  

          
      



