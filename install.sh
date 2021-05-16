#! /bin/bash


function main
{
	local -r EXECUTABLE='target/release/as-root'

	function build-sources
	{
		cargo build --release
	}
	function install-exe
	{
		install "$1" /bin -o root -m 4755 -s
	}

	build-sources
	install-exe $EXECUTABLE
}

main