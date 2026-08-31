<!-- machine-translated from doc/readme.md (source-hash: df18cffffe239932); please review and edit as needed -->

# Paperback - versão 0.9.1

## Introdução

Paperback é um leitor leve, rápido e acessível de ebooks e documentos para todos, desde leitores casuais até usuários avançados. Ele foi projetado para acessibilidade com leitor de tela, velocidade rápida e uma experiência sem inchaço.

## Requisitos do Sistema

Paperback atualmente funciona em Windows 10/11 e em todas as versões modernas de ARM macOS. Aplicativos nativos para iOS e Android estão em desenvolvimento ativo, com compilações de teste público planejadas logo após o lançamento da versão 0.9.0 para desktop, antes de um lançamento unificado 1.0 cobrindo todas as quatro plataformas.

## Recursos

* Completamente independente, não exigindo nenhum software instalado no seu computador para começar a ler.
* Incrivelmente rápido, mesmo em hardware antigo.
* Interface simples com abas, permitindo que você abra quantos documentos desejar lado a lado.
* Salva sua posição exata de leitura em todos os documentos que você abre.
* Opcionalmente lembra quais documentos você tinha abertos quando fechou o programa e os restaura no próximo lançamento.
* Inclui funcionalidade de navegação semelhante à encontrada no modo de navegação da web de muitos leitores de tela para navegar rápida e facilmente pelos documentos.
* Inclui um diálogo de busca robusto, com recursos como histórico e suporte a expressões regulares.
* Pode ser executado de forma totalmente portátil ou instalado com associações de arquivo configuradas automaticamente.
* Suporta uma grande variedade de formatos de arquivo comuns.

## Compatibilidade com Leitores de Tela

Paperback funciona bem com todos os principais leitores de tela. Há, no entanto, um problema conhecido para usuários do JAWS.

### JAWS e Displays Braille

Se você usar JAWS com um display Braille, pode descobrir que parágrafos longos são truncados ao navegar para frente com as teclas de navegação do seu display. O comando de ler parágrafo atual também é afetado. Este é um bug no tratamento do controle de texto RICHEDIT50W do JAWS, não algo no próprio Paperback, e um que demorou bastante para encontrar uma solução, dada a falta de entusiasmo da Vispero em responder a problemas com software de código aberto.

A solução alternativa, eventualmente descoberta através do grupo de discussão do JAWS após meses de espera, é editar `paperback.jcf` e definir "Braille Presentation and Panning" como "Always use DOM if available". Você também vai querer habilitar "Pan Text by Paragraph", caso contrário seu display permanecerá no parágrafo ativo em vez de avançar. Com as duas configurações em vigor, a navegação deve funcionar corretamente.

## Tipos de arquivo atualmente suportados

Paperback suporta os seguintes formatos e extensões:

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
* Arquivos de texto simples e logs (`.txt`, `.log`)

## Atalhos de teclado

Paperback foi projetado para uso com foco no teclado. Aqui estão os atalhos atuais.

Os atalhos abaixo são para Windows. Quando macOS difere, o equivalente é indicado entre parênteses — principalmente porque `Ctrl+G`, `Ctrl+W` e `Alt+Left`/`Alt+Right` já são reivindicados por outras convenções de sistema ou aplicativo nessa plataforma.

### Menu Arquivo

* `Ctrl+O`: Abre um documento.
* `Ctrl+F4` (macOS: `Cmd+W`): Fecha o documento atual.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Fecha todos os documentos abertos.
* `Ctrl+Shift+T`: Reabre o último documento fechado.
* `Ctrl+R`: Mostra o diálogo "Todos os Documentos" (de Documentos Recentes).
* `Ctrl+Q`: Sai (apenas Windows; no macOS isso está no menu do aplicativo).

### Menu Ir

