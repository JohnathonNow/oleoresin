target/package.zip: package.json *.js node_modules package-lock.json
	zip -r $@ $^

.PHONY: deploy invoke

deploy: target/package.zip
	wsk action -i create test $< --kind nodejs:14 ||\
	wsk action -i update test $< --kind nodejs:14

invoke: target/package.zip
	wsk action -i invoke --result test --param name "Jeff" --param place "home"
