test:
	cargo run --bin server
	grpcurl -plaintext -import-path ./proto -proto service.proto -d '{"message": "Hello World!"}' '[::1]:50551' simple_service.SimpleService/Message

