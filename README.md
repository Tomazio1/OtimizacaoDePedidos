# Sistema de Planejamento e Controle de Produção (PCP)

## Sobre o Projeto

Este projeto é um Sistema de Planejamento e Controle de Produção (PCP) que visa otimizar a produção, logística de matérias-primas, armazenamento, produção e entrega de pedidos. O objetivo é maximizar a produção seguindo as restrições impostas.

O programa lê como entrada os arquivos JSON `fabrica`, `fornecedor`, `materias_primas`, `pedidos` e `produtos`, e realiza alguns cálculos para fazer as recomendações.

## O Problema

O problema consiste na otimização de um sistema de produção, especificamente para maximizar a produção enquanto se segue várias restrições. Isso inclui:

- **Logística de matérias-primas**: Compra e armazenamento.
- **Logística de produção**: Quantidades, ordem e tempo restrito.
- **Logística de pedidos**: Produção e entrega na data correta.

## Tecnologias Utilizadas

- **Rust**
- **JSON**
- **Chrono**
