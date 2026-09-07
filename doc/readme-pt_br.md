<!-- machine-translated from doc/readme.md (source-hash: c5a741eb100e6fbc; sections: f48ce952,d4d8160f,a02f4421,a4ffb7f7,91be3b41,55bac79e,a548b5d0,71df8e94,e9860ee8,c7735cbe); please review and edit as needed -->

# Paperback - versão 0.9.2

## Introdução

Paperback é um leitor de ebook e documentos leve, rápido e acessível para todos, desde leitores ocasionais até usuários avançados. É projetado para acessibilidade com leitores de tela, velocidade rápida e uma experiência livre de inchaço.

## Requisitos do Sistema

Paperback atualmente funciona no Windows 10/11 e em todas as versões modernas do macOS ARM. Aplicativos nativos para iOS e Android estão em desenvolvimento ativo, com compilações de teste públicas planejadas logo após o lançamento da versão 0.9.0 para desktop, antes de um lançamento unificado da versão 1.0 cobrindo todas as quatro plataformas.

## Recursos

* Completamente autossuficiente, não requerendo que nenhum software seja instalado em seu computador para começar a ler.
* Incrivelmente rápido, mesmo em hardware antigo.
* Interface com abas simples, permitindo que você abra quantos documentos desejar lado a lado.
* Salva sua posição de leitura exata em cada documento que você abre.
* Opcionalmente, lembra quais documentos você tinha abertos quando fechou o programa e os restaura no próximo lançamento.
* Inclui funcionalidade de navegação semelhante à encontrada no modo de navegação na web de muitos leitores de tela para navegar rápida e facilmente pelos documentos.
* Inclui um robusto diálogo de busca, incluindo recursos como histórico e suporte a expressões regulares.
* Pode ser executado totalmente de forma portátil ou instalado com associações de arquivo configuradas automaticamente.
* Suporta uma enorme variedade de formatos de arquivo comuns.

## Compatibilidade com Leitores de Tela

Paperback funciona bem com todos os principais leitores de tela. Há, no entanto, um problema conhecido para usuários do JAWS.

### JAWS e Linhas Braille

Se você usar JAWS com uma linha braille, pode descobrir que parágrafos longos são truncados ao fazer panning para frente com as teclas de navegação da sua linha. O comando ler parágrafo atual também é afetado. Esse é um bug no tratamento do JAWS do controle de texto RICHEDIT50W, não algo no Paperback em si, e um que levou bastante tempo para surgir um corretivo considerando o entusiasmo da Vispero em responder a problemas de software de código aberto.

A solução alternativa, eventualmente descoberta através do grupo de discussão do JAWS após meses de espera, é editar `paperback.jcf` e definir "Braille Presentation and Panning" para "Always use DOM if available". Você também vai querer habilitar "Pan Text by Paragraph", caso contrário sua linha permanecerá no parágrafo ativo em vez de avançar. Com ambas as configurações em vigor, o panning deve funcionar corretamente.

## Tipos de arquivo atualmente suportados

Paperback suporta os seguintes formatos e extensões:

* Arquivos de ajuda CHM (`.chm`)
* Livros DAISY (`.opf`, `.zip`)
* Livros EPUB (`.epub`)
* Ebooks FB2 (`.fb2`)
* Documentos HTML (`.htm`, `.html`, `.xhtml`)
* Documentos Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Documentos Microsoft Word (`.docx`, `.docm`, `.doc`)
* Audiolivros M4B (`.m4b`)
* Livros MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
* Apresentações OpenDocument (`.odp`, `.fodp`)
* Arquivos de texto OpenDocument (`.odt`, `.fodt`)
* Documentos PDF (`.pdf`)
* Apresentações PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Documentos RTF (`.rtf`)
* Arquivos de texto simples e logs (`.txt`, `.log`)

## Atalhos de teclado

O Paperback foi projetado para uso orientado pelo teclado. Aqui estão os atalhos atuais.

Os atalhos abaixo são para Windows. Quando o macOS difere, o equivalente é anotado entre parênteses — principalmente porque Ctrl+G, Ctrl+W e Alt+Left/Right já são usados por outras convenções de sistema ou aplicativo nessa plataforma.

### Menu Arquivo

* `Ctrl+O`: Abrir um documento.
* `Ctrl+F4` (macOS: `Cmd+W`): Fechar o documento atual.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Fechar todos os documentos abertos.
* `Ctrl+Shift+T`: Reabrir o último documento fechado.
* `Ctrl+R`: Mostrar o diálogo "Todos os Documentos" (a partir de Documentos Recentes).
* `Ctrl+Q`: Sair (apenas Windows; no macOS isso fica no menu do aplicativo).

