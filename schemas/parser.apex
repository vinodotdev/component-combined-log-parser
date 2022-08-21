namespace "parser"

type Inputs {
  log_entry: string,
  log_format: string,
  field_seperator: string
}

type Outputs {
  log_message: {string:string}
}

type Config {

}