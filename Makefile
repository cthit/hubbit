setup_gamma:
	docker exec -i hubbit-gamma-db-1 psql -U gamma gamma < setup/gamma.sql

setup_hubbit:
	docker exec -i hubbit-hubbit-db-1 psql -U hubbit hubbit < setup/hubbit.sql

setup: setup_gamma setup_hubbit
