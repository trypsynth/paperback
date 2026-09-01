<!-- machine-translated from doc/readme.md (source-hash: c5a741eb100e6fbc); please review and edit as needed -->

# Paperback - versão 0.9.2

## Introdução

Paperback é um leitor de ebooks e documentos leve, rápido e acessível para todos, de leitores ocasionais a usuários avançados. É projetado para acessibilidade com leitores de tela, velocidade rápida e uma experiência livre de inchaço.

## Requisitos de Sistema

Paperback atualmente funciona no Windows 10/11 e em todas as versões modernas de ARM macOS. Aplicativos nativos para iOS e Android estão em desenvolvimento ativo, com compilações de teste público planejadas em breve após o lançamento do desktop 0.9.0, antes de um lançamento unificado 1.0 cobrindo todas as quatro plataformas.

## Recursos

* Completamente autossuficiente, não exigindo que nenhum software seja instalado no seu computador para começar a ler.
* Incrivelmente rápido, mesmo em hardware antigo.
* Interface simples com abas, permitindo que você abra quantos documentos quiser lado a lado.
* Salva sua posição exata de leitura em cada documento que você abre.
* Opcionalmente lembra quais documentos você tinha abertos quando fechou o programa e os restaura no próximo lançamento.
* Inclui funcionalidade de navegação semelhante à encontrada no modo de navegação web de muitos leitores de tela para navegar rápida e facilmente pelos documentos.
* Inclui um robusto diálogo de busca, incluindo recursos como histórico e suporte a expressões regulares.
* Pode ser executado completamente de forma portátil ou instalado com associações de arquivo configuradas automaticamente.
* Suporta uma grande variedade de formatos de arquivo comuns.

## Compatibilidade com Leitores de Tela

Paperback funciona bem com todos os leitores de tela principais. Há, no entanto, um problema conhecido para usuários de JAWS.

### JAWS e Exibidores Braille

Se você usar JAWS com um exibidor Braille, pode descobrir que parágrafos longos são truncados ao navegar para frente com as teclas de navegação do seu exibidor. O comando de leitura do parágrafo atual também é afetado. Este é um bug no tratamento de JAWS do controle de texto RICHEDIT50W, não algo no Paperback em si, e um que levou bastante tempo para surgir uma correção, dado o entusiasmo da Vispero em responder a problemas com software de código aberto.

A solução alternativa, eventualmente surgida através do grupo de discussão de JAWS após meses de espera, é editar `paperback.jcf` e definir "Braille Presentation and Panning" para "Always use DOM if available". Você também vai querer ativar "Pan Text by Paragraph", caso contrário seu exibidor permanecerá no parágrafo ativo em vez de avançar. Com ambas as configurações em vigor, a navegação deve funcionar corretamente.

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
* Arquivos de texto simples e log (`.txt`, `.log`)

## Atalhos de teclado

Paperback foi projetado para uso com prioridade no teclado. Aqui estão os atalhos atuais.

Os atalhos abaixo são para Windows. Onde macOS diferencia, o equivalente é anotado entre parênteses — principalmente porque Ctrl+G, Ctrl+W e Alt+Left/Right já são usados por outras convenções do sistema ou do aplicativo nessa plataforma.

### Menu Arquivo

* `Ctrl+O`: Abrir um documento.
* `Ctrl+F4` (macOS: `Cmd+W`): Fechar o documento atual.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Fechar todos os documentos abertos.
* `Ctrl+Shift+T`: Reabrir o último documento fechado.
* `Ctrl+R`: Mostrar a caixa de diálogo "Todos os Documentos" (de Documentos Recentes).
* `Ctrl+Q`: Sair (somente Windows; no macOS isso está no menu do aplicativo).

### Menu Ir

* `Ctrl+F`: Mostrar a caixa de diálogo Localizar.
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

* `Ctrl+W` (macOS: `RawCtrl+W`, ou seja, a tecla Control física em vez de Cmd): Mostrar contagem de palavras para o documento atual.
* `Ctrl+I`: Mostrar informações do documento.
* `Ctrl+T`: Mostrar sumário.
* `F7`: Mostrar lista de elementos.
* `Ctrl+Shift+C`: Abrir pasta contendo.
* `Ctrl+Shift+V`: Abrir conteúdo atual em Visualização Web.
* `Ctrl+U`: Visualizar fonte do documento em uma nova aba.
* `Ctrl+Shift+E`: Exportar dados do documento (`.paperback`).
* `Ctrl+Shift+I`: Importar dados do documento (`.paperback`).
* `Ctrl+E`: Exportar o documento atual para texto plano.
* `Ctrl+Shift+B`: Alternar marcador na seleção/cursor atual.
* `Ctrl+Shift+N`: Adicionar ou editar nota de marcador na seleção/cursor atual.
* `Ctrl+Alt+W`: Alternar quebra de palavra.
* `Ctrl+Space`: Reproduzir/pausar narração de áudio.
* `'`: Avançar narração de áudio.
* `;`: Retroceder narração de áudio.
* `Ctrl+'`: Aumentar a quantidade de busca de áudio.
* `Ctrl+;`: Diminuir a quantidade de busca de áudio.
* `F11` (macOS: `RawCtrl+Ctrl+F`, ou seja, Control+Command+F): Alternar tela cheia.
* `Ctrl+,`: Abrir opções (macOS: Preferências, no menu do aplicativo).
* `Ctrl+Shift+S`: Alternar temporizador de sono.

