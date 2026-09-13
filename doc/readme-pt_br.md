<!-- machine-translated from doc/readme.md (source-hash: d583a89d8ac391f5; sections: f48ce952,d4d8160f,a02f4421,a4ffb7f7,91be3b41,a028b7f6,af36028a,71df8e94,e9860ee8,93dd8dd6); please review and edit as needed -->

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

## Formatos de arquivo atualmente suportados

Paperback suporta os seguintes formatos e extensões:

* Arquivos de histórias em quadrinhos (`.cbz`, `.cbr`)
* Arquivos de ajuda CHM (`.chm`)
* Livros DAISY (`.opf`, `.zip`)
* Livros EPUB (`.epub`)
* Ebooks FB2 (`.fb2`)
* Documentos HTML (`.htm`, `.html`, `.xhtml`)
* Páginas de manual, tanto `man` quanto BSD `mdoc` (`.1` a `.9`, `.man`, `.roff` e as formas compactadas com gzip de cada)
* Documentos Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Documentos Microsoft Word (`.docx`, `.docm`, `.doc`)
* Audiolivros M4B (`.m4b`)
* Livros MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
* Apresentações OpenDocument (`.odp`, `.fodp`)
* Arquivos de texto OpenDocument (`.odt`, `.fodt`)
* Documentos PDF (`.pdf`)
* Apresentações PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Documentos RTF (`.rtf`)
* Arquivos WinHelp (`.hlp`)
* Arquivos de texto simples e de log (`.txt`, `.log`)

## Atalhos de teclado

O Paperback foi projetado para uso com prioridade ao teclado. Aqui estão os atalhos atuais.

Os atalhos abaixo são para Windows. Onde o macOS difere, o equivalente é indicado entre parênteses — principalmente porque Ctrl+G, Ctrl+W e Alt+Left/Right já são usados por outras convenções do sistema ou do aplicativo nessa plataforma.

### Menu Arquivo

* `Ctrl+O`: Abrir um documento.
* `Ctrl+F4` (macOS: `Cmd+W`): Fechar o documento atual.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Fechar todos os documentos abertos.
* `Ctrl+Shift+T`: Reabrir o último documento fechado.
* `Ctrl+R`: Mostrar o diálogo "Todos os Documentos" (de Documentos Recentes).
* `Ctrl+Q`: Sair (somente Windows; no macOS está no menu do aplicativo).

### Menu Ir

* `Ctrl+F`: Mostrar o diálogo Localizar.
* `F3` (macOS: `Cmd+G`): Localizar próximo.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Localizar anterior.
* `Ctrl+G` (macOS: `Cmd+L`): Ir para linha.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Ir para percentual.
* `Ctrl+P`: Ir para página (quando suportado pelo documento atual).
* `=`: Anunciar o seu percentual de leitura atual e página, p. ex. "15%, página 30". A página é omitida para documentos que não têm números de página.
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
* `Shift+B`: Indicador anterior.
* `B`: Próximo indicador.
* `/`: Definir o seu indicador temporário.
* `\`: Ir para o seu indicador temporário.
* `Shift+N`: Nota anterior.
* `N`: Próxima nota.
* `Ctrl+B`: Ir para todos os indicadores e notas.
* `Ctrl+Alt+B`: Ir para indicadores apenas.
* `Ctrl+Alt+M`: Ir para notas apenas.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, ou seja, a tecla Control física em vez de Cmd): Visualizar texto da nota na posição atual.
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
* `,`: Ir além do final do contêiner atual (lista ou tabela).

### Menu Ferramentas

* `Ctrl+W` (macOS: `RawCtrl+W`, ou seja, a tecla Control física em vez de Cmd): Mostrar contagem de palavras do documento atual.
* `Ctrl+I`: Mostrar informações do documento.
* `Ctrl+T`: Mostrar índice.
* `F7`: Mostrar lista de elementos.
* `Ctrl+Shift+C`: Abrir pasta contendo.
* `Ctrl+Shift+V`: Abrir conteúdo atual na Visualização da Web.
* `Ctrl+U`: Visualizar a fonte do documento em uma nova aba.
* `Ctrl+Shift+E`: Exportar dados do documento (`.paperback`).
* `Ctrl+Shift+I`: Importar dados do documento (`.paperback`).
* `Ctrl+E`: Exportar o documento atual para texto sem formatação.
* `Ctrl+Shift+B`: Alternar indicador na seleção/cursor atual.
* `Ctrl+Shift+N`: Adicionar ou editar nota de indicador na seleção/cursor atual.
* `Ctrl+Alt+W`: Alternar quebra de linha.
* `Ctrl+Space`: Reproduzir/pausar narração de áudio.
* `'`: Buscar narração de áudio para frente.
* `;`: Buscar narração de áudio para trás.
* `Ctrl+'`: Aumentar a quantidade de busca de áudio.
* `Ctrl+;`: Diminuir a quantidade de busca de áudio.
* `F11` (macOS: `RawCtrl+Ctrl+F`, ou seja, Control+Command+F): Alternar tela cheia.
* `Ctrl+,`: Abrir opções (macOS: Preferências, no menu do aplicativo).
* `Ctrl+Shift+S`: Alternar temporizador de suspensão.

