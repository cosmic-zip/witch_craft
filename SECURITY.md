 Security Policy

Although we do our best to prevent vulnerabilities, and have tools to help
catch most of them, we are humans after all, and there will be inevitably
issues slipping through the cracks.

They can fall into two categories, either in any of the tools in the
witch_craft suite, or in our presence online (website, forum, emails,
DNS, etc.).

Depending on which category they fall into, different information is
needed. We
do believe in coordinated disclosure, so in order to address them, and
coordinate disclosure with you (and properly credit you for the
discovery),
report them to us. Do not open bug reports or pull requests.

Our contact email for security issues is TBA

If you are unsure how to proceed, need clarifications or have questions or
remarks about this policy, feel free to email us to inquire.

## witch_craft suite vulnerabilities

### Supported versions

We only support the latest stable present on TBA

For security issues present in our GitHub repository (master or any
recently
active branch), open a pull request or bug report.

For any security issue affecting older versions of witch_craft still
present
in currently supported Linux or BSD distributions, file a report with
them,
and email us a short description of the vulnerability along with a link to
the bug report.

### Reporting

There is no particular template to report the vulnerabilities. Keep
in mind
that a vulnerability is essentially a bug, so please provide us detailed
information on how to reproduce it, such as:

- Which witch_craft tools are affected? And how? Any proof of concept to
demonstrate it?
- Operating systems involved, kernel versions (`uname -a` and
`lsb_release -a` for example).
- CPU architecture (`witch_craft -v` output is useful); a vulnerability
on a
x86 32 bit may not be exploitable on ARM 64 bit. A bug may also only
be present
when witch_craft is compiled a certain way.
- All the commands needed to trigger the issue.
- Did you compile it yourself or did you get it from a package?
- What equipment did you use? A packet capture may be useful; different
equipment behaves differently, they have different Wi-Fi stacks,
drivers, and
firmware.
- A patch to fix the issue, if available.
- If CVE numbers have been assigned, please provide them as well.

### Public disclosure

Although it is essentially a bug, do not submit a bug report or a pull
request,
but email us the data first, so we can coordinate fixing the issue
and assist
you in filing the bug reports, and if you provided a patch, the pull
request; a
patch may need to be broken down in multiple commits for clarity,
for example.

## Online presence

For any security issue affecting us specifically (any witch_craft.org
subdomain) such as misconfiguration of our hosting, DNS, email, servers,
or misconfiguration of the software we are using, email us with all the
details regarding your findings.

Anything else should be reported to the author or provider of the
software,
hardware, or hosting.