* `Ctrl+F`: Mostra o diálogo Localizar.
* `F3` (macOS: `Cmd+G`): Localiza o próximo.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Localiza o anterior.
* `Ctrl+G` (macOS: `Cmd+L`): Vai para a linha.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Vai para o percentual.
* `Ctrl+P`: Vai para a página (quando suportado pelo documento atual).
* `=`: Anuncia seu percentual atual de leitura.
* `Alt+Left` (macOS: `Cmd+[`): Volta no histórico de navegação.
* `Alt+Right` (macOS: `Cmd+]`): Avança no histórico de navegação.
* `[`: Seção anterior.
* `]`: Próxima seção.
* `Shift+H`: Título anterior.
* `H`: Próximo título.
* `Shift+1` a `Shift+6`: Título anterior no nível 1-6.
* `1` a `6`: Próximo título no nível 1-6.
* `Shift+P`: Página anterior.
* `P`: Próxima página.
* `Shift+B`: Marcador anterior.
* `B`: Próximo marcador.
* `/`: Define seu marcador temporário.
* `\`: Vai para seu marcador temporário.
* `Shift+N`: Nota anterior.
* `N`: Próxima nota.
* `Ctrl+B`: Vai para todos os marcadores e notas.
* `Ctrl+Alt+B`: Vai para marcadores apenas.
* `Ctrl+Alt+M`: Vai para notas apenas.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, ou seja, a tecla Control física em vez de Cmd): Visualiza o texto da nota na posição atual.
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
* `Shift+,`: Vai para o início do contêiner atual (lista ou tabela).
* `,`: Vai além do final do contêiner atual (lista ou tabela).

### Menu Ferramentas

* `Ctrl+W` (macOS: `RawCtrl+W`, ou seja, a tecla Control física em vez de Cmd): Mostra a contagem de palavras do documento atual.
* `Ctrl+I`: Mostra informações do documento.
* `Ctrl+T`: Mostra o sumário.
* `F7`: Mostra a lista de elementos.
* `Ctrl+Shift+C`: Abre a pasta contendo.
* `Ctrl+Shift+V`: Abre o conteúdo atual no Web View.
* `Ctrl+U`: Visualiza a fonte do documento em uma nova aba.
* `Ctrl+Shift+E`: Exporta dados do documento (`.paperback`).
* `Ctrl+Shift+I`: Importa dados do documento (`.paperback`).
* `Ctrl+E`: Exporta o documento atual como texto simples.
* `Ctrl+Shift+B`: Alterna marcador na seleção/cursor atual.
* `Ctrl+Shift+N`: Adiciona ou edita nota de marcador na seleção/cursor atual.
* `Ctrl+Alt+W`: Alterna quebra de linha.
* `Ctrl+Space`: Reproduz/pausa a narração de áudio.
* `'`: Avança a narração de áudio.
* `;`: Retrocede a narração de áudio.
* `Ctrl+'`: Aumenta o tempo de busca de áudio.
* `Ctrl+;`: Diminui o tempo de busca de áudio.
* `F11` (macOS: `RawCtrl+Ctrl+F`, ou seja, Control+Command+F): Alterna tela inteira.
* `Ctrl+,`: Abre opções (macOS: Preferências, no menu do aplicativo).
* `Ctrl+Shift+S`: Alterna temporizador de sono.

### Menu Ajuda

* `Ctrl+F1`: Mostra o diálogo Sobre.
* `F1`: Visualiza ajuda no seu navegador padrão.
* `Shift+F1`: Visualiza ajuda no Paperback.
* `Ctrl+Shift+U`: Verifica atualizações.
* `Ctrl+D`: Abre a página de doação no seu navegador padrão.

### Teclas adicionais da exibição de documentos

* `Delete` / `Numpad Delete` no controle de aba: Fecha a aba de documento selecionada.
* `Enter` ou `Space` no texto do documento: Ativa o link no cursor, ou abre uma exibição de tabela quando está em um marcador de tabela.
* `Shift+F10` ou a tecla Menu/Aplicativo no texto do documento: Abre o menu de contexto.

## Idiomas suportados

Paperback é traduzido para muitos idiomas diferentes, com mais sendo adicionados o tempo todo. Uma lista completa segue abaixo.

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
* Aryan Choudhary: principal contribuidor.

### Doações
As seguintes pessoas fizeram doações de algum valor para o desenvolvimento do Paperback. Se você fizer uma doação, seu nome não será adicionado automaticamente aqui; apenas adiciono pessoas que desejam que sua doação seja pública.

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

## Histórico de versões

### Versão 0.9.2
* Audiolivros não fazem mais seu leitor de tela ler uma sequência de espaços quando você foca o campo de texto.
* Audiolivros agora nomeiam o arquivo conforme você avança pelas seções.
* Audiolivros agora relatam seu comprimento real, em vez de afirmar que cada arquivo dura 24 horas.
* Fechar Web View com Escape não lança mais um alerta de debug depois que você seguiu um link dentro dela.
* Copiar após Selecionar Tudo agora fornece o documento inteiro, em vez de apenas a parte carregada no momento.
* Localizar agora vai direto para a linha encontrada, em vez de fazer você ouvir o leitor de tela ler a janela novamente enquanto o foco retorna ao livro.
* Corrigido EPUBs que carregam um bloco ZIP64 isolado recusando abrir com "Invalid local file header".
* Corrigido documentos longos voltando para o início enquanto um leitor de tela os lia continuamente.
* Links em Web View agora o levam para a seção para a qual apontam, em vez de falharem com "File not found".
* O anúncio automático "Document reloaded" não interrompe mais seu leitor de tela no meio de uma frase, aguardando que ele termine o que estava dizendo.
* A aba Geral do diálogo Configurações agora passa por suas opções na ordem em que aparecem na tela, com o canal de atualização logo após a opção de verificar atualizações.
* Windows agora sempre mostrará "Paperback" no menu Abrir Com, em vez da tagline completa do programa.
* Word Count e Document Info agora mostram quantos arquivos um audiolivro contém e quanto tempo ele dura no total.

