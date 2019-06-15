HOST=fin.toidiu.com

webBuildAndRelease:
	export WEB_PORT=4000; export WEB_HOST=fin.toidiu.com && \
	npm run build && \
	scp -r dist toidiu@${HOST}:website/

.PHONY: webBuildAndRelease