### Menu Ajuda

* `Ctrl+F1`: Mostrar caixa de diálogo Sobre.
* `F1`: Visualizar ajuda no seu navegador padrão.
* `Shift+F1`: Visualizar ajuda no Paperback.
* `Ctrl+Shift+U`: Verificar atualizações.
* `Ctrl+D`: Abrir a página de doações no seu navegador padrão.

### Teclas adicionais de visualização de documentos

* `Delete` / `Numpad Delete` no controle de aba: Fechar a aba do documento selecionado.
* `Enter` ou `Space` no texto do documento: Ativar link no cursor ou abrir visualização de tabela quando em um marcador de tabela.
* `Shift+F10` ou a tecla Menu/Aplicativo no texto do documento: Abrir o menu de contexto.

## Idiomas suportados

Paperback é traduzido para muitos idiomas diferentes, com mais sendo adicionados o tempo todo. Uma lista completa segue abaixo.

Para saber como contribuir, leia nosso [Guia de Tradução](translating.md).

* Bósnio
* Tcheco
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
* Aryan Choudhary: contribuidor principal.

### Doações
As seguintes pessoas fizeram doações de algum valor para o desenvolvimento do Paperback. Se você fizer uma doação, seu nome não será adicionado automaticamente aqui; eu só adiciono pessoas que desejam que suas doações sejam públicas.

Nota: Considero um patrocínio público do GitHub como motivo para inclusão automática nesta lista.

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

## Registro de mudanças

### Versão 0.9.2
* Audiobooks não fazem mais o leitor de tela ler uma sequência de espaços quando você foca o campo de texto.
* Audiobooks agora nomeiam o arquivo conforme você passa por eles seção por seção.
* Audiobooks agora relatam seu comprimento real, em vez de alegar que cada arquivo dura 24 horas.
* Fechar a Visualização Web com Escape não dispara mais um alerta de depuração depois que você segue um link dentro dela.
* Copiar após Selecionar Tudo agora oferece o documento inteiro, em vez de apenas a parte carregada no momento.
* Encontrar agora vai direto para a linha encontrada, em vez de forçá-lo a ouvir o leitor de tela ler a janela novamente enquanto o foco retorna ao livro.
* EPUBs corrigidos que carregavam um bloco ZIP64 solto recusavam abrir com "Invalid local file header".
* Documentos longos corrigidos que voltavam para o início enquanto um leitor de tela lia continuamente através deles.
* Links na WebView agora o levam para a seção que apontam, em vez de falharem com "File not found".
* O anúncio automático "Document reloaded" não corta mais seu leitor de tela no meio da frase, aguardando que ele termine o que estava dizendo.
* A aba Geral do diálogo Configurações agora percorre suas opções na ordem em que aparecem na tela, com o canal de atualização logo após a opção verificar atualizações.
* Windows agora sempre mostrará "Paperback" no menu Abrir com, em vez da tagline completa do programa.
* Contagem de palavras e Informações do documento agora mostram quantos arquivos um audiobook contém e quanto tempo leva no total.

### Versão 0.9.1
* Sons de marcadores e notas agora são reproduzidos no macOS.
* Livros DAISY agora reproduzem seu áudio no macOS, em vez de abrir e rastrear sua linha do tempo em silêncio.
* Aspas curvas, travessões em e caracteres semelhantes corrigidos desaparecendo de documentos RTF, juntando as palavras ao redor deles.
* Imagens RTF corrigidas vazando seus dados brutos para o documento como texto embaralhado.
* Submenu de Documentos Recentes corrigido mantendo entradas obsoletas até algo mais acontecer para reconstruí-lo.
* Aceleradores de teclado estão de volta em cada tradução, então os menus em russo têm acesso ao teclado novamente.
* Grandes documentos CHM agora abrem até sete vezes mais rápido.
* Documentos abertos agora são registrados no Windows, então aparecem na lista de saltos da barra de tarefas e na lista recente do menu Iniciar.
* Opções foi renomeado para Configurações, combinando com os aplicativos móveis e, no macOS, a convenção da plataforma.
* Paperback agora lembra sua posição da janela, tamanho e estado maximizado entre execuções.
* Formas plurais agora são traduzidas, então mensagens que contam coisas são lidas corretamente em idiomas que precisam de mais de uma forma.
* Selecionar o ncc.html de um livro DAISY agora abre o audiobook completo em vez de apenas seu texto.
* Os nomes de ação do diálogo Personalizar Atalhos de Teclado agora podem ser traduzidos.
* O título do documento agora vem em primeiro lugar na barra de título, então livros abertos podem ser diferenciados na barra de tarefas e Alt+Tab.
* O diálogo de atualização agora é traduzido.

