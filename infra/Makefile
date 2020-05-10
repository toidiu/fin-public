HOST_FILE='hosts'

ansible-ping:
	ansible -i ${HOST_FILE} fin -m ping

ansible-run:
	ansible-playbook playbook.yml -i ${HOST_FILE}

ansible-galaxy:
	ansible-galaxy install -r requirements.yml




.PHONY: ansible-ping ansible-run ansible-galaxy
