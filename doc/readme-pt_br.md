<!-- machine-translated from doc/readme.md (source-hash: d49e7044d9856698); please review and edit as needed -->

# Paperback - versão 0.9.1

## Introdução

Paperback é um leitor de ebooks e documentos leve, rápido e acessível para todos, desde leitores ocasionais até usuários experientes. É projetado para acessibilidade com leitores de tela, velocidade rápida e uma experiência sem inchaço.

## Requisitos do Sistema

O Paperback atualmente é executado no Windows 10/11 e em todas as versões modernas do ARM macOS. Aplicativos nativos para iOS e Android estão em desenvolvimento ativo, com compilações de teste público planejadas logo após o lançamento da versão 0.9.0 do desktop, antes de um lançamento unificado da versão 1.0 cobrindo as quatro plataformas.

## Recursos

* Completamente autossuficiente, não exigindo que nenhum software seja instalado no seu computador para começar a ler.
* Incrivelmente rápido, mesmo em hardware antigo.
* Interface com abas simples, permitindo que você abra quantos documentos desejar lado a lado.
* Salva sua posição exata de leitura em todos os documentos que você abre.
* Opcionalmente lembra quais documentos você tinha abertos quando fechou o programa e os restaura no próximo lançamento.
* Inclui funcionalidade de navegação semelhante à encontrada no modo de navegação na web de muitos leitores de tela para navegar rápida e facilmente pelos documentos.
* Inclui um diálogo de busca robusto, com recursos como histórico e suporte a expressões regulares.
* Pode ser executado completamente de forma portátil ou instalado com associações de arquivo configuradas automaticamente.
* Suporta um grande número de formatos de arquivo comuns.

## Compatibilidade com Leitores de Tela

O Paperback funciona bem com todos os principais leitores de tela. Há, no entanto, um problema conhecido para usuários de JAWS.

### JAWS e Displays Braille

Se você usar JAWS com um display Braille, pode descobrir que parágrafos longos são truncados ao fazer pan para frente com as teclas de navegação do seu display. O comando de leitura do parágrafo atual também é afetado. Este é um bug no tratamento do controle de texto RICHEDIT50W do JAWS, não algo no próprio Paperback, e um que levou bastante tempo para uma correção aparecer, dado o entusiasmo da Vispero em responder a problemas com software de código aberto.

A solução alternativa, eventualmente surgida através do grupo de discussão do JAWS após meses de espera, é editar `paperback.jcf` e definir "Braille Presentation and Panning" como "Always use DOM if available". Você também vai querer habilitar "Pan Text by Paragraph", caso contrário seu display permanecerá no parágrafo ativo em vez de avançar. Com ambas as configurações em vigor, o pan deve funcionar corretamente.

## Tipos de arquivo atualmente suportados

O Paperback suporta os seguintes formatos e extensões:

* Arquivos de ajuda CHM (`.chm`)
* Livros DAISY (`.opf`, `.zip`)
* Livros EPUB (`.epub`)
* Ebooks FB2 (`.fb2`)
* Documentos HTML (`.htm`, `.html`, `.xhtml`)
* Documentos Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Documentos do Microsoft Word (`.docx`, `.docm`, `.doc`)
* Livros MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
* Apresentações OpenDocument (`.odp`, `.fodp`)
* Arquivos de texto OpenDocument (`.odt`, `.fodt`)
* Documentos PDF (`.pdf`)
* Apresentações PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Documentos RTF (`.rtf`)
* Arquivos de texto simples e log (`.txt`, `.log`)

## Atalhos de teclado

O Paperback foi projetado para uso com teclado em primeiro lugar. Aqui estão os atalhos atuais.

Os atalhos abaixo são para Windows. Onde o macOS difere, o equivalente é anotado entre parênteses — principalmente porque Ctrl+G, Ctrl+W e Alt+Left/Right já são usados por outras convenções de sistema ou aplicativo nessa plataforma.

### Menu Arquivo