### Menu Ir

* `Ctrl+F`: Mostrar o diálogo Localizar.
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
* `Shift+N`: Nota anterior.
* `N`: Próxima nota.
* `Ctrl+B`: Ir para todos os marcadores e notas.
* `Ctrl+Alt+B`: Ir para marcadores apenas.
* `Ctrl+Alt+M`: Ir para notas apenas.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, ou seja, a tecla Control física em vez de Cmd): Visualizar texto de nota na posição atual.
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
* `Ctrl+E`: Exportar o documento atual como texto simples.
* `Ctrl+Shift+B`: Alternar marcador na seleção/cursor atual.
* `Ctrl+Shift+N`: Adicionar ou editar nota de marcador na seleção/cursor atual.
* `Ctrl+Alt+W`: Alternar quebra de linha de palavras.
* `Ctrl+Space`: Reproduzir/pausar narração de áudio.
* `'`: Avançar narração de áudio.
* `;`: Retroceder narração de áudio.
* `Ctrl+'`: Aumentar a quantidade de busca de áudio.
* `Ctrl+;`: Diminuir a quantidade de busca de áudio.
* `F11` (macOS: `RawCtrl+Ctrl+F`, ou seja, Control+Command+F): Alternar tela inteira.
* `Ctrl+,`: Abrir opções (macOS: Preferências, no menu do aplicativo).
* `Ctrl+Shift+S`: Alternar temporizador de sono.

### Menu Ajuda

* `Ctrl+F1`: Mostrar diálogo Sobre.
* `F1`: Visualizar ajuda no seu navegador padrão.
* `Shift+F1`: Visualizar ajuda no Paperback.
* `Ctrl+Shift+U`: Verificar atualizações.
* `Ctrl+D`: Abrir página de doação no seu navegador padrão.

### Teclas adicionais de visualização de documento

* `Delete` / `Numpad Delete` no controle de aba: Fechar a aba do documento selecionada.
* `Enter` ou `Space` no texto do documento: Ativar link no cursor, ou abrir uma visualização de tabela quando em um marcador de tabela.
* `Shift+F10` ou a tecla Menu/Aplicação no texto do documento: Abrir o menu de contexto.

## Idiomas suportados

O Paperback é traduzido para muitos idiomas diferentes, com mais sendo adicionados o tempo todo. Uma lista completa segue abaixo.

Para aprender como contribuir, leia nosso [Guia de Tradução](translating.md).

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
* Aryan Choudhary: principal colaborador.

### Doações
As seguintes pessoas fizeram doações de algum tamanho para o desenvolvimento do Paperback. Se você fizer uma doação, seu nome não será adicionado automaticamente aqui. Apenas adiciono pessoas que desejam que sua doação seja pública.

Nota: Considero um patrocinador público do GitHub como motivo para inclusão automática nesta lista.

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

## Registro de alterações

### Versão 0.9.2
* Audiolivros deixaram de fazer o leitor de tela ler uma sequência de espaços quando você coloca o foco no campo de texto.
* Audiolivros agora nomeiam o arquivo conforme você avança por seções.
* Audiolivros agora informam seu comprimento real, em vez de afirmar que cada arquivo neles dura 24 horas.
* Fechar a Web View com Escape não lança mais um alerta de depuração depois que você segue um link dentro dela.
* Copiar após Selecionar Tudo agora oferece o documento inteiro, em vez de apenas a parte atualmente carregada.
* Find agora vai direto para a linha encontrada, em vez de fazer você ouvir o leitor de tela ler a janela novamente conforme o foco retorna ao livro.
* Corrigidos EPUBs que carregam um bloco ZIP64 isolado recusando abrir com "Invalid local file header".
* Corrigidos documentos longos voltando ao início enquanto um leitor de tela os lia continuamente.
* Links na WebView agora levam você para a seção para a qual apontam, em vez de falharem com "File not found".
* O anúncio automático "Document reloaded" não corta mais seu leitor de tela no meio da frase, esperando em vez disso que ele termine o que estava dizendo.
* A aba General do diálogo Settings agora tira dúvida por suas opções na ordem em que aparecem na tela, com o canal de atualização diretamente após a opção de verificação de atualizações.
* Windows agora sempre mostrará "Paperback" no menu Open With, em vez da tagline completa do programa.
* Word Count e Document Info agora mostram quantos arquivos um audiolivro contém e por quanto tempo ele é executado no total.