### Menu Ajuda

* `Ctrl+F1`: Mostrar diálogo Sobre.
* `F1`: Visualizar ajuda no seu navegador padrão.
* `Shift+F1`: Visualizar ajuda no Paperback.
* `Ctrl+Shift+U`: Verificar atualizações.
* `Ctrl+D`: Abrir a página de doação no seu navegador padrão.

### Teclas adicionais de visualização de documento

* `Delete` / `Numpad Delete` no controle de aba: Fechar a aba de documento selecionada.
* `Enter` ou `Space` no texto do documento: Ativar link no cursor, ou abrir uma visualização de tabela quando em um marcador de tabela.
* `Shift+F10` ou a tecla Menu/Aplicativo no texto do documento: Abrir o menu de contexto.

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

## Histórico de Alterações

### Versão 0.9.2
* Audiolivros não fazem mais seu leitor de tela ler uma sequência de espaços quando você foca no campo de texto.
* Audiolivros agora nomeiam o arquivo conforme você navega pelas seções.
* Audiolivros agora relatam seu comprimento real, em vez de afirmar que cada arquivo dura 24 horas.
* Fechar a Web View com Escape não mostra mais um alerta de depuração depois de você ter seguido um link dentro dela.
* Copiar após Selecionar Tudo agora fornece o documento completo, em vez de apenas a parte carregada no momento.
* Localizar agora vai direto para a linha encontrada, em vez de fazer você ouvir o leitor de tela ler a janela novamente conforme o foco retorna ao livro.
* Corrigido EPUBs que contêm um bloco ZIP64 perdido recusando-se a abrir com "Invalid local file header".
* Corrigido documentos longos voltando para o início enquanto um leitor de tela lê continuamente através deles.
* Links na WebView agora o levam à seção para a qual apontam, em vez de falharem com "File not found".
* O atalho `=` agora anuncia a página além da porcentagem, por exemplo "15%, página 30", e permanece como estava para documentos sem números de página.
* O anúncio automático "Document reloaded" não interrompe mais seu leitor de tela no meio da frase, esperando que ele termine o que estava dizendo.
* A aba Geral do diálogo Configurações agora percorre suas opções na ordem em que aparecem na tela, com o canal de atualização logo após a opção de verificar atualizações.
* Atualizar agora traz a janela reiniciada para a frente, em vez de deixá-la atrás de todas as outras janelas em Alt+Tab.
* Windows agora sempre mostrará "Paperback" no menu Abrir com, em vez do slogan completo do programa.
* Contagem de Palavras e Informações do Documento agora mostram quantos arquivos um audiolivro possui e quanto tempo ele dura no total.

### Versão 0.9.1
* Sons de indicador e nota agora tocam no macOS.
* Livros DAISY agora reproduzem seu áudio no macOS, em vez de abrir e rastrear sua linha do tempo em silêncio.
* Corrigido aspas curvas, travessões e caracteres similares desaparecendo de documentos RTF, unindo as palavras ao redor conforme desaparecem.
* Corrigido imagens RTF vazando seus dados brutos no documento como texto distorcido.
* Corrigido o submenu Documentos Recentes mantendo entradas obsoletas até que algo else acontecesse para reconstruir.
* Aceleradores de teclado estão de volta em todas as traduções, então os menus em russo têm acesso ao teclado novamente.
* Grandes documentos CHM agora abrem até sete vezes mais rápido.
* Documentos abertos agora são registrados no Windows, para que apareçam na lista de saltos da barra de tarefas e na lista recente do menu Iniciar.
* Opções foi renomeado para Configurações, combinando com os aplicativos móveis e, no macOS, a convenção da plataforma.
* Paperback agora lembra sua posição de janela, tamanho e estado maximizado entre execuções.
* Formas plurais agora são traduzidas, então mensagens que contam coisas são lidas corretamente em idiomas que precisam de mais de uma forma.
* Selecionar o ncc.html de um livro DAISY agora abre o audiolivro completo em vez de apenas seu texto.
* Os nomes de ação do diálogo Personalizar Atalhos de Teclado agora podem ser traduzidos.
* O título do documento agora vem primeiro na barra de título, então livros abertos podem ser diferenciados na barra de tarefas e Alt+Tab.
* O diálogo de atualização agora é traduzido.

