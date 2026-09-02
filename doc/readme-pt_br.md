<!-- machine-translated from doc/readme.md (source-hash: efe922e94821c70e); please review and edit as needed -->

# Paperback - versão 0.9.2

## Introdução

Paperback é um leitor leve, rápido e acessível de livros eletrônicos e documentos para todos, desde leitores casuais até usuários avançados. Foi projetado para acessibilidade com leitores de tela, velocidade rápida e uma experiência sem recursos desnecessários.

## Requisitos do Sistema

Paperback atualmente funciona em Windows 10/11 e todas as versões modernas do macOS ARM. Aplicativos nativos para iOS e Android estão em desenvolvimento ativo, com compilações de teste público planejadas logo após o lançamento da versão 0.9.0 para desktop, antes de um lançamento unificado da versão 1.0 cobrindo todas as quatro plataformas.

## Recursos

* Totalmente independente, não requer nenhum software instalado no seu computador para começar a ler.
* Incrivelmente rápido, mesmo em hardware antigo.
* Interface com abas simples, permitindo que você abra quantos documentos quiser lado a lado.
* Salva sua posição exata de leitura em cada documento que você abre.
* Opcionalmente lembra quais documentos você tinha abertos quando fechou o programa e os restaura no próximo lançamento.
* Inclui funcionalidade de navegação semelhante à encontrada no modo de navegação web de muitos leitores de tela para navegar de forma rápida e fácil pelos documentos.
* Inclui um diálogo de busca robusto, com recursos como histórico e suporte a expressões regulares.
* Pode ser executado totalmente de forma portátil ou instalado com associações de arquivo configuradas automaticamente.
* Suporta uma enorme variedade de formatos de arquivo comuns.

## Compatibilidade com Leitor de Tela

Paperback funciona bem com todos os principais leitores de tela. Existe, no entanto, um problema conhecido para usuários de JAWS.

### JAWS e Linhas Braille

Se você usar JAWS com uma linha Braille, poderá descobrir que parágrafos longos são truncados ao fazer panorâmica para frente com as teclas de navegação de sua linha. O comando ler parágrafo atual também é afetado. Este é um bug no tratamento do JAWS do controle de texto RICHEDIT50W, não algo no Paperback em si, e um que levou bastante tempo para superficializar um conserto dado o entusiasmo da Vispero em responder a problemas com software de código aberto.

A solução alternativa, finalmente apresentada através do grupo de discussão de JAWS após meses de espera, é editar `paperback.jcf` e definir "Braille Presentation and Panning" como "Always use DOM if available". Você também vai querer ativar "Pan Text by Paragraph", caso contrário sua linha permanecerá no parágrafo ativo em vez de avançar. Com ambas as configurações em vigor, o panorama deve funcionar corretamente.

## Tipos de arquivo suportados atualmente

Paperback suporta os seguintes formatos e extensões:

* Arquivos de ajuda CHM (`.chm`)
* Livros DAISY (`.opf`, `.zip`)
* Livros EPUB (`.epub`)
* Livros eletrônicos FB2 (`.fb2`)
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

Paperback é projetado para uso prioritário pelo teclado. Aqui estão os atalhos atuais.

Os atalhos abaixo são para Windows. Onde macOS difere, o equivalente é anotado entre parênteses — principalmente porque Ctrl+G, Ctrl+W e Alt+Left/Right já estão reservados por outras convenções de sistema ou aplicativo nessa plataforma.

### Menu Arquivo

* `Ctrl+O`: Abrir um documento.
* `Ctrl+F4` (macOS: `Cmd+W`): Fechar o documento atual.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Fechar todos os documentos abertos.
* `Ctrl+Shift+T`: Reabrir o último documento fechado.
* `Ctrl+R`: Mostrar o diálogo "Todos os Documentos" (de Documentos Recentes).
* `Ctrl+Q`: Sair (apenas Windows; no macOS isso está no menu do aplicativo).

### Menu Ir

* `Ctrl+F`: Mostrar o diálogo Localizar.
* `F3` (macOS: `Cmd+G`): Localizar próximo.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Localizar anterior.
* `Ctrl+G` (macOS: `Cmd+L`): Ir para linha.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Ir para percentual.
* `Ctrl+P`: Ir para página (quando suportado pelo documento atual).
* `=`: Anunciar seu percentual de leitura atual.
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
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, ou seja, a tecla Control física em vez de Cmd): Ver texto de nota na posição atual.
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
* `Ctrl+Shift+V`: Abrir conteúdo atual em Web View.
* `Ctrl+U`: Ver fonte do documento em uma nova aba.
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
* `Ctrl+,`: Abrir opções (macOS: Preferências, no menu do aplicativo).
* `Ctrl+Shift+S`: Alternar temporizador de sono.

