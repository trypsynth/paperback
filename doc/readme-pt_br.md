<!-- machine-translated from doc/readme.md (source-hash: 13c58fb50049f608); please review and edit as needed -->

# Paperback - versão 0.9.1

## Introdução

Paperback é um leitor de ebook e documentos leve, rápido e acessível para todos, desde leitores ocasionais até usuários avançados. Ele foi projetado com foco em acessibilidade com leitores de tela, velocidade e uma experiência sem recursos desnecessários.

## Requisitos do Sistema

Paperback atualmente funciona em Windows 10/11 e em todas as versões modernas de ARM macOS. Aplicativos nativos para iOS e Android estão em desenvolvimento ativo, com compilações de teste público planejadas em breve após o lançamento do desktop 0.9.0, antes de um lançamento unificado 1.0 cobrindo todas as quatro plataformas.

## Recursos

* Completamente autossuficiente, não exigindo nenhum software instalado no seu computador para começar a ler.
* Incrivelmente rápido, mesmo em hardware antigo.
* Interface simples com abas, permitindo que você abra quantos documentos desejar lado a lado.
* Salva sua posição exata de leitura em todos os documentos que você abre.
* Opcionalmente lembra quais documentos você tinha abertos quando fechou o programa e os restaura no próximo lançamento.
* Inclui funcionalidade de navegação semelhante à encontrada no modo de navegação na web de muitos leitores de tela para navegar rápida e facilmente pelos documentos.
* Inclui um diálogo de busca robusto, com recursos como histórico e suporte a expressões regulares.
* Pode ser executado completamente de forma portátil ou instalado com associações de arquivo configuradas automaticamente.
* Oferece suporte a uma grande variedade de formatos de arquivo comuns.

## Compatibilidade com Leitor de Tela

Paperback funciona bem com todos os principais leitores de tela. Há, no entanto, um problema conhecido para usuários de JAWS.

### JAWS e Displays Braille

Se você usar JAWS com um display Braille, pode descobrir que parágrafos longos são truncados ao percorrer para frente com as teclas de navegação do seu display. O comando de leitura do parágrafo atual também é afetado. Este é um bug no tratamento do controle de texto RICHEDIT50W do JAWS, não algo no próprio Paperback, e um que levou bastante tempo para surgir uma correção considerando o entusiasmo da Vispero em responder a problemas com software de código aberto.

A solução alternativa, eventualmente descoberta através do grupo de discussão do JAWS após meses de espera, é editar `paperback.jcf` e definir "Braille Presentation and Panning" como "Always use DOM if available". Você também vai querer habilitar "Pan Text by Paragraph", caso contrário seu display permanecerá no parágrafo ativo em vez de avançar. Com ambas as configurações em vigor, o deslocamento deve funcionar corretamente.

## Tipos de arquivo atualmente suportados

Paperback oferece suporte aos seguintes formatos e extensões:

* Arquivos de ajuda CHM (`.chm`)
* Livros DAISY (`.opf`, `.zip`)
* Livros EPUB (`.epub`)
* Ebooks FB2 (`.fb2`)
* Documentos HTML (`.htm`, `.html`, `.xhtml`)
* Documentos Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Documentos Microsoft Word (`.docx`, `.docm`, `.doc`)
* Livros MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
* Apresentações OpenDocument (`.odp`, `.fodp`)
* Arquivos de texto OpenDocument (`.odt`, `.fodt`)
* Documentos PDF (`.pdf`)
* Apresentações PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Documentos RTF (`.rtf`)
* Arquivos de texto simples e log (`.txt`, `.log`)

## Atalhos de teclado

Paperback é projetado para uso baseado em teclado. Aqui estão os atalhos atuais.

Os atalhos abaixo são para Windows. Onde o macOS difere, o equivalente é anotado entre parênteses — principalmente porque Ctrl+G, Ctrl+W e Alt+Left/Right já são utilizados por outras convenções de sistema ou app nessa plataforma.

### Menu Arquivo

* `Ctrl+O`: Abrir um documento.
* `Ctrl+F4` (macOS: `Cmd+W`): Fechar o documento atual.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Fechar todos os documentos abertos.
* `Ctrl+Shift+T`: Reabrir o último documento fechado.
* `Ctrl+R`: Mostrar a caixa de diálogo "Todos os Documentos" (de Documentos Recentes).
* `Ctrl+Q`: Sair (Apenas Windows; no macOS isso está no menu do app).

### Menu Ir