### Versão 0.9.0

#### Adicionado

##### Geral
* Uma ferramenta CLI, chamada pb, para converter rapidamente qualquer um dos formatos suportados do Paperback para HTML, Markdown ou texto simples.
* Uma opção para recarregar documentos que foram modificados por outros programas no disco.
* Uma opção Ver Fonte para abrir a fonte de um documento em uma nova aba, útil para editar Markdown, por exemplo.
* O texto do documento agora é paginado, o que significa que você pode carregar livros com dezenas de milhões de palavras em apenas alguns segundos. Por favor, relate qualquer estranheza encontrada com isso.

##### Suporte de Plataforma
* Suporte para Windows ARM64!
* Suporte nativo para macOS!
* Uma alternância de tela cheia.

##### Diálogo Todos os Documentos
* Um botão localizar para localizar livros desaparecidos que mudaram seu caminho.
* Um filtro de status e barra de status, para que você possa filtrar por status do documento e ver quantos documentos são mostrados e selecionados.
* O atalho `Ctrl+Shift+A` para desselecionar todos os documentos.

##### Opções e Legibilidade
* Uma aba de legibilidade, com as seguintes opções:
    * Quebra de linha (movida de geral);
    * Renderizar tabelas em linha (novo nesta versão, veja abaixo);
    * Fonte;
    * Cor de fundo;
    * Espaçamento entre linhas;
    * Espaçamento entre parágrafos;
    * Espaçamento entre letras;
    * Alinhamento de texto.
* Um item de menu de quebra de linha e tecla de atalho subsequente.
* Uma alternância para determinar como você deseja que as tabelas sejam exibidas e unificar como as tabelas são exibidas em documentos.

##### Navegação
* Suporte para navegar por contêiner.
* Uma opção para mover automaticamente o cursor para o início da linha ao navegar entre linhas, semelhante ao modo de navegação em leitores de tela.
* O atalho de teclado igual para anunciar sua porcentagem atual através de um documento.

##### Indicadores
* Indicadores temporários: você pode ter um por documento, e eles persistem. Use barra para definir um e barra invertida para pular para ele.

##### Contagem de Palavras
* Tempo de leitura estimado no diálogo de contagem de palavras, bem como a capacidade de definir sua velocidade de leitura para tornar essa métrica realmente útil.
* Se uma seleção estiver ativa quando você abrir o diálogo de contagem de palavras, quantas palavras você selecionou será mostrado.

##### Atalhos de Teclado
* A capacidade de personalizar cada atalho de teclado no aplicativo através de um diálogo simples.
* Um atalho de teclado configurável para restaurar o Paperback da bandeja do sistema.

##### Idiomas
* Holandês, finlandês e polonês.

##### Exportar
* Expandido o item de menu exportar para permitir exportar para HTML e Markdown, além de texto simples.

##### Atualizador
* Um botão de cancelamento para o diálogo de atualização em andamento.
* O atualizador agora valida o arquivo baixado não foi adulterado.

##### Web View
* A web view agora é aberta em sua posição de leitura atual.

##### Livros DAISY
* Suporte para livros DAISY 2.0.
* Suporte para reprodução de áudio DAISY 2.02.

##### Audiolivros
* A capacidade de reproduzir audiolivros, suportando atualmente tanto DAISY áudio (incluindo DAISY áudio + texto) quanto zips de arquivos de áudio.
* Atalhos de teclado e itens de menu para pausar/reproduzir narração, buscar para frente e para trás e ajustar a quantidade de busca.
* Opções para sincronizar o cursor de leitura com reprodução de áudio, definir a quantidade de busca de áudio e escolher se buscar além do final de um capítulo continua no próximo.

##### Documentos CHM
* Suporte para listas, itens de lista, figuras e imagens.

##### PowerPoint
* Documentos PowerPoint agora suportam tabelas.

#### Corrigido