### Menu Ajuda

* `Ctrl+F1`: Mostrar diálogo Sobre.
* `F1`: Ver ajuda no seu navegador padrão.
* `Shift+F1`: Ver ajuda no Paperback.
* `Ctrl+Shift+U`: Verificar atualizações.
* `Ctrl+D`: Abrir página de doação no seu navegador padrão.

### Teclas adicionais da visualização de documento

* `Delete` / `Numpad Delete` no controle de abas: Fechar a aba do documento selecionada.
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
As seguintes pessoas fizeram doações de algum valor para o desenvolvimento do Paperback. Se você fizer uma doação, seu nome não será automaticamente adicionado aqui, eu apenas adiciono pessoas que desejam que sua doação seja pública.

Nota: Considero um patrocínio público do GitHub motivo para inclusão automática nesta lista.

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
* Livros de áudio não fazem mais seu leitor de tela ler uma série de espaços ao focar no campo de texto.
* Livros de áudio agora nomeiam o arquivo conforme você navega por seções.
* Livros de áudio agora informam seu comprimento real, em vez de afirmar que cada arquivo dura 24 horas.
* Fechar a Exibição na Web com Escape não gera mais um alerta de depuração após seguir um link dentro dela.
* Copiar após Selecionar Tudo agora fornece todo o documento, em vez de apenas a parte carregada no momento.
* Localizar agora vai direto para a linha encontrada, em vez de fazer você ouvir o leitor de tela ler a janela novamente conforme o foco retorna ao livro.
* Corrigidos EPUBs que contêm um bloco ZIP64 órfão recusando abrir com "Cabeçalho de arquivo local inválido".
* Corrigidos documentos longos voltando ao início enquanto um leitor de tela os lia continuamente.
* Links na Exibição na Web agora o levam para a seção para a qual apontam, em vez de falhar com "Arquivo não encontrado".
* O anúncio automático "Documento recarregado" não corta mais seu leitor de tela no meio da frase, esperando que termine de falar.
* A aba Geral do diálogo Configurações agora navega por suas opções na ordem em que aparecem na tela, com o canal de atualização diretamente após a opção de verificar atualizações.
* Windows agora sempre mostra "Paperback" no menu Abrir com, em vez da tagline completa do programa.
* Contagem de Palavras e Informações do Documento agora mostram quantos arquivos um livro de áudio contém e quanto tempo leva no total.

### Versão 0.9.1
* Sons de marcador e nota agora tocam no macOS.
* Livros DAISY agora reproduzem seu áudio no macOS, em vez de abrir e rastrear sua linha do tempo em silêncio.
* Corrigidas aspas curvas, travessões e caracteres similares desaparecendo de documentos RTF, unindo as palavras ao redor.
* Corrigidas imagens RTF vazando seus dados brutos no documento como texto corrompido.
* Corrigido o submenu Documentos Recentes mantendo entradas obsoletas até que outra coisa o reconstruísse.
* Aceleradores de teclado estão de volta em cada tradução, então os menus em russo têm acesso ao teclado novamente.
* Grandes documentos CHM agora abrem até sete vezes mais rápido.
* Documentos abertos agora são registrados no Windows, então aparecem na lista de saltos da barra de tarefas e na lista recente do menu Iniciar.
* Opções foi renomeado para Configurações, correspondendo aos aplicativos móveis e, no macOS, à convenção da plataforma.
* Paperback agora lembra sua posição, tamanho e estado maximizado da janela entre execuções.
* Formas plurais agora são traduzidas, então mensagens que contam coisas são lidas corretamente em idiomas que precisam de mais de uma forma.
* Selecionar o ncc.html de um livro DAISY agora abre o livro de áudio completo em vez de apenas seu texto.
* Os nomes de ação do diálogo Personalizar Atalhos de Teclado agora podem ser traduzidos.
* O título do documento agora vem em primeiro lugar na barra de título, então livros abertos podem ser distinguidos na barra de tarefas e Alt+Tab.
* O diálogo de atualização agora é traduzido.

### Versão 0.9.0

#### Adicionado

