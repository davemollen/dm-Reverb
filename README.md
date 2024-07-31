## dm-Reverb

A reverb effect written in Rust.

The effect can be compiled to a [MOD devices](https://moddevices.com/), VST3 or CLAP plugin.
VST is a trademark of Steinberg Media Technologies GmbH, registered in Europe and other countries.

<img src="https://steinbergmedia.github.io/vst3_dev_portal/resources/licensing_6.png" width="60" height="auto" alt="VST trademark">

## Table of contents:

- [Download VST3 and CLAP plugin](#Download-VST3-and-CLAP-plugin)
- [MOD devices installation](#MOD-devices-installation)
- [Other plugin formats](#Other-plugin-formats)

## Download VST3 and CLAP plugin

You can download the VST3 and CLAP plugins for Linux, Windows and macOS from the [release page](https://github.com/davemollen/dm-Reverb/releases).

On macOS you may need to [disable Gatekeeper](https://disable-gatekeeper.github.io/) as Apple has recently made it more difficult to run unsigned code on macOS.

If you want to build the plugin on your own machine, check out the [nih-plug readme](https://github.com/robbert-vdh/nih-plug). Build scripts for macOS specifically can be found in the [./scripts](./scripts/) folder of this repository.

## MOD devices installation

This plugin is not yet in the MOD Audio plugin store.
However, you can find a build of the plugin for the MOD Dwarf at [./lv2/dm-Reverb.lv2](./lv2/dm-Reverb.lv2/). Copy this file to your MOD devices' plugin folder. If you want a build for MOD Duo or MOD Duo X you'll need to build the plugin yourself for now. For more information about building this plugin for your MOD device, see [these instructions](https://github.com/moddevices/mod-plugin-builder).

## Other plugin formats

Code for a LV2 plugin is also in this repository. Automated builds are excluded for LV2 because the rust lv2 crate doesn't have GUI support. The LV2 plugin format is being used for the MOD devices plugin which in turn does have a GUI. Because MOD has it's own GUI layer on top of the LV2 plugin.
