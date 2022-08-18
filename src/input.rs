use syn::{parse::{Parse, ParseStream}, Result, LitStr, Ident, token};

pub(crate) struct StaticFileMacroInput {
    pub app_ident: Ident,
    pub path: LitStr,
    pub directory: LitStr,
}

impl Parse for StaticFileMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let app_ident: Ident = input.parse()?;
        input.parse::<token::Comma>()?;
        let path: LitStr = input.parse()?;
        input.parse::<token::Comma>()?;
        let directory: LitStr = input.parse()?;
        Ok(
            StaticFileMacroInput{
                app_ident,
                path,
                directory,
            }
        )
    }
}