* `Ctrl+O`: Abrir um documento.
* `Ctrl+F4` (macOS: `Cmd+W`): Fechar o documento atual.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Fechar todos os documentos abertos.
* `Ctrl+Shift+T`: Reabrir o último documento fechado.
* `Ctrl+R`: Mostrar o diálogo "Todos os Documentos" (a partir de Documentos Recentes).
* `Ctrl+Q`: Sair (apenas Windows; no macOS está no menu do aplicativo).

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
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, ou seja, a tecla Control física em vez de Cmd): Ver texto da nota na posição atual.
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
* `Ctrl+T`: Mostrar índice.
* `F7`: Mostrar lista de elementos.
* `Ctrl+Shift+C`: Abrir pasta contendo.
* `Ctrl+Shift+V`: Abrir conteúdo atual em Visualização da Web.
* `Ctrl+U`: Ver a fonte do documento em uma nova aba.
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
* `F1`: Ver ajuda no navegador padrão.
* `Shift+F1`: Ver ajuda no Paperback.
* `Ctrl+Shift+U`: Verificar atualizações.
* `Ctrl+D`: Abrir a página de doação no navegador padrão.

### Teclas de visualização de documento adicionais

* `Delete` / `Numpad Delete` no controle de aba: Fechar a aba do documento selecionada.
* `Enter` ou `Space` no texto do documento: Ativar link no cursor ou abrir uma visualização de tabela quando em um marcador de tabela.
* `Shift+F10` ou a tecla Menu/Aplicativo no texto do documento: Abrir o menu de contexto.

## Idiomas suportados

O Paperback é traduzido para muitos idiomas diferentes, com mais sendo adicionados o tempo todo. Uma lista completa está abaixo.

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
As seguintes pessoas fizeram doações de algum tamanho para o desenvolvimento do Paperback. Se você fizer uma doação, seu nome não será adicionado automaticamente aqui; só adiciono pessoas que desejam que sua doação seja pública.

Nota: Considero um patrocinador público do GitHub motivo para inclusão automática nesta lista.

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

### Versão 0.9.1
* Sons de favoritos e anotações agora tocam no macOS.
* Livros DAISY agora reproduzem seu áudio no macOS, em vez de abrir e rastrear sua linha do tempo em silêncio.
* Corrigidas aspas curvas, travessões em e caracteres similares desaparecendo de documentos RTF, unindo as palavras circundantes à medida que desapareciam.
* Corrigidas imagens RTF vazando seus dados brutos no documento como texto corrompido.
* Corrigido o submenu Documentos Recentes mantendo entradas desatualizadas até que algo mais acontecesse para reconstruí-lo.
* Aceleradores de teclado estão de volta em todas as traduções, então os menus do russo têm acesso ao teclado novamente.
* Documentos CHM grandes agora abrem até sete vezes mais rápido.
* Documentos abertos agora são registrados no Windows, para que apareçam na lista de saltos da barra de tarefas e na lista recente do menu Iniciar.
* Opções foi renomeado para Configurações, correspondendo aos aplicativos móveis e, no macOS, à convenção da plataforma.
* Paperback agora lembra sua posição de janela, tamanho e estado maximizado entre as execuções.
* Formas plurais agora são traduzidas, então mensagens que contam coisas leem apropriadamente em idiomas que precisam de mais de uma forma.
* Selecionar o ncc.html de um livro DAISY agora abre o audiolivro completo em vez de apenas seu texto.
* Os nomes de ações do diálogo Personalizar Atalhos de Teclado agora podem ser traduzidos.
* O título do documento agora vem em primeiro lugar na barra de título, para que livros abertos possam ser distinguidos na barra de tarefas e Alt+Tab.
* O diálogo de atualização agora é traduzido.

### Versão 0.9.0

#### Adicionado

##### Geral
* Uma ferramenta CLI, chamada pb, para converter rapidamente qualquer um dos formatos suportados pelo Paperback para HTML, Markdown ou texto simples.
* Uma opção para recarregar documentos que foram modificados por outros programas no disco.
* Uma opção Ver Fonte para abrir o código-fonte de um documento em uma nova aba, útil para editar Markdown, por exemplo.
* O texto do documento agora é paginado, o que significa que você pode carregar livros com dezenas de milhões de palavras em apenas alguns segundos. Por favor, relate qualquer estranheza encontrada com isso.

##### Suporte de Plataforma
* Suporte ARM64 Windows!
* Suporte nativo macOS!
* Um botão de tela cheia.

##### Diálogo Todos os Documentos
* Um botão localizar para localizar livros ausentes que acabaram de mudar seu caminho.
* Um filtro de status e barra de status, para que você possa filtrar por status de documento e ver quantos documentos são mostrados e selecionados.
* O atalho `Ctrl+Shift+A` para desselecionar todos os documentos.

