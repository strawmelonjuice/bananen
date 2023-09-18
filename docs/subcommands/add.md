- [Docs - Getting started](https://strawmelonjuice.com/?p=projects/bananen/docs)

- [Get bananen](https://strawmelonjuice.com/?p=projects/bananen/get)
- [GitHub](https://github.com/strawmelonjuice/bananen/)

# <span style="background-color: #24273a; color: #ffcc00">Bananen! 🍌</span> -> [Docs](https://strawmelonjuice.com/?p=projects/bananen/docs) -> Subcommand: `Add`

## Function

`add` adds new changes to the 'unreleased' section of your changelog.

## Aliases

`a`

## Usage

```bash
bananen add <type> "<change description>" [--breaking]
```

| Parameter              | Usage                                                        |
| ---------------------- | ------------------------------------------------------------ |
| `<type>`               | Type of change:<br /><br />`addition|add|a`: Addition<br />`update|up|u`: Update<br />`fix|solve|f`: Fix<br />`removal|rem|del|r`: Removal |
| `<change description>` | A line describing the change you are documenting. One change per command is preferred, as <span style="background-color: #24273a; color: #ffcc00">Bananen! 🍌</span> generates bullet point change logs. |
| `--breaking`           | (Optional) Warns readers that this change breaks compatibility with earlier versions. |

