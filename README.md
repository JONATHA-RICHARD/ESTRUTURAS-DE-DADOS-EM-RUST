# ESTRUTURAS-DE-DADOS-EM-RUST
Introdução
Este repositório contém a implementação de diversas estruturas de dados fundamentais utilizando a linguagem Rust. O objetivo é demonstrar o funcionamento de Listas Encadeadas, Pilhas, Filas, Tabela Hash e Árvores Binárias, destacando suas características, aplicações e código correspondente.

 Estruturas Implementadas
 1. Lista Encadeada
A Lista Encadeada é uma estrutura de dados dinâmica composta por nós, onde cada nó contém um valor e uma referência ao próximo nó da lista.

📄 Código: lista_encadeada.rs

 Pilha (Stack)
A Pilha segue a estrutura LIFO (Last In, First Out), onde o último elemento inserido é o primeiro a ser removido.

📄 Código: pilha.rs

 Fila (Queue)
A Fila segue a estrutura FIFO (First In, First Out), onde o primeiro elemento inserido é o primeiro a ser removido.

📄 Código: fila.rs

 Tabela Hash
A Tabela Hash utiliza uma função hash para armazenar pares chave-valor, permitindo buscas rápidas.

📄 Código: tabela_hash.rs

 Árvore Binária de Busca
A Árvore Binária organiza os elementos de forma hierárquica, facilitando operações como busca, inserção e remoção.

📄 Código: arvore_binaria.rs

 Como Executar os Códigos
Para testar cada estrutura de dados, siga os passos abaixo:

1️⃣ Clone o repositório:

bash
Copiar
Editar
git clone https://github.com/seu-usuario/seu-repositorio.git
cd seu-repositorio

2️⃣ Compile e execute um código específico (exemplo para a lista encadeada):

bash
Copiar
Editar
rustc lista_encadeada.rs -o lista
./lista

3️⃣ Repita o processo para os outros arquivos conforme necessário
