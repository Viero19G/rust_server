use buff_value::wb_valuation;
use tide::Request;

#[async_std::main]
async fn main() -> tide::Result<()> {
    // On this project, the function main is async
    //println!("Hello, world!");

    
    let mut app = tide::new();

    // definir rotas do server
    // Calcula os Lucros do Proprietário conforme a definição de Buffett.
    app.at("/owners_earnings/:net_income/:depreciation_amortization/:maintenance_capex").get(owners_earnings);
    // Calcula o Retorno sobre o Patrimônio (ROE) como percentual.
    app.at("/return_on_equity/:net_income/:shareholders_equity").get(return_on_equity);
    // Calcula o Retorno sobre Ativos Tangíveis Líquidos (RONTA) como percentual.
    app.at("/return_on_net_tangible_assets/:net_income/:total_assets/:total_liabilities/:intangible_assets").get(return_on_net_tangible_assets);
    // Calcula a Razão Dívida/Patrimônio.
    app.at("/debt_to_equity/:total_liabilities/:shareholders_equity").get(debt_to_equity);
    // Calcula o Lucro por Ação (EPS).
    app.at("/earnings_per_share/:net_income/:shares_outstanding").get(earnings_per_share);
    // Calcula a Taxa de Crescimento Anual Composta (CAGR) para o EPS em um período.
    app.at("/eps_cagr/:initial_eps/:final_eps/:years").get(eps_cagr);
    // Calcula o valor intrínseco usando um modelo simplificado de Fluxo de Caixa Descontado (DCF).
    app.at("/intrinsic_value/:initial_owners_earnings/:growth_rate/:discount_rate/:years").get(intrinsic_value);
    // Calcula o valor intrínseco por ação.
    app.at("/intrinsic_value_per_share/:initial_owners_earnings/:growth_rate/:discount_rate/:years/:shares_outstanding").get(intrinsic_value_per_share);
    app.listen("127.0.0.1:3003").await?;

    println!("Iniciando server: http://127.0.0.1:3003 ");

    Ok(())
}

async fn owners_earnings( req: Request<()>) -> tide::Result {
    // Ao parsear os parâmetros da URL, é crucial especificar que o tipo esperado é `f64`.
    // Usar `.parse::<f64>()` garante a conversão para o tipo correto.
    // O `unwrap_or(0.0)` também deve usar um literal float para corresponder ao tipo.
    let net_income = req.param("net_income")
        .unwrap_or("0.0") // Use "0.0" como string padrão caso o parâmetro não exista
        .parse::<f64>()   // Tenta parsear como f64
        .unwrap_or(0.0);  // Valor padrão f64 caso o parse falhe

    let depreciation_amortization = req.param("depreciation_amortization")
        .unwrap_or("0.0")
        .parse::<f64>()
        .unwrap_or(0.0);

    let maintenance_capex = req.param("maintenance_capex")
        .unwrap_or("0.0")
        .parse::<f64>()
        .unwrap_or(0.0);

    let result = wb_valuation::owners_earnings(net_income, depreciation_amortization, maintenance_capex);
    
    Ok(format!("Os lucros do proprietario sao: {}", result).into())
}

async fn return_on_equity(req: Request<()>) -> tide::Result {
    let net_income = req.param("net_income").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let shareholders_equity = req.param("shareholders_equity").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let result = wb_valuation::return_on_equity(net_income, shareholders_equity);
    // As funções da biblioteca retornam Option<f64> e como 
    // Option é um enum em Rust que representa a possibilidade de um valor estar presente ou ausente
    // precisamos de uma maneira para lidar com as possibilidades, por isso match!
    // match é usado para desempacotar o Option<f64>
    // permitindo que o codigo reaja de maneira diferente se o cálculo foi bem-sucedido (Some) 
    // ou se houve um erro (como divisão por zero, None).
    match result {
        Some(value) => Ok(format!("O Retorno sobre o Patrimônio (ROE) é: {}%", value).into()),
        None => Ok("Erro: Patrimônio dos acionistas não pode ser zero.".into()),
    }
}

async fn return_on_net_tangible_assets(req: Request<()>) -> tide::Result {
    let net_income = req.param("net_income").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let total_assets = req.param("total_assets").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let total_liabilities = req.param("total_liabilities").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let intangible_assets = req.param("intangible_assets").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let result = wb_valuation::return_on_net_tangible_assets(net_income, total_assets, total_liabilities, intangible_assets);
    match result {
        Some(value) => Ok(format!("O Retorno sobre Ativos Tangíveis Líquidos (RONTA) é: {}%", value).into()),
        None => Ok("Erro: Ativos tangíveis líquidos não podem ser zero.".into()),
    }
}

async fn debt_to_equity(req: Request<()>) -> tide::Result {
    let total_liabilities = req.param("total_liabilities").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let shareholders_equity = req.param("shareholders_equity").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let result = wb_valuation::debt_to_equity(total_liabilities, shareholders_equity);
    match result {
        Some(value) => Ok(format!("A Razão Dívida/Patrimônio é: {}", value).into()),
        None => Ok("Erro: Patrimônio dos acionistas não pode ser zero.".into()),
    }
}

async fn earnings_per_share(req: Request<()>) -> tide::Result {
    let net_income = req.param("net_income").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let shares_outstanding = req.param("shares_outstanding").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let result = wb_valuation::earnings_per_share(net_income, shares_outstanding);
    match result {
        Some(value) => Ok(format!("O Lucro por Ação (EPS) é: {}", value).into()),
        None => Ok("Erro: Ações em circulação não podem ser zero.".into()),
    }
}

async fn eps_cagr(req: Request<()>) -> tide::Result {
    let initial_eps = req.param("initial_eps").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let final_eps = req.param("final_eps").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let years = req.param("years").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0); // Anos como f64 para a função
    let result = wb_valuation::eps_cagr(initial_eps, final_eps, years);
    match result {
        Some(value) => Ok(format!("A Taxa de Crescimento Anual Composta (CAGR) do EPS é: {}%", value).into()),
        None => Ok("Erro: EPS inicial ou anos inválidos não podem ser zero.".into()),
    }
}

async fn intrinsic_value(req: Request<()>) -> tide::Result {
    let initial_owners_earnings = req.param("initial_owners_earnings").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let growth_rate = req.param("growth_rate").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let discount_rate = req.param("discount_rate").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    // 'years' é u32, então parseamos para u32
    let years = req.param("years").unwrap_or("0").parse::<u32>().unwrap_or(0); 
    let result = wb_valuation::intrinsic_value(initial_owners_earnings, growth_rate, discount_rate, years);
    Ok(format!("O Valor Intrínseco é: {}", result).into())
}

async fn intrinsic_value_per_share(req: Request<()>) -> tide::Result {
    let initial_owners_earnings = req.param("initial_owners_earnings").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let growth_rate = req.param("growth_rate").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let discount_rate = req.param("discount_rate").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let years = req.param("years").unwrap_or("0").parse::<u32>().unwrap_or(0); // Anos como u32
    let shares_outstanding = req.param("shares_outstanding").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let result = wb_valuation::intrinsic_value_per_share(initial_owners_earnings, growth_rate, discount_rate, years, shares_outstanding);
    match result {
        Some(value) => Ok(format!("O Valor Intrínseco por Ação é: {}", value).into()),
        None => Ok("Erro: Ações em circulação não podem ser zero.".into()),
    }
}