* `Ctrl+F`: Mostrar a caixa de diálogo Localizar.
* `F3` (macOS: `Cmd+G`): Localizar próximo.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Localizar anterior.
* `Ctrl+G` (macOS: `Cmd+L`): Ir para linha.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Ir para porcentagem.
* `Ctrl+P`: Ir para página (quando suportado pelo documento atual).
* `=`: Anunciar sua porcentagem de leitura atual.
* `Alt+Left` (macOS: `Cmd+[`): Voltar no histórico de navegação.
* `Alt+Right` (macOS: `Cmd+]`): Avançar no histórico de navegação.
* `[`: Seção anterior.
* `]`: Próxima seção.
* `Shift+H`: Título anterior.
* `H`: Próximo título.
* `Shift+1` até `Shift+6`: Título anterior no nível 1-6.
* `1` até `6`: Próximo título no nível 1-6.
* `Shift+P`: Página anterior.
* `P`: Próxima página.
* `Shift+B`: Marcador anterior.
* `B`: Próximo marcador.
* `/`: Definir seu marcador temporário.
* `\`: Ir para seu marcador temporário.
* `Shift+N`: Anotação anterior.
* `N`: Próxima anotação.
* `Ctrl+B`: Ir para todos os marcadores e anotações.
* `Ctrl+Alt+B`: Ir para marcadores apenas.
* `Ctrl+Alt+M`: Ir para anotações apenas.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, ou seja, a tecla Control física em vez de Cmd): Visualizar texto de anotação na posição atual.
* `Shift+K`: Link anterior.
* `K`: Próximo link.
* `Shift+G`: Imagem anterior.
* `G`: Próxima imagem.
* `Shift+F`: Figura anterior.
* `F`: Próxima figura.
* `Shift+T`: Tabela anterior.
* `T`: Próxima tabela.
* `Shift+S`: Separador anterior.
* `S`: Próximo separador.
* `Shift+L`: Lista anterior.
* `L`: Próxima lista.
* `Shift+I`: Item de lista anterior.
* `I`: Próximo item de lista.
* `Shift+,`: Ir para o início do contêiner atual (lista ou tabela).
* `,`: Ir além do fim do contêiner atual (lista ou tabela).

### Menu Ferramentas

* `Ctrl+W` (macOS: `RawCtrl+W`, ou seja, a tecla Control física em vez de Cmd): Mostrar contagem de palavras do documento atual.
* `Ctrl+I`: Mostrar informações do documento.
* `Ctrl+T`: Mostrar sumário.
* `F7`: Mostrar lista de elementos.
* `Ctrl+Shift+C`: Abrir pasta contendo.
* `Ctrl+Shift+V`: Abrir conteúdo atual em Web View.
* `Ctrl+U`: Visualizar a fonte do documento em uma nova aba.
* `Ctrl+Shift+E`: Exportar dados do documento (`.paperback`).
* `Ctrl+Shift+I`: Importar dados do documento (`.paperback`).
* `Ctrl+E`: Exportar o documento atual para texto simples.
* `Ctrl+Shift+B`: Alternar marcador na seleção/cursor atual.
* `Ctrl+Shift+N`: Adicionar ou editar nota de marcador na seleção/cursor atual.
* `Ctrl+Alt+W`: Alternar quebra de linha.
* `Ctrl+Space`: Reproduzir/pausar narração de áudio.
* `'`: Avançar narração de áudio.
* `;`: Retroceder narração de áudio.
* `Ctrl+'`: Aumentar a quantidade de busca de áudio.
* `Ctrl+;`: Diminuir a quantidade de busca de áudio.
* `F11` (macOS: `RawCtrl+Ctrl+F`, ou seja, Control+Command+F): Alternar tela cheia.
* `Ctrl+,`: Abrir opções (macOS: Preferências, no menu do app).
* `Ctrl+Shift+S`: Alternar temporizador de sono.

### Menu Ajuda

* `Ctrl+F1`: Mostrar caixa de diálogo Sobre.
* `F1`: Visualizar ajuda no seu navegador padrão.
* `Shift+F1`: Visualizar ajuda no Paperback.
* `Ctrl+Shift+U`: Verificar atualizações.
* `Ctrl+D`: Abrir página de doações no seu navegador padrão.

### Teclas adicionais de visualização de documento

* `Delete` / `Numpad Delete` no controle de aba: Fechar a aba do documento selecionado.
* `Enter` ou `Space` no texto do documento: Ativar link no cursor, ou abrir uma visualização de tabela quando em um marcador de tabela.
* `Shift+F10` ou a tecla Menu/Aplicação no texto do documento: Abrir o menu de contexto.

## Idiomas suportados

Paperback é traduzido para muitos idiomas diferentes, com mais sendo adicionados o tempo todo. Uma lista completa segue abaixo.

Para saber como contribuir, leia nosso [Guia de Tradução](translating.md).

* Bósnio
* Checo
* Holandês
* Finlandês
* Francês
* Alemão
* Japonês
* Polonês
* Português (Brasil)
* Russo
* Chinês Simplificado
* Sérvio
* Espanhol
* Vietnamita

## Créditos
### Desenvolvimento
* Quin Gillespie: desenvolvedor principal e fundador do projeto.
* Aryan Choudhary: principal contribuidor.

### Doações
As seguintes pessoas fizeram doações de algum valor para o desenvolvimento do Paperback. Se você fizer uma doação, seu nome não será adicionado automaticamente aqui, eu apenas adiciono pessoas que desejam tornar sua doação pública.

Nota: Considero um patrocinador público do GitHub motivo suficiente para inclusão automática nesta lista.

