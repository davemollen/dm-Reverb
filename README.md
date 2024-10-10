## dm-Reverb

A reverb effect written in Rust inspired by the Make Noise Erbe-Verb.

The effect can be compiled to a [MOD audio](https://mod.audio/), VST3 or CLAP plugin.

More technical information about the Erbe-Verb design can be found in this [talk](https://youtu.be/Il_qdtQKnqk?si=oGm_0KLFcN9MhcBE) and this [research paper](https://quod.lib.umich.edu/cgi/p/pod/dod-idx/building-the-erbe-verb-extending-the-feedback-delay-network.pdf?c=icmc;idno=bbp2372.2015.054;format=pdf).

## Table of contents:

- [VST3 and CLAP installation](#VST3-and-CLAP-installation)
- [MOD installation](#MOD-installation)
- [Copyright notices](#Copyright-notices)

## VST3 and CLAP installation

You can download the VST3 and CLAP plugins for Linux, Windows and macOS from the [releases page](https://github.com/davemollen/dm-Reverb/releases).

On macOS you may need to [disable Gatekeeper](https://disable-gatekeeper.github.io/) as Apple has recently made it more difficult to run unsigned code on macOS.

If you want to build the plugin on your own machine check out the [nih-plug repository](https://github.com/robbert-vdh/nih-plug) for instructions.

## MOD installation

Install the plugin from the MOD Audio plugin store.

The latest MOD builds can also be found on the [releases page](https://github.com/davemollen/dm-Reverb/releases).

If you want to build the plugin on your own machine check out the [mod-plugin-builder repository](https://github.com/moddevices/mod-plugin-builder) for instructions.

## Copyright notices

Make Noise Erbe-Verb is a trademark or trade name of another manufacturer and was used merely to identify the product whose sound was reviewed in the creation of this product.

VST is a trademark of Steinberg Media Technologies GmbH, registered in Europe and other countries.

All other trademarks are the property of their respective holders.

<img src="https://steinbergmedia.github.io/vst3_dev_portal/resources/licensing_6.png" width="60" height="auto" alt="VST trademark">