##### Opções e Legibilidade
* Uma aba de legibilidade, com as seguintes opções:
    * Quebra de linha (movida de geral);
    * Renderizar tabelas inline (novo nesta versão, veja abaixo);
    * Fonte;
    * Cor de fundo;
    * Espaçamento de linhas;
    * Espaçamento de parágrafos;
    * Espaçamento de letras;
    * Alinhamento de texto.
* Um item de menu de quebra de linha e tecla de atalho subsequente.
* Um botão para determinar como você quer que as tabelas sejam exibidas, e unificou como as tabelas são exibidas em documentos.

##### Navegação
* Suporte para navegação por contêiner.
* Uma opção para mover automaticamente o cursor para o início da linha ao navegar entre linhas, semelhante ao modo de navegação em leitores de tela.
* O atalho de teclado igual para anunciar sua percentagem atual em um documento.

##### Favoritos
* Favoritos temporários: você pode ter um por documento, e eles persistem. Use barra para definir um e barra invertida para ir para ele.

##### Contagem de Palavras
* Tempo de leitura estimado no diálogo de contagem de palavras, bem como a capacidade de definir sua velocidade de leitura para tornar essa métrica realmente útil.
* Se uma seleção estiver ativa quando você abrir o diálogo de contagem de palavras, quantas palavras você selecionou agora será mostrado.

##### Atalhos de Teclado
* A capacidade de personalizar cada atalho de teclado no aplicativo através de um diálogo simples.
* Um atalho de teclado configurável para restaurar Paperback da bandeja do sistema.

##### Idiomas
* Holandês, finlandês e polonês.

##### Exportar
* Expandido o item de menu de exportação para permitir exportar para HTML e Markdown, além de texto simples.

##### Atualizador
* Um botão cancelar para o diálogo de atualização em progresso.
* O atualizador agora valida que o arquivo baixado não foi adulterado.

##### Visualização da Web
* A visualização da web agora é aberta na sua posição de leitura atual.

##### Livros DAISY
* Suporte para livros DAISY 2.0.
* Suporte para reprodução de áudio DAISY 2.02.

##### Audiolivros
* A capacidade de reproduzir audiolivros, atualmente suportando tanto áudio DAISY (incluindo áudio DAISY + texto) quanto zips de arquivos de áudio.
* Atalhos de teclado e itens de menu para reproduzir/pausar narração, buscar para frente e para trás, e ajustar a quantidade de busca.
* Opções para sincronizar o cursor de leitura com reprodução de áudio, definir a quantidade de busca de áudio e escolher se buscar além do final de um capítulo continua no próximo.

##### Documentos CHM
* Suporte para listas, itens de lista, figuras e imagens.

##### PowerPoint
* Documentos PowerPoint agora suportam tabelas.

#### Corrigido

##### Geral
* Documentos codificados em codificações CJK legadas, como GBK, Big5 e Shift_JIS, agora serão renderizados corretamente em vez de como um monte de mojibake.
* "Reabrir último fechado" tentando reabrir o readme em pacote.
* Sua aba selecionada não sendo adequadamente focada após reiniciar Paperback.
* O tratamento do Paperback de arquivos em unidades de rede do Windows: pressionar mostrar arquivo em pasta agora foca corretamente o arquivo no armazenamento de rede, e os caminhos não contêm mais caracteres estranhos.
* Arquivos .paperback não serão mais carregados forçadamente na restauração de documentos; em vez disso, você será solicitado a confirmar quando um for encontrado.
* Abrir pasta contendo agora foca o arquivo fornecido no explorer.
* Abrir o readme agora respeitará seu idioma selecionado.
* A interface do usuário do Paperback agora será dimensionada corretamente em displays de alta DPI.
* O menu agora atualiza corretamente, e o foco se move para o controle de texto, ao abrir ajuda no Paperback.
* Mudado para um método muito mais seguro de IPC no Windows.
* O título do documento ativo agora será lido ao alternar entre abas.
* Reduzido o uso de memória em documentos grandes reduzindo pela metade o tamanho das tabelas de índice interno por caractere.

##### Diálogo Todos os Documentos
* Escape não fechando os diálogos Informações do Documento e Todos os Documentos.
* A barra de título não atualizando após fechar um documento no diálogo de todos os documentos.
* Readme.html não será mais adicionado à sua lista de todos os documentos quando aberto via Shift+F1.
* Remover documentos do diálogo recentes agora também fechará sua aba ativa.
* Seu filtro de busca agora é preservado após remover um documento.

