fn usdcny(usd: u16) -> String
{
   format!("{:.2} Chinese Yuan", f64::from(usd) * 6.75)
}
