# linux-evil-toolkit


![LINUX EVIL TOOLKIT](https://user-images.githubusercontent.com/36008397/92520390-83da0980-f1e9-11ea-821a-4f4fe3420e2e.png)


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

    Outros: clear, reset, logout, exit, rawCommands

    doc               Gera um arquivo de documentação
    wdns              Mostra servidores web para pesquida de dns
    help              Exibe ajuda
    server            Criar um servidor em localhost
    ngrok             Criar um servidor web externo
    metasploit        Instala o metasploit
    install           Instala todas as ferramentas no ArchLinux
    ssh               Faz uso de ssh
    telnet            Faz uso de telnet
    dumptcp           Faz dump de pacote da rede atual
    mac               Altera o endereço mac
    footprint         Faz o ping scan
    scanner           Faz Varredura de pportas, whois, 
                      encontra flahas,pequisa dns, e brute
                      force de dns
    nmap              Faz uso de nmap com alias TCP SYN, UDP,
                      e bypass de firewall"                                                

## Função INIT()

    Não implementado 

## Funções backend

    props(msg)                              Exibe uma mensagem e recebe um valor (gets)
    getErro(msg = '', command = nil)        Exibe uma mensagem em caso de erro
    proxy(props)                            Faz uso de proxy, props != null
    interpreter(command)                    Interpreta comandos do LETK e do shell

##  

          
      