##### Geral
* Documentos codificados em codificações CJK legadas, como GBK, Big5 e Shift_JIS, agora serão renderizados adequadamente em vez de como um monte de mojibake.
* "Reabrir último fechado" tentando reabrir o readme fornecido.
* Sua aba selecionada não recebendo o foco apropriado após reiniciar o Paperback.
* Tratamento do Paperback de arquivos em unidades de rede do Windows: pressionar mostrar arquivo em pasta agora foca adequadamente o arquivo no armazenamento de rede, e os caminhos não contêm mais caracteres estranhos.
* Arquivos .paperback não serão mais carregados à força na restauração de documentos; em vez disso, você será solicitado de confirmação quando um for encontrado.
* Abrir pasta contendo agora foca o arquivo fornecido no explorador.
* Abrir o readme agora respeitará seu idioma selecionado.
* A interface do usuário do Paperback agora será dimensionada adequadamente em displays de alto DPI.
* O menu agora atualiza adequadamente, e o foco se move para o controle de texto, ao abrir ajuda no Paperback.
* Mudou para um método muito mais seguro de IPC no Windows.
* O título do documento ativo será lido ao alternar entre abas.
* Reduzido o uso de memória em documentos grandes reduzindo pela metade o tamanho das tabelas de índice interno por caractere.

##### Diálogo Todos os Documentos
* Escape não fechando os diálogos Informações do Documento e Todos os Documentos.
* A barra de título não atualizando após fechar um documento do diálogo de todos os documentos.
* Readme.html não será mais adicionado à sua lista de todos os documentos quando aberto via Shift+F1.
* Remover documentos do diálogo recentes também fechará sua aba ativa.
* Seu filtro de pesquisa agora é preservado após remover um documento.

##### Navegação
* Navegação de página anunciando texto de linha incorreto em algumas situações.
* Ir para Linha, Ir para Página e Ir para Porcentagem colocando seu cursor na posição errada em documentos grandes.
* Localizar e Localizar Próximo não respeitando a janela de documento carregada em documentos grandes.

##### Indicadores
* Sons de indicador/nota devem agora tocar adequadamente exclusivamente quando você navega sobre uma palavra contendo um.

##### Legibilidade
* Aplicar quebra de linha o enviando para o início do seu documento.

##### Web View
* O diálogo de web view não sendo redimensionável e aparecendo em um tamanho inicial muito pequeno.
* Imagens agora devem ser exibidas adequadamente na web view incorporada.

##### Atualizador
* O atualizador agora mostra adequadamente o conteúdo das tags de código markdown nas notas de lançamento.

##### Livros DAISY
* Livros DAISY mostrando informações incorretas na barra de status.
* Carregando livros DAISY com declarações de codificação falsas.

##### Documentos RTF
* Analisando documentos RTF com caracteres não-latinos neles.
* Grupos RTF `\pict` para que dados de imagem incorporada não vazem mais no texto do documento.

##### Livros Mobi/AZW3
* Âncoras filepos em livros Mobi dividindo tags HTML e colocando lixo no texto do livro.
* Links em livros Mobi legados.
* Análise AZW3 muito melhorada.

##### Documentos Word
* Documentos Word com nomes de estilo específicos de localidade não renderizando seus títulos adequadamente.

##### Documentos HTML/XHTML
* Elementos dl, dt e dd não produzindo quebras de linha em documentos XHTML.

##### Documentos PDF
* Paperback agora retorna à extração de texto simples para PDFs falsamente marcados.
* Documentos PDF contendo caracteres de controle em seus títulos e/ou marcadores não causarão mais pane no Paperback ao abrir.

### Versão 0.8.5
* Adicionado suporte de página para livros epub.
* Adicionado suporte para documentos Microsoft Office criptografados. Atualmente são suportados Word legado, Word moderno e Powerpoint moderno, com Powerpoint legado planejado para o futuro.
* Adicionado suporte para documentos Microsoft Word legados!
* Adicionado suporte para apresentações Powerpoint legadas!
* Adicionado suporte para livros mobi e AZW3!
* Adicionado suporte para arquivos PDF marcados!
* Adicionado o atalho ctrl+q para sair do aplicativo.
* Adicionado suporte para livros compactados do Bookshare (DAISY e Word)!
* Texto alternativo para imagens incorporadas agora deve ser mostrado adequadamente.
* Documentos CHM agora suportam adequadamente navegação por link interno.
* Corrigido ir para página estar off by 1.
* Corrigido a tecla escape não funcionar para fechar o diálogo abrir como.
* Corrigido o menu de contexto do leitor não aparecendo ao clicar com o botão direito ou a tecla Aplicações.
* Corrigido o documento errado às vezes sendo focado ao abrir documentos da linha de comando.
* PDFs somente com imagens são novamente detectados e o alertam sobre sua existência.
* Agora é possível navegar através de imagens e figuras com g/shift+g e f/shift+f, respectivamente.
* Paperback agora respeitará sua configuração de modo escuro da aplicação.
* Removido suporte DAISY XML, pois não é mais necessário.
* Voltado para a navegação de primeira letra nativa do Win32 na árvore de conteúdo.
* O diálogo de carregamento de erro agora mostra mensagens de erro mais detalhadas.
* A web view agora abrirá muito mais rápido e suavemente.

