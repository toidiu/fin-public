HOST=fin.toidiu.com

webBuildAndRelease:
	export PORT=4000; export HOST=fin.toidiu.com && \
	npm run build && \
	scp -r dist toidiu@${HOST}:website/

.PHONY: webBuildAndRelease