### Versão 0.9.1
* Sons de marcadores e notas agora tocam no macOS.
* Audiolivros DAISY agora reproduzem seu áudio no macOS, em vez de abrir e rastrear sua linha do tempo em silêncio.
* Corrigido aspas curvas, travessões e caracteres similares desaparecendo de documentos RTF, unindo as palavras ao redor enquanto desapareciam.
* Corrigido imagens RTF vazando seus dados brutos no documento como texto corrompido.
* Corrigido o submenu Documentos Recentes mantendo entradas obsoletas até que algo else acontecesse para reconstruí-lo.
* Aceleradores de teclado estão de volta em cada tradução, então os menus do russo têm acesso ao teclado novamente.
* Grandes documentos CHM agora abrem até sete vezes mais rápido.
* Documentos abertos agora são registrados no Windows, então aparecem na jump list da barra de tarefas e na lista recente do menu Iniciar.
* Options foi renomeado para Settings, coincidindo com os aplicativos móveis e, no macOS, a convenção da plataforma.
* Paperback agora lembra sua posição de janela, tamanho e estado maximizado entre execuções.
* Formas plurais agora são traduzidas, então mensagens que contam coisas leem corretamente em idiomas que precisam de mais de uma forma.
* Selecionar o ncc.html de um livro DAISY agora abre o audiolivro completo em vez de apenas seu texto.
* Os nomes de ações do diálogo Customize Keyboard Shortcuts agora podem ser traduzidos.
* O título do documento agora vem primeiro na barra de título, então livros abertos podem ser distinguidos na barra de tarefas e Alt+Tab.
* O diálogo de atualização agora é traduzido.

### Versão 0.9.0

#### Adicionado

##### Geral
* Uma ferramenta CLI, chamada pb, para converter rapidamente qualquer um dos formatos suportados do Paperback para HTML, Markdown ou texto simples.
* Uma opção para recarregar documentos que foram modificados por outros programas em disco.
* Uma opção View Source para abrir o código-fonte de um documento em uma nova aba, útil para editar Markdown, por exemplo.
* O texto do documento agora é paginado, significando que você pode carregar livros com dezenas de milhões de palavras em apenas alguns segundos. Por favor, reporte qualquer estranheza encontrada com isso.

##### Suporte de Plataforma
* Suporte ARM64 Windows!
* Suporte nativo macOS!
* Um botão de alternância de tela cheia.

##### Diálogo Todos os Documentos
* Um botão localizar para localizar livros perdidos que apenas mudaram de caminho.
* Um filtro de status e barra de status, então você pode filtrar por status do documento e ver quantos documentos são mostrados e selecionados.
* O atalho `Ctrl+Shift+A` para desselecionar todos os documentos.

##### Opções e Legibilidade
* Uma aba legibilidade, com as seguintes opções:
    * Quebra de linha (movida de geral);
    * Renderizar tabelas inline (novo nesta versão, veja abaixo);
    * Fonte;
    * Cor de fundo;
    * Espaçamento de linha;
    * Espaçamento de parágrafo;
    * Espaçamento de letra;
    * Alinhamento de texto.
* Um item de menu quebra de linha e tecla de atalho subsequente.
* Um alternador para determinar como você quer que as tabelas sejam exibidas, e unificou como as tabelas são exibidas em documentos.

##### Navegação
* Suporte para navegação por container.
* Uma opção para mover automaticamente o cursor para o início da linha ao navegar entre linhas, similar ao modo de navegação em leitores de tela.
* O atalho de teclado equals para anunciar sua porcentagem atual através de um documento.

##### Marcadores
* Marcadores temporários: você pode ter um por documento, e eles persistem. Use slash para definir um e backslash para pular para ele.

##### Word Count
* Tempo de leitura estimado no diálogo contagem de palavras, bem como a capacidade de definir sua velocidade de leitura para tornar essa métrica realmente útil.
* Se uma seleção estiver ativa quando você abrir o diálogo contagem de palavras, quantas palavras você selecionou agora será mostrado.

##### Atalhos de Teclado
* A capacidade de personalizar cada atalho de teclado no aplicativo através de um diálogo simples.
* Um atalho de teclado configurável para restaurar Paperback da bandeja do sistema.

##### Idiomas
* Holandês, Finlandês e Polonês.

##### Exportar
* Expandido o item de menu exportar para permitir exportar para HTML e Markdown, além de texto simples.

##### Atualizador
* Um botão cancelar no diálogo de atualização em andamento.
* O atualizador agora valida que o arquivo baixado não foi adulterado.

##### Web View
* A webview agora é aberta em sua posição de leitura atual.

