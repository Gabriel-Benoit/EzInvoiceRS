cd ./json-schema;
typescript-json-schema "./*.ts" InvoiceData --aliasRefs --noExtraProps --required --strictNullCheck -o ./schema/InvoiceData.json
cd ..;