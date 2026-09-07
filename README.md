# <span style="background-color: #24273a; color: #ffcc00">Bananen! 🍌</span>

Bananen v0 (0.1.x) was a very simple changelog generator, working only on a file level 
and not interfering with anything else. Bananen v1 (0.2.x and 1.x.x) will assist on a much
more personal level: It'll commit for you.

Sounds scary, right? Yeah, that's because it kinda is.

Bananen will become a CLI wrapper around [git](https://git-scm.com) and [Jujutsu](https://www.jj-vcs.dev/latest/), 
allowing you to focus on what you're doing while asking you questions that will both allow it to generate a proper 
changelog on release (recognizing your entries and other's to also be able to properly put your change in between 
changes from people who do not use Bananen) as well as enforcing [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/#specification).

Some conceptual Bananen use:

```
$ bn todos

> Todos on your clone:
> | Type |                                   | Change id   | Todo id  |
> ---------------------------------------------------------------------
> | Fix  | Updating usernames crashes system | zltwzszm    | zltwzszm | 
> | Feat | Domain name-based verification    | none yet    | -13dskxc |

> Todos stored in remote repository:
> | Type |                                   | Change id   | Todo id  |
> ---------------------------------------------------------------------


$ bn new --from -13dskxc

> Todo -13dskxc is being checked out into a Jujutsu change on top of the latest commit bookmarked with misstress.
> running: jj new --onto misstress -m "feat: domain name-based verification\n\nSigned-off-by: Joe Appelboom\n\nBananen-commit: U29tZSBtZXRhZGF0YSBiYW5hbmVuIHN0b3JlcyBpbiB5b3VyIGpqIGNvbW1pdHM"
> JJ: Working copy  (@) now at: rmzuoqzy 0b52486a (empty) feat: domain name-based verification
> JJ: Parent commit (@-): lwzyyzmo d27d10bc misstress | fix: user not found error showing for new users

$  
```

This would mean creating commits off of a todo is as easy as... well, that!

Of course, describing change into a commit without a todo is also trivial this way:

```
$ bn describe

> Changed files:
> =====================================
>  ./README.md             10++++---
>  ./public/spa.js        300+++++++
>  ./CONTRIBUTION.MD       20-------
> =====================================
> 
> Are these all related to the same change? (y/n) < n
> 
> running: jj split --interactive -m "\n\n\nBananen-commit: eyJ0cmFja2VkLWFzIjogImM0NDBhMTRjLWEwYmYtNDMxMy1hYWRhLTYzYTEzMmFlZjM0ZiJ9" -B @
> 
> JJ change id: vumuzzqr
> 
> Is this a [new feature]...ok!
>
> What functionality does this add in a single line: < domain name-based verification
> 
> running nvim: /tmp/eyJ0cmFja2VkLWFzIjogImM0NDBhMTRjLWEwYmYtNDMxMy1hYWRhLTYzYTEzMmFlZjM0ZiJ9/long-description.markdown
>
> 12 lines of summary ✅
> 
> running: jj describe -r vumuzzqr -m ...
```

I'm not going to hand-roll that long of an escaped string, but you get the gist!

I think instead of fantasizing about possible command output I should just get writing already, and get me some actual output!