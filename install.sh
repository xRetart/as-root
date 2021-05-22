#! /bin/bash


function main
{
	local -r EXECUTABLE='target/release/as-root'

	function build-sources
	{
		cargo build --release
	}
	function mod-binary
	{
		chown root.root $1
		chmod u+x $1
	}
	function install-exe
	{
		install "$1" /bin -o root -m 4755 -s
	}

	build-sources

	mod-binary $EXECUTABLE
	install-exe $EXECUTABLE
}

main