* Alex Hall
* Brandon McGinty
* Brian Hartgen
* Debbie Yuille
* Devin Prater
* Felix Steindorff
* Hamish Mackenzie
* James Scholes
* Jayson Smith
* Jonathan Rodriguez
* Jonathan Schuster
* Keao Wright
* Michael Marshall
* Pratik Patel
* Roberto Perez
* Sean Randall
* Timothy Wynn
* Tyler Rodick

## Changelog

### Versão 0.9.2
* Audiolivros não fazem mais o leitor de tela ler uma sequência de espaços quando você foca no campo de texto.
* Audiolivros agora nomeiam o arquivo conforme você percorre as seções.
* Audiolivros agora informam seu comprimento real, em vez de afirmar que cada arquivo tem duração de 24 horas.
* Fechar a Visualização da Web com Escape não gera mais um alerta de depuração após você seguir um link dentro dela.
* Copiar após Selecionar Tudo agora oferece o documento inteiro, em vez de apenas a parte carregada no momento.
* Localizar agora vai direto para a linha encontrada, em vez de fazer o leitor de tela ler a janela novamente conforme o foco retorna ao livro.
* Corrigidos EPUBs que carregam um bloco ZIP64 perdido recusando-se a abrir com "Invalid local file header".
* Corrigidos documentos longos voltando para o início enquanto um leitor de tela faz leitura contínua.
* Links na Visualização da Web agora levam você para a seção para a qual apontam, em vez de falhar com "File not found".
* O anúncio automático "Document reloaded" não corta mais o leitor de tela no meio da frase, esperando que ele termine o que está dizendo.
* A aba Geral do diálogo Configurações agora passa por suas opções na ordem em que aparecem na tela, com o canal de atualização diretamente após a opção de verificar atualizações.
* Windows agora sempre mostra "Paperback" no menu Abrir com, em vez da tagline completa do programa.
* Contagem de Palavras e Informações do Documento agora mostram quantos arquivos um audiolivro contém e quanto tempo ele dura no total.

### Versão 0.9.1
* Sons de marcadores e notas agora tocam no macOS.
* Livros DAISY agora reproduzem seu áudio no macOS, em vez de abrir e rastrear sua linha do tempo em silêncio.
* Corrigidas aspas curvas, travessões e caracteres semelhantes desaparecendo de documentos RTF, juntando as palavras ao redor conforme desapareciam.
* Corrigidas imagens RTF vazando seus dados brutos no documento como texto corrompido.
* Corrigido o submenu Documentos Recentes mantendo entradas obsoletas até que algo else acontecesse para reconstruí-lo.
* Aceleradores de teclado estão de volta em cada tradução, então os menus russos têm acesso de teclado novamente.
* Grandes documentos CHM agora abrem até sete vezes mais rapidamente.
* Documentos abertos agora são registrados no Windows, aparecendo na jump list da barra de tarefas e na lista recente do menu Iniciar.
* Opções foi renomeado para Configurações, correspondendo aos aplicativos móveis e, no macOS, à convenção da plataforma.
* Paperback agora lembra a posição, tamanho e estado maximizado da janela entre execuções.
* Formas plurais agora são traduzidas, então mensagens que contam coisas leem corretamente em idiomas que precisam de mais de uma forma.
* Selecionar o ncc.html de um livro DAISY agora abre o audiolivro completo em vez de apenas seu texto.
* Os nomes de ações no diálogo Personalizar Atalhos de Teclado agora podem ser traduzidos.
* O título do documento agora vem primeiro na barra de título, então livros abertos podem ser diferenciados na barra de tarefas e Alt+Tab.
* O diálogo de atualização agora está traduzido.

### Versão 0.9.0

#### Adicionado

##### Geral
* Uma ferramenta CLI, chamada pb, para converter rapidamente qualquer um dos formatos suportados pelo Paperback em HTML, Markdown ou texto simples.
* Uma opção para recarregar documentos que foram modificados por outros programas no disco.
* Uma opção Ver Código-Fonte para abrir o código-fonte de um documento em uma nova aba, útil para editar Markdown, por exemplo.
* O texto do documento agora é paginado, significando que você pode carregar livros com dezenas de milhões de palavras em apenas alguns segundos. Por favor, reporte qualquer estranheza encontrada com isto.

##### Suporte de Plataforma
* Suporte para Windows ARM64!
* Suporte nativo para macOS!
* Um botão de alternância de tela inteira.

##### Diálogo Todos os Documentos
* Um botão localizar para localizar livros perdidos que mudaram de caminho.
* Um filtro de status e barra de status, para que você possa filtrar por status do documento e ver quantos documentos são mostrados e selecionados.
* O atalho `Ctrl+Shift+A` para desselecionar todos os documentos.

##### Opções e Legibilidade
* Uma aba de legibilidade, com as seguintes opções:
    * Quebra de linha (movida de geral);
    * Renderizar tabelas em linha (novo neste lançamento, veja abaixo);
    * Fonte;
    * Cor de fundo;
    * Espaçamento de linha;
    * Espaçamento de parágrafo;
    * Espaçamento de letra;
    * Alinhamento de texto.