##### Audiolivros DAISY
* Suporte para audiolivros DAISY 2.0.
* Suporte para reprodução de áudio DAISY 2.02.

##### Audiolivros
* A capacidade de reproduzir audiolivros, atualmente suportando audiolivros DAISY (incluindo DAISY áudio + texto) e zips de arquivos de áudio.
* Atalhos de teclado e itens de menu para reproduzir/pausar narração, buscar para frente e para trás, e ajustar a quantidade de busca.
* Opções para sincronizar o cursor de leitura com reprodução de áudio, definir a quantidade de busca de áudio e escolher se a busca após o final de um capítulo continua no próximo.

##### Documentos CHM
* Suporte para listas, itens de lista, figuras e imagens.

##### PowerPoint
* Documentos PowerPoint agora suportam tabelas.

#### Corrigido

##### Geral
* Documentos codificados em codificações CJK legadas, como GBK, Big5 e Shift_JIS, agora serão renderizados corretamente em vez de como um monte de mojibake.
* "Reopen last closed" tentando reabrir o readme incluído.
* Sua aba selecionada não sendo focada corretamente após reiniciar Paperback.
* O tratamento de Paperback para arquivos em unidades de rede do Windows: pressionar mostrar arquivo na pasta agora foca corretamente o arquivo no armazenamento de rede, e os caminhos não contêm mais caracteres estranhos.
* Arquivos .paperback não serão mais carregados à força na restauração de documentos; em vez disso, você será solicitado para confirmação quando um for encontrado.
* Abrir pasta contendo agora foca o arquivo dado no explorador.
* Abrir o readme agora respeitará seu idioma selecionado.
* A interface de usuário do Paperback agora dimensionará corretamente em displays de alta DPI.
* O menu agora atualiza adequadamente, e o foco se move para o controle de texto, ao abrir ajuda no Paperback.
* Mudado para um método muito mais seguro de IPC no Windows.
* O título do documento ativo agora será lido ao alternar entre abas.
* Reduzido o uso de memória em documentos grandes reduzindo pela metade o tamanho das tabelas de índice por caractere internas.

##### Diálogo Todos os Documentos
* Escape não fechando os diálogos Document Info e All Documents.
* A barra de título não atualizando após fechar um documento do diálogo todos os documentos.
* Readme.html não será mais adicionado à sua lista de todos os documentos quando aberto via Shift+F1.
* Remover documentos do diálogo recentes agora também fecha sua aba ativa.
* Seu filtro de pesquisa agora é preservado após remover um documento.

##### Navegação
* Navegação de página anunciando texto de linha incorreto em algumas situações.
* Ir para Linha, Ir para Página e Ir para Percentual colocando seu cursor na posição errada em documentos grandes.
* Localizar e Localizar Próximo não respeitando a janela de documento carregada em documentos grandes.

##### Marcadores
* Sons de marcador/nota devem agora tocar corretamente exclusivamente quando você navega sobre uma palavra contendo um.

##### Legibilidade
* Aplicar quebra de linha tiro você para o início do seu documento.

##### Web View
* O diálogo webview não sendo redimensionável e aparecendo com um tamanho inicial muito pequeno.
* As imagens devem agora exibir corretamente na webview incorporada.

##### Atualizador
* O atualizador agora mostra corretamente o conteúdo de tags de código markdown nas notas de versão.

##### Audiolivros DAISY
* Audiolivros DAISY mostrando informações incorretas na barra de status.
* Carregando audiolivros DAISY com declarações de codificação falsas.

##### Documentos RTF
* Análise de documentos RTF com caracteres não-latinos neles.
* Grupos RTF `\pict` para que dados de imagem incorporados não vazem mais para o texto do documento.

##### Audiolivros Mobi/AZW3
* Âncoras filepos em audiolivros Mobi dividindo tags HTML e colocando lixo no texto do livro.
* Links em audiolivros Mobi legados.
* Análise de AZW3 muito melhorada.

##### Documentos Word
* Documentos Word com nomes de estilo específicos de localidade não renderizando seus títulos corretamente.

##### Documentos HTML/XHTML
* Elementos dl, dt e dd não produzindo quebras de linha em documentos XHTML.

##### Documentos PDF
* Paperback agora volta para extração de texto simples para PDFs falsamente marcados.
* Documentos PDF contendo caracteres de controle em seus títulos e/ou marcadores não causar mais crash no Paperback ao abrir.

