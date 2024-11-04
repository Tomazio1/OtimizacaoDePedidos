use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;
use chrono::{Duration, NaiveDate, NaiveDateTime};

// Estruturas de dados para representar os dados do problema

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct MateriaPrima {
    pub nome: String,
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct Fornecedor {
    pub tempo_entrega: i64,
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct Produto {
    pub nome: String,
    pub materias_primas: Vec<(MateriaPrima, f64)>, // lista de materias primas e suas quantidades
    pub tempo_fabricacao: i64,
    pub capacidade_producao: i32, // adicionado campo de capacidade de produção
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct Fabrica {
    pub capacidade_armazenamento: f64,
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct Pedido {
    pub produto: Produto,
    pub quantidade_pedido: f64,
    pub data_entrega: String,
    pub solicitar_materia_prima: Option<String>,
}
impl MateriaPrima {
    pub fn from_json(file_name: &str) -> Vec<MateriaPrima> {
        let materias_primas = std::fs::read_to_string(file_name).expect("Erro ao ler arquivo");
        let materias_primas: Vec<MateriaPrima> = serde_json::from_str(&materias_primas).expect("Erro ao desserializar");
        materias_primas
    }
}
impl Pedido {
    pub fn from_json(file_name: &str) -> Vec<Pedido> {
        let pedido = std::fs::read_to_string(file_name).expect("Erro ao ler arquivo");
        let pedido: Vec<Pedido> = serde_json::from_str(&pedido).expect("Erro ao desserializar");
        pedido
    }
}

impl Produto {
    pub fn from_json(file_name: &str) -> Vec<Produto> {
        let produto = std::fs::read_to_string(file_name).expect("Erro ao ler arquivo");
        let produto: Vec<Produto> = serde_json::from_str(&produto).expect("Erro ao desserializar");
        produto
    }
}
impl Fabrica {
    pub fn from_json(file_name: &str) -> Vec<Fabrica> {
        let fabrica = std::fs::read_to_string(file_name).expect("Erro ao ler arquivo");
        let fabrica: Vec<Fabrica> = serde_json::from_str(&fabrica).expect("Erro ao desserializar");
        fabrica
    }
}
impl Fornecedor {
    pub fn from_json(file_name: &str) -> Vec<Fornecedor> {
        let fornecedor = std::fs::read_to_string(file_name).expect("Erro ao ler arquivo");
        let fornecedor: Vec<Fornecedor> = serde_json::from_str(&fornecedor).expect("Erro ao desserializar");
        fornecedor
    }
}

pub fn calcula_data_pedido(tempo_entrega: i64, tempo_fabricacao: i64, data_entrega: &str) -> NaiveDate {
    let data_entrega = NaiveDate::from_str(data_entrega).unwrap();
    let tempo_total = tempo_entrega + tempo_fabricacao;

    let data_pedido = data_entrega - Duration::days(tempo_total);

    data_pedido
}
pub fn acumular_quantidades(quantidades_acumuladas: &mut HashMap<String, f64>, materia_prima: &str, quantidade: f64) {
    let mut existe = false;
    for (chave, valor) in quantidades_acumuladas.iter_mut() {
        if chave == materia_prima {
            existe = true;
            *valor += quantidade;
            break;
        }
    }
    if !existe {
        quantidades_acumuladas.insert(materia_prima.to_string(), quantidade);
    }
}
impl fmt::Display for MateriaPrima {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MateriaPrima")
    }
}