##### Geral
* Uma ferramenta CLI, chamada pb, para converter rapidamente qualquer um dos formatos suportados do Paperback para HTML, Markdown ou texto simples.
* Uma opção para recarregar documentos que foram modificados por outros programas no disco.
* Uma opção Ver Origem para abrir a origem de um documento em uma nova aba, útil para editar Markdown por exemplo.
* O texto do documento agora é paginado, o que significa que você pode carregar livros com dezenas de milhões de palavras em apenas alguns segundos. Relate qualquer estranheza encontrada com isso.

##### Suporte de Plataforma
* Suporte a Windows ARM64!
* Suporte nativo a macOS!
* Um alternador de tela cheia.

##### Diálogo Todos os Documentos
* Um botão de localizar para localizar livros desaparecidos que mudaram de caminho.
* Um filtro de status e barra de status, para que você possa filtrar por status do documento e ver quantos documentos estão sendo exibidos e selecionados.
* O atalho `Ctrl+Shift+A` para desselecionar todos os documentos.

##### Opções e Legibilidade
* Uma aba de legibilidade, com as seguintes opções:
    * Quebra de linha (movida de geral);
    * Renderizar tabelas inline (nova nesta versão, veja abaixo);
    * Fonte;
    * Cor de fundo;
    * Espaçamento de linhas;
    * Espaçamento de parágrafos;
    * Espaçamento de letras;
    * Alinhamento de texto.
* Um item de menu de quebra de linha e atalho subsequente.
* Um alternador para determinar como você deseja que as tabelas sejam exibidas e unificado como as tabelas são exibidas em todos os documentos.

##### Navegação
* Suporte para navegação por contêiner.
* Uma opção para mover automaticamente o cursor para o início da linha ao navegar entre linhas, semelhante ao modo de navegação em leitores de tela.
* O atalho de teclado equals para anunciar sua porcentagem atual através de um documento.

##### Marcadores
* Marcadores temporários: você pode ter um por documento, e eles persistem. Use barra para definir um e contrabarra para pular para ele.

##### Contagem de Palavras
* Tempo de leitura estimado no diálogo de contagem de palavras, bem como a capacidade de definir sua velocidade de leitura para tornar essa métrica realmente útil.
* Se uma seleção estiver ativa ao abrir o diálogo de contagem de palavras, quantas palavras você selecionou será mostrado.

##### Atalhos de Teclado
* A capacidade de personalizar cada atalho de teclado no aplicativo através de um diálogo simples.
* Um atalho de teclado configurável para restaurar Paperback da bandeja do sistema.

##### Idiomas
* Holandês, finlandês e polonês.

##### Exportar
* Expandido o item do menu de exportação para permitir exportar para HTML e Markdown, além de texto simples.

##### Atualizador
* Um botão de cancelar para o diálogo de atualização em progresso.
* O atualizador agora valida se o arquivo baixado não foi alterado.

##### Exibição na Web
* A webview agora é aberta em sua posição de leitura atual.

##### Livros DAISY
* Suporte para livros DAISY 2.0.
* Suporte para reprodução de áudio DAISY 2.02.

##### Livros de Áudio
* A capacidade de reproduzir livros de áudio, atualmente suportando DAISY de áudio (incluindo DAISY de áudio + texto) e zips de arquivos de áudio.
* Atalhos de teclado e itens de menu para reproduzir/pausar narração, buscar para frente e para trás e ajustar a quantidade de busca.
* Opções para sincronizar o cursor de leitura com reprodução de áudio, definir a quantidade de busca de áudio e escolher se buscar além do final de um capítulo continua no próximo.

##### Documentos CHM
* Suporte para listas, itens de lista, figuras e imagens.

##### PowerPoint
* Documentos PowerPoint agora suportam tabelas.

#### Corrigido

##### Geral
* Documentos codificados em codificações CJK legadas, como GBK, Big5 e Shift_JIS, agora serão renderizados corretamente em vez de um bando de mojibake.
* "Reabrir último fechado" tentando reabrir o readme fornecido.
* Sua aba selecionada não sendo devidamente focada após reiniciar Paperback.
* Tratamento de Paperback de arquivos em unidades de rede do Windows: pressionar mostrar arquivo na pasta agora foca adequadamente o arquivo no armazenamento de rede, e os caminhos não contêm mais caracteres estranhos.
* Arquivos .paperback não serão mais carregados à força na restauração de documentos; em vez disso, você será solicitado a confirmar quando um for encontrado.
* Abrir pasta contendo agora foca o arquivo fornecido no explorador.
* Abrir o readme agora respeitará seu idioma selecionado.
* A interface do usuário do Paperback agora será dimensionada adequadamente em exibições de alto DPI.
* O menu agora se atualiza corretamente e o foco se move para o controle de texto ao abrir a ajuda no Paperback.
* Alternado para um método muito mais seguro de IPC no Windows.
* O título do documento ativo será lido ao alternar entre abas.
* Reduzido o uso de memória em documentos grandes reduzindo pela metade o tamanho das tabelas de índice internas por caractere.

