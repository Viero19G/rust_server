use buff_value::wb_valuation;
use tide::{Request, Response, StatusCode};
use serde_json::json; 
#[async_std::main]
async fn main() -> tide::Result<()> { // CORREÇÃO: Altere o tipo de retorno para Result<()>
    let mut app = tide::new(); // Mudei para 'let app' pois não é mutável após a configuração inicial das rotas

    // Lista de rotas para impressão dinâmica
    let routes = vec![
        "/owners_earnings/:net_income/:depreciation_amortization/:maintenance_capex",
        "/return_on_equity/:net_income/:shareholders_equity",
        "/return_on_net_tangible_assets/:net_income/:total_assets/:total_liabilities/:intangible_assets",
        "/debt_to_equity/:total_liabilities/:shareholders_equity",
        "/earnings_per_share/:net_income/:shares_outstanding",
        "/eps_cagr/:initial_eps/:final_eps/:years",
        "/intrinsic_value/:initial_owners_earnings/:growth_rate/:discount_rate/:years",
        "/intrinsic_value_per_share/:initial_owners_earnings/:growth_rate/:discount_rate/:years/:shares_outstanding",
    ];

    // --- Configuração das rotas do server ---
    // Calcula os Lucros do Proprietário conforme a definição de Buffett.
    app.at(routes[0]).get(owners_earnings);
    // Calcula o Retorno sobre o Patrimônio (ROE) como percentual.
    app.at(routes[1]).get(return_on_equity);
    // Calcula o Retorno sobre Ativos Tangíveis Líquidos (RONTA) como percentual.
    app.at(routes[2]).get(return_on_net_tangible_assets);
    // Calcula a Razão Dívida/Patrimônio.
    app.at(routes[3]).get(debt_to_equity);
    // Calcula o Lucro por Ação (EPS).
    app.at(routes[4]).get(earnings_per_share);
    // Calcula a Taxa de Crescimento Anual Composta (CAGR) para o EPS em um período.
    app.at(routes[5]).get(eps_cagr);
    // Calcula o valor intrínseco usando um modelo simplificado de Fluxo de Caixa Descontado (DCF).
    app.at(routes[6]).get(intrinsic_value);
    // Calcula o valor intrínseco por ação.
    app.at(routes[7]).get(intrinsic_value_per_share);
    
    
    println!("Iniciando server:🙌");
    println!("😶");
    println!(" 🚀 http://127.0.0.1:3003 ");

    println!("Rotas disponíveis:");
    for route in routes {
        println!("  GET http://127.0.0.1:3003{}", route);
    }
    // Inicia o listener do servidor
    app.listen("127.0.0.1:3003").await?;

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
    // Antigo return criado em aula
    // Ok(format!("Os lucros do proprietario sao: {}", result).into())

    // Constrói uma nova resposta HTTP 200 (OK), define seu corpo como um objeto JSON
    // (serializado a partir da macro json!) e, em caso de sucesso, retorna essa resposta.
    // O '?' propaga erros de serialização de JSON, se houverem.

    // criando response 'resposta' mutavel 
    let mut response = Response::new(StatusCode::Ok);
    // definindo corpo Json
    response.set_body(tide::Body::from_json(&json!({"owners_earnings": result}))?);
    // retornando resposta
    Ok(response)
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
    
    /*  Estrutura inicial com match em acordo ao uso da lib e da proposta do curso
    match result {
        Some(value) => Ok(format!("O Retorno sobre o Patrimônio (ROE) é: {}%", value).into()),
       None => Ok("Erro: Patrimônio dos acionistas não pode ser zero.".into()),
    } */

    // Novo match com resposta em Json
    match result {
        Some(value) => {
            let mut response = Response::new(StatusCode::Ok);
            response.set_body(tide::Body::from_json(&json!({"return_on_equity": value}))?);
            Ok(response)
        },
        None => {
            let mut response = Response::new(StatusCode::BadRequest);
            response.set_body(tide::Body::from_json(&json!({"error": "Patrimônio dos acionistas não pode ser zero."}))?);
            Ok(response)
        },
    }
}

async fn return_on_net_tangible_assets(req: Request<()>) -> tide::Result {
    let net_income = req.param("net_income").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let total_assets = req.param("total_assets").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let total_liabilities = req.param("total_liabilities").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let intangible_assets = req.param("intangible_assets").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let result = wb_valuation::return_on_net_tangible_assets(net_income, total_assets, total_liabilities, intangible_assets);
    /* 
    match result {
        Some(value) => Ok(format!("O Retorno sobre Ativos Tangíveis Líquidos (RONTA) é: {}%", value).into()),
        None => Ok("Erro: Ativos tangíveis líquidos não podem ser zero.".into()),
    }

    */
    match result {
        Some(value) => {
            let mut response = Response::new(StatusCode::Ok);
            response.set_body(tide::Body::from_json(&json!({"return_on_net_tangible_assets": value}))?);
            Ok(response)
        },
        None => {
            let mut response = Response::new(StatusCode::BadRequest);
            response.set_body(tide::Body::from_json(&json!({"error": "Ativos tangíveis líquidos não podem ser zero."}))?);
            Ok(response)
        },
    }
}