### Versão 0.8.2
* Adicionado suporte de página para documentos RTF!
* Corrigido um bug onde abrir a web view em epubs contendo links externos os ativaria automaticamente.
* Corrigido um bug onde o analisador RTF não colocaria um espaço entre palavras em casos raros.
* Parágrafos sendo divididos em múltiplas linhas curtas em alguns documentos PDF.
* Documentos PDF agora têm suporte básico de navegação por link e título!
* Abas e alimentações de linha RTF agora são renderizadas exatamente como aparecem no documento.
* Voltado para a biblioteca pdfium conhecida e confiável para análise de PDFs, tornando a renderização PDF muito mais confiável novamente.

### Versão 0.8.1
* Adicionado Ctrl+Shift+T para reabrir o último documento fechado.
* O diálogo Todos os Documentos agora suporta seleção de múltiplos documentos para abrir de uma vez.
* Corrigidos alguns bugs com o analisador RTF.
* Corrigido caminhos de arquivo contendo caracteres não-ASCII (como sérvio š, č, ć, ž) ficando corrompidos ao abrir um arquivo por uma segunda instância do Paperback.
* Corrigido texto PDF sendo lido na ordem errada e espaçamento incorreto ao redor de palavras capitalizadas.
* Corrigido carregamento lento de documentos ao abrir arquivos grandes.
* Corrigido a localização dos botões Sim/Não em diálogos de confirmação.

### Versão 0.8.0
* Adicionadas traduções para japonês, chinês simplificado e vietnamita!
* Adicionado um atualizador automático que agora substituirá sua versão instalada do Paperback em vez de apenas baixar a nova versão!
* Adicionado feedback de som opcional por alcançar um indicador ou uma nota, obrigado Andre Louis pelos sons!
* Adicionado suporte para documentos RTF!
* Adicionado suporte para documentos DAISY XML.
* Adicionado suporte para arquivos Flat Open Document Text!
* Adicionado suporte para apresentações Flat Open Document!
* Adicionado suporte para separadores com s e shift+s.
* Qualquer movimento maior que 300 caracteres agora adicionará automaticamente ao seu histórico de navegação.
* Corrigido restaurar a janela do Paperback da bandeja do sistema.
* Corrigido documentos Markdown mostrando texto bruto em vez de HTML renderizado na Web View.
* Corrigido tabelas não sendo renderizadas adequadamente em arquivos Markdown.
* PDFs somente com imagens agora o alertarão sobre sua existência ao tentar carregar um.
* Informações de versão adequadamente incorporadas no executável do Paperback.
* Divida o diálogo de opções em abas para facilidade de uso e navegação.
* Mudou para Hayro para análise de PDFs, levando a mais confiabilidade, velocidade e menos DLLs.
* Reescreveu todo o aplicativo em Rust. A nova base de código é mais segura, carrega documentos mais rápido e é mais fácil de manter e estender.
* O menu de contexto do controle de texto agora incluirá ações específicas do leitor em vez de itens genéricos como cortar e colar.

### Versão 0.7.0
* Adicionado suporte de tabela para documentos baseados em HTML e XHTML! Navegue entre tabelas usando T e Shift+T e pressione Enter para visualizar uma em uma web view.
* Adicionado um recurso básico de renderização da web! Pressione Ctrl+Shift+V para abrir a seção atual de seu documento em um renderizador baseado em web, útil para conteúdo como formatação complexa ou amostras de código.
* Adicionada uma tradução em russo, obrigado Ruslan Gulmagomedov!
* Adicionado um botão Limpar Tudo ao diálogo Todos os Documentos.
* O verificador de atualização agora exibe notas de lançamento quando uma nova versão está disponível.
* Corrigido restaurar a janela da bandeja do sistema.
* Corrigido traduções de botões Sim/Não em diálogos de confirmação.
* Corrigido carregamento de configs ao executar como administrador.
* Corrigido tratamento de comentários em documentos XML e HTML.
* Corrigido análise de TOC em livros Epub 2.
* Corrigido navegação para o próximo item com a mesma letra no índice de conteúdo.
* Corrigido o diálogo de localização não se ocultando adequadamente ao usar os botões próximo/anterior.
* Corrigido TOCs de epub ocasionalmente o enviando para o item errado.
* Corrigido vários problemas de tratamento de espaço em branco em XML, HTML e tags pre.
* Corrigido erro off-by-one na navegação por link.
* Corrigido alguns livros tendo espaço em branco à direita em suas linhas.
* Corrigido vários problemas de analisador.
* Itens de menu relacionados a indicadores bem como a lista de elementos agora estão adequadamente desabilitados quando nenhum documento está aberto.
* Melhorado o tratamento de lista em vários formatos de documento.
* Melhorado o fluxo de trabalho de tradução para colaboradores.
* Muitas refatorações internas, movendo a maioria da lógica de negócio do aplicativo de C++ para Rust para melhor desempenho e manutenibilidade.