##### Diálogo Todos os Documentos
* Escape não fechando os diálogos Informações do Documento e Todos os Documentos.
* A barra de título não atualizando após fechar um documento do diálogo todos os documentos.
* Readme.html não será mais adicionado à sua lista todos os documentos ao abrir via Shift+F1.
* Remover documentos do diálogo recentes agora também fechará sua aba ativa.
* Seu filtro de busca agora é preservado após remover um documento.

##### Navegação
* Navegação de página anunciando texto de linha incorreto em algumas situações.
* Ir para Linha, Ir para Página e Ir para Porcentagem colocando seu cursor na posição errada em documentos grandes.
* Localizar e Localizar Próximo não respeitando a janela do documento carregado em documentos grandes.

##### Marcadores
* Sons de marcador/nota agora devem ser reproduzidos adequadamente exclusivamente quando você navega sobre uma palavra contendo um.

##### Legibilidade
* Aplicar quebra de linha o atirando para o início do seu documento.

##### Exibição na Web
* O diálogo de webview não sendo redimensionável e surgindo em um tamanho inicial muito pequeno.
* Imagens agora devem ser exibidas adequadamente na webview incorporada.

##### Atualizador
* O atualizador agora mostra adequadamente o conteúdo de tags de código de markdown nas notas de versão.

##### Livros DAISY
* Livros DAISY mostrando informações incorretas na barra de status.
* Carregando livros DAISY com declarações de codificação falsas.

##### Documentos RTF
* Análise de documentos RTF com caracteres não-latinos neles.
* Grupos RTF `\pict` para que dados de imagem incorporados não vazem mais no texto do documento.

##### Livros Mobi/AZW3
* Âncoras de filepos em livros Mobi dividindo tags HTML e colocando lixo no texto do livro.
* Links em livros Mobi legados.
* Análise de AZW3 dramaticamente melhorada.

##### Documentos Word
* Documentos Word com nomes de estilos específicos de locale não renderizando seus títulos adequadamente.

##### Documentos HTML/XHTML
* Elementos dl, dt e dd não produzindo quebras de linha em documentos XHTML.

##### Documentos PDF
* Paperback agora volta à extração de texto simples para PDFs falsamente marcados.
* Documentos PDF contendo caracteres de controle em seus títulos e/ou marcadores não farão mais o Paperback travar ao abrir.

### Versão 0.8.5
* Suporte de página adicionado aos livros epub.
* Suporte adicionado para documentos Microsoft Office criptografados. Atualmente, Word legado, Word moderno e Powerpoint moderno são suportados, com Powerpoint legado planejado para o futuro.
* Suporte adicionado para documentos Microsoft Word legados!
* Suporte adicionado para apresentações Powerpoint legadas!
* Suporte adicionado para livros mobi e AZW3!
* Suporte adicionado para arquivos PDF marcados!
* Adicionado o atalho ctrl+q para sair do aplicativo.
* Suporte adicionado para livros compactados da Bookshare (tanto DAISY quanto Word)!
* O texto alternativo para imagens incorporadas agora deve ser exibido adequadamente.
* Documentos CHM agora suportam adequadamente navegação de link interno.
* Corrigido ir para página estar incorreto por 1.
* Corrigida a tecla escape não funcionando para fechar o diálogo abrir como.
* Corrigido o menu de contexto do leitor não aparecendo ao clicar com botão direito ou pressionar a tecla Aplicativos.
* Corrigido o documento errado às vezes sendo focado ao abrir documentos da linha de comando.
* PDFs somente com imagem são novamente detectados e alertam você de sua existência.
* Agora é possível navegar por imagens e figuras com g/shift+g e f/shift+f, respectivamente.
* Paperback agora respeitará sua configuração de modo escuro do aplicativo.
* Removido suporte a DAISY XML, pois não é mais necessário.
* Voltado para a navegação de primeira letra nativa Win32 no modo de exibição em árvore do índice.
* O diálogo de carregamento de erro agora mostra mensagens de erro mais detalhadas.
* A webview agora abrirá muito mais rápido e suavemente.

