use crate::api_token::APITokenCmd;
use crate::csv::loaders::CSVDataCmd;
use crate::init_db::DB;
use crate::task::ExecTask;
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum CLIApp {
    /// API token related operations
    APIToken(APITokenCmd),
    /// CSV data loaders
    CSVData(CSVDataCmd),
    /// DB related operations
    DB(DB),
}

impl ExecTask for CLIApp {
    type ResultValue = ();

    fn exec(&self) -> std::io::Result<Self::ResultValue> {
        match self {
            CLIApp::APIToken(api_token) => api_token.exec(),
            CLIApp::CSVData(csv_data) => csv_data.exec(),
            CLIApp::DB(db_cmd) => db_cmd.exec(),
        }
    }
}