##### Navegação
* Navegação de página anunciando texto de linha incorreto em algumas situações.
* Ir para Linha, Ir para Página e Ir para Percentagem colocando seu cursor na posição errada em documentos grandes.
* Localizar e Localizar Próximo não respeitando a janela de documento carregada em documentos grandes.

##### Favoritos
* Sons de favorito/anotação agora devem reproduzir exclusivamente quando você navega sobre uma palavra contendo um.

##### Legibilidade
* Aplicar quebra de linha o atirando para o início de seu documento.

##### Visualização da Web
* O diálogo da visualização da web não sendo redimensionável e aparecendo em um tamanho inicial muito pequeno.
* Imagens agora devem exibir corretamente na visualização da web incorporada.

##### Atualizador
* O atualizador agora mostra corretamente o conteúdo de tags de código markdown nas notas de lançamento.

##### Livros DAISY
* Livros DAISY mostrando informações incorretas na barra de status.
* Carregando livros DAISY com declarações de codificação falsas.

##### Documentos RTF
* Analisar documentos RTF com caracteres não-latinos neles.
* Grupos RTF `\pict` para que dados de imagem incorporados não vazem mais no texto do documento.

##### Livros Mobi/AZW3
* Âncoras filepos em livros Mobi dividindo tags HTML e colocando lixo no texto do livro.
* Links em livros Mobi legados.
* Análise AZW3 muito melhorada.

##### Documentos Word
* Documentos do Word com nomes de estilo específicos de localidade não renderizando seus títulos corretamente.

##### Documentos HTML/XHTML
* Elementos dl, dt e dd não produzindo quebras de linha em documentos XHTML.

##### Documentos PDF
* Paperback agora recua para extração de texto simples para PDFs falsamente marcados.
* Documentos PDF contendo caracteres de controle em seus títulos e/ou favoritos não travará mais Paperback ao abrir.

### Versão 0.8.5
* Adicionado suporte de página a livros epub.
* Adicionado suporte para documentos Microsoft Office criptografados. Atualmente Word legado, Word moderno e Powerpoint moderno são suportados, com Powerpoint legado planejado para o futuro.
* Adicionado suporte para documentos Microsoft Word legados (*.doc)!
* Adicionado suporte para apresentações Powerpoint legadas (*.ppt)!
* Adicionado suporte para livros mobi e AZW3!
* Adicionado suporte para arquivos PDF marcados!
* Adicionado o atalho ctrl+q para sair do aplicativo.
* Adicionado suporte para livros compactados da Bookshare (DAISY e Word)!
* Texto alternativo para imagens incorporadas agora deve ser mostrado adequadamente.
* Documentos CHM agora suportam adequadamente navegação de links internos.
* Corrigidos sons de favorito acionando no início de parágrafo em vez da posição do favorito.
* Corrigido ir para página estar desativado por 1.
* Corrigida a tecla escape não funcionando para fechar o diálogo abrir como.
* Corrigido o menu de contexto do leitor não aparecendo ao clicar com o botão direito ou na tecla Aplicativos.
* Corrigido o documento errado às vezes sendo focado ao abrir documentos da linha de comando.
* PDFs apenas com imagem são novamente detectados e o alertam de sua existência.
* Agora é possível navegar através de imagens e figuras com g/shift+g e f/shift+f, respectivamente.
* Paperback agora respeitará sua configuração de modo escuro do aplicativo.
* Removido suporte DAISY XML, pois não é mais necessário.
* Mudado de volta para a navegação de primeira letra nativa Win32 na árvore do índice.
* O diálogo de erro ao carregar agora mostra mensagens de erro mais detalhadas.
* A visualização da web agora abrirá muito mais rápido e suave.

### Versão 0.8.2
* Adicionado suporte de página a documentos RTF!
* Corrigido um bug onde abrir a visualização da web em epubs contendo links externos os ativaria automaticamente.
* Corrigido um bug onde o analisador RTF não colocaria um espaço entre palavras em casos raros.
* Corrigidos parágrafos sendo divididos em várias linhas curtas em alguns documentos PDF.
* Documentos PDF agora têm suporte básico de navegação de links e títulos!
* Abas e alimentações de linha RTF agora são renderizadas exatamente como aparecem no documento.
* Mudado de volta para a biblioteca pdfium comprovada para analisar PDFs, tornando a renderização de PDF muito mais confiável novamente.