### Versão 0.9.1
* Sons de marcador e nota agora tocam no macOS.
* Livros DAISY agora reproduzem seu áudio no macOS, em vez de abrir e rastrear sua linha do tempo em silêncio.
* Corrigidas aspas curvas, travessões e caracteres similares desaparecendo de documentos RTF, juntando as palavras ao redor conforme desapareciam.
* Corrigidas imagens RTF vazando seus dados brutos no documento como texto corrompido.
* Corrigido o submenu Documentos Recentes mantendo entradas obsoletas até que algo mais acontecesse para reconstruí-lo.
* Aceleradores de teclado estão de volta em todas as traduções, então os menus do russo têm acesso ao teclado novamente.
* Grandes documentos CHM agora abrem até sete vezes mais rápido.
* Documentos abertos agora são registrados no Windows, aparecendo na jump list da barra de tarefas e na lista recente do menu Iniciar.
* Options foi renomeado para Settings, combinando com os aplicativos móveis e, no macOS, com a convenção da plataforma.
* Paperback agora se lembra da posição, tamanho e estado maximizado da janela entre execuções.
* Formas plurais agora são traduzidas, então mensagens que contam coisas são lidas adequadamente em idiomas que precisam de mais de uma forma.
* Selecionar o ncc.html de um livro DAISY agora abre o audiolivro completo em vez de apenas seu texto.
* Os nomes de ações no diálogo Customize Keyboard Shortcuts agora podem ser traduzidos.
* O título do documento agora vem primeiro na barra de título, então livros abertos podem ser diferenciados na barra de tarefas e Alt+Tab.
* O diálogo de atualização agora é traduzido.

### Versão 0.9.0

#### Adicionado

##### Geral
* Uma ferramenta CLI, chamada pb, para converter rapidamente qualquer um dos formatos suportados do Paperback para HTML, Markdown ou texto simples.
* Uma opção para recarregar documentos que foram modificados por outros programas em disco.
* Uma opção View Source para abrir a fonte de um documento em uma nova aba, útil para editar Markdown, por exemplo.
* O texto do documento agora é paginado, o que significa que você pode carregar livros com dezenas de milhões de palavras em apenas alguns segundos. Por favor, informe qualquer coisa estranha encontrada com isso.

##### Suporte de Plataforma
* Suporte a ARM64 no Windows!
* Suporte nativo a macOS!
* Um toggle de tela inteira.

##### Diálogo All Documents
* Um botão localize para localizar livros ausentes que apenas mudaram seu caminho.
* Um filtro de status e barra de status, então você pode filtrar por status do documento e ver quantos documentos são mostrados e selecionados.
* O atalho `Ctrl+Shift+A` para desselecionar todos os documentos.

##### Opções e Legibilidade
* Uma aba de legibilidade, com as seguintes opções:
    * Word wrap (movido de general);
    * Render tables inline (novo neste lançamento, veja abaixo);
    * Font;
    * Background color;
    * Line spacing;
    * Paragraph spacing;
    * Letter spacing;
    * Text alignment.
* Um item de menu word wrap e hotkey subsequente.
* Um toggle para determinar como você deseja que as tabelas sejam exibidas, e unificado como as tabelas são exibidas em documentos.

##### Navegação
* Suporte para navegação por container.
* Uma opção para mover automaticamente o cursor para o início da linha ao navegar entre linhas, similar ao modo de navegação em leitores de tela.
* O atalho de teclado equals para anunciar sua porcentagem atual através de um documento.

##### Marcadores
* Marcadores temporários: você pode ter um por documento, e eles persistem. Use slash para definir um e backslash para pular para ele.

##### Contagem de Palavras
* Tempo de leitura estimado no diálogo de contagem de palavras, bem como a capacidade de definir sua velocidade de leitura para tornar esta métrica realmente útil.
* Se uma seleção estiver ativa quando você abrir o diálogo de contagem de palavras, quantas palavras você selecionou agora será mostrado.

##### Atalhos de Teclado
* A capacidade de personalizar cada atalho de teclado no aplicativo através de um diálogo simples.
* Um atalho de teclado configurável para restaurar Paperback da bandeja do sistema.

##### Idiomas
* Holandês, Finlandês e Polonês.

##### Exportar
* Expandido o item de menu exportar para permitir exportação para HTML e Markdown, além de texto simples.

##### Atualizador
* Um botão de cancelar para o diálogo de atualização em andamento.
* O atualizador agora valida se o arquivo baixado não foi alterado.

##### Web View
* A webview agora é aberta na sua posição de leitura atual.

##### Livros DAISY
* Suporte para livros DAISY 2.0.
* Suporte para reprodução de áudio DAISY 2.02.

##### Audiolivros
* A capacidade de reproduzir audiolivros, atualmente suportando DAISY audio (incluindo DAISY audio + texto) e zips de arquivos de áudio.
* Atalhos de teclado e itens de menu para reproduzir/pausar narração, buscar para frente e para trás, e ajustar a quantidade de busca.
* Opções para sincronizar o cursor de leitura com reprodução de áudio, definir a quantidade de busca de áudio e escolher se buscar além do final de um capítulo continua para o próximo.