### Versão 0.6.1
* Adicionado suporte para PDF protegido por senha!
* Adicionado um recurso básico de ir para posição anterior/próxima. Se você pressionar enter em um link interno e ele mover seu cursor, essa posição será lembrada agora e pode ser navegada com setas alt+left/right.
* Adicionada uma lista de elementos! Atualmente mostra apenas uma árvore de todos os títulos em seu documento ou uma lista de links, mas há planos para expandi-la no futuro.
* Adicionada uma opção para iniciar o Paperback em modo maximizado por padrão.
* Corrigido links em alguns documentos Epub não funcionando adequadamente.
* Corrigido análise de TOCs de Epub contendo caminhos relativos.
* Corrigido alguns documentos epub não mostrando um título ou autor.
* Corrigido títulos de alguns capítulos epub não aparecendo adequadamente no diálogo TOC.
* Corrigido você não conseguir usar a barra de espaço para ativar os botões OK/cancelar no diálogo TOC.
* Melhorado o tratamento de títulos em documentos Word.
* Você receberá feedback falado se a lista de documentos recentes estiver vazia quando tentar trazer o diálogo.

### Versão 0.6.0
* Uma nova opção para mostrar o menu ir em uma forma muito mais compacta foi adicionada ao diálogo de opções, marcada por padrão.
* Adicionada uma opção para fazer a navegação por elementos estruturais envolver.
* Adicionada uma opção ao menu ferramentas para abrir a pasta contendo do documento focado no momento.
* Adicionado um sistema de atualização bastante simples, mas muito eficaz.
* Adicionado um recurso básico de temporizador de sono, acessível com Ctrl+Shift+S.
* Adicionado suporte para análise de livros FB2!
* Adicionado suporte para análise de apresentações OpenDocument!
* Adicionado suporte para análise de arquivos OpenDocument Text!
* Indicadores agora podem ser feitos para indicar uma linha inteira ou para marcar apenas algum texto especificado. Se você não tiver seleção ativa ao colocar um indicador, o comportamento é como pré-0.6 e marcará a linha inteira. Porém, se você selecionar algum texto, apenas esse texto será incluído no indicador.
* Indicadores agora podem ter notas de texto opcional anexadas a eles! Navegue entre indicadores contendo notas com N e Shift+N, ou abra o diálogo de indicadores com todos os indicadores, apenas notas ou apenas não-notas selecionados com teclas de atalho específicas.
* Indicadores no diálogo de indicadores não terão mais um incômodo prefixo "indicador x".
* Livros Epub contendo conteúdo HTML fingindo ser XML agora serão tratados adequadamente.
* Corrigido carregamento de grandes documentos Markdown.
* Corrigido pressionar espaço na árvore de visualização de índice de conteúdo ativando o botão OK.
* Corrigido tratamento de espaço em branco no início de tags pre em documentos HTML e XHTML.
* Corrigido o controle de texto não recuperando o foco às vezes ao retornar à janela do Paperback.
* Corrigido o campo de texto no diálogo ir para porcentagem não atualizando o valor do controle deslizante.
* Corrigido a renderização de IDs HTML personalizados em documentos Markdown.
* HTML dentro de blocos de código Markdown agora será renderizado adequadamente.
* Se carregar um livro com um parâmetro de linha de comando enquanto uma instância existente do Paperback está em execução, você não receberá mais um erro se carregar seu documento demorar mais de 5 segundos.
* Se executar o Paperback como administrador, a configuração agora será carregada e salva adequadamente.
* Agora é possível excluir um indicador diretamente de dentro do diálogo de indicadores.
* Agora é possível importar e exportar seus indicadores e posição de leitura para um documento particular. O arquivo gerado é nomeado após o arquivo com uma extensão .paperback. Se tal arquivo for encontrado no mesmo diretório que um arquivo durante seu carregamento, será carregado automaticamente. Caso contrário, você pode importá-los manualmente usando um item no menu ferramentas.
* Links dentro de documentos agora são totalmente suportados! Use k e shift+k para mover para frente e para trás através deles e pressione enter para abrir/ativar um.
* Muitas refatorações internas, tornando o aplicativo mais rápido e o binário menor.
* Conteúdo Markdown agora é pré-processado para ser compatível com CommonMark antes de renderizar.
* Navegação por listas e seus itens agora é totalmente suportada! Use L e Shift+L para ir por listas em si e I e Shift+I para ir através de itens de lista.
* Numpad delete agora funciona para remover documentos da barra de guias além do delete normal.
* Paperback agora pode opcionalmente minimizar para sua bandeja do sistema! Esta opção está desativada por padrão, mas ativá-la fará com que a opção minimizar no menu do sistema coloque o Paperback em sua bandeja, capaz de ser restaurado clicando no ícone gerado.
* Paperback agora é totalmente traduzível! A lista de idiomas que suporta é atualmente bastante pequena, mas está crescendo constantemente!
* Documentos PPTX agora mostrarão um índice de conteúdo básico, contendo todos os slides.
* O caminho completo para o documento aberto agora será mostrado no diálogo de informações do documento.
* O instalador agora inclui uma opção para visualizar o readme em seu navegador após a instalação.
* A lista de documentos recentes foi dramaticamente expandida! Em vez de simplesmente mostrar os últimos 10 documentos que você abriu, agora mostrará um número personalizável, com o restante dos documentos que você já abriu sendo acessível por meio de um pequeno diálogo.
* Várias pequenas melhorias nos analisadores em todo o quadro, incluindo colocar uma linha em branco entre slides em apresentações PPTX, corrigindo o tratamento de quebra de linha dentro de parágrafos em documentos word e adicionando marcadores a itens de lista.