### Versão 0.8.1
* Adicionado Ctrl+Shift+T para reabrir o último documento fechado.
* O diálogo Todos os Documentos agora suporta selecionar múltiplos documentos para abrir de uma vez.
* Corrigidos alguns bugs com o analisador RTF.
* Corrigidos caminhos de arquivo contendo caracteres não-ASCII (como Bósnio š, č, ć, ž) ficando corrompidos ao abrir um arquivo através de uma segunda instância do Paperback.
* Corrigido texto PDF sendo lido na ordem errada, e espaçamento incorreto ao redor de palavras capitalizadas.
* Corrigido carregamento lento de documentos ao abrir arquivos grandes.
* Corrigida a localização dos botões Sim/Não em diálogos de confirmação.

### Versão 0.8.0
* Adicionadas traduções para japonês, chinês simplificado e vietnamita!
* Adicionado um atualizador automático que agora substituirá sua versão instalada do Paperback em vez de apenas baixar a nova versão!
* Adicionado feedback de som opcional para alcançar um favorito ou uma anotação, obrigado Andre Louis pelos sons!
* Adicionado suporte a documentos RTF!
* Adicionado suporte para documentos DAISY XML.
* Adicionado suporte para arquivos Flat Open Document Text!
* Adicionado suporte para apresentações Flat Open Document!
* Adicionado suporte para separadores com s e shift+s.
* Qualquer movimento maior que 300 caracteres agora adicionará automaticamente seu histórico de navegação.
* Corrigida a restauração da janela do Paperback da bandeja do sistema.
* Corrigidos documentos Markdown mostrando texto bruto em vez de HTML renderizado na Visualização da Web.
* Corrigidas tabelas não renderizando corretamente em arquivos Markdown.
* PDFs apenas com imagem agora o avisarão de sua existência quando você tentar carregar um.
* Agora é possível verificar novas compilações de desenvolvimento em vez de versões estáveis ao verificar atualizações.
* Adequadamente incorpore informações de versão no executável Paperback.
* Divida o diálogo de opções em abas para facilidade de uso e navegação.
* Mudado para Hayro para analisar PDFs, levando a mais confiabilidade, velocidade e menos DLLs.
* Reescrevi todo o aplicativo em Rust. A nova base de código é mais segura, carrega documentos mais rápido e é mais fácil de manter e estender.
* O menu de contexto do controle de texto agora incluirá ações específicas do leitor em vez de itens genéricos como cortar e colar.

### Versão 0.7.0
* Adicionado suporte a tabelas para documentos baseados em HTML e XHTML! Navegue entre tabelas usando T e Shift+T, e pressione Enter para visualizar uma em uma visualização da web.
* Adicionado um recurso básico de renderização da web! Pressione Ctrl+Shift+V para abrir a seção atual de seu documento em um renderizador baseado na web, útil para conteúdo como formatação complexa ou amostras de código.
* Adicionada uma tradução russa, obrigado Ruslan Gulmagomedov!
* Adicionado um botão Limpar Tudo ao diálogo Todos os Documentos.
* O verificador de atualização agora exibe notas de lançamento quando uma nova versão está disponível.
* Corrigida a restauração da janela da bandeja do sistema.
* Corrigidas traduções de botão Sim/Não em diálogos de confirmação.
* Corrigido carregamento de configs ao executar como administrador.
* Corrigido tratamento de comentários em documentos XML e HTML.
* Corrigida análise de TOC em livros Epub 2.
* Corrigida navegação para o próximo item com a mesma letra no índice.
* Corrigido o diálogo de localização não se ocultando adequadamente ao usar os botões próximo/anterior.
* Corrigidos TOCs de epub ocasionalmente o arremessando para o item errado.
* Corrigidos vários problemas de tratamento de espaços em branco em XML, HTML e tags pre.
* Corrigido erro off-by-one na navegação de links.
* Corrigidos alguns livros tendo espaços em branco à direita em suas linhas.
* Corrigidos vários problemas de analisador.
* Itens de menu relacionados a favoritos, bem como a lista de elementos, agora são adequadamente desabilitados quando nenhum documento está aberto.
* Melhorado o tratamento de listas em vários formatos de documento.
* Melhorado o fluxo de trabalho de tradução para contribuintes.
* Muitas refatorações internas, movendo a maioria da lógica de negócios do aplicativo de C++ para Rust para melhor desempenho e manutenibilidade.

