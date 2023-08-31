verify: jwt
	cd $@ && cargo run

jwt: ES256-private.pem
	cd sign && cargo run

ES256-private.pem : 
	./generate-key-pair.sh
