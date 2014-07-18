[![Build Status](https://travis-ci.org/ujh/iomrascalai.svg?branch=master)](https://travis-ci.org/ujh/iomrascalai)
[![Gitter chat](https://badges.gitter.im/ujh/iomrascalai.png)](https://gitter.im/ujh/iomrascalai)

Iomrascálaí
===========

Iomrascálaí
([see here for the pronunciation](https://raw.githubusercontent.com/ujh/iomrascalai/master/pronunciation.mp4))
is an AI for the game of Go/Weiqi/Baduk written in Rust. Please note
that we're using the Rust nightly build and not 0.10!

Development
===========

See the [Github issues](https://github.com/ujh/iomrascalai/issues) for
planned features and bugs and
[join the mailing list](https://groups.google.com/forum/#!forum/iomrascalai)
and [the chat](https://gitter.im/ujh/iomrascalai) for discussion.
Please note that (almost) all development on Iomrascálaí is supposed
to happen with pair programming. If you want to contribute please
[contact me](http://urbanhafner.com) and I can set you up with a
pairing partner and (eventually) give you access to the Trello board
and Github repository. If that is not possible then we're also
accepting pull requests in a pinch.

Testing
=======

To play 10 games against GnuGo, install GoGui and run the
following command in the top level folder:

```
gogui-twogtp -auto -black "gnugo --positional-superko --chinese-rules \
--mode gtp" -white "./bin/iomrascalai --mode-gtp" -verbose -size 9 \
-alternate -games 10 -sgffile test
```

To run a game against GnuGo and view it in GoGui in real time use the following command (add `-auto` if a new game should automatically be started when a game is finished):

```
gogui -computer-both -program "gogui-twogtp -black \"gnugo \
--positional-superko --chinese-rules --mode gtp\" -white \
\"./bin/iomrascalai --mode-gtp\" -verbose -size 9" -size 9
```
Resources
=========

The following Go programs are available as source code and can serve
as inspiration:

* [HouseBot](https://github.com/ujh/HouseBot)
* [Pachi](http://pachi.or.cz/)
* [Orego](https://github.com/Orego/Orego)
* [libego](https://github.com/lukaszlew/libego)

License
=======

Iomrascálaí is licensed under GPL v3. See the
[LICENSE file](https://github.com/ujh/iomrascalai/blob/master/LICENSE)
for the complete license text.