### Versão 0.9.0

#### Adicionado

##### Geral
* Uma ferramenta de CLI, chamada `pb`, para converter rapidamente qualquer um dos formatos suportados do Paperback para HTML, Markdown ou texto simples.
* Uma opção para recarregar documentos que foram modificados por outros programas no disco.
* Uma opção Ver Fonte para abrir a fonte de um documento em uma nova aba, útil para editar Markdown por exemplo.
* O texto do documento agora é paginado, o que significa que você pode carregar livros com dezenas de milhões de palavras em apenas alguns segundos. Por favor, reporte qualquer estranheza encontrada com isso.

##### Suporte de Plataforma
* Suporte para Windows ARM64!
* Suporte nativo para macOS!
* Um alternador de tela cheia.

##### Diálogo Todos os Documentos
* Um botão de localizar para localizar livros perdidos que apenas mudaram seu caminho.
* Um filtro de status e barra de status, para que você possa filtrar por status do documento e ver quantos documentos são mostrados e selecionados.
* O atalho `Ctrl+Shift+A` para desselecionar todos os documentos.

##### Opções e Legibilidade
* Uma aba de legibilidade, com as seguintes opções:
    * Quebra de linha (movida de geral);
    * Renderizar tabelas em linha (nova nesta versão, veja abaixo);
    * Fonte;
    * Cor de fundo;
    * Espaçamento de linha;
    * Espaçamento de parágrafo;
    * Espaçamento de letra;
    * Alinhamento de texto.
* Um item do menu de quebra de linha e tecla de atalho subsequente.
* Um alternador para determinar como você quer que as tabelas sejam exibidas e unificou como as tabelas são exibidas em todos os documentos.

##### Navegação
* Suporte para navegação por contêiner.
* Uma opção para mover automaticamente o cursor para o início da linha ao navegar entre linhas, similar ao modo de navegação em leitores de tela.
* O atalho de teclado igual para anunciar sua porcentagem atual através de um documento.

##### Marcadores
* Marcadores temporários: você pode ter um por documento, e eles persistem. Use barra para definir um e barra invertida para pular para ele.

##### Contagem de Palavras
* Tempo de leitura estimado no diálogo de contagem de palavras, bem como a capacidade de definir sua velocidade de leitura para tornar essa métrica realmente útil.
* Se uma seleção estiver ativa quando você abrir o diálogo de contagem de palavras, quantas palavras você selecionou agora será mostrado.

##### Atalhos de Teclado
* A capacidade de personalizar cada atalho de teclado no aplicativo através de um diálogo simples.
* Um atalho de teclado configurável para restaurar Paperback da bandeja do sistema.

##### Idiomas
* Holandês, finlandês e polonês.

##### Exportar
* Menu de exportação expandido para permitir exportação para HTML e Markdown, além de texto simples.

##### Atualizador
* Um botão de cancelar para o diálogo de atualização em andamento.
* O atualizador agora valida que o arquivo baixado não foi adulterado.

##### Visualização Web
* A visualização web agora é aberta em sua posição de leitura atual.

##### Livros DAISY
* Suporte para livros DAISY 2.0.
* Suporte para reprodução de áudio DAISY 2.02.

##### Audiobooks
* A capacidade de reproduzir audiobooks, suportando atualmente DAISY de áudio (incluindo DAISY de áudio + texto) e zips de arquivos de áudio.
* Atalhos de teclado e itens de menu para reproduzir/pausar narração, buscar para frente e para trás e ajustar a quantidade de busca.
* Opções para sincronizar o cursor de leitura com a reprodução de áudio, definir a quantidade de busca de áudio e escolher se buscar para o final de um capítulo continua para o próximo.

##### Documentos CHM
* Suporte para listas, itens de lista, figuras e imagens.

##### PowerPoint
* Documentos PowerPoint agora suportam tabelas.

#### Corrigido