### Versão 0.8.2
* Suporte de página adicionado aos documentos RTF!
* Corrigido um bug onde abrir a webview em epubs contendo links externos os ativaria automaticamente.
* Corrigido um bug onde o analisador RTF não colocaria um espaço entre palavras em casos raros.
* Corrigidos parágrafos sendo divididos em múltiplas linhas curtas em alguns documentos PDF.
* Documentos PDF agora têm suporte básico a navegação de link e título!
* Abas RTF e feeds de linha agora são renderizados exatamente como aparecem no documento.
* Voltado para a biblioteca pdfium comprovada e testada para análise de PDFs, tornando a renderização de PDF muito mais confiável novamente.

### Versão 0.8.1
* Adicionado Ctrl+Shift+T para reabrir o último documento fechado.
* O diálogo Todos os Documentos agora suporta seleção de múltiplos documentos para abrir de uma vez.
* Corrigidos alguns bugs com o analisador RTF.
* Corrigidos caminhos de arquivo contendo caracteres não-ASCII (como š, č, ć, ž bósnios) tornando-se corrompidos ao abrir um arquivo via uma segunda instância do Paperback.
* Corrigido texto PDF sendo lido na ordem errada e espaçamento incorreto ao redor de palavras em maiúsculas.
* Corrigido carregamento lento de documento ao abrir arquivos grandes.
* Corrigida a localização dos botões Sim/Não em diálogos de confirmação.

### Versão 0.8.0
* Adicionadas traduções para japonês, chinês simplificado e vietnamita!
* Adicionado um atualizador automático que agora substituirá sua versão instalada atual do Paperback em vez de apenas baixar a nova versão!
* Adicionado feedback de som opcional para alcançar um marcador ou uma nota, obrigado Andre Louis pelos sons!
* Adicionado suporte para documentos RTF!
* Adicionado suporte para documentos DAISY XML.
* Adicionado suporte para arquivos de Texto Aberto de Documento Plano!
* Adicionado suporte para apresentações de Documento Aberto Plano!
* Adicionado suporte para separadores com s e shift+s.
* Qualquer movimento maior que 300 caracteres agora adicionará automaticamente ao seu histórico de navegação.
* Corrigida restauração da janela Paperback da bandeja do sistema.
* Corrigidos documentos Markdown mostrando texto bruto em vez de HTML renderizado na Exibição na Web.
* Corrigidas tabelas não sendo renderizadas adequadamente em arquivos Markdown.
* PDFs somente com imagem agora avisarão você de sua existência ao tentar carregar um.
* Informações de versão corretamente incorporadas no executável Paperback.
* Divida o diálogo de opções em abas para facilitar o uso e a navegação.
* Alternado para Hayro para análise de PDFs, levando a mais confiabilidade, velocidade e menos DLLs.
* Reescrito todo o aplicativo em Rust. A nova base de código é mais segura, carrega documentos mais rápido e é mais fácil de manter e expandir.
* O menu de contexto do controle de texto agora incluirá ações específicas do leitor em vez de itens genéricos como cortar e colar.

### Versão 0.7.0
* Adicionado suporte a tabelas para documentos baseados em HTML e XHTML! Navegue entre tabelas usando T e Shift+T, e pressione Enter para visualizar uma em uma webview.
* Adicionado um recurso de renderização web básico! Pressione Ctrl+Shift+V para abrir a seção atual de seu documento em um renderizador baseado em web, útil para conteúdo como formatação complexa ou exemplos de código.
* Adicionada uma tradução em russo, obrigado Ruslan Gulmagomedov!
* Adicionado um botão Limpar Tudo ao diálogo Todos os Documentos.
* O verificador de atualização agora exibe notas de versão quando uma nova versão está disponível.
* Corrigida restauração da janela da bandeja do sistema.
* Corrigidas traduções de botão Sim/Não em diálogos de confirmação.
* Corrigido carregamento de configs ao executar como administrador.
* Corrigido tratamento de comentários em documentos XML e HTML.
* Corrigida análise do TOC em livros Epub 2.
* Corrigida navegação para o próximo item com a mesma letra no índice.
* Corrigido o diálogo de busca não se escondendo adequadamente ao usar os botões próximo/anterior.
* Corrigidos TOCs epub ocasionalmente o jogando para o item errado.
* Corrigidos vários problemas de tratamento de espaço em branco em XML, HTML e tags pre.
* Corrigido erro off-by-one na navegação de link.
* Corrigidos alguns livros tendo espaço em branco à direita nas linhas.
* Corrigidos vários problemas de analisador.
* Itens de menu relacionados a marcadores bem como a lista de elementos agora são adequadamente desabilitados quando nenhum documento está aberto.
* Melhorado tratamento de lista em vários formatos de documento.
* Melhorado o fluxo de trabalho de tradução para colaboradores.
* Muitos refatoramentos internos, movendo a maioria da lógica de negócios do aplicativo de C++ para Rust para melhor desempenho e manutenibilidade.