### Versão 0.8.5
* Suporte de página adicionado aos audiolivros epub.
* Suporte adicionado para documentos Microsoft Office criptografados. Atualmente Word legado, Word moderno e Powerpoint moderno são suportados, com Powerpoint legado planejado para o futuro.
* Suporte adicionado para documentos Microsoft Word legados (*.doc)!
* Suporte adicionado para apresentações Powerpoint legadas (*.ppt)!
* Suporte adicionado para audiolivros mobi e AZW3!
* Suporte adicionado para arquivos PDF marcados!
* Adicionado o atalho ctrl+q para sair do aplicativo.
* Suporte adicionado para audiolivros compactados do Bookshare (tanto DAISY quanto Word)!
* Texto alternativo para imagens incorporadas deve agora ser mostrado corretamente.
* Documentos CHM agora suportam corretamente navegação de link interno.
* Corrigido sons de marcadores acionando no início do parágrafo em vez da posição do marcador.
* Corrigido ir para página estar desativado por 1.
* Corrigido a tecla escape não funcionando para fechar o diálogo abrir como.
* Corrigido o menu de contexto do leitor não aparecendo ao clicar com o botão direito ou pressionar a tecla Aplicativos.
* Corrigido o documento errado às vezes sendo focado ao abrir documentos da linha de comando.
* PDFs somente de imagem são novamente detectados e o alertam de sua existência.
* Agora é possível navegar através de imagens e figuras com g/shift+g e f/shift+f, respectivamente.
* Paperback agora respeitará sua configuração de modo escuro de aplicativo.
* Removido suporte DAISY XML, pois não é mais necessário.
* Voltado para a navegação de primeira letra Win32 nativa no modo de exibição em árvore do sumário.
* O diálogo de erro de carregamento agora mostra mensagens de erro mais detalhadas.
* A webview agora abrirá muito mais rápido e suavemente.

### Versão 0.8.2
* Suporte de página adicionado aos documentos RTF!
* Corrigido um bug onde abrir a webview em epubs contendo links externos os ativaria automaticamente.
* Corrigido um bug onde o analisador RTF não colocaria um espaço entre palavras em casos raros.
* Corrigido parágrafos sendo divididos em várias linhas curtas em alguns documentos PDF.
* Documentos PDF agora têm suporte básico de navegação de link e títulos!
* Abas e alimentações de linha RTF agora são renderizadas exatamente como aparecem no documento.
* Voltado para a biblioteca pdfium comprovada para analisar PDFs, tornando a renderização de PDF muito mais confiável novamente.

### Versão 0.8.1
* Adicionado Ctrl+Shift+T para reabrir o último documento fechado.
* O diálogo Todos os Documentos agora suporta seleção de vários documentos para abrir de uma vez.
* Corrigido alguns bugs com o analisador RTF.
* Corrigido caminhos de arquivo contendo caracteres não-ASCII (como Sérvio š, č, ć, ž) ficando corrompidos ao abrir um arquivo através de uma segunda instância de Paperback.
* Corrigido texto PDF sendo lido na ordem errada, e espaçamento incorreto ao redor de palavras capitalizadas.
* Corrigido carregamento lento de documentos ao abrir arquivos grandes.
* Corrigido a localização dos botões Sim/Não em diálogos de confirmação.

### Versão 0.8.0
* Adicionadas traduções para Japonês, Chinês simplificado e Vietnamita!
* Adicionado um atualizador automático que agora substituirá sua versão atualmente instalada do Paperback em vez de apenas fazer download da nova versão!
* Adicionado feedback de som opcional para atingir um marcador ou nota, obrigado Andre Louis pelos sons!
* Suporte a documentos RTF adicionado!
* Suporte adicionado para documentos DAISY XML.
* Suporte adicionado para arquivos de Texto de Documento Aberto Plano!
* Suporte adicionado para apresentações de Documento Aberto Plano!
* Suporte adicionado para separadores com s e shift+s.
* Qualquer movimento maior que 300 caracteres agora adicionará automaticamente ao seu histórico de navegação.
* Corrigido restaurar a janela do Paperback da bandeja do sistema.
* Corrigido documentos Markdown mostrando texto bruto em vez de HTML renderizado na Web View.
* Corrigido tabelas não renderizando corretamente em arquivos Markdown.
* PDFs somente de imagem agora o avisarão de sua existência quando você tentar carregar um.
* Agora é possível verificar novas compilações de dev em vez de lançamentos estáveis ao verificar atualizações.
* Informações de versão adequadamente incorporadas no executável do Paperback.
* Divida o diálogo de opções em abas para facilitar o uso e a navegação.
* Voltado para Hayro para analisar PDFs, levando a mais confiabilidade, velocidade e menos DLLs.
* Reescreveu todo o aplicativo em Rust. A nova base de código é mais segura, carrega documentos mais rápido e é mais fácil de manter e estender.
* O menu de contexto do controle de texto agora incluirá ações específicas do leitor em vez de itens genéricos como cortar e colar.

