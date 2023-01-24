# EzInvoiceRS

An invoice generator made in rust.

## Language support

Currently only french language is supported but it is planned that English will be supported in the next iterations.

## Changing data structure & template

Powershell scripts are available to re-generate a json schema from the typescript file (/json-schema/interface.ts). Thus it is easy to change types and template in your own fork of this repository.

### Requirements

-   npm
-   cargo

### Procedure

1.  Install scripts dependencies using the `install-scripts-deps.ps1` script
2.  Modify the file `json-schema/interface.ts`
3.  Run the script `ts-to-types-rs.ps1`
4.  Adapt the template in `src/invoice_template/mod.rs`