### Versão 0.6.1
* Adicionado suporte a PDF protegido por senha!
* Adicionado um recurso de ir para posição anterior/próxima muito básico. Se você pressionar enter em um link interno e ele mover seu cursor, essa posição será lembrada e poderá ser navegada com setas alt+left/right.
* Adicionada uma lista de elementos! Atualmente, ela mostra apenas uma árvore de todos os títulos em seu documento ou uma lista de links, mas há planos para expandi-la no futuro.
* Adicionada uma opção para iniciar o Paperback no modo maximizado por padrão.
* Corrigidos links em alguns documentos Epub não funcionando adequadamente.
* Corrigida análise do TOC do Epub contendo caminhos relativos.
* Corrigidos alguns documentos epub não mostrando um título ou autor.
* Corrigidos os títulos de alguns capítulos do epub não aparecendo adequadamente no diálogo do TOC.
* Corrigido você não ser capaz de usar a barra de espaço para ativar os botões OK/cancelar no diálogo do TOC.
* Melhorado o tratamento de títulos em documentos Word.
* Você receberá agora feedback falado se a lista de documentos recentes estiver vazia ao tentar abrir o diálogo.

### Versão 0.6.0
* Uma nova opção para mostrar o menu de ir em uma forma muito mais compacta foi adicionada ao diálogo de opções, marcado por padrão.
* Adicionada uma opção para fazer a navegação por elementos estruturais envolver.
* Adicionada uma opção ao menu de ferramentas para abrir a pasta contendo o documento atualmente focado.
* Adicionado um sistema de atualização bastante simples, mas muito eficaz.
* Adicionado um recurso de temporizador de sono básico, acessível com Ctrl+Shift+S.
* Adicionado suporte para análise de livros FB2!
* Adicionado suporte para análise de apresentações OpenDocument!
* Adicionado suporte para análise de arquivos OpenDocument Text!
* Marcadores agora podem ser feitos para marcar uma linha inteira ou apenas marcar um texto especificado. Se você não tiver seleção ativa ao colocar um marcador, o comportamento é como pré-0.6, e marcará a linha inteira. No entanto, se você selecionar algum texto, apenas esse texto será incluído no marcador.
* Marcadores agora podem ter notas de texto opcionais anexadas a eles! Navegue entre marcadores contendo notas com N e Shift+N, ou abra o diálogo de marcadores com todos os marcadores, apenas notas ou apenas não-notas selecionadas com atalhos específicos.
* Marcadores no diálogo de marcadores não terão mais um prefixo irritante "marcador x".
* Livros Epub contendo conteúdo HTML pretendendo ser XML agora serão tratados adequadamente.
* Corrigido carregamento de grandes documentos Markdown.
* Corrigido pressionar espaço no modo de exibição em árvore do índice ativando o botão OK.
* Corrigido tratamento de espaço em branco no início de tags pre em documentos HTML e XHTML.
* Corrigido o campo de texto não retomando o foco às vezes ao retornar à janela do Paperback.
* Corrigido o campo de texto no diálogo ir para porcentagem não atualizando o valor do controle deslizante.
* Corrigida renderização de IDs HTML personalizados em documentos Markdown.
* HTML dentro de blocos de código Markdown agora será renderizado adequadamente.
* Se carregando um livro com parâmetro de linha de comando enquanto uma instância existente do Paperback está em execução, você não receberá mais um erro se carregar seu documento levar mais de 5 segundos.
* Se executando o Paperback como administrador, a configuração agora será adequadamente carregada e salva.
* Agora é possível excluir um marcador diretamente do diálogo de marcadores.
* Agora é possível importar e exportar seus marcadores e posição de leitura de um documento específico. O arquivo gerado é nomeado de acordo com o arquivo com uma extensão .paperback. Se tal arquivo for encontrado no mesmo diretório como um arquivo ao carregá-lo, ele será automaticamente carregado. Caso contrário, você pode importá-los manualmente usando um item no menu de ferramentas.
* Links dentro de documentos agora são totalmente suportados! Use k e shift+k para se mover para frente e para trás através deles, e pressione enter para abrir/ativar um.
* Muitos refatoramentos internos, tornando o aplicativo mais rápido e o binário menor.
* Conteúdo Markdown agora é pré-processado para ser compatível com CommonMark antes da renderização.
* Navegação por listas e seus itens agora é totalmente suportada! Use L e Shift+L para ir por listas em si e I e Shift+I para passar por itens de lista.
* Numpad delete agora funciona para remover documentos da barra de abas além de delete normal.
* Paperback agora pode opcionalmente minimizar para sua bandeja do sistema! Esta opção está desativada por padrão, mas ativá-la fará com que a opção minimizar no menu do sistema coloque Paperback em sua bandeja, podendo ser restaurado clicando no ícone gerado.
* Paperback agora é totalmente traduzível! A lista de idiomas que suporta é atualmente bastante pequena, mas está crescendo constantemente!
* Documentos PPTX agora mostrarão um índice básico, contendo todos os slides.
* O caminho completo para o documento aberto agora será mostrado no diálogo de informações do documento.
* O instalador agora inclui uma opção para visualizar o readme em seu navegador após a instalação.
* A lista de documentos recentes foi dramaticamente expandida! Em vez de simplesmente mostrar os últimos 10 documentos que você abriu, agora mostrará um número personalizável, com o resto dos documentos que você já abriu sendo acessível através de um pequeno diálogo.
* Várias pequenas melhorias nos analisadores em geral, incluindo colocar uma linha em branco entre slides em apresentações PPTX, corrigir o tratamento de nova linha dentro de parágrafos em documentos word e adicionar pontos de marcação a itens de lista.