##### Documentos CHM
* Suporte para listas, itens de lista, figuras e imagens.

##### PowerPoint
* Documentos PowerPoint agora suportam tabelas.

#### Corrigido

##### Geral
* Documentos codificados em codificações CJK legadas, como GBK, Big5 e Shift_JIS, agora serão renderizados adequadamente em vez de como um monte de mojibake.
* "Reopen last closed" tentando reabrir o readme agrupado.
* Sua aba selecionada não recebendo o foco adequadamente após reiniciar o Paperback.
* O manuseio do Paperback de arquivos em unidades de rede do Windows: pressionar show file in folder agora coloca o foco adequadamente no arquivo no armazenamento de rede, e os caminhos não contêm mais caracteres estranhos.
* Arquivos .paperback não serão mais carregados à força na restauração de documentos; em vez disso, você será pedido para confirmação quando um for encontrado.
* Open containing folder agora coloca o foco no arquivo fornecido no explorer.
* Abrir o readme agora respeitará seu idioma selecionado.
* A interface do usuário do Paperback agora será dimensionada adequadamente em exibições de alta DPI.
* O menu agora se atualiza adequadamente, e o foco se move para o controle de texto, quando abre ajuda no Paperback.
* Mudado para um método muito mais seguro de IPC no Windows.
* O título do documento ativo agora será lido ao alternar entre abas.
* Reduzido o uso de memória em documentos grandes reduzindo pela metade o tamanho das tabelas de índice interno por caractere.

##### Diálogo All Documents
* Escape não fechando os diálogos Document Info e All Documents.
* A barra de título não se atualizando após fechar um documento do diálogo de todos os documentos.
* Readme.html não será mais adicionado à sua lista de todos os documentos quando aberto via Shift+F1.
* Remover documentos do diálogo recentes agora também fechará sua aba ativa.
* Seu filtro de busca agora é preservado após remover um documento.

##### Navegação
* Navegação de página anunciando texto de linha incorreto em algumas situações.
* Go to Line, Go to Page e Go to Percent colocando seu cursor na posição errada em documentos grandes.
* Find e Find Next não respeitando a janela de documento carregada em documentos grandes.

##### Marcadores
* Sons de marcador/nota agora devem tocar adequadamente exclusivamente quando você navega sobre uma palavra contendo um.

##### Legibilidade
* Aplicar word wrap o atirando para o início do seu documento.

##### Web View
* O diálogo webview não sendo redimensionável e aparecendo em um tamanho inicial muito pequeno.
* Imagens agora devem ser exibidas adequadamente na webview incorporada.

##### Atualizador
* O atualizador agora mostra adequadamente o conteúdo das tags de código markdown nas notas de lançamento.

##### Livros DAISY
* Livros DAISY mostrando informações incorretas na barra de status.
* Carregando livros DAISY com declarações de codificação falsas.

##### Documentos RTF
* Analisando documentos RTF com caracteres não-latinos neles.
* Grupos RTF `\pict` para que dados de imagem incorporados não vazem mais no texto do documento.

##### Livros Mobi/AZW3
* Âncoras filepos em livros Mobi dividindo tags HTML e colocando lixo no texto do livro.
* Links em livros Mobi legados.
* Análise de AZW3 significativamente melhorada.

##### Documentos Word
* Documentos Word com nomes de estilo específicos de localidade não renderizando seus títulos adequadamente.

##### Documentos HTML/XHTML
* Elementos dl, dt e dd não produzindo quebras de linha em documentos XHTML.

##### Documentos PDF
* Paperback agora volta para extração de texto simples para PDFs falsamente marcados.
* Documentos PDF contendo caracteres de controle em seus títulos e/ou marcadores não deixarão mais o Paperback travar ao abrir.