### Versão 0.6.1
* Adicionado suporte a PDF protegido por senha!
* Adicionado um recurso muito básico de ir para posição anterior/próxima. Se você pressionar enter em um link interno e isso mover seu cursor, essa posição agora será lembrada, e poderá ser navegada com as setas alt+esquerda/direita.
* Adicionada uma lista de elementos! Atualmente, ela apenas mostra uma árvore de todos os títulos em seu documento ou uma lista de links, mas há planos para expandi-la no futuro.
* Adicionada uma opção para iniciar o Paperback no modo maximizado por padrão.
* Corrigidos links em alguns documentos Epub não funcionando adequadamente.
* Corrigida análise de TOCs de Epub contendo caminhos relativos.
* Corrigidos alguns documentos epub não mostrando um título ou autor.
* Corrigidos os títulos de alguns capítulos epub não aparecendo adequadamente no diálogo TOC.
* Corrigido você não sendo capaz de usar a barra de espaço para ativar os botões OK/cancelar no diálogo TOC.
* Melhorado o tratamento de títulos em documentos Word.
* Você agora receberá feedback falado se a lista de documentos recentes estiver vazia quando tentar abrir o diálogo.

### Versão 0.6.0
* Uma nova opção para mostrar o menu ir em uma forma muito mais compacta foi adicionada ao diálogo de opções, verificado por padrão.
* Adicionada uma opção para fazer a navegação por elementos estruturais envolver.
* Adicionada uma opção ao menu de ferramentas para abrir a pasta contendo do documento atualmente focado.
* Adicionado um sistema de atualização bastante simples, mas muito eficaz.
* Adicionado um recurso básico de temporizador de sono, acessível com Ctrl+Shift+S.
* Adicionado suporte para análise de livros eletrônicos FB2!
* Adicionado suporte para análise de apresentações OpenDocument!
* Adicionado suporte para análise de arquivos OpenDocument Text!
* Os favoritos agora podem ser feitos para marcar uma linha inteira ou para marcar apenas algum texto especificado. Se você não tiver seleção ativa ao colocar um favorito, o comportamento é como pré-0.6, e marcará a linha inteira. Porém, se você selecionar algum texto, apenas esse texto será incluído no favorito.
* Os favoritos agora podem ter notas de texto opcionais anexadas a eles! Navegue entre favoritos contendo notas com N e Shift+N, ou abra o diálogo de favoritos com todos os favoritos, apenas notas ou apenas não-notas selecionados com teclas de atalho específicas.
* Favoritos no diálogo de favoritos não terão mais um incômodo prefixo "favorito x".
* Livros Epub contendo conteúdo HTML fingindo ser XML agora serão manipulados corretamente.
* Corrigido carregamento de documentos Markdown grandes.
* Corrigido pressionar espaço na árvore de visualização do índice ativando o botão OK.
* Corrigido tratamento de espaços em branco no início de tags pre em documentos HTML e XHTML.
* Corrigido o controle de texto não recuperando o foco às vezes ao retornar à janela do Paperback.
* Corrigido o campo de texto no diálogo ir para percentagem não atualizando o valor do controle deslizante.
* Corrigida a renderização de IDs HTML personalizados em documentos Markdown.
* HTML dentro de blocos de código Markdown agora será renderizado corretamente.
* Se carregar um livro com um parâmetro de linha de comando enquanto uma instância existente do Paperback está em execução, você não receberá mais um erro se carregar seu documento levar mais de 5 segundos.
* Se executar o Paperback como administrador, a configuração agora será adequadamente carregada e salva.
* Agora é possível excluir um favorito diretamente dentro do diálogo de favoritos.
* Agora é possível importar e exportar seus favoritos e posição de leitura para um documento específico. O arquivo gerado é nomeado após o arquivo com uma extensão .paperback. Se tal arquivo for encontrado no mesmo diretório de um arquivo ao carregá-lo, será automaticamente carregado. Caso contrário, você pode importá-los manualmente usando um item no menu de ferramentas.
* Links dentro de documentos agora são totalmente suportados! Use k e shift+k para avançar e recuar através deles, e pressione enter para abrir/ativar um.
* Muitas refatorações internas, tornando o aplicativo mais rápido e o binário menor.
* O conteúdo Markdown agora é pré-processado para ser compatível com CommonMark antes da renderização.
* Navegação por listas e seus itens agora é totalmente suportada! Use L e Shift+L para ir por listas em si, e I e Shift+I para percorrer itens de lista.
* Exclusão em teclado numérico agora funciona para remover documentos da barra de abas além de exclusão normal.
* Paperback agora pode opcionalmente minimizar para sua bandeja do sistema! Esta opção está desabilitada por padrão, mas ativá-la fará com que a opção minimizar no menu do sistema coloque Paperback em sua bandeja, podendo ser restaurado clicando no ícone gerado.
* Paperback agora é totalmente traduzível! A lista de idiomas que suporta é atualmente bastante pequena, mas está em crescimento constante!
* Paperback agora tem um site oficial, em [paperback.dev](https://paperback.dev)!
* Documentos PPTX agora mostrarão um índice básico, contendo todos os slides.
* O caminho completo para o documento aberto agora será mostrado no diálogo de informações do documento.
* O instalador agora inclui uma opção para visualizar o readme no navegador após a instalação.
* A lista de documentos recentes foi dramaticamente expandida! Em vez de simplesmente mostrar os últimos 10 documentos que você abriu, agora mostrará um número personalizável, com o restante dos documentos que você já abriu sendo acessível através de um pequeno diálogo.
* Várias pequenas melhorias nos analisadores em geral, incluindo colocar uma linha em branco entre slides em apresentações PPTX, corrigindo o tratamento de quebra de linha dentro de parágrafos em documentos Word e adicionando pontos de lista aos itens de lista.

### Versão 0.5.0
* Adicionado suporte a documentos Microsoft Word!
* Adicionado suporte a apresentações PowerPoint!
* Corrigidos certos itens de menu não sendo desabilitados sem documentos abertos.
* Corrigida a orientação do controle deslizante ir para percentagem.
* Corrigido o índice em livros Epub com caminhos de arquivo codificados em URL e/ou IDs de fragmento.
* Corrigido espaço em branco sendo removido de títulos XHTML de formas estranhas.
* Corrigido tratamento de espaço em branco dentro de tags pre aninhadas em documentos HTML.
* Documentos HTML e Markdown agora suportam o recurso de índice! Quando você carrega um documento HTML/Markdown, o Paperback construirá seu próprio índice a partir da estrutura dos títulos em seu documento, e o mostrará para você no diálogo ctrl+t.
* Documentos HTML agora terão o título conforme definido na tag título, se existir. Caso contrário, continuarão a usar o nome do arquivo sem a extensão.
* Mudado de UniversalSpeech para usar uma região ativa para relatar fala. Isso significa que nenhuma DLL de leitor de tela é mais embarcada com o programa, e mais leitores de tela serão suportados agora, como o Microsoft Narrator.
* Mudado bibliotecas zip para permitir abrir uma variedade mais ampla de livros epub.
* O diálogo perguntando se você quer abrir seu documento como texto simples foi completamente refeito, e agora permite que você abra seu documento como texto simples, HTML ou Markdown.
* O diálogo ir para percentagem agora inclui um campo de texto permitindo que você insira manualmente uma percentagem para pular.
* O analisador HTML agora reconhecerá dd, dt e dl como elementos de lista.
* O índice em livros Epub será novamente preservado exatamente.
* O espaço não-separável unicode agora é considerado ao remover linhas em branco.
* Você não será mais solicitado como deseja abrir um arquivo não reconhecido toda vez que o carrega, apenas na primeira vez.

### Versão 0.4.1
* Adicionado um ícone de menu inicial opcional ao instalador.
* O índice agora deve estar mais limpo em alguns casos, por exemplo, se você tiver um item filho e pai com o mesmo texto na mesma posição, você agora verá apenas o item pai.
* Corrigido o índice em certos documentos CHM.
* Corrigido o índice em livros Epub 3 com caminhos absolutos neles.
* Documentos CHM agora devem mostrar seu título conforme definido no arquivo de metadados.

### Versão 0.4.0
* Adicionado suporte a arquivos CHM!
* Adicionado suporte a favoritos! Você pode ter quantos favoritos quiser em quantos documentos quiser. Você pode pular para frente e para trás através deles com b e shift+b, definir um com control+shift+b, e abrir um diálogo para pular para um favorito específico com control+b.
* Adicionado um instalador ao lado do arquivo zip portátil! O instalador instalará Paperback em seu diretório Program Files e configurará automaticamente associações de arquivo para você.
* Arquivos de texto com BOMs agora devem ser decodificados corretamente, e o BOM não será mais exibido no início do texto.
* Adicionada muito mais informação à barra de status. Agora mostrará sua linha atual, caractere e percentagem de leitura.
* Comentários HTML, bem como o conteúdo de tags script e style, não serão mais mostrados na saída de texto.
* Se passar um caminho relativo para Paperback na linha de comando, agora o resolverá adequadamente.
* Movimento de percentagem agora é manipulado por seu próprio diálogo baseado em controle deslizante, acessível com control+shift+g.
* Documentos sem títulos ou autores conhecidos agora sempre terão um padrão.
* A lógica de salvamento de posição agora é muito mais inteligente e deve apenas escrever no disco quando absolutamente necessário.
* O documento que você teve focado quando fechou o Paperback agora é lembrado entre reinicializações de aplicativos.
* Entrada nos diálogos ir para linha e ir para página agora deve ser mais rigorosamente higienizada.
* Corrigida navegação de índice em livros epub 3 com caminhos relativos em seus manifestos.

### Versão 0.3.0
* Corrigido o índice em livros epub com manifestos codificados em URL.
* Corrigida navegação de título em documentos HTML contendo caracteres Unicode de vários bytes.
* Corrigido uso elevado de CPU em documentos com títulos longos devido a uma regressão em wxWidgets.
* Corrigido carregamento de arquivos de texto UTF-8.
* Corrigidos itens de TOC aninhados em livros Epub colocando seu cursor na posição errada.
* Corrigido um travamento na saída do aplicativo em certos casos.
* Adicionada uma caixa de seleção no diálogo de opções para ativar ou desativar quebra de linha!
* Agora é possível doar para o desenvolvimento do Paperback, através do novo item de doação no menu de ajuda ou através do link patrocinar este projeto na parte inferior da página principal do repositório do GitHub.
* Documentos Markdown agora sempre terão um título, e Paperback agora deve ser capaz de carregar praticamente qualquer arquivo Markdown.
* Documentos PDF agora sempre terão um título, mesmo que os metadados estejam faltando.
* Mudado bibliotecas PDF para a usada no Chromium, levando a análise de PDF muito mais confiável em geral.
* Você agora só pode ter uma instância do Paperback em execução por vez. Executar paperback.exe com um nome de arquivo enquanto já está em execução abrirá esse documento na instância já em execução.
* Você agora pode pressionar delete em um documento no controle de abas para fechá-lo.

### Versão 0.2.1
* Adicionado o número total de páginas ao rótulo de página no diálogo ir para página.
* Permitir tabar do conteúdo do documento para sua lista de documentos abertos.
* Corrigidas algumas situações onde os atalhos de título às vezes abriam documentos recentes se você tivesse o suficiente deles.
* Paperback agora removerá hífens suaves desnecessários da saída de texto.
* Corrigida navegação de título às vezes o colocando no caractere errado.

### Versão 0.2.0
* Adicionado suporte a documentos markdown!
* Adicionado suporte a documentos PDF, incluindo a capacidade de navegar entre páginas!
* Adicionados atalhos para navegar por títulos em conteúdo HTML, incluindo livros epub e documentos markdown. Estes atalhos foram projetados para funcionar de forma semelhante a um leitor de tela.
* Corrigido carregamento de epubs com nomes de arquivo codificados em URL em seus manifestos.
* Corrigido carregamento de livros epub 3 com XHTML incorporado neles.
* Uma mensagem agora é falada se o documento não suportar um índice ou seções, em vez dos itens de menu serem desabilitados.
* Adicionado um menu de documentos recentes! Atualmente armazena seus últimos 10 documentos abertos, e pressionar enter em um os abrirá para leitura.
* Completamente reescrito o diálogo Localizar, tornando-o muito mais simples de usar, enquanto também adicionando um histórico de suas últimas 25 buscas e suporte a expressão regular!
* Documentos abertos anteriormente agora são lembrados entre reinicializações de aplicativos. Isto é configurável através do novo item de opções no menu de ferramentas.
* Adicionado shift+f1 para abrir o readme diretamente no Paperback.

### Versão 0.1.0
* Lançamento inicial.
