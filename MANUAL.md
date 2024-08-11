🔘 [ ENTRY ] :: nuke.hd


	Securely deletes and overwrites the contents of a device seven
	times

	--device :: Need and device localtion like: /dev/sdX

🔘 [ ENTRY ] :: nuke.file


	Securely deletes and overwrites a file seven times, then removes
	it from the filesystem to prevent data recovery.

	--file :: File location, suport start with ./

🔘 [ ENTRY ] :: firewall.flush


	Remove all firewall rules non-resible


🔘 [ ENTRY ] :: firewall.drop.in.all


	Drop all incomme connections


🔘 [ ENTRY ] :: firewall.drop.out.all


	Derop all outcome connections


🔘 [ ENTRY ] :: firewall.drop.bigger


	Drop all outcome port bigger than expecified port

	--port


🔘 [ ENTRY ] :: web.donwload


	Full website downloader

	--wait=--wait
	--url :: Target complete URL path with http/https

🔘 [ ENTRY ] :: scp.copy


	Creates a directory named 'out' and copies files from a specified
	folder to a remote host via SCP.

	--port :: Port number
	--folder/*
	--@ip
	./out/

🔘 [ ENTRY ] :: blue.enable


	Enables Bluetooth functionality by bringing up the hci0 interface,
	unblocking Bluetooth, and printing 'done'.


🔘 [ ENTRY ] :: blue.list


	Lists Bluetooth devices and their status.


🔘 [ ENTRY ] :: dos.longpass


	Generates a long random username and password, then performs a
	POST request to a specified domain with the generated credentials.

	--domain
	--size)"password"

	--size)}'

🔘 [ ENTRY ] :: span.text


	Sends a POST request with JSON data containing a 'main' field
	and a message field specified by @@message to a specified domain.

	--message"

	--domain

🔘 [ ENTRY ] :: capture.all


	Capture all packets on a specified network interface.

	--interface

🔘 [ ENTRY ] :: capture.all.from_ip


	Capture all packets from/to a specified IP address on a specified
	network interface.

	--interface
	--ip :: IP address

🔘 [ ENTRY ] :: capture.one


	Capture only one packet on a specified network interface.

	--interface

🔘 [ ENTRY ] :: capture.one.from_ip


	Capture only one packet from/to a specified IP address on a
	specified network interface.

	--interface
	--ip :: IP address

🔘 [ ENTRY ] :: disk.dump


	Create a disk dump from a specified input device to an output
	file.

	--device
	--file :: File location, suport start with ./

🔘 [ ENTRY ] :: disk.recover


	Carve files from a specified disk image using foremost.

	--image
	--folder

🔘 [ ENTRY ] :: file.by


	lsof alias

	--file :: File location, suport start with ./

🔘 [ ENTRY ] :: file.hex


	creates  a hex dump of a given file or standard input.

	--file :: File location, suport start with ./

🔘 [ ENTRY ] :: file.bin


	Switch to bits (binary digits) dump, rather than hex dump.

	--file :: File location, suport start with ./

🔘 [ ENTRY ] :: file.dec


	Takes a hexadecimal dump and converts it back into binary format.

	--file :: File location, suport start with ./

🔘 [ ENTRY ] :: file.dump


	Output in PostScript continuous hex dump style. Also known as
	plain hex dump style.

	--file :: File location, suport start with ./

🔘 [ ENTRY ] :: file.list


	Create a C header file from a binary file. This header file
	contains a C array definition representing the binary data in
	hexadecimal format.

	--file :: File location, suport start with ./

🔘 [ ENTRY ] :: file


	Utility used to determine the type of a file.

	--file :: File location, suport start with ./

🔘 [ ENTRY ] :: file.meta


	Extracts and displays all metadata from an image file using
	exiftool.

	--file :: File location, suport start with ./

🔘 [ ENTRY ] :: file.clean.meta


	Removes all metadata from an image to protect privacy

	--image

🔘 [ ENTRY ] :: map.dns.dirs


	scans a domain for directories with a specified wordlist, and
	saves the results to an output file.

	--domain
	--wordlist :: Path to wordlist
	--output :: Output file

🔘 [ ENTRY ] :: map.dns.subdomain


	It scans a domain for subdomains using the specified wordlist
	and saves the results to an output file.

	--domain
	--wordlist :: Path to wordlist
	--output :: Output file

🔘 [ ENTRY ] :: map.default


	Default set of nmap NSE scripts, same as nmap [options] -sC
	[ip or dns]

	--target :: Refers to IP or DOMAIN name

🔘 [ ENTRY ] :: map.discovery


	Try to actively discover more about the network by querying
	public registries, SNMP-enabled devices, directory services,
	and the like.

	--target :: Refers to IP or DOMAIN name

🔘 [ ENTRY ] :: map.auth


	Deal with authentication credentials (or bypassing them) on the
	target system

	--target :: Refers to IP or DOMAIN name

🔘 [ ENTRY ] :: map.safe


	Perform general network discovery, are less likely to offend
	remote administrators

	--target :: Refers to IP or DOMAIN name

🔘 [ ENTRY ] :: map.vuln


	Check for specific known vulnerabilities and generally only
	report results if they are found.

	--target :: Refers to IP or DOMAIN name

🔘 [ ENTRY ] :: map.local


	Scan local open connections


🔘 [ ENTRY ] :: map.myip


	Show the current ip address


🔘 [ ENTRY ] :: crack.zip


	Crack ZIP file encryption using Hashcat

	--file :: File location, suport start with ./
	--wordlist :: Path to wordlist

🔘 [ ENTRY ] :: crack.luks1


	Crack LUKS1 disk image encryption using Hashcat

	--file :: File location, suport start with ./
	--wordlist :: Path to wordlist

🔘 [ ENTRY ] :: crack.luks2


	Crack LUKS2 disk image encryption using Hashcat

	--file :: File location, suport start with ./
	--wordlist :: Path to wordlist

🔘 [ ENTRY ] :: crack.keepass


	Crack KeePass database encryption using Hashcat

	--file :: File location, suport start with ./
	--wordlist :: Path to wordlist

🔘 [ ENTRY ] :: crack.pcap


	Crack WPA/WPA2 encryption in PCAP files using Hashcat

	--file :: File location, suport start with ./
	--wordlist :: Path to wordlist

🔘 [ ENTRY ] :: view.applogs


	Applogs typically refer to logs generated by specific applications
	or software.
    They contain information about the operation, performance, and errors
    of the application.


🔘 [ ENTRY ] :: view.syslog


	syslog: General system messages.


🔘 [ ENTRY ] :: view.messages


	messages: System messages, including kernel and service messages.


🔘 [ ENTRY ] :: view.authlog


	authlog: Authentication-related messages.


🔘 [ ENTRY ] :: view.secure


	secure: Security-related messages, including authentication and
	authorization events.


🔘 [ ENTRY ] :: view.kernlog


	kernlog: Kernel-related messages.


🔘 [ ENTRY ] :: view.auditlog


	auditlog: Audit messages, tracking system events for security
	auditing.


🔘 [ ENTRY ] :: view.wtmp


	wtmp: Records all user logins and logouts.


🔘 [ ENTRY ] :: view.btmp


	btmp: Records failed login attempts.


🔘 [ ENTRY ] :: view.lastlog


	lastlog: Records last login information for all users.


🔘 [ ENTRY ] :: backup.mysql


	MySQL database backup

	--username :: Username; well, it setup user name
	--password :: Specify password; it will be shown in plaintext
	--database_name

🔘 [ ENTRY ] :: backup.postgresql


	PostgreSQL database backup

	--username :: Username; well, it setup user name
	--database_name

🔘 [ ENTRY ] :: backup.sqlite


	SQLite database backup

	--database_name

🔘 [ ENTRY ] :: backup.microsoft_sql_server


	Microsoft SQL Server database backup

	--server_name
	--username :: Username; well, it setup user name
	--password :: Specify password; it will be shown in plaintext
	--database_name]

🔘 [ ENTRY ] :: backup.oracle_database


	Oracle Database backup

	--username/--password--@database_name

🔘 [ ENTRY ] :: backup.mongodb


	MongoDB database backup

	--host :: Host name ip address
	--port :: Port number
	--username :: Username; well, it setup user name
	--password :: Specify password; it will be shown in plaintext
	--database_name

🔘 [ ENTRY ] :: backup.redis


	Redis database backup


🔘 [ ENTRY ] :: backup.cassandra


	Cassandra database backup

	--keyspace_name

🔘 [ ENTRY ] :: backup.mariadb


	MariaDB database backup

	--username :: Username; well, it setup user name
	--password :: Specify password; it will be shown in plaintext
	--database_name

🔘 [ ENTRY ] :: backup.ibm_db2


	IBM Db2 database backup

	--database_name

🔘 [ ENTRY ] :: backup.elasticsearch


	Elasticsearch backup

	-d'{"indices"
	"--index_name""ignore_unavailable"
	true"include_global_state"
	false}'

🔘 [ ENTRY ] :: backup.firebase_realtime_database


	Firebase Realtime Database backup

	--project_id
	--access_token'

🔘 [ ENTRY ] :: backup.couchbase


	Couchbase database backup

	--host
	--path/to/local/backup/couchbase_backup
	--username :: Username; well, it setup user name
	--password :: Specify password; it will be shown in plaintext

🔘 [ ENTRY ] :: backup.neo4j


	Neo4j database backup

	--database_name

🔘 [ ENTRY ] :: backup.amazon_dynamodb


	Amazon DynamoDB backup

	--table_name

🔘 [ ENTRY ] :: backup.snowflake


	Snowflake database backup

	--account
	--username :: Username; well, it setup user name
	--password :: Specify password; it will be shown in plaintext
	--database_name
	--database_name;'

🔘 [ ENTRY ] :: backup.teradata


	Teradata database backup

	--username :: Username; well, it setup user name
	--password :: Specify password; it will be shown in plaintext
	--database_name

🔘 [ ENTRY ] :: backup.hbase


	HBase database backup

	--snapshot_name

🔘 [ ENTRY ] :: backup.influxdb


	InfluxDB database backup

	--database_name

🔘 [ ENTRY ] :: backup.memcached


	Memcached database backup

	--host
	--port