##### Geral
* Documentos codificados em codificações CJK legadas, como GBK, Big5 e Shift_JIS, agora serão renderizados adequadamente em vez de como um monte de mojibake.
* "Reopen last closed" tentando reabrir o readme fornecido.
* Sua aba selecionada não ficando devidamente focada após reiniciar Paperback.
* Manipulação de arquivos do Paperback em unidades de rede do Windows: pressionar mostrar arquivo em pasta agora foca adequadamente o arquivo no armazenamento de rede, e os caminhos não contêm mais caracteres estranhos.
* Arquivos .paperback não serão mais carregados forçosamente na restauração de documentos; em vez disso, você será solicitado a confirmar quando um for encontrado.
* Abrir pasta contendo agora foca o arquivo fornecido no explorador.
* Abrir o readme agora respeitará seu idioma selecionado.
* A interface do usuário do Paperback agora será dimensionada adequadamente em exibições de alta DPI.
* O menu agora será atualizado adequadamente, e o foco se moverá para o controle de texto ao abrir ajuda no Paperback.
* Alternado para um método muito mais seguro de IPC no Windows.
* O título do documento ativo agora será lido ao alternar entre abas.
* Uso de memória reduzido em documentos grandes pela metade do tamanho das tabelas de índice interno por caractere.

##### Diálogo Todos os Documentos
* Escape não fechando os diálogos Informações do Documento e Todos os Documentos.
* A barra de título não atualiza após fechar um documento do diálogo todos os documentos.
* Readme.html não será mais adicionado à sua lista de todos os documentos quando aberto via `Shift+F1`.
* Remover documentos do diálogo recentes agora também fechará sua aba ativa.
* Seu filtro de pesquisa agora é preservado após remover um documento.

##### Navegação
* Navegação por página anunciando texto de linha incorreto em algumas situações.
* Ir para linha, Ir para página e Ir para porcentagem colocando seu cursor na posição errada em documentos grandes.
* Encontrar e Encontrar próximo não respeitando a janela de documento carregada em documentos grandes.

##### Marcadores
* Sons de marcador/nota agora devem ser reproduzidos adequadamente exclusivamente quando você navega sobre uma palavra contendo um.

##### Legibilidade
* Aplicar quebra de linha atirando você para o início do seu documento.

##### Visualização Web
* O diálogo de visualização web não sendo redimensionável e aparecendo em um tamanho inicial muito pequeno.
* As imagens agora devem aparecer corretamente na visualização web incorporada.

##### Atualizador
* O atualizador agora mostra adequadamente o conteúdo das tags de código markdown nas notas de versão.

##### Livros DAISY
* Livros DAISY mostrando informações incorretas na barra de status.
* Carregamento de livros DAISY com declarações de codificação falsas.

##### Documentos RTF
* Análise de documentos RTF com caracteres não-latinos neles.
* Grupos RTF `\pict` para que dados de imagem incorporados não vazem mais para o texto do documento.

##### Livros Mobi/AZW3
* Âncoras filepos em livros Mobi dividindo tags HTML e colocando lixo no texto do livro.
* Links em livros Mobi legados.
* Análise AZW3 bastante melhorada.

##### Documentos Word
* Documentos Word com nomes de estilo específicos da localidade não renderizando seus títulos corretamente.

##### Documentos HTML/XHTML
* Elementos dl, dt e dd não produzindo quebras de linha em documentos XHTML.

##### Documentos PDF
* Paperback agora retorna para extração de texto simples para PDFs falsamente marcados.
* Documentos PDF contendo caracteres de controle em seus títulos e/ou marcadores não causarão mais crash no Paperback ao abrir.

### Versão 0.8.5
* Suporte a página adicionado a livros epub.
* Adicionado suporte para documentos do Microsoft Office criptografados. Atualmente, Word legado, Word moderno e Powerpoint moderno são suportados, com Powerpoint legado planejado para o futuro.
* Adicionado suporte para documentos do Microsoft Word legados!
* Adicionado suporte para apresentações Powerpoint legadas!
* Adicionado suporte para livros mobi e AZW3!
* Adicionado suporte para arquivos PDF marcados!
* Adicionado o atalho `ctrl+q` para sair do aplicativo.
* Adicionado suporte para livros compactados do Bookshare (DAISY e Word)!
* O texto alternativo para imagens incorporadas agora deve ser mostrado adequadamente.
* Documentos CHM agora suportam adequadamente navegação de link interno.
* Ir para página corrigido sendo desativado por 1.
* Tecla de escape corrigida não funcionando para fechar o diálogo abrir como.
* Menu de contexto do leitor corrigido não aparecendo ao clicar com o botão direito ou a tecla Aplicativos.
* Documento errado às vezes focado ao abrir documentos da linha de comando.
* PDFs somente de imagem são novamente detectados e o alertam de sua existência.
* Agora é possível navegar através de imagens e figuras com `g`/`shift+g` e `f`/`shift+f`, respectivamente.
* Paperback agora respeitará sua configuração de modo escuro do aplicativo.
* Suporte DAISY XML removido, pois não é mais necessário.
* Alternado de volta para a navegação de primeira letra nativa do Win32 na árvore de índice de conteúdo.
* O diálogo de carregamento de erro agora mostra mensagens de erro mais detalhadas.
* A visualização web agora será aberta muito mais rápido e suavemente.