### Versão 0.8.5
* Adicionado suporte de página para livros epub.
* Adicionado suporte para documentos Microsoft Office criptografados. Atualmente Word legado, Word moderno e Powerpoint moderno são suportados, com Powerpoint legado planejado para o futuro.
* Adicionado suporte para documentos Microsoft Word legados!
* Adicionado suporte para apresentações Powerpoint legadas!
* Adicionado suporte para livros mobi e AZW3!
* Adicionado suporte para arquivos PDF marcados!
* Adicionado o atalho ctrl+q para sair do aplicativo.
* Adicionado suporte para livros zipados do Bookshare (DAISY e Word)!
* Texto alternativo para imagens incorporadas agora deve ser exibido adequadamente.
* Documentos CHM agora suportam adequadamente navegação de link interno.
* Corrigido go to page estar errado por 1.
* Corrigido a tecla escape não funcionando para fechar o diálogo open as.
* Corrigido o menu de contexto do leitor não aparecer ao clicar com o botão direito ou pressionar a tecla Applications.
* Corrigido o documento errado às vezes sendo colocado em foco ao abrir documentos da linha de comando.
* PDFs somente de imagem são novamente detectados e alertam você da existência deles.
* Agora é possível navegar através de imagens e figuras com g/shift+g e f/shift+f, respectivamente.
* Paperback agora respeitará sua configuração de modo escuro de aplicativo.
* Removido suporte para DAISY XML, pois não é mais necessário.
* Voltado para a navegação de primeira letra nativa do Win32 na árvore do sumário.
* O diálogo de erro de carregamento agora mostra mensagens de erro mais detalhadas.
* A webview agora abrirá muito mais rápido e suavemente.

### Versão 0.8.2
* Adicionado suporte de página para documentos RTF!
* Corrigido um bug onde abrir a webview em epubs contendo links externos os ativaria automaticamente.
* Corrigido um bug onde o analisador RTF não colocaria espaço entre palavras em casos raros.
* Corrigidos parágrafos sendo divididos em várias linhas curtas em alguns documentos PDF.
* Documentos PDF agora têm suporte básico a navegação de link e título!
* Abas e feeds de linha RTF agora são renderizados exatamente como aparecem no documento.
* Voltado para a biblioteca pdfium conhecida e confiável para analisar PDFs, tornando a renderização de PDF muito mais confiável novamente.

### Versão 0.8.1
* Adicionado Ctrl+Shift+T para reabrir o último documento fechado.
* O diálogo All Documents agora suporta selecionar vários documentos para abrir de uma vez.
* Corrigidos alguns bugs com o analisador RTF.
* Corrigidos caminhos de arquivo contendo caracteres não-ASCII (como Bosnian š, č, ć, ž) ficando corrompidos ao abrir um arquivo via uma segunda instância do Paperback.
* Corrigido texto PDF sendo lido na ordem errada, e espaçamento incorreto em torno de palavras capitalizadas.
* Corrigido carregamento lento de documentos ao abrir arquivos grandes.
* Corrigida a localização dos botões Sim/Não em diálogos de confirmação.

### Versão 0.8.0
* Adicionadas traduções para Japonês, Chinês Simplificado e Vietnamita!
* Adicionado um atualizador automático que agora substituirá sua versão atualmente instalada do Paperback em vez de apenas baixar a nova versão!
* Adicionado feedback de som opcional para atingir um marcador ou uma nota, obrigado Andre Louis pelos sons!
* Adicionado suporte para documentos RTF!
* Adicionado suporte para documentos DAISY XML.
* Adicionado suporte para arquivos Flat Open Document Text!
* Adicionado suporte para apresentações Flat Open Document!
* Adicionado suporte para separadores com s e shift+s.
* Qualquer movimento maior que 300 caracteres agora adicionará automaticamente ao seu histórico de navegação.
* Corrigida a restauração da janela do Paperback da bandeja do sistema.
* Corrigidos documentos Markdown mostrando texto bruto em vez de HTML renderizado na Web View.
* Corrigidas tabelas não sendo renderizadas adequadamente em arquivos Markdown.
* PDFs somente de imagem agora avisarão você da existência deles quando você tentar carregar um.
* Informações de versão adequadamente incorporadas no executável do Paperback.
* Dividir o diálogo de opções em abas para facilitar o uso e navegação.
* Mudado para Hayro para analisar PDFs, levando a mais confiabilidade, velocidade e menos DLLs.
* Reescrito o aplicativo inteiro em Rust. A nova base de código é mais segura, carrega documentos mais rápido e é mais fácil de manter e estender.
* O menu de contexto do controle de texto agora incluirá ações específicas do leitor em vez de itens genéricos como cortar e colar.