### Versão 0.5.0
* Adicionado suporte para documentos Microsoft Word!
* Adicionado suporte para apresentações PowerPoint!
* Corrigidos certos itens de menu não sendo desabilitados com nenhum documento aberto.
* Corrigida a orientação do controle deslizante ir para porcentagem.
* Corrigido o índice em livros Epub com caminhos de arquivo codificados em URL e/ou IDs de fragmento.
* Corrigido espaço em branco sendo removido de títulos XHTML de formas estranhas.
* Corrigido tratamento de espaço em branco dentro de tags pre aninhadas em documentos HTML.
* Documentos HTML e Markdown agora suportam o recurso de índice! Quando você carrega um documento HTML/Markdown, Paperback construirá seu próprio índice da estrutura dos títulos em seu documento, e o mostrará a você no diálogo ctrl+t.
* Documentos HTML agora terão o título como definido na tag título, se existir. Caso contrário, eles continuarão a usar o nome do arquivo sem a extensão.
* Alternado de UniversalSpeech para usar uma região ao vivo para relatar fala. Isto significa que nenhuma DLL do leitor de tela é enviada com o programa, e mais leitores de tela serão suportados, como o Microsoft Narrator.
* Alternadas bibliotecas zip para permitir abrir uma gama mais ampla de livros epub.
* O diálogo pedindo se você deseja abrir seu documento como texto simples foi completamente refeito, e agora permite abrir seu documento como texto simples, HTML ou Markdown.
* O diálogo ir para porcentagem agora inclui um campo de texto permitindo que você insira manualmente uma porcentagem para pular para.
* O analisador HTML agora reconhecerá dd, dt e dl como elementos de lista.
* O índice em livros Epub será preservado novamente exatamente.
* O espaço sem quebra unicode agora é considerado ao remover linhas em branco.
* Você não será mais perguntado como deseja abrir um arquivo não reconhecido toda vez que o carrega, apenas na primeira vez.

### Versão 0.4.1
* Adicionado um ícone de menu Iniciar opcional ao instalador.
* O índice agora deve estar mais limpo em alguns casos, por exemplo, se você tiver um item filho e pai com o mesmo texto na mesma posição, agora você verá apenas o item pai.
* Corrigido o índice em certos documentos CHM.
* Corrigido o índice em livros Epub 3 com caminhos absolutos neles.
* Documentos CHM agora devem mostrar seu título como definido no arquivo de metadados.