### Versão 0.5.0
* Adicionado suporte para documentos Microsoft Word!
* Adicionado suporte para apresentações PowerPoint!
* Corrigido certos itens de menu não sendo desabilitados sem documentos abertos.
* Corrigido a orientação do controle deslizante ir para porcentagem.
* Corrigido o índice de conteúdo em livros Epub com caminhos de arquivo codificados em URL e/ou IDs de fragmento.
* Corrigido espaço em branco sendo removido de títulos XHTML de formas estranhas.
* Corrigido tratamento de espaço em branco dentro de tags pre aninhadas em documentos HTML.
* Documentos HTML e Markdown agora suportam o recurso de índice de conteúdo! Quando você carrega um documento HTML/Markdown, o Paperback construirá seu próprio índice de conteúdo a partir da estrutura dos títulos em seu documento e o mostrará para você no diálogo ctrl+t.
* Documentos HTML agora terão o título conforme definido na tag title, se existir. Caso contrário, continuarão usando o nome do arquivo sem a extensão.
* Mudou de UniversalSpeech para usar uma região ao vivo para relatar fala. Isso significa que nenhuma DLL do leitor de tela é enviada junto com o programa, e mais leitores de tela agora serão suportados, como Microsoft Narrator.
* Mudou bibliotecas zip para permitir a abertura de uma gama mais ampla de livros epub.
* O diálogo pedindo se você quer abrir seu documento como texto simples foi completamente refeito, e agora permite que você abra seu documento como texto simples, HTML ou Markdown.
* O diálogo ir para porcentagem agora inclui um campo de texto permitindo que você insira manualmente uma porcentagem para pular para.
* O analisador HTML agora reconhecerá dd, dt e dl como elementos de lista.
* O índice de conteúdo em livros Epub será preservado exatamente novamente.
* O espaço não-quebra unicode agora é considerado ao remover linhas em branco.
* Você não será mais perguntado como quer abrir um arquivo não reconhecido toda vez que o carrega, apenas na primeira vez.

### Versão 0.4.1
* Adicionado um ícone de menu iniciar opcional ao instalador.
* O índice de conteúdo agora deve estar mais limpo em alguns casos, por exemplo se você tiver um item filho e pai com o mesmo texto na mesma posição você verá apenas o item pai.
* Corrigido o índice de conteúdo em certos documentos CHM.
* Corrigido o índice de conteúdo em livros Epub 3 com caminhos absolutos neles.
* Documentos CHM agora devem mostrar seu título conforme definido no arquivo de metadados.