### Versão 0.8.2
* Suporte a página adicionado a documentos RTF!
* Correção de um bug onde abrir a visualização web em epubs contendo links externos os ativaria automaticamente.
* Correção de um bug onde o analisador RTF não colocaria um espaço entre palavras em casos raros.
* Parágrafos sendo divididos em múltiplas linhas curtas em alguns documentos PDF.
* Documentos PDF agora têm suporte básico de navegação de link e título!
* Abas e alimentações de linha RTF agora são renderizadas exatamente como aparecem no documento.
* Alternado de volta para a biblioteca pdfium comprovada para análise de PDFs, tornando a renderização de PDF muito mais confiável novamente.

### Versão 0.8.1
* Adicionado `Ctrl+Shift+T` para reabrir o último documento fechado.
* O diálogo Todos os Documentos agora suporta seleção de múltiplos documentos para abrir de uma vez.
* Corrigidos alguns bugs com o analisador RTF.
* Caminhos de arquivo corrigidos contendo caracteres não-ASCII (como bósnio š, č, ć, ž) ficando corrompidos ao abrir um arquivo por meio de uma segunda instância do Paperback.
* Texto PDF corrigido sendo lido na ordem errada e espaçamento incorreto ao redor de palavras capitalizadas.
* Carregamento de documento lento corrigido ao abrir arquivos grandes.
* Localização dos botões Sim/Não corrigida em diálogos de confirmação.

### Versão 0.8.0
* Adicionadas traduções para japonês, chinês simplificado e vietnamita!
* Adicionado um atualizador automático que agora substituirá sua versão instalada atual do Paperback em vez de apenas baixar a nova versão!
* Adicionado feedback de som opcional para atingir um marcador ou uma nota, obrigado Andre Louis pelos sons!
* Adicionado suporte a documento RTF!
* Adicionado suporte para documentos DAISY XML.
* Adicionado suporte para arquivos de Texto de Documento Aberto Plano!
* Adicionado suporte para apresentações de Documento Aberto Plano!
* Adicionado suporte para separadores com `s` e `shift+s`.
* Qualquer movimento maior que 300 caracteres agora adicionará automaticamente ao seu histórico de navegação.
* Restauração do Paperback da bandeja do sistema corrigida.
* Documentos Markdown corrigidos mostrando texto bruto em vez de HTML renderizado na Visualização Web.
* Tabelas não renderizando corretamente em arquivos Markdown.
* PDFs somente de imagem agora o avisarão de sua existência quando você tentar carregar um.
* Incorporar adequadamente informações de versão no executável do Paperback.
* Dividir o diálogo de opções em abas para facilitar seu uso e navegação.
* Alternado para Hayro para análise de PDFs, levando a mais confiabilidade, velocidade e menos DLLs.
* Reescreveu todo o aplicativo em Rust. A nova base de código é mais segura, carrega documentos mais rápido e é mais fácil de manter e expandir.
* O menu de contexto do controle de texto agora incluirá ações específicas do leitor em vez de itens genéricos como cortar e colar.

### Versão 0.7.0
* Adicionado suporte a tabelas para documentos baseados em HTML e XHTML! Navegue entre tabelas usando `T` e `Shift+T`, e pressione `Enter` para visualizar uma em uma visualização web.
* Adicionado um recurso básico de renderização web! Pressione `Ctrl+Shift+V` para abrir a seção atual do seu documento em um renderizador web, útil para conteúdo como formatação complexa ou exemplos de código.
* Adicionada uma tradução em russo, obrigado Ruslan Gulmagomedov!
* Adicionado um botão Limpar tudo ao diálogo Todos os Documentos.
* O verificador de atualização agora exibe notas de versão quando uma nova versão está disponível.
* Restauração da janela a partir da bandeja do sistema corrigida.
* Tradução de botões Sim/Não corrigida em diálogos de confirmação.
* Carregamento de configs corrigido ao executar como administrador.
* Tratamento de comentários corrigido em documentos XML e HTML.
* Análise de TOC corrigida em livros Epub 2.
* Navegação para o próximo item com a mesma letra na tabela de conteúdo corrigida.
* Diálogo de encontrar não se ocultando adequadamente ao usar os botões próximo/anterior.
* TOCs de epub ocasionalmente o jogando para o item errado.
* Vários problemas de tratamento de espaço em branco em XML, HTML e pré-tags.
* Erro desativado por um em navegação de link.
* Alguns livros com espaço em branco à direita em suas linhas.
* Vários problemas do analisador.
* Itens de menu relacionados a marcadores, bem como a lista de elementos, agora estão adequadamente desabilitados quando nenhum documento está aberto.
* Tratamento de lista melhorado em vários formatos de documento.
* Fluxo de trabalho de tradução melhorado para colaboradores.
* Muitas refatorações internas, movendo a maioria da lógica comercial do aplicativo de C++ para Rust para melhor desempenho e manutenibilidade.