### Versão 0.7.0
* Suporte a tabelas adicionado para documentos baseados em HTML e XHTML! Navegue entre tabelas usando T e Shift+T, e pressione Enter para visualizar uma em uma webview.
* Recurso básico de renderização da web adicionado! Pressione Ctrl+Shift+V para abrir a seção atual do seu documento em um renderizador baseado na web, útil para conteúdo como formatação complexa ou amostras de código.
* Adicionada uma tradução para o russo, obrigado Ruslan Gulmagomedov!
* Adicionado um botão Clear All ao diálogo Todos os Documentos.
* O verificador de atualização agora exibe notas de versão quando uma nova versão está disponível.
* Corrigido restaurar a janela da bandeja do sistema.
* Corrigido traduções de botão Sim/Não em diálogos de confirmação.
* Corrigido carregamento de configs ao executar como administrador.
* Corrigido tratamento de comentários em documentos XML e HTML.
* Corrigido análise de TOC em audiolivros Epub 2.
* Corrigido navegação para o próximo item com a mesma letra no sumário.
* Corrigido o diálogo de localização não se ocultando corretamente ao usar os botões próximo/anterior.
* Corrigido TOCs epub ocasionalmente o jogando para o item errado.
* Corrigido vários problemas de tratamento de espaço em branco em tags XML, HTML e pre.
* Corrigido erro off-by-one na navegação de link.
* Corrigido alguns audiolivros tendo espaço em branco à direita em suas linhas.
* Corrigido vários problemas do analisador.
* Itens de menu relacionados a marcadores, bem como a lista de elementos, agora estão desabilitados quando nenhum documento está aberto.
* Tratamento de lista melhorado em vários formatos de documento.
* Fluxo de trabalho de tradução melhorado para colaboradores.
* Muitos refatores internos, movendo a maioria da lógica de negócios do aplicativo de C++ para Rust para melhor desempenho e manutenibilidade.

### Versão 0.6.1
* Suporte a PDF protegido por senha adicionado!
* Adicionado um recurso muito básico de ir para posição anterior/próxima. Se você pressionar enter em um link interno e isso mover seu cursor, essa posição agora será lembrada e poderá ser navegada com setas alt+left/right.
* Adicionada uma lista de elementos! Atualmente mostra apenas uma árvore de todos os títulos do seu documento ou uma lista de links, mas há planos para expandi-la no futuro.
* Adicionada uma opção para iniciar o Paperback em modo maximizado por padrão.
* Corrigido links em alguns documentos Epub não funcionando corretamente.
* Corrigido análise de TOCs Epub contendo caminhos relativos.
* Corrigido alguns documentos epub não mostrando um título ou autor.
* Corrigido os títulos de alguns capítulos epub não aparecendo corretamente no diálogo TOC.
* Corrigido você não conseguir usar a barra de espaço para ativar os botões OK/cancelar no diálogo TOC.
* Tratamento de títulos em documentos Word melhorado.
* Você agora receberá feedback falado se a lista de documentos recentes estiver vazia quando tentar abrir o diálogo.