### Versão 0.4.0
* Adicionado suporte para arquivos CHM!
* Adicionado suporte a marcadores! Você pode ter quantos marcadores quiser em quantos documentos quiser. Você pode pular para frente e para trás através deles com b e shift+b, definir um com control+shift+b e abrir um diálogo para pular para um marcador específico com control+b.
* Adicionado um instalador ao lado do arquivo zip portátil! O instalador instalará Paperback em seu diretório Program Files e configurará automaticamente associações de arquivo para você.
* Arquivos de texto com BOMs agora devem ser decodificados adequadamente e o BOM não será mais exibido no início do texto.
* Adicionadas informações muito mais à barra de status. Agora mostrará sua linha atual, caractere e porcentagem de leitura.
* Comentários HTML, bem como conteúdo de tags de script e estilo, não serão mais mostrados na saída de texto.
* Se passando um caminho relativo para Paperback na linha de comando, agora o resolverá adequadamente.
* Movimento de porcentagem agora é tratado por seu próprio diálogo baseado em controle deslizante, acessível com control+shift+g.
* Documentos sem títulos ou autores conhecidos agora sempre terão um padrão.
* A lógica de salvamento de posição agora é muito mais inteligente e deve apenas escrever no disco quando absolutamente necessário.
* O documento que você tinha focado ao fechar o Paperback agora é lembrado entre reinicializações do aplicativo.
* A entrada nos diálogos ir para linha e ir para página agora deve ser desinfetada mais estritamente.
* Corrigida navegação do índice em livros epub 3 com caminhos relativos em seus manifestos.

### Versão 0.3.0
* Corrigido o índice em livros epub com manifestos codificados em URL.
* Corrigida navegação de título em documentos HTML contendo caracteres Unicode multibyte.
* Corrigido uso alto de CPU em documentos com títulos longos devido a uma regressão no wxWidgets.
* Corrigido carregamento de arquivos de texto UTF-8.
* Corrigidos itens do TOC aninhados em livros Epub colocando seu cursor na posição errada.
* Corrigido travamento na saída do aplicativo em certos casos.
* Adicionada uma caixa de seleção no diálogo de opções para ativar ou desativar quebra de linha!
* Agora é possível doar para o desenvolvimento do Paperback, seja através do novo item de doação no menu de ajuda ou através do link patrocinar este projeto na parte inferior da página principal do repositório do GitHub.
* Documentos Markdown agora sempre terão um título, e Paperback agora deve ser capaz de carregar praticamente qualquer arquivo Markdown.
* Documentos PDF agora sempre terão um título, mesmo que os metadados estejam faltando.
* Alternadas bibliotecas PDF para a usada no Chromium, levando a análise de PDF muito mais confiável em geral.
* Você agora pode ter apenas uma instância do Paperback em execução por vez. Executar paperback.exe com um nome de arquivo enquanto já está em execução abrirá esse documento na instância já em execução.
* Você agora pode pressionar delete em um documento no controle de abas para fechá-lo.

### Versão 0.2.1
* Adicionado o número total de páginas ao rótulo de página no diálogo ir para página.
* Permita tabulação do conteúdo do documento para sua lista de documentos abertos.
* Corrigidos alguns pressionamentos de tecla de título às vezes abrindo documentos recentes se você tivesse o suficiente deles.
* Paperback agora removerá hífens suaves desnecessários da saída de texto.
* Corrigida navegação de título às vezes o colocando no caractere errado.

### Versão 0.2.0
* Adicionado suporte para documentos markdown!
* Adicionado suporte para documentos PDF, incluindo a capacidade de navegar entre páginas!
* Adicionados pressionamentos de tecla para navegar por títulos em conteúdo HTML, incluindo livros epub e documentos markdown. Esses pressionamentos de tecla foram projetados para funcionar semelhante a um leitor de tela.
* Corrigido carregamento de epubs com nomes de arquivo codificados em URL em seus manifestos.
* Corrigido carregamento de livros epub 3 com XHTML incorporado neles.
* Uma mensagem agora é falada se o documento não suportar um índice ou seções, em vez dos itens de menu serem desabilitados.
* Adicionado um menu de documentos recentes! Atualmente armazena seus últimos 10 documentos abertos, e pressionar enter em um os abrirá para leitura.
* Completamente reescrito o diálogo Localizar, tornando-o muito mais simples de usar, enquanto também adiciona um histórico de suas últimas 25 buscas e suporte a expressão regular!
* Documentos abertos anteriormente agora são lembrados entre reinicializações do aplicativo. Isto é configurável através do novo item de opções no menu de ferramentas.
* Adicionado shift+f1 para abrir o readme diretamente no Paperback em si.

### Versão 0.1.0
* Lançamento inicial.