### Versão 0.4.0
* Adicionado suporte para arquivos CHM!
* Adicionado suporte para indicadores! Você pode ter quantos indicadores desejar em quantos documentos desejar. Você pode pular para frente e para trás através deles com b e shift+b, definir um com control+shift+b e trazer um diálogo para pular para um indicador específico com control+b.
* Adicionado um instalador junto com o arquivo zip portátil! O instalador instalará o Paperback em seu diretório Program Files e configurará automaticamente as associações de arquivo para você.
* Arquivos de texto com BOMs agora devem ser decodificados adequadamente e o BOM não será mais exibido no início do texto também.
* Adicionado muito mais informações à barra de status. Agora mostrará sua linha atual, caractere e porcentagem de leitura.
* Comentários HTML, bem como o conteúdo de tags de script e estilo não serão mais mostrados na saída de texto.
* Se passar um caminho relativo para o Paperback na linha de comando, agora ele será resolvido adequadamente.
* O movimento de porcentagem agora é tratado por seu próprio diálogo baseado em controle deslizante, acessível com control+shift+g.
* Documentos sem títulos ou autores conhecidos agora sempre terão um padrão.
* A lógica de poupança de posição agora é muito mais inteligente e deve gravar no disco apenas quando absolutamente necessário.
* O documento no qual você tinha o foco quando fechou o Paperback agora é lembrado entre reinicializações da aplicação.
* A entrada nos diálogos ir para linha e ir para página agora deve ser higienizada mais estritamente.
* Corrigido navegação de índice de conteúdo em livros epub 3 com caminhos relativos em seus manifestos.

### Versão 0.3.0
* Corrigido o índice de conteúdo em livros epub com manifestos codificados em URL.
* Corrigido navegação de título em documentos HTML contendo caracteres Unicode multibyte.
* Corrigido alto uso de CPU em documentos com títulos longos devido a uma regressão no wxWidgets.
* Corrigido carregamento de arquivos de texto UTF-8.
* Corrigido itens TOC aninhados em livros Epub colocando seu cursor na posição errada.
* Corrigido uma pane na saída da aplicação em certos casos.
* Adicionada uma caixa de seleção no diálogo de opções para ativar ou desativar quebra de linha!
* Agora é possível doar para o desenvolvimento do Paperback, seja através do novo item de doação no menu de ajuda ou através do link de patrocinar este projeto na parte inferior da página principal do repositório GitHub.
* Documentos Markdown agora sempre terão um título, e o Paperback agora deve ser capaz de carregar praticamente qualquer arquivo Markdown.
* Documentos PDF agora sempre terão um título, mesmo que os metadados estejam faltando.
* Mudou bibliotecas PDF para a usada no Chromium, levando a análise de PDF muito mais confiável em todo o quadro.
* Você agora pode ter apenas uma instância do Paperback em execução por vez. Executar paperback.exe com um nome de arquivo enquanto já está em execução abrirá esse documento na instância já em execução.
* Você agora pode pressionar delete em um documento no controle de guia para fechá-lo.

### Versão 0.2.1
* Adicionado o número total de páginas ao rótulo de página no diálogo ir para página.
* Permitir tabulação do conteúdo do documento para sua lista de documentos abertos.
* Corrigido as tabulações de título às vezes abrindo documentos recentes se você tivesse o suficiente deles.
* Paperback agora removerá hífens suaves desnecessários da saída de texto.
* Corrigido navegação de título às vezes o colocando no caractere errado.

### Versão 0.2.0
* Adicionado suporte para documentos Markdown!
* Adicionado suporte para documentos PDF, incluindo a capacidade de navegar entre páginas!
* Adicionadas sequências de teclas para navegar por títulos em conteúdo HTML, incluindo livros epub e documentos Markdown. Essas sequências de teclas foram projetadas para funcionar semelhante a um leitor de tela.
* Corrigido carregamento de epubs com nomes de arquivo codificados em URL em seus manifestos.
* Corrigido carregamento de livros epub 3 com XHTML incorporado neles.
* Uma mensagem agora é falada se o documento não suportar um índice de conteúdo ou seções, em vez de os itens de menu serem desabilitados.
* Adicionado um menu de documentos recentes! Atualmente armazena seus últimos 10 documentos abertos e pressionar enter em um o abrirá para leitura.
* Completamente reescreveu o diálogo Localizar, tornando-o muito mais simples de usar, enquanto também adicionando um histórico de suas últimas 25 buscas e suporte a expressões regulares!
* Documentos previamente abertos agora são lembrados entre reinicializações da aplicação. Isso é configurável através do novo item de opções no menu ferramentas.
* Adicionado shift+f1 para abrir o readme diretamente no Paperback.

### Versão 0.1.0
* Lançamento inicial.