### Versão 0.6.0
* Uma nova opção para mostrar o menu de ir de forma muito mais compacta foi adicionada ao diálogo de opções, verificada por padrão.
* Adicionada uma opção para fazer a navegação por elementos estruturais envolver.
* Adicionada uma opção ao menu de ferramentas para abrir a pasta contendo do documento atualmente focado.
* Adicionado um sistema de atualização simples, mas muito eficaz.
* Adicionado um recurso básico de temporizador de sono, acessível com Ctrl+Shift+S.
* Suporte adicionado para análise de audiolivros FB2!
* Suporte adicionado para análise de apresentações de Documento Aberto!
* Suporte adicionado para análise de arquivos de Texto de Documento Aberto!
* Os marcadores agora podem ser criados para marcar uma linha inteira ou marcar apenas um texto especificado. Se você não tiver seleção ativa ao colocar um marcador, o comportamento é como pré-0.6 e marcará a linha inteira. No entanto, se você selecionar algum texto, apenas esse texto será incluído no marcador.
* Os marcadores agora podem ter notas de texto opcionais anexadas a eles! Navegue entre marcadores contendo notas com N e Shift+N, ou abra o diálogo de marcadores com todos os marcadores, apenas notas ou apenas não-notas selecionados com hotkeys específicos.
* Os marcadores no diálogo de marcadores não terão mais um prefixo "bookmark x" irritante.
* Audiolivros Epub contendo conteúdo HTML fingindo ser XML agora serão tratados corretamente.
* Corrigido carregamento de grandes documentos Markdown.
* Corrigido pressionar espaço na árvore de exibição de sumário ativando o botão OK.
* Corrigido tratamento de espaço em branco no início de tags pre em documentos HTML e XHTML.
* Corrigido o campo de texto não recuperando o foco às vezes ao retornar à janela do Paperback.
* Corrigido o campo de texto no diálogo ir para percentual não atualizando o valor do controle deslizante.
* Corrigido a renderização de IDs HTML personalizados em documentos Markdown.
* HTML dentro de blocos de código Markdown agora será renderizado corretamente.
* Se carregar um livro com um parâmetro de linha de comando enquanto uma instância existente do Paperback estiver em execução, você não terá mais um erro se o carregamento do seu documento levar mais de 5 segundos.
* Se executar Paperback como administrador, a configuração agora será carregada e salva corretamente.
* Agora é possível excluir um marcador diretamente de dentro do diálogo de marcadores.
* Agora é possível importar e exportar seus marcadores e posição de leitura para um documento particular. O arquivo gerado é nomeado após o arquivo com uma extensão .paperback. Se tal arquivo for encontrado no mesmo diretório que um arquivo ao carregá-lo, será carregado automaticamente. Caso contrário, você pode importá-los manualmente usando um item no menu ferramentas.
* Links dentro de documentos agora são totalmente suportados! Use k e shift+k para se mover para frente e para trás através deles, e pressione enter para abrir/ativar um.
* Muitos refatores internos, tornando o aplicativo mais rápido e o binário menor.
* O conteúdo de Markdown agora é pré-processado para ser em conformidade com CommonMark antes de renderizar.
* Navegação por listas e seus itens agora é totalmente suportada! Use L e Shift+L para ir por listas em si, e I e Shift+I para passar por itens de lista.
* Numpad delete agora funciona para remover documentos da barra de abas além de delete normal.
* Paperback agora pode opcionalmente minimizar para sua bandeja do sistema! Esta opção está desabilitada por padrão, mas ativá-la fará com que a opção minimizar no menu do sistema coloque Paperback em sua bandeja, podendo ser restaurado clicando no ícone gerado.
* Paperback agora é totalmente traduzível! A lista de idiomas que suporta é atualmente bastante pequena, mas está crescendo constantemente!
* Os documentos PPTX agora mostrarão um sumário básico, contendo todos os slides.
* O caminho completo para o documento aberto agora será mostrado no diálogo de informações do documento.
* O instalador agora inclui uma opção para visualizar o readme no seu navegador após a instalação.
* A lista de documentos recentes foi dramaticamente expandida! Em vez de simplesmente mostrar os últimos 10 documentos que você abriu, agora mostrará um número personalizável, com o resto dos documentos que você já abriu sendo acessível através de um pequeno diálogo.
* Várias pequenas melhorias nos analisadores em todo o conselho, incluindo colocar uma linha em branco entre slides em apresentações PPTX, corrigindo o tratamento de nova linha dentro de parágrafos em documentos word e adicionando pontos de lista a itens de lista.

### Versão 0.5.0
* Suporte a documentos Microsoft Word adicionado!
* Suporte adicionado para apresentações PowerPoint!
* Corrigido certos itens de menu não sendo desabilitados sem documentos abertos.
* Corrigido a orientação do controle deslizante ir para percentual.
* Corrigido o sumário em audiolivros Epub com caminhos de arquivo codificados em URL e/ou IDs de fragmento.
* Corrigido espaço em branco sendo removido de títulos XHTML de formas estranhas.
* Corrigido tratamento de espaço em branco dentro de tags pre aninhadas em documentos HTML.
* Documentos HTML e Markdown agora suportam o recurso de sumário! Quando você carrega um documento HTML/Markdown, Paperback construirá seu próprio sumário fora da estrutura dos títulos no seu documento, e mostrará isso no diálogo ctrl+t.
* Documentos HTML agora terão o título conforme definido na tag de título, se existir. Caso contrário, continuarão usando o nome do arquivo sem a extensão.
* Mudado de UniversalSpeech para usar uma região em tempo real para relatar fala. Isto significa que nenhuma DLL do leitor de tela é enviada junto com o programa, e mais leitores de tela serão suportados, como o Narrador do Microsoft.
* Bibliotecas de zip mudadas para permitir a abertura de uma gama mais ampla de audiolivros epub.
* O diálogo perguntando se você quer abrir seu documento como texto simples foi completamente reformulado, e agora permite abrir seu documento como texto simples, HTML ou Markdown.
* O diálogo ir para percentual agora inclui um campo de texto permitindo que você digite manualmente um percentual para pular.
* O analisador HTML agora reconhecerá dd, dt e dl como elementos de lista.
* O sumário em audiolivros Epub será mais uma vez preservado exatamente.
* O espaço não-quebrável unicode agora é considerado ao remover linhas em branco.
* Você não será mais perguntado como deseja abrir um arquivo não reconhecido toda vez que o carregar, apenas na primeira vez.

### Versão 0.4.1
* Adicionado um ícone de menu iniciar opcional ao instalador.
* O sumário deve agora ser mais limpo em alguns casos, por exemplo se você tiver um item filho e pai com o mesmo texto na mesma posição você verá apenas o item pai.
* Corrigido o sumário em certos documentos CHM.
* Corrigido o sumário em audiolivros Epub 3 com caminhos absolutos neles.
* Documentos CHM devem agora mostrar seu título conforme definido no arquivo de metadados.

