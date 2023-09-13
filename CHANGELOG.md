version 0.0.0

Date: Fri Mar 10 19:17:57 2023 →0300 

    → Added main rust files
    → Added main rust workflow
    → Added common modules
        → common.rs → common function for all application
        → extras.rs → function for handle help texts
    → Removed security module
    → Fix operational system call functions
    → Added error handling for system calls

version 0.0.69

Date: Wed May 10 04:24:54 2023 →0300

    → Update rust dependencies
    → Update main rust workflow
    → Fix unformatted manual and help texts
    → Added new manual samples
    → Added new modules
        → secman.rs → help text and manuals
        → automation → sample automation function
    → Update development branches

Version 0.0.98 

Date: Wed Jun 14 04:21:15 PM →03 2023

    → Removed unformatted manual and help texts
    → Removed split files
    → Removed non→standard manual pages 
      (they will be reintroduced as proper man pages)
    → Updated unit tests
    → Updated installer for dnf package manager
    → Updated Rust dependencies

Version 0.2.20

Date: Чт 29 июн 2023 20:58:11 MSK

    → Added function to read shell parameters
    → Implemented automation's base
    → Added automation's for basic enumeration 
    → Added automation's for basic exploitation
    → Added manual page for NetMaid
    → Added new test suit
    → Added new test script
    → Remove unused path file (maidlists file path/list) 
    → Removed non→standard manual pages
    → Updated unit tests
    → Updated installer for dnf package manager
    → Updated Rust dependencies


Version 0.2.60

Date: Пт 30 июн 2023 12:45:53 MSK

    → Move all functions from automatons.rs to automation's 
      module and create one file for each pentest step
    → Added automation's module:
        enumeration.rs      → scans, ping, dns lookup
        exploitation.rs     → social, exploits, wireless
        fakedump.rs         → create a fake database
        footprint.rs        → not implemented yet, osint 
                              google/brave dorks
        harvester.rs        → not implemented yet, web crawler
        keepkit.rs          → not implemented yet, rootkit
    → Update code base
    → Update main version
    → Update CHANGELOG
    → Remove unused temp files
    → Remove unused imports
    → Apply cargo recommended improvements
    → Correcting misspelled words and improving grammar

Version 0.2.62

Date: Tue Jul 25 02:39:05 PM →03 2023

    → Update the README.
    → Fix the panic when the number of arguments is less than 2.
    → Remove unused variables from the enumeration module.

Version 0.5.0 Rebuild for #maid_runner_toolkit

Date: Tue, Aug 8, 2023, 06:14:42 PM →03

    → Old Code Base Revamped: The old code base has been dropped 
    due to extensive experimentation and issues arising from an 
    overly complex shell, which made adding new features challenging.
        → Removed outdated documentation: Discarded old documents and 
          readme files.
        → Trimmed redundancies: Eliminated obsolete modules and files.
        → Preserved Files: Retained fuzzy files.
        → Kept User Assistance: Help pages are maintained, excluding 
          command manuals.
    → Fresh Code Base Introduced: A new code base has been added from 
      scratch, addressing the limitations of the previous implementation.
    → Enhanced Return Types: Custom return types have been incorporated 
      to refine data output accuracy.
    → Optimized Minimal Shell: A customized minimal shell has been 
      developed to enhance efficiency and user→friendliness.
    → Integration of hacklib: The 'hacklib' interface library has been 
      included, enriching system capabilities and interactions.

Version 0.6.1

Date: Qua 23 ago 2023 16:24:22 -03

  → Merge new shell commands
	→ Merge new commond execute methods
	→ Merge new features
		→ New shell
		→ String handle
		→ Scanners
		→ Lookups
		→ Files
	→ Fix smal bugs at web_scanner

Version 0.6.4

Date: ter 29 ago 2023 11:12:17 -03

	→ Added report creation and export as JSON.
	→ Added lookup exif metadata
	→ Allow execution of external commands
	→ Fix bugs and crash them when shell commands are executed.
	→ Fix the system_time function result; change sdt::time to    
      chronos.
	→ Remove unused imports.
	→ Fix conflicts.
	→ Fix dependencies on the git workflow
	→ Fix duplicate files.
	→ Finish the current base
	→ Report JSON with known bugs.

Version 0.7.0

Date: sáb 09 set 2023 09:48:03 -03
  
	→ Added curl binds (all http verbs)
	→ Added curl authentication (all methods)
	→ Added curl content type bindings (all kinds)
	→ Added a curl help page to the manual and readme.
	→ Added curl test (not all implemented yet)
	→ Fixed github workflow dependencies

Version 0.7.1

Date: qua 13 set 2023 17:07:56 -03

	→ Updated main README.md
	→ Updated general/readme.md
	→ Rename files folder to maid_lists
	→ Updated version
	→ Fix config file paths
	→ Merged maid_list project to MaisRunner
		→ Added maid_lists xss-payload-list.txt 
		→ Added maid_lists user_names.ascii 
		→ Added maid_lists maid_list_universal.ascii 
		→ Added maid_lists files_extensions.ascii 
		→ Added maid_lists mallware_hash.ascii 
		→ Added maid_lists repo_keyword_scan.ascii 
		→ Added maid_lists dice_eff_words.ascii   