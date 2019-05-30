HOST_FILE='hosts'

ansible-ping:
	ansible -i ${HOST_FILE} fin -m ping

ansible-run:
	ansible-playbook playbook.yml -i ${HOST_FILE}




.PHONY: ansible-ping ansible-run