async fn debt_to_equity(req: Request<()>) -> tide::Result {
    let total_liabilities = req.param("total_liabilities").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let shareholders_equity = req.param("shareholders_equity").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let result = wb_valuation::debt_to_equity(total_liabilities, shareholders_equity);
    /*
    match result {
        Some(value) => Ok(format!("A Razão Dívida/Patrimônio é: {}", value).into()),
        None => Ok("Erro: Patrimônio dos acionistas não pode ser zero.".into()),
    } */
   match result {
        Some(value) => {
            let mut response = Response::new(StatusCode::Ok);
            response.set_body(tide::Body::from_json(&json!({"debt_to_equity": value}))?);
            Ok(response)
        },
        None => {
            let mut response = Response::new(StatusCode::BadRequest);
            response.set_body(tide::Body::from_json(&json!({"error": "Patrimônio dos acionistas não pode ser zero."}))?);
            Ok(response)
        },
   }
}

async fn earnings_per_share(req: Request<()>) -> tide::Result {
    let net_income = req.param("net_income").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let shares_outstanding = req.param("shares_outstanding").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let result = wb_valuation::earnings_per_share(net_income, shares_outstanding);
    /* 
    match result {
        Some(value) => Ok(format!("O Lucro por Ação (EPS) é: {}", value).into()),
        None => Ok("Erro: Ações em circulação não podem ser zero.".into()),
    }*/
    match result {
        Some(value) => {
            let mut response = Response::new(StatusCode::Ok);
            response.set_body(tide::Body::from_json(&json!({"earnings_per_share": value}))?);
            Ok(response)
        },
        None => {
            let mut response = Response::new(StatusCode::BadRequest);
            response.set_body(tide::Body::from_json(&json!({"error": "Ações em circulação não podem ser zero."}))?);
            Ok(response)
        },
    }
}

async fn eps_cagr(req: Request<()>) -> tide::Result {
    let initial_eps = req.param("initial_eps").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let final_eps = req.param("final_eps").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let years = req.param("years").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0); // Anos como f64 para a função
    let result = wb_valuation::eps_cagr(initial_eps, final_eps, years);
    /*
    match result {
        Some(value) => Ok(format!("A Taxa de Crescimento Anual Composta (CAGR) do EPS é: {}%", value).into()),
        None => Ok("Erro: EPS inicial ou anos inválidos não podem ser zero.".into()),
    }
    */
    match result {
        Some(value) => {
            let mut response = Response::new(StatusCode::Ok);
            response.set_body(tide::Body::from_json(&json!({"eps_cagr": value}))?);
            Ok(response)
        },
        None => {
            let mut response = Response::new(StatusCode::BadRequest);
            response.set_body(tide::Body::from_json(&json!({"error": "EPS inicial ou anos inválidos não podem ser zero."}))?);
            Ok(response)
        },
    }
}

async fn intrinsic_value(req: Request<()>) -> tide::Result {
    let initial_owners_earnings = req.param("initial_owners_earnings").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let growth_rate = req.param("growth_rate").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let discount_rate = req.param("discount_rate").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    // 'years' é u32, então parseamos para u32
    let years = req.param("years").unwrap_or("0").parse::<u32>().unwrap_or(0); 
    let result = wb_valuation::intrinsic_value(initial_owners_earnings, growth_rate, discount_rate, years);
    
    // Ok(format!("O Valor Intrínseco é: {}", result).into())

    let mut response = Response::new(StatusCode::Ok);
    response.set_body(tide::Body::from_json(&json!({"intrinsic_value": result}))?);
    Ok(response)
}

async fn intrinsic_value_per_share(req: Request<()>) -> tide::Result {
    let initial_owners_earnings = req.param("initial_owners_earnings").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let growth_rate = req.param("growth_rate").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let discount_rate = req.param("discount_rate").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let years = req.param("years").unwrap_or("0").parse::<u32>().unwrap_or(0); // Anos como u32
    let shares_outstanding = req.param("shares_outstanding").unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
    let result = wb_valuation::intrinsic_value_per_share(initial_owners_earnings, growth_rate, discount_rate, years, shares_outstanding);
    /*
    match result {
        Some(value) => Ok(format!("O Valor Intrínseco por Ação é: {}", value).into()),
        None => Ok("Erro: Ações em circulação não podem ser zero.".into()),
    }
    */
    match result {
        Some(value) => {
            let mut response = Response::new(StatusCode::Ok);
            response.set_body(tide::Body::from_json(&json!({"intrinsic_value_per_share": value}))?);
            Ok(response)
        },
        None => {
            let mut response = Response::new(StatusCode::BadRequest);
            response.set_body(tide::Body::from_json(&json!({"error": "Ações em circulação não podem ser zero."}))?);
            Ok(response)
        },
    }
}