* Um item do menu quebra de linha e hotkey subsequente.
* Um alternador para determinar como você quer tabelas exibidas e unificou como tabelas são exibidas nos documentos.

##### Navegação
* Suporte para navegação por container.
* Uma opção para mover automaticamente o cursor para o início da linha ao navegar entre linhas, semelhante ao modo de navegação em leitores de tela.
* O atalho de teclado equals para anunciar sua porcentagem atual através de um documento.

##### Marcadores
* Marcadores temporários: você pode ter um por documento, e eles persistem. Use `/` para definir um e `\` para pular para ele.

##### Contagem de Palavras
* Tempo de leitura estimado no diálogo de contagem de palavras, bem como a capacidade de definir sua velocidade de leitura para tornar esta métrica realmente útil.
* Se uma seleção está ativa quando você abre o diálogo de contagem de palavras, quantas palavras você selecionou serão mostradas agora.

##### Atalhos de Teclado
* A capacidade de personalizar cada atalho de teclado no aplicativo através de um diálogo simples.
* Um atalho de teclado configurável para restaurar Paperback da bandeja do sistema.

##### Idiomas
* Holandês, Finlandês e Polonês.

##### Exportar
* Expandido o item do menu exportar para permitir exportação para HTML e Markdown, além de texto simples.

##### Atualizador
* Um botão cancelar no diálogo de atualização em progresso.
* O atualizador agora valida que o arquivo baixado não foi adulterado.

##### Visualização da Web
* A visualização da web agora é aberta na sua posição de leitura atual.

##### Livros DAISY
* Suporte para livros DAISY 2.0.
* Suporte para reprodução de áudio DAISY 2.02.

##### Audiolivros
* A capacidade de reproduzir audiolivros, suportando atualmente tanto áudio DAISY (incluindo áudio DAISY + texto) quanto zips de arquivos de áudio.
* Atalhos de teclado e itens de menu para reproduzir/pausar narração, buscar para frente e para trás, e ajustar a quantidade de busca.
* Opções para sincronizar o cursor de leitura à reprodução de áudio, definir a quantidade de busca de áudio e escolher se buscar além do final de um capítulo continua no próximo.

##### Documentos CHM
* Suporte para listas, itens de lista, figuras e imagens.

##### PowerPoint
* Documentos PowerPoint agora suportam tabelas.

#### Corrigido

##### Geral
* Documentos codificados em codificações CJK legadas, como GBK, Big5 e Shift_JIS, agora serão renderizados corretamente em vez de como um monte de mojibake.
* "Reabrir último fechado" tentando reabrir o readme agrupado.
* Sua aba selecionada não sendo devidamente focada após reiniciar o Paperback.
* Manipulação do Paperback de arquivos em unidades de rede Windows: pressionar mostrar arquivo na pasta agora foca corretamente o arquivo no armazenamento de rede, e os caminhos não contêm mais caracteres estranhos.
* Arquivos .paperback não serão mais carregados à força na restauração de documentos; em vez disso, você será solicitado a confirmar quando um for encontrado.
* Abrir pasta contendo agora foca o arquivo fornecido no explorer.
* Abrir o readme agora respeitará seu idioma selecionado.
* A interface do usuário do Paperback agora será dimensionada corretamente em exibições de alta DPI.
* O menu agora é atualizado corretamente, e o foco se move para o controle de texto, ao abrir ajuda no Paperback.
* Mudado para um método muito mais seguro de IPC no Windows.
* O título do documento ativo agora será lido ao alternar entre abas.
* Uso reduzido de memória em documentos grandes reduzindo pela metade o tamanho das tabelas de índice por caractere interno.

##### Diálogo Todos os Documentos
* Escape não fechando os diálogos Informações do Documento e Todos os Documentos.
* A barra de título não sendo atualizada após fechar um documento do diálogo todos os documentos.
* Readme.html não será mais adicionado à sua lista de todos os documentos quando aberto via Shift+F1.
* Remover documentos do diálogo recentes agora também fechará suas abas ativas.
* Seu filtro de pesquisa agora é preservado após remover um documento.

##### Navegação
* Navegação de página anunciando texto de linha incorreto em algumas situações.
* Ir para Linha, Ir para Página e Ir para Porcentagem colocando seu cursor na posição errada em documentos grandes.
* Localizar e Localizar Próximo não respeitando a janela do documento carregado em documentos grandes.

##### Marcadores
* Sons de marcador/nota devem agora tocar adequadamente exclusivamente quando você navega sobre uma palavra contendo um.

##### Legibilidade
* Aplicar quebra de linha disparando você para o início do seu documento.

##### Visualização da Web
* O diálogo de visualização da web não sendo redimensionável e aparecendo em um tamanho inicial muito pequeno.
* Imagens devem agora exibir corretamente na visualização da web incorporada.

##### Atualizador
* O atualizador agora exibe corretamente o conteúdo de tags de código markdown nas notas de lançamento.

##### Livros DAISY
* Livros DAISY mostrando informações incorretas na barra de status.
* Carregando livros DAISY com declarações de codificação falsas.

##### Documentos RTF
* Análise de documentos RTF com caracteres não-latinos neles.
* Grupos RTF `\pict` para que dados de imagem incorporados não vazem mais para o texto do documento.

##### Livros Mobi/AZW3
* Âncoras filepos em livros Mobi dividindo tags HTML e colocando lixo no texto do livro.
* Links em livros Mobi legados.
* Análise de AZW3 bastante melhorada.

##### Documentos Word
* Documentos Word com nomes de estilo específicos de localidade não renderizando seus títulos corretamente.

##### Documentos HTML/XHTML
* Elementos dl, dt e dd não produzindo quebras de linha em documentos XHTML.

##### Documentos PDF
* Paperback agora recua para extração de texto simples para PDFs falsamente marcados.
* Documentos PDF contendo caracteres de controle em seus títulos e/ou marcadores não mais travam o Paperback ao abrir.

### Versão 0.8.5
* Adicionado suporte de página para livros epub.
* Adicionado suporte para documentos Microsoft Office criptografados. Atualmente Word legado, Word moderno e Powerpoint moderno são suportados, com Powerpoint legado planejado para o futuro.
* Adicionado suporte para documentos Microsoft Word legados!
* Adicionado suporte para apresentações Powerpoint legadas!
* Adicionado suporte para livros mobi e AZW3!
* Adicionado suporte para arquivos PDF marcados!
* Adicionado o atalho ctrl+q para sair do aplicativo.
* Adicionado suporte para livros compactados do Bookshare (DAISY e Word)!
* Texto alternativo para imagens incorporadas agora deve ser exibido corretamente.
* Documentos CHM agora suportam corretamente navegação de link interno.
* Corrigido ir para página estar desligado por 1.
* Corrigida a tecla escape não funcionando para fechar o diálogo abrir como.
* Corrigido o menu de contexto do leitor não aparecendo ao clicar com o botão direito ou pressionar a tecla Aplicações.
* Corrigido o documento errado sendo focado às vezes ao abrir documentos da linha de comando.
* PDFs somente com imagem são novamente detectados e o alertam sobre sua existência.
* Agora é possível navegar por imagens e figuras com g/shift+g e f/shift+f, respectivamente.
* Paperback agora respeitará sua configuração de modo escuro do aplicativo.
* Removido suporte a DAISY XML, pois não é mais necessário.
* Voltado para a navegação nativa de primeira letra Win32 na árvore do sumário.
* O diálogo de erro de carregamento agora mostra mensagens de erro mais detalhadas.
* A visualização da web agora abrirá muito mais rápido e suavemente.

### Versão 0.8.2
* Adicionado suporte de página para documentos RTF!
* Corrigido um bug onde abrir a visualização da web em epubs contendo links externos os ativaria automaticamente.
* Corrigido um bug onde o analisador RTF não colocaria espaço entre palavras em casos raros.
* Corrigidos parágrafos sendo divididos em várias linhas curtas em alguns documentos PDF.
* Documentos PDF agora têm suporte básico de navegação de link e título!
* Abas RTF e feeds de linha agora são renderizados exatamente como aparecem no documento.
* Voltado para a biblioteca pdfium confiável para análise de PDFs, tornando a renderização de PDF muito mais confiável novamente.

### Versão 0.8.1
* Adicionado Ctrl+Shift+T para reabrir o último documento fechado.
* O diálogo Todos os Documentos agora suporta seleção de múltiplos documentos para abrir de uma vez.
* Corrigidos alguns bugs com o analisador RTF.
* Corrigidos caminhos de arquivo contendo caracteres não-ASCII (como Bósnio š, č, ć, ž) sendo corrompidos ao abrir um arquivo via uma segunda instância do Paperback.
* Corrigido texto PDF sendo lido na ordem errada e espaçamento incorreto ao redor de palavras capitalizadas.
* Corrigido carregamento lento de documento ao abrir arquivos grandes.
* Corrigida a localização dos botões Sim/Não em diálogos de confirmação.

### Versão 0.8.0
* Adicionadas traduções para Japonês, Chinês Simplificado e Vietnamita!
* Adicionado um atualizador automático que agora substituirá sua versão instalada do Paperback em vez de apenas baixar a nova versão!
* Adicionado feedback sonoro opcional para alcançar um marcador ou uma nota, obrigado Andre Louis pelos sons!
* Adicionado suporte para documentos RTF!
* Adicionado suporte para documentos DAISY XML.
* Adicionado suporte para arquivos Flat Open Document Text!
* Adicionado suporte para apresentações Flat Open Document!
* Adicionado suporte para separadores com s e shift+s.
* Qualquer movimento maior que 300 caracteres agora adicionará automaticamente ao seu histórico de navegação.
* Corrigida a restauração da janela do Paperback da bandeja do sistema.
* Corrigidos documentos Markdown mostrando texto bruto em vez de HTML renderizado na Visualização da Web.
* Corrigidas tabelas não sendo renderizadas corretamente em arquivos Markdown.
* PDFs somente com imagem agora avisam sobre sua existência quando você tenta carregar um.
* Informações de versão apropriadamente incorporadas no executável do Paperback.
* Dividir o diálogo de opções em abas para facilitar o uso e a navegação.
* Voltado para Hayro para análise de PDFs, levando a mais confiabilidade, velocidade e menos DLLs.
* Reescrito o aplicativo inteiro em Rust. A nova base de código é mais segura, carrega documentos mais rapidamente e é mais fácil de manter e estender.
* O menu de contexto do controle de texto agora incluirá ações específicas do leitor em vez de itens genéricos como cortar e colar.

### Versão 0.7.0
* Adicionado suporte a tabelas para documentos baseados em HTML e XHTML! Navegue entre tabelas usando T e Shift+T, e pressione Enter para visualizar uma em uma visualização da web.
* Adicionado um recurso básico de renderização web! Pressione Ctrl+Shift+V para abrir a seção atual do seu documento em um renderizador baseado na web, útil para conteúdo como formatação complexa ou exemplos de código.
* Adicionada uma tradução russa, obrigado Ruslan Gulmagomedov!
* Adicionado um botão Limpar Tudo ao diálogo Todos os Documentos.
* O verificador de atualização agora exibe notas de lançamento quando uma nova versão está disponível.
* Corrigida a restauração da janela da bandeja do sistema.
* Corrigidas traduções de botões Sim/Não em diálogos de confirmação.
* Corrigido carregamento de configs ao executar como administrador.
* Corrigido tratamento de comentários em documentos XML e HTML.
* Corrigida análise de TOC em livros Epub 2.
* Corrigida navegação para o próximo item com a mesma letra no sumário.
* Corrigido o diálogo localizar não se escondendo corretamente ao usar os botões próximo/anterior.
* Corrigidos TOCs de epub ocasionalmente jogando você para o item errado.
* Corrigidos vários problemas de tratamento de espaço em branco em XML, HTML e tags pre.
* Corrigido erro off-by-one na navegação de link.
* Corrigidos alguns livros tendo espaço em branco à direita em suas linhas.
* Corrigidos vários problemas de análise.
* Itens de menu relacionados a marcadores bem como a lista de elementos agora estão devidamente desabilitados quando nenhum documento está aberto.
* Melhorado tratamento de lista em vários formatos de documento.
* Melhorado o fluxo de trabalho de tradução para colaboradores.
* Muitas refatorações internas, movendo a maioria da lógica de negócios do aplicativo de C++ para Rust para melhor desempenho e manutenibilidade.

### Versão 0.6.1
* Adicionado suporte a PDF protegido por senha!
* Adicionado um recurso muito básico de ir para a posição anterior/próxima. Se você pressionar enter em um link interno e isso mover seu cursor, aquela posição agora será lembrada e pode ser navegada com setas Alt+Left/Right.
* Adicionada uma lista de elementos! Atualmente, ela mostra apenas uma árvore de todos os títulos em seu documento ou uma lista de links, mas há planos para expandi-la no futuro.
* Adicionada uma opção para iniciar o Paperback em modo maximizado por padrão.
* Corrigidos links em alguns documentos Epub não funcionando corretamente.
* Corrigida análise de TOCs de Epub contendo caminhos relativos.
* Corrigidos alguns documentos epub não mostrando um título ou autor.
* Corrigidos os títulos de alguns capítulos de epub não aparecendo corretamente no diálogo TOC.
* Corrigido você não ser capaz de usar a barra de espaço para ativar os botões OK/cancelar no diálogo TOC.
* Melhorado o tratamento de títulos em documentos Word.
* Você agora receberá feedback falado se a lista de documentos recentes estiver vazia quando você tentar trazer o diálogo.

### Versão 0.6.0
* Uma nova opção para mostrar o menu ir em uma forma muito mais compacta foi adicionada ao diálogo de opções, marcada por padrão.
* Adicionada uma opção para fazer navegação por elementos estruturais encapsular.
* Adicionada uma opção ao menu ferramentas para abrir a pasta contendo o documento focado no momento.
* Adicionado um sistema de atualização bastante simples, mas muito eficaz.
* Adicionado um recurso básico de temporizador de sono, acessível com Ctrl+Shift+S.
* Adicionado suporte para análise de livros FB2!
* Adicionado suporte para análise de apresentações OpenDocument!
* Adicionado suporte para análise de arquivos OpenDocument Text!
* Marcadores agora podem ser feitos para marcar uma linha inteira ou para marcar apenas alguns textos especificados. Se você não tiver seleção ativa ao colocar um marcador, o comportamento é como pré-0.6 e marcará a linha inteira. No entanto, se você selecionar algum texto, apenas esse texto será incluído no marcador.
* Marcadores agora podem ter notas de texto opcionais anexadas a eles! Navegue entre marcadores contendo notas com N e Shift+N, ou apareça o diálogo de marcadores com todos os marcadores, apenas notas ou apenas não-notas selecionados com hotkeys específicas.
* Marcadores no diálogo de marcadores não terão mais um prefixo "bookmark x" irritante.
* Livros Epub contendo conteúdo HTML fingindo ser XML agora serão tratados corretamente.
* Corrigido carregamento de grandes documentos Markdown.
* Corrigido pressionar espaço na árvore de visualização do sumário ativando o botão OK.
* Corrigido tratamento de espaço em branco no início de tags pre em documentos HTML e XHTML.
* Corrigido o campo de texto não recuperando o foco às vezes ao retornar à janela do Paperback.
* Corrigido o campo de texto no diálogo ir para porcentagem não atualizando o valor do controle deslizante.
* Corrigida a renderização de IDs HTML personalizados em documentos Markdown.
* HTML dentro de blocos de código Markdown agora será renderizado corretamente.
* Se carregar um livro com um parâmetro de linha de comando enquanto uma instância existente do Paperback está em execução, você não obterá mais um erro se carregar seu documento levar mais de 5 segundos.
* Se executar o Paperback como administrador, a configuração agora será corretamente carregada e salva.
* Agora é possível excluir um marcador diretamente no diálogo de marcadores.
* Agora é possível importar e exportar seus marcadores e posição de leitura para um documento específico. O arquivo gerado é nomeado após o arquivo com uma extensão .paperback. Se tal arquivo for encontrado no mesmo diretório do arquivo ao carregá-lo, será automaticamente carregado. Caso contrário, você pode importá-los manualmente usando um item no menu ferramentas.
* Links dentro de documentos agora são totalmente suportados! Use k e shift+k para se mover para frente e para trás através deles, e pressione enter para abrir/ativar um.
* Muitas refatorações internas, tornando o aplicativo mais rápido e o binário menor.
* Conteúdo Markdown agora é pré-processado para ser compatível com CommonMark antes de ser renderizado.
* Navegação por listas e seus itens agora é totalmente suportada! Use L e Shift+L para ir por listas em si, e I e Shift+I para passar por itens de lista.
* Delete da Numpad agora funciona para remover documentos da barra de abas além do delete normal.
* Paperback agora pode opcionalmente minimizar para sua bandeja do sistema! Esta opção está desativada por padrão, mas ativá-la fará a opção minimizar no menu do sistema colocar o Paperback em sua bandeja, podendo ser restaurado clicando no ícone gerado.
* Paperback agora é totalmente traduzível! A lista de idiomas que suporta é atualmente bastante pequena, mas está constantemente crescendo!
* Paperback agora tem um site oficial, em [paperback.dev](https://paperback.dev)!
* Documentos PPTX agora mostram um sumário básico, contendo todos os slides.
* O caminho completo para o documento aberto agora será mostrado no diálogo informações do documento.
* O instalador agora inclui uma opção para visualizar o readme no seu navegador após a instalação.
* A lista de documentos recentes foi dramaticamente expandida! Em vez de simplesmente mostrar os últimos 10 documentos que você abriu, agora mostrará um número personalizável, com o resto dos documentos que você já abriu sendo acessível através de um pequeno diálogo.
* Várias pequenas melhorias nos analisadores em toda a placa, incluindo colocar uma linha em branco entre slides em apresentações PPTX, corrigindo o tratamento de nova linha dentro de parágrafos em documentos word e adicionando pontos de bala a itens de lista.

### Versão 0.5.0
* Adicionado suporte para documentos Microsoft Word!
* Adicionado suporte para apresentações PowerPoint!
* Corrigidos certos itens de menu não sendo desabilitados com nenhum documento aberto.
* Corrigida a orientação do controle deslizante ir para porcentagem.
* Corrigido o sumário em livros Epub com caminhos de arquivo codificados em URL e/ou IDs de fragmento.
* Corrigido espaço em branco sendo retirado de títulos XHTML de maneiras estranhas.
* Corrigido tratamento de espaço em branco dentro de tags pre aninhadas em documentos HTML.
* Documentos HTML e Markdown agora suportam o recurso de sumário! Quando você carrega um documento HTML/Markdown, Paperback construirá seu próprio sumário fora da estrutura dos títulos no seu documento, e o mostrará a você no diálogo ctrl+t.
* Documentos HTML agora terão o título definido na tag title, se existir. Caso contrário, continuarão usando o nome do arquivo sem a extensão.
* Alternado de UniversalSpeech para usar uma live region para relatar fala. Isto significa que nenhuma DLL do leitor de tela é enviada junto com o programa, e mais leitores de tela agora serão suportados, como o Microsoft Narrator.
* Biblioteca de zip alternada para permitir abertura de uma gama mais ampla de livros epub.
* O diálogo pedindo se você quer abrir seu documento como texto simples foi completamente refeito, e agora permite que você abra seu documento como texto simples, HTML ou Markdown.
* O diálogo ir para porcentagem agora inclui um campo de texto permitindo que você insira manualmente uma porcentagem para pular para.
* O analisador HTML agora reconhecerá dd, dt e dl como elementos de lista.
* O sumário em livros Epub será novamente preservado exatamente.
* O espaço não quebrável unicode agora é considerado ao retirar linhas em branco.
* Você não será mais perguntado como quer abrir um arquivo não reconhecido toda vez que o carregar, apenas na primeira vez.

### Versão 0.4.1
* Adicionado um ícone do menu Iniciar opcional ao instalador.
* O sumário agora deve estar mais limpo em alguns casos, por exemplo se você tiver um item filho e pai com o mesmo texto na mesma posição você agora verá apenas o item pai.
* Corrigido o sumário em certos documentos CHM.
* Corrigido o sumário em livros Epub 3 com caminhos absolutos neles.
* Documentos CHM agora devem mostrar seu título conforme definido no arquivo de metadados.

### Versão 0.4.0
* Adicionado suporte para arquivos CHM!
* Adicionado suporte para marcadores! Você pode ter tantos marcadores em quantos documentos desejar. Você pode pular para frente e para trás através deles com b e shift+b, definir um com control+shift+b, e trazer um diálogo para pular para um marcador específico com control+b.
* Adicionado um instalador junto com o arquivo zip portátil! O instalador instalará o Paperback no seu diretório Program Files e configurará automaticamente as associações de arquivo para você.
* Arquivos de texto com BOMs agora devem ser decodificados corretamente, e o BOM não será mais exibido no início do texto.
* Adicionadas muito mais informações à barra de status. Agora mostrará sua linha atual, caractere e porcentagem de leitura.
* Comentários HTML, bem como o conteúdo de tags script e style, não serão mais mostrados em saída de texto.
* Se passar um caminho relativo para Paperback na linha de comando, agora resolverá corretamente.
* Movimento de porcentagem agora é tratado por seu próprio diálogo baseado em controle deslizante, acessível com control+shift+g.
* Documentos sem títulos ou autores conhecidos agora sempre terão um padrão.
* A lógica de salvamento de posição agora é muito mais inteligente e deve escrever no disco apenas quando absolutamente necessário.
* O documento que você focou quando fechou o Paperback agora é lembrado entre reinicializações do aplicativo.
* Entrada nos diálogos ir para linha e ir para página agora deve ser higienizada mais rigidamente.
* Corrigida navegação do sumário em livros epub 3 com caminhos relativos em seus manifestos.

### Versão 0.3.0
* Corrigido o sumário em livros epub com manifestos codificados em URL.
* Corrigida navegação de título em documentos HTML contendo caracteres Unicode multibyte.
* Corrigido alto uso de CPU em documentos com títulos longos devido a uma regressão em wxWidgets.
* Corrigido carregamento de arquivos de texto UTF-8.
* Corrigidos itens de TOC aninhados em livros Epub colocando seu cursor na posição errada.
* Corrigido um crash na saída do aplicativo em certos casos.
* Adicionada uma caixa de seleção no diálogo de opções para ativar ou desativar quebra de linha!
* Agora é possível doar ao desenvolvimento do Paperback, através do novo item doação no menu ajuda ou através do link sponsor this project na parte inferior da página principal do repositório GitHub.
* Documentos Markdown agora sempre terão um título, e Paperback agora deve ser capaz de carregar praticamente qualquer arquivo Markdown.
* Documentos PDF agora sempre terão um título, mesmo se os metadados estiverem faltando.
* Alternado as bibliotecas de PDF para a usada no Chromium, levando a análise de PDF muito mais confiável em toda a placa.
* Agora você pode ter apenas uma instância do Paperback em execução por vez. Executar paperback.exe com um nome de arquivo enquanto já está em execução abrirá esse documento na instância já em execução.
* Agora você pode pressionar delete em um documento no controle de aba para fechá-lo.

### Versão 0.2.1
* Adicionado o número total de páginas ao rótulo de página no diálogo ir para página.
* Permitir tabbing do conteúdo do documento para sua lista de documentos abertos.
* Corrigidos alguns bugs com os atalhos de título às vezes abrindo documentos recentes se você tivesse o suficiente deles.
* Paperback agora removerá hífens leves desnecessários da saída de texto.
* Corrigida navegação de título às vezes colocando você no caractere errado.

### Versão 0.2.0
* Adicionado suporte para documentos markdown!
* Adicionado suporte para documentos PDF, incluindo a capacidade de navegar entre páginas!
* Adicionadas teclas de atalho para navegação por títulos em conteúdo HTML, incluindo livros epub e documentos markdown. Estas teclas de atalho foram projetadas para funcionar semelhante a um leitor de tela.
* Corrigido carregamento de epubs com nomes de arquivo codificados em URL em seus manifestos.
* Corrigido carregamento de livros epub 3 com XHTML incorporado neles.
* Uma mensagem agora é falada se o documento não suporta um sumário ou seções, em oposição aos itens do menu serem desabilitados.
* Adicionado um menu de documentos recentes! Atualmente armazena seus últimos 10 documentos abertos, e pressionar enter em um os abrirá para leitura.
* Completamente reescrito o diálogo Localizar, tornando muito mais simples de usar, enquanto também adicionado um histórico de suas últimas 25 pesquisas e suporte a expressão regular!
* Documentos previamente abertos agora são lembrados entre reinicializações do aplicativo. Isto é configurável através do novo item opções no menu ferramentas.
* Adicionado shift+f1 para abrir o readme diretamente no Paperback.

### Versão 0.1.0
* Lançamento inicial.
