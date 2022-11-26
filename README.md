# Sember

![](https://raw.githubusercontent.com/askonomm/sember/master/cover.png)

Sember is a opionated static site generator. It builds an entire website from just one configuration file and does all the design decisions for you.

Sember makes some smart decisions for you based on the sember.toml file, such as whether to show you as employed somewhere or how to contact you, if and how to create a CV page for you, and so on.

## Install

Sember runs on Linux and Mac, and can be installed in both platforms via singular command:

```bash
curl -s https://raw.githubusercontent.com/askonomm/sember/master/install | bash -s -- -g
```

And then all you have to do is run `sember` in any directory that contains the `sember.toml` configuration file.