### Versão 0.4.0
* Suporte a arquivo CHM adicionado!
* Suporte a marcadores adicionado! Você pode ter tantos marcadores quantos quiser em quantos documentos quiser. Você pode pular para frente e para trás através deles com b e shift+b, definir um com control+shift+b, e abrir um diálogo para pular para um marcador específico com control+b.
* Adicionado um instalador ao lado do arquivo zip portátil! O instalador instalará o Paperback no seu diretório Program Files e configurará automaticamente as associações de arquivo para você.
* Arquivos de texto com BOMs devem agora ser decodificados corretamente, e o BOM não será mais exibido no início do texto.
* Adicionadas muitas mais informações à barra de status. Agora mostrará sua linha atual, caractere e porcentagem de leitura.
* Comentários HTML, bem como o conteúdo de tags script e style, não serão mais mostrados na saída de texto.
* Se passar um caminho relativo para Paperback na linha de comando, ele agora será resolvido corretamente.
* O movimento percentual agora é tratado por seu próprio diálogo baseado em controle deslizante, acessível com control+shift+g.
* Documentos sem títulos ou autores conhecidos agora sempre terão um padrão.
* A lógica de economia de posição é agora muito mais inteligente e deve escrever apenas no disco quando absolutamente necessário.
* O documento que você focou quando fechou o Paperback agora é lembrado em reinicializações do aplicativo.
* A entrada nos diálogos ir para linha e ir para página deve agora ser sanitizada mais rigorosamente.
* Corrigido navegação do sumário em audiolivros epub 3 com caminhos relativos em seus manifestos.

### Versão 0.3.0
* Corrigido o sumário em audiolivros epub com manifestos codificados em URL.
* Corrigido navegação de título em documentos HTML contendo caracteres Unicode multibyte.
* Corrigido alto uso de CPU em documentos com títulos longos devido a uma regressão em wxWidgets.
* Corrigido carregamento de arquivos de texto UTF-8.
* Corrigido itens de TOC aninhados em audiolivros Epub colocando seu cursor na posição errada.
* Corrigido um crash ao sair do aplicativo em certos casos.
* Adicionada uma caixa de seleção no diálogo de opções para ativar ou desativar quebra de linha!
* Agora é possível doar para o desenvolvimento do Paperback, seja através do novo item de doação no menu ajuda ou através do link sponsor this project na parte inferior da página principal do repositório GitHub.
* Documentos Markdown agora sempre terão um título, e Paperback deve agora ser capaz de carregar praticamente qualquer arquivo Markdown.
* Documentos PDF agora sempre terão um título, mesmo que os metadados estejam faltando.
* Bibliotecas PDF mudadas para a usada no Chromium, levando a análise de PDF muito mais confiável em todo o quadro.
* Você agora pode ter apenas uma instância de Paperback em execução por vez. Executar paperback.exe com um nome de arquivo enquanto já está em execução abrirá esse documento na instância já em execução.
* Agora você pode pressionar delete em um documento no controle de aba para fechá-lo.

### Versão 0.2.1
* Adicionado o número total de páginas ao rótulo de página no diálogo ir para página.
* Permitir tabulação do conteúdo do documento para sua lista de documentos abertos.
* Corrigido os atalhos de título às vezes abrindo documentos recentes se você tivesse o suficiente deles.
* Paperback agora removerá hífens suaves desnecessários da saída de texto.
* Corrigido navegação de título às vezes colocando você no caractere errado.

### Versão 0.2.0
* Suporte a documentos markdown adicionado!
* Suporte a documentos PDF adicionado, incluindo a capacidade de navegar entre páginas!
* Adicionados atalhos de teclado para navegação por títulos em conteúdo HTML, incluindo audiolivros epub e documentos markdown. Estes atalhos foram projetados para funcionar semelhante a um leitor de tela.
* Corrigido carregamento de epubs com nomes de arquivo codificados em URL em seus manifestos.
* Corrigido carregamento de audiolivros epub 3 com XHTML incorporado neles.
* Uma mensagem agora é falada se o documento não suportar um sumário ou seções, em vez de os itens de menu serem desabilitados.
* Adicionado um menu de documentos recentes! Atualmente armazena seus últimos 10 documentos abertos, e pressionar enter em um o abrirá para leitura.
* Completamente reescreveu o diálogo Localizar, tornando-o muito mais simples de usar, enquanto também adicionando um histórico de suas últimas 25 buscas e suporte a expressão regular!
* Documentos previamente abertos agora são lembrados em reinicializações do aplicativo. Isto é configurável através do novo item de opções no menu ferramentas.
* Adicionado shift+f1 para abrir o readme diretamente no próprio Paperback.

### Versão 0.1.0
* Lançamento inicial.
