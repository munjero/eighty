.PHONY: build serve-corepaper serve-classic serve-multiverse serve-wei

build:
	nix-build .

serve-corepaper: build
	ruby -run -ehttpd result/corepaper -p8000

serve-classic: build
	ruby -rwebrick -e'brick = WEBrick::HTTPServer.new(:Port => 8000); brick.mount("/~classic/", WEBrick::HTTPServlet::FileHandler, "result/classic", true); trap("INT"){ brick.shutdown }; brick.start'

serve-wei: build
	ruby -rwebrick -e'brick = WEBrick::HTTPServer.new(:Port => 8000); brick.mount("/~wei/", WEBrick::HTTPServlet::FileHandler, "result/wei", true); trap("INT"){ brick.shutdown }; brick.start'

serve-multiverse: build
	ruby -run -ehttpd result/multiverse -p8000