### Versão 0.7.0
* Adicionado suporte de tabela para documentos baseados em HTML e XHTML! Navegue entre tabelas usando T e Shift+T, e pressione Enter para visualizar uma em uma webview.
* Adicionado um recurso básico de renderização web! Pressione Ctrl+Shift+V para abrir a seção atual do seu documento em um renderizador baseado na web, útil para conteúdo como formatação complexa ou exemplos de código.
* Adicionada uma tradução em Russo, obrigado Ruslan Gulmagomedov!
* Adicionado um botão Clear All ao diálogo All Documents.
* O verificador de atualização agora exibe notas de lançamento quando uma nova versão está disponível.
* Corrigida a restauração da janela da bandeja do sistema.
* Corrigidas as traduções dos botões Sim/Não em diálogos de confirmação.
* Corrigido carregamento de configs ao executar como administrador.
* Corrigido manuseio de comentários em documentos XML e HTML.
* Corrigido análise de TOC em livros Epub 2.
* Corrigida navegação para o próximo item com a mesma letra no sumário.
* Corrigido o diálogo de localização não se ocultando adequadamente ao usar os botões next/previous.
* Corrigidos TOCs de epub ocasionalmente jogando você para o item errado.
* Corrigidos vários problemas de manuseio de espaço em branco em tags XML, HTML e pre.
* Corrigido erro off-by-one na navegação de link.
* Corrigidos alguns livros tendo espaço em branco à direita em suas linhas.
* Corrigidos vários problemas do analisador.
* Itens de menu relacionados a marcador, bem como a lista de elementos, agora são adequadamente desabilitados quando nenhum documento está aberto.
* Melhorado manuseio de lista em vários formatos de documento.
* Melhorado o fluxo de trabalho de tradução para colaboradores.
* Muitas refatorações internas, movendo a maioria da lógica comercial do aplicativo de C++ para Rust para melhor desempenho e manutenibilidade.

### Versão 0.6.1
* Adicionado suporte para PDF protegido por senha!
* Adicionado um recurso muito básico de navegação para posição anterior/próxima. Se você pressionar enter em um link interno e isso mover seu cursor, essa posição agora será lembrada, e pode ser navegada com setas Alt+Left/Right.
* Adicionada uma lista de elementos! Atualmente mostra apenas uma árvore de todos os títulos do seu documento ou uma lista de links, mas há planos para expandi-la no futuro.
* Adicionada uma opção para iniciar o Paperback em modo maximizado por padrão.
* Corrigidos links em alguns documentos Epub não funcionando adequadamente.
* Corrigida análise de Epub TOCs contendo caminhos relativos.
* Corrigidos alguns documentos epub não mostrando título ou autor.
* Corrigidos os títulos de alguns capítulos epub não aparecendo adequadamente no diálogo TOC.
* Corrigido você não conseguir usar a barra de espaço para ativar os botões OK/cancel no diálogo TOC.
* Melhorado o manuseio de títulos em documentos Word.
* Você agora receberá feedback falado se a lista de documentos recentes estiver vazia quando você tentar trazer o diálogo.

### Versão 0.6.0
* Uma nova opção para mostrar o menu go em uma forma muito mais compacta foi adicionada ao diálogo de opções, marcado por padrão.
* Adicionada uma opção para fazer a navegação por elementos estruturais envolver.
* Adicionada uma opção ao menu tools para abrir a pasta contendo o documento atualmente em foco.
* Adicionado um sistema de atualização bem simples, mas muito eficaz.
* Adicionado um recurso básico de temporizador de sono, acessível com Ctrl+Shift+S.
* Adicionado suporte para análise de livros FB2!
* Adicionado suporte para análise de apresentações OpenDocument!
* Adicionado suporte para análise de arquivos OpenDocument Text!
* Marcadores agora podem ser feitos para marcar uma linha inteira, ou marcar apenas algum texto especificado. Se você não tiver seleção ativa ao colocar um marcador, o comportamento é como pré-0.6, e marcará a linha inteira. No entanto, se você selecionar algum texto, apenas esse texto será incluído no marcador.
* Marcadores agora podem ter notas de texto opcionais anexadas a eles! Navegue entre marcadores contendo notas com N e Shift+N, ou abra o diálogo de marcadores com todos os marcadores, apenas notas ou apenas não-notas selecionados com hotkeys específicas.
* Marcadores no diálogo de marcadores não terão mais um prefixo irritante "bookmark x".
* Livros Epub contendo conteúdo HTML fingindo ser XML agora serão tratados adequadamente.
* Corrigido carregamento de grandes documentos Markdown.
* Corrigido pressionar espaço na árvore do sumário ativando o botão OK.
* Corrigido manuseio de espaço em branco no início de tags pre em documentos HTML e XHTML.
* Corrigido o controle de texto não reganhando foco às vezes ao retornar à janela do Paperback.
* Corrigido o campo de texto no diálogo go to percent não atualizando o valor do slider.
* Corrigida a renderização de IDs HTML personalizados em documentos Markdown.
* HTML dentro de blocos de código Markdown agora será renderizado adequadamente.
* Se carregar um livro com um parâmetro de linha de comando enquanto uma instância existente do Paperback está em execução, você não receberá mais um erro se carregar seu documento levar mais de 5 segundos.
* Se executar Paperback como administrador, a configuração agora será adequadamente carregada e salva.
* Agora é possível excluir um marcador diretamente do diálogo de marcadores.
* Agora é possível importar e exportar seus marcadores e posição de leitura para um documento específico. O arquivo gerado é nomeado após o arquivo com uma extensão .paperback. Se tal arquivo for encontrado no mesmo diretório de um arquivo ao carregá-lo, ele será automaticamente carregado. Caso contrário, você pode importá-los manualmente usando um item no menu tools.
* Links dentro de documentos agora são totalmente suportados! Use k e shift+k para se mover para frente e para trás através deles, e pressione enter para abrir/ativar um.
* Muitas refatorações internas, tornando o aplicativo mais rápido e o binário menor.
* Conteúdo Markdown agora é pré-processado para ser compatível com CommonMark antes da renderização.
* Navegação por listas e seus itens agora é totalmente suportada! Use L e Shift+L para ir por listas em si, e I e Shift+I para ir através de itens de lista.
* Numpad delete agora funciona para remover documentos da barra de abas além do delete normal.
* Paperback agora pode opcionalmente minimizar para sua bandeja do sistema! Esta opção está desligada por padrão, mas ativá-la fará com que a opção minimizar no menu do sistema coloque o Paperback em sua bandeja, capaz de ser restaurado clicando no ícone gerado.
* Paperback agora é totalmente traduzível! A lista de idiomas que suporta é atualmente bem pequena, mas está crescendo constantemente!
* Paperback agora tem um site oficial, em [paperback.dev](https://paperback.dev)!
* Documentos PPTX agora mostrarão um sumário básico, contendo todos os slides.
* O caminho completo para o documento aberto agora será mostrado no diálogo de informações do documento.
* O instalador agora inclui uma opção para visualizar o readme em seu navegador após a instalação.
* A lista de documentos recentes foi dramaticamente expandida! Em vez de simplesmente mostrar os últimos 10 documentos que você abriu, agora mostrará um número personalizável, com o restante dos documentos que você já abriu sendo acessível através de um pequeno diálogo.
* Vários pequenos aprimoramentos nos analisadores em toda a placa, incluindo colocar uma linha em branco entre slides em apresentações PPTX, corrigindo o manuseio de nova linha dentro de parágrafos em documentos word, e adicionando pontos de bala aos itens de lista.

### Versão 0.5.0
* Adicionado suporte para documentos Microsoft Word!
* Adicionado suporte para apresentações PowerPoint!
* Corrigidos certos itens de menu não sendo desabilitados sem documentos abertos.
* Corrigida a orientação do slider go to percent.
* Corrigido o sumário em livros Epub com caminhos de arquivo codificados em URL e/ou IDs de fragmento.
* Corrigido espaço em branco sendo removido de títulos XHTML de maneiras estranhas.
* Corrigido manuseio de espaço em branco dentro de tags pre aninhadas em documentos HTML.
* Documentos HTML e Markdown agora suportam o recurso de sumário! Quando você carrega um documento HTML/Markdown, o Paperback construirá seu próprio sumário fora da estrutura dos títulos do seu documento, e o mostrará a você no diálogo ctrl+t.
* Documentos HTML agora terão o título conforme definido na tag title, se existir. Caso contrário, eles continuarão a usar o nome do arquivo sem a extensão.
* Mudado de UniversalSpeech para usar uma região ativa para relatar fala. Isso significa que nenhuma DLL do leitor de tela é enviada junto com o programa, e mais leitores de tela agora serão suportados, como Microsoft Narrator.
* Mudado bibliotecas zip para permitir abrir um array mais amplo de livros epub.
* O diálogo pedindo se você quer abrir seu documento como texto simples foi completamente refeito, e agora permite abrir seu documento como texto simples, HTML ou Markdown.
* O diálogo go to percent agora inclui um campo de texto permitindo que você insira manualmente uma porcentagem para pular.
* O analisador HTML agora reconhecerá dd, dt e dl como elementos de lista.
* O sumário em livros Epub será novamente preservado exatamente.
* O espaço não-quebrável unicode agora é considerado ao remover linhas em branco.
* Você não será mais perguntado como deseja abrir um arquivo não reconhecido todas as vezes que você o carrega, apenas na primeira vez.

### Versão 0.4.1
* Adicionado um ícone de menu inicial opcional ao instalador.
* O sumário agora deve ser mais limpo em alguns casos, por exemplo se você tiver um item filho e pai com o mesmo texto na mesma posição, você agora verá apenas o item pai.
* Corrigido o sumário em certos documentos CHM.
* Corrigido o sumário em livros Epub 3 com caminhos absolutos neles.
* Documentos CHM agora devem mostrar seu título conforme definido no arquivo de metadados.

### Versão 0.4.0
* Adicionado suporte para arquivo CHM!
* Adicionado suporte para marcadores! Você pode ter quantos marcadores você goste em quantos documentos você goste. Você pode pular para frente e para trás através deles com b e shift+b, definir um com control+shift+b, e trazer um diálogo para pular para um marcador específico com control+b.
* Adicionado um instalador junto do arquivo zip portável! O instalador instalará Paperback no seu diretório Program Files, e configurará automaticamente associações de arquivo para você.
* Arquivos de texto com BOMs agora devem ser decodificados adequadamente, e o BOM não será mais exibido no início do texto.
* Adicionada muito mais informação à barra de status. Agora mostrará sua linha atual, caractere e porcentagem de leitura.
* Comentários HTML, bem como o conteúdo das tags script e style, não serão mais mostrados na saída de texto.
* Se passar um caminho relativo para Paperback na linha de comando, agora será resolvido adequadamente.
* Movimento de porcentagem agora é tratado por seu próprio diálogo baseado em slider, acessível com control+shift+g.
* Documentos sem títulos ou autores conhecidos agora sempre terão um padrão.
* A lógica de salvamento de posição agora é muito mais inteligente e deve apenas escrever no disco quando absolutamente necessário.
* O documento que você teve em foco quando fechou o Paperback agora é lembrado entre reinicializações de aplicativo.
* Input nos diálogos go to line e go to page agora deve ser sanitizado mais estritamente.
* Corrigida navegação de sumário em livros epub 3 com caminhos relativos em seus manifestos.

### Versão 0.3.0
* Corrigido o sumário em livros epub com manifestos codificados em URL.
* Corrigida navegação de título em documentos HTML contendo caracteres Unicode multi-byte.
* Corrigido alto uso de CPU em documentos com títulos longos devido a uma regressão no wxWidgets.
* Corrigido carregamento de arquivos de texto UTF-8.
* Corrigidos itens aninhados de TOC em livros Epub colocando seu cursor na posição errada.
* Corrigido um crash na saída do aplicativo em certos casos.
* Adicionada uma caixa de seleção no diálogo de opções para ativar ou desativar word wrap!
* Agora é possível doar ao desenvolvimento do Paperback, através do novo item de doação no menu de ajuda ou através do link sponsor this project na parte inferior da página principal do repositório GitHub.
* Documentos Markdown agora sempre terão um título, e o Paperback agora deve conseguir carregar praticamente qualquer arquivo Markdown.
* Documentos PDF agora sempre terão um título, mesmo se os metadados estiverem faltando.
* Mudado para a biblioteca PDF usada no Chromium, levando a análise de PDF muito mais confiável em toda a placa.
* Você agora pode ter apenas uma instância do Paperback em execução de cada vez. Executar paperback.exe com um nome de arquivo enquanto já está em execução abrirá esse documento na instância já em execução.
* Você agora pode pressionar delete em um documento no controle de aba para fechá-lo.

### Versão 0.2.1
* Adicionado o número total de páginas ao rótulo de página no diálogo go to page.
* Permitir tabulação do conteúdo do documento para sua lista de documentos abertos.
* Corrigidos os atalhos de título às vezes abrindo documentos recentes se você tivesse o suficiente deles.
* Paperback agora removerá hífens macios desnecessários da saída de texto.
* Corrigida navegação de título às vezes o colocando no caractere errado.

### Versão 0.2.0
* Adicionado suporte para documentos markdown!
* Adicionado suporte para documentos PDF, incluindo a capacidade de navegar entre páginas!
* Adicionadas traços de teclado para navegar por títulos em conteúdo HTML, incluindo livros epub e documentos markdown. Esses traços foram projetados para funcionar similar a um leitor de tela.
* Corrigido carregamento de epubs com nomes de arquivo codificados em URL em seus manifestos.
* Corrigido carregamento de livros epub 3 com XHTML incorporado dentro deles.
* Uma mensagem agora é falada se o documento não suporta um sumário ou seções, em vez dos itens de menu serem desabilitados.
* Adicionado um menu de documentos recentes! Atualmente armazena seus últimos 10 documentos abertos, e pressionar enter em um abrirá para leitura.
* Completamente reescrito o diálogo Find, tornando-o muito mais simples de usar, enquanto também adicionava um histórico de suas últimas 25 buscas e suporte para expressão regular!
* Documentos anteriormente abertos agora são lembrados entre reinicializações de aplicativo. Isso é configurável através do novo item de opções no menu tools.
* Adicionado shift+f1 para abrir o readme diretamente no Paperback em si.

### Versão 0.1.0
* Lançamento inicial.
