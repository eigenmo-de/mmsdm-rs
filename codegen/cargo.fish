#!/bin/fish
chcon -Rt svirt_sandbox_file_t $PWD
chcon -t svirt_sandbox_file_t $HOME/.gitconfig;
mkdir -p $HOME/Applications/cargo-usr/registry;
chcon -t svirt_sandbox_file_t $HOME/Applications/cargo-usr/registry;
podman run --rm \
	-v $PWD:/workingdir \
	-v $HOME/Applications/cargo-usr/registry:/root/.cargo/registry \
	-v $HOME/.gitconfig:/root/.gitconfig \
	-e USER=$USER \
	--entrypoint=cargo \
	-it neovim-2 $argv