### Versão 0.6.1
* Adicionado suporte a PDF protegido por senha!
* Adicionado um recurso muito básico de ir para posição anterior/próxima. Se você pressionar enter em um link interno e ele mover seu cursor, essa posição agora será lembrada e poderá ser navegada com as teclas `alt+left`/`right`.
* Adicionada uma lista de elementos! Atualmente mostra apenas uma árvore de todos os títulos do seu documento ou uma lista de links, mas há planos para expandir no futuro.
* Adicionada uma opção para iniciar Paperback no modo maximizado por padrão.
* Links corrigidos em alguns documentos Epub não funcionando adequadamente.
* Análise de Epub TOCs corrigida contendo caminhos relativos.
* Alguns documentos epub não mostrando título ou autor.
* Títulos de alguns capítulos de epub não aparecendo adequadamente no diálogo TOC.
* Você agora pode usar a barra de espaço para ativar os botões OK/cancelar no diálogo TOC.
* Tratamento de títulos melhorado em documentos Word.
* Você agora obterá feedback falado se a lista de documentos recentes estiver vazia quando tentar abrir o diálogo.

### Versão 0.6.0
* Uma nova opção para mostrar o menu ir em um formulário muito mais compacto foi adicionada ao diálogo de opções, marcada por padrão.
* Adicionada uma opção para fazer navegação por elementos estruturais envolver.
* Adicionada uma opção ao menu ferramentas para abrir a pasta contendo o documento atualmente focado.
* Adicionado um sistema de atualização bastante simples, mas muito eficaz.
* Adicionado um recurso básico de temporizador de sono, acessível com `Ctrl+Shift+S`.
* Adicionado suporte para análise de ebooks FB2!
* Adicionado suporte para análise de apresentações OpenDocument!
* Adicionado suporte para análise de arquivos OpenDocument Text!
* Marcadores agora podem ser feitos para marcar uma linha inteira ou apenas para marcar algum texto especificado. Se você não tiver seleção ativa ao colocar um marcador, o comportamento é como antes de 0.6, e marcará toda a linha. No entanto, se você selecionar algum texto, apenas esse texto será incluído no marcador.
* Marcadores agora podem ter notas de texto opcionais anexadas a eles! Navegue entre marcadores contendo notas com `N` e `Shift+N`, ou abra o diálogo de marcadores com todos os marcadores, apenas notas ou apenas não-notas selecionados com teclas de atalho específicas.
* Marcadores no diálogo de marcadores não terão mais um prefixo irritante "bookmark x".
* Livros Epub contendo conteúdo HTML fingindo ser XML agora serão manipulados corretamente.
* Carregamento de grandes documentos Markdown corrigido.
* Pressionar espaço na árvore de conteúdo da tabela de conteúdo corrigido ativando o botão OK.
* Tratamento de espaço em branco corrigido no início de tags pré em documentos HTML e XHTML.
* Controle de texto não recuperando o foco às vezes ao retornar à janela do Paperback.
* Campo de texto corrigido no diálogo ir para porcentagem não atualizando o valor do controle deslizante.
* Renderização de IDs HTML personalizados corrigida em documentos Markdown.
* HTML dentro de blocos de código Markdown agora será renderizado adequadamente.
* Se carregar um livro com um parâmetro de linha de comando enquanto uma instância existente do Paperback está em execução, você não terá mais um erro se o carregamento do seu documento levar mais de 5 segundos.
* Se executar Paperback como administrador, a configuração agora será carregada e salva corretamente.
* Agora é possível deletar um marcador diretamente de dentro do diálogo de marcadores.
* Agora é possível importar e exportar seus marcadores e posição de leitura para um documento particular. O arquivo gerado é nomeado após o arquivo com uma extensão .paperback. Se tal arquivo for encontrado no mesmo diretório de um arquivo ao carregá-lo, será carregado automaticamente. Caso contrário, você pode importá-los manualmente usando um item no menu ferramentas.
* Links dentro de documentos agora são totalmente suportados! Use `k` e `shift+k` para se mover para frente e para trás através deles, e pressione enter para abrir/ativar um.
* Muitas refatorações internas, tornando o aplicativo mais rápido e o binário menor.
* Conteúdo Markdown agora é pré-processado para estar em conformidade com CommonMark antes de renderizar.
* Navegação por listas e seus itens agora é totalmente suportada! Use `L` e `Shift+L` para ir por listas em si, e `I` e `Shift+I` para ir através de itens de lista.
* Excluir do Numpad agora funciona para remover documentos da barra de abas além de excluir normal.
* Paperback agora pode opcionalmente minimizar para sua bandeja do sistema! Esta opção está desativada por padrão, mas ligá-la fará com que a opção minimizar no menu do sistema coloque Paperback em sua bandeja, podendo ser restaurado clicando no ícone gerado.
* Paperback agora é totalmente traduzível! A lista de idiomas que suporta é atualmente bastante pequena, mas está em constante crescimento!
* Paperback agora tem um site oficial, em [paperback.dev](https://paperback.dev)!
* Documentos PPTX agora mostram uma tabela de conteúdo básica, contendo todos os slides.
* O caminho completo para o documento aberto agora será mostrado no diálogo de informações do documento.
* O instalador agora inclui uma opção para visualizar o readme no seu navegador após a instalação.
* A lista de documentos recentes foi dramaticamente expandida! Em vez de simplesmente mostrar-lhe os últimos 10 documentos que abriu, agora mostrará um número personalizável, com o resto dos documentos que já abriu sendo acessível através de um pequeno diálogo.
* Várias pequenas melhorias nos analisadores em toda a placa, incluindo colocar uma linha em branco entre slides em apresentações PPTX, corrigindo o tratamento de nova linha dentro de parágrafos em documentos Word e adicionando pontos de bala a itens de lista.

### Versão 0.5.0
* Adicionado suporte a documentos do Microsoft Word!
* Adicionado suporte para apresentações PowerPoint!
* Itens de menu corrigidos não sendo desabilitados sem documentos abertos.
* Orientação do controle deslizante ir para porcentagem corrigida.
* Tabela de conteúdo corrigida em livros Epub com caminhos de arquivo codificados em URL e/ou IDs de fragmento.
* Espaço em branco corrigido sendo retirado de títulos XHTML de forma estranha.
* Tratamento de espaço em branco corrigido dentro de tags pré aninhadas em documentos HTML.
* Documentos baseados em HTML e Markdown agora suportam o recurso de tabela de conteúdo! Quando você carrega um documento HTML/Markdown, Paperback construirá sua própria tabela de conteúdo a partir da estrutura dos títulos no seu documento, e a mostrará no diálogo `ctrl+t`.
* Documentos HTML agora terão o título conforme definido na tag de título, se existir. Caso contrário, continuarão a usar o nome do arquivo sem a extensão.
* Alternado de UniversalSpeech para usar uma região ativa para relatar fala. Isso significa que nenhuma DLL do leitor de tela é enviada junto com o programa, e mais leitores de tela agora serão suportados, como Microsoft Narrator.
* Bibliotecas zip alternadas para permitir abertura de uma gama mais ampla de livros epub.
* O diálogo perguntando se você quer abrir seu documento como texto simples foi completamente refeito, e agora permite abrir seu documento como texto simples, HTML ou Markdown.
* O diálogo ir para porcentagem agora inclui um campo de texto permitindo entrada manual de uma porcentagem para pular para.
* O analisador HTML agora reconhecerá dd, dt e dl como elementos de lista.
* A tabela de conteúdo em livros Epub será mais uma vez preservada exatamente.
* O espaço não quebrável unicode agora é considerado ao retirar linhas em branco.
* Você não será mais solicitado como deseja abrir um arquivo não reconhecido toda vez que o carregar, apenas a primeira vez.

### Versão 0.4.1
* Adicionado um ícone do menu iniciar opcional ao instalador.
* A tabela de conteúdo agora deve estar mais limpa em alguns casos, por exemplo, se você tiver um item filho e pai com o mesmo texto na mesma posição, verá apenas o item pai.
* Tabela de conteúdo corrigida em certos documentos CHM.
* Tabela de conteúdo corrigida em livros Epub 3 com caminhos absolutos neles.
* Documentos CHM agora devem mostrar seu título conforme definido no arquivo de metadados.

### Versão 0.4.0
* Adicionado suporte a arquivo CHM!
* Adicionado suporte a marcadores! Você pode ter quantos marcadores quiser em quantos documentos quiser. Você pode pular para frente e para trás através deles com `b` e `shift+b`, definir um com `control+shift+b` e abrir um diálogo para pular para um marcador específico com `control+b`.
* Adicionado um instalador junto ao arquivo zip portátil! O instalador instalará Paperback em seu diretório Arquivos de Programas e configurará automaticamente associações de arquivo para você.
* Arquivos de texto com BOMs agora devem ser decodificados adequadamente, e o BOM não será mais exibido no início do texto.
* Adicionadas informações muito mais completas à barra de status. Agora mostrará sua linha atual, caractere e porcentagem de leitura.
* Comentários HTML, bem como o conteúdo de tags de script e estilo, não serão mais mostrados na saída de texto.
* Se passar um caminho relativo para Paperback na linha de comando, agora será resolvido corretamente.
* Movimento de porcentagem agora é manipulado por seu próprio diálogo baseado em controle deslizante, acessível com `control+shift+g`.
* Documentos sem títulos ou autores conhecidos agora sempre terão um padrão.
* A lógica de economia de posição agora é muito mais inteligente e deve escrever no disco apenas quando absolutamente necessário.
* O documento que você tinha focado quando fechou Paperback agora é lembrado entre reinicializações do aplicativo.
* Entrada nos diálogos ir para linha e ir para página agora deve ser sanitizada mais estritamente.
* Navegação de tabela de conteúdo corrigida em livros epub 3 com caminhos relativos em seus manifestos.

### Versão 0.3.0
* Tabela de conteúdo corrigida em livros epub com manifestos codificados em URL.
* Navegação de título corrigida em documentos HTML contendo caracteres Unicode de vários bytes.
* Uso de CPU alto corrigido em documentos com títulos longos devido a uma regressão no wxWidgets.
* Carregamento de arquivos de texto UTF-8 corrigido.
* Itens TOC aninhados em livros Epub colocando seu cursor na posição errada.
* Falha na saída do aplicativo corrigida em certos casos.
* Adicionada uma caixa de seleção no diálogo de opções para habilitar ou desabilitar quebra de linha!
* Agora é possível doar para o desenvolvimento do Paperback, seja através do novo item de doação no menu de ajuda ou através do link patrocinar este projeto na parte inferior da página principal do repositório GitHub.
* Documentos Markdown agora sempre terão um título, e Paperback agora deve conseguir carregar praticamente qualquer arquivo Markdown.
* Documentos PDF agora sempre terão um título, mesmo que os metadados estejam ausentes.
* Bibliotecas PDF alternadas para a usada no Chromium, levando a análise de PDF muito mais confiável em toda a placa.
* Você agora pode ter apenas uma instância do Paperback em execução por vez. Executar `paperback.exe` com um nome de arquivo enquanto já está em execução abrirá esse documento na instância já em execução.
* Você agora pode pressionar delete em um documento no controle de aba para fechá-lo.

### Versão 0.2.1
* Adicionado o número total de páginas ao rótulo de página no diálogo ir para página.
* Permitir tabulação do conteúdo do documento para sua lista de documentos abertos.
* Atalhos de título corrigidos às vezes abrindo documentos recentes se você tivesse bastante deles.
* Paperback agora removerá hífens suaves desnecessários da saída de texto.
* Navegação de título corrigida às vezes o colocando no caractere errado.

### Versão 0.2.0
* Adicionado suporte a documento Markdown!
* Adicionado suporte a documento PDF, incluindo a capacidade de navegar entre páginas!
* Adicionados atalhos para navegar por títulos em conteúdo HTML, incluindo livros epub e documentos Markdown. Estes atalhos foram projetados para funcionar de forma semelhante a um leitor de tela.
* Carregamento de epubs corrigido com nomes de arquivo codificados em URL em seus manifestos.
* Carregamento de livros epub 3 corrigido com XHTML incorporado neles.
* Uma mensagem agora é falada se o documento não suportar uma tabela de conteúdo ou seções, em vez de os itens de menu serem desabilitados.
* Adicionado um menu de documentos recentes! Atualmente armazena seus últimos 10 documentos abertos, e pressionar enter em um o abrirá para leitura.
* Reescreveu completamente o diálogo Encontrar, tornando muito mais simples de usar, enquanto também adicionava um histórico de suas últimas 25 buscas e suporte a expressão regular!
* Documentos abertos anteriormente agora são lembrados entre reinicializações do aplicativo. Isso é configurável através do novo item de opções no menu ferramentas.
* Adicionado `shift+f1` para abrir o readme diretamente no Paperback.

### Versão 0.1.0
* Lançamento inicial.
