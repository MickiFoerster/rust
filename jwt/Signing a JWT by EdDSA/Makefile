verify: jwt
	cd $@ && cargo run

jwt: ED25519-private.pem
	cd sign && cargo run

ED25519-private.pem : 
	./generate-key-pair.sh
