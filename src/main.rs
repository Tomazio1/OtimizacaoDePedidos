use std::collections::HashMap;
use crate::pcp::{Fabrica, Fornecedor, MateriaPrima, Pedido, Produto};
use std::string::ToString;
use chrono::NaiveDate;
mod pcp;
use pcp::calcula_data_pedido;
use pcp::acumular_quantidades;

fn main() {
    println!("Sistema de Planejamento e Controle de Produção (PCP)");

    // Carregando dados de materiais, fornecedores, produtos e pedidos a partir de arquivos JSON
    let materias_primas: Vec<MateriaPrima> = pcp::MateriaPrima::from_json("materias_primas.json");
    let fornecedores: Vec<Fornecedor> = pcp::Fornecedor::from_json("fornecedor.json");
    let produtos: Vec<Produto> = pcp::Produto::from_json("produtos.json");
    let pedidos: Vec<Pedido> = pcp::Pedido::from_json("pedidos.json");
    let fabricas: Vec<Fabrica> = pcp::Fabrica::from_json("fabrica.json");

    let mut quantidades_acumuladas: HashMap<String, f64> = HashMap::new();
    let mut mapinha: HashMap<String, f64> = HashMap::new();

    // Iterando sobre cada pedido
    for pedido in pedidos.iter() {
        let mut data: NaiveDate = Default::default();
        let mut capacidade_armazem = fabricas[0].capacidade_armazenamento; // Usando a capacidade da primeira fábrica
        let mut lote1 = 0.0; // Inicializando lote

        let quantidade = pedido.quantidade_pedido as usize;

        // Processando cada lote do pedido
        for _ in 0..quantidade {
            // Percorrendo as matérias-primas do produto do pedido
            for a in 0..pedido.produto.materias_primas.len() { // Usando len() para evitar acesso fora dos limites
                let quantidade_materia = pedido.produto.materias_primas[a].1;

                // Verificando se há capacidade suficiente no armazém
                if quantidade_materia <= capacidade_armazem {
                    capacidade_armazem -= quantidade_materia; // Atualizando a capacidade do armazém
                    lote1 += quantidade_materia; // Acumulando a quantidade utilizada
                }

                // Atualizando a quantidade das matérias-primas utilizadas
                let nome_materia = pedido.produto.materias_primas[a].0.nome.to_string();
                *mapinha.entry(nome_materia.clone()).or_insert(0.0) += quantidade_materia;
                acumular_quantidades(&mut quantidades_acumuladas, &nome_materia, quantidade_materia);
            }
        }

        // Calculando a data recomendada
        let nova_data = calcula_data_pedido(fornecedores[0].tempo_entrega, pedido.produto.tempo_fabricacao, &pedido.data_entrega);
        if data < nova_data {
            data = nova_data;
        }

        // Exibindo pedido
        println!("Ingredientes a serem comprados: {:?}", mapinha);
        println!();
        println!("A data recomendada para realizar a compra é: {:?}", data);
        println!();
        println!("----------------VALOR TOTAL DE INGREDIENTES UTILIZADOS----------------");
        println!();
        lote1 = 0.0; // Resetando o lote

        // Limpando o mapa para a próxima iteração
        mapinha.clear();
        // Exibindo quantidades acumuladas de ingredientes
        for (chave, valor) in &quantidades_acumuladas {
            let valor_formatado = format!("{:.2}", valor);
            println!("               Ingrediente: {}  Quantidade utilizada: {}", chave, valor_formatado);
            println!("----------------------------------------------------------------------");
        }

        // Limpar quantidades acumuladas
        quantidades_acumuladas.clear();
    }
}