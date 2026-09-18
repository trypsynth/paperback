<!-- machine-translated from doc/readme.md (source-hash: 11f05688d690d71a; sections: f48ce952,d4d8160f,a02f4421,a4ffb7f7,91be3b41,a028b7f6,2fb18876,71df8e94,e9860ee8,a7ac6234); please review and edit as needed -->

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

Paperback foi projetado para uso com foco no teclado. Aqui estão os atalhos atuais.

Os atalhos abaixo são para Windows. Onde o macOS difere, o equivalente é indicado entre parênteses — principalmente porque Ctrl+G, Ctrl+W e Alt+Left/Right já são usados por outras convenções do sistema ou de aplicativos naquela plataforma.

### Menu Arquivo

* `Ctrl+O`: Abre um documento.
* `Ctrl+F4` (macOS: `Cmd+W`): Fecha o documento atual.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Fecha todos os documentos abertos.
* `Ctrl+Shift+T`: Reabre o último documento fechado.
* `Ctrl+R`: Mostra o diálogo "Todos os Documentos" (a partir de Documentos Recentes).
* `Ctrl+Q`: Sai (apenas Windows; no macOS isso fica no menu do aplicativo).

### Menu Ir

* `Ctrl+F`: Mostra o diálogo Localizar.
* `F3` (macOS: `Cmd+G`): Localiza próximo.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Localiza anterior.
* `Ctrl+G` (macOS: `Cmd+L`): Ir para linha.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Ir para percentual.
* `Ctrl+P`: Ir para página (quando suportado pelo documento atual).
* `=`: Anuncia seu percentual de leitura atual e página, por exemplo, "15%, página 30". A página é omitida para documentos que não possuem números de página.
* `Alt+Left` (macOS: `Cmd+[`): Volta no histórico de navegação.
* `Alt+Right` (macOS: `Cmd+]`): Avança no histórico de navegação.
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
* `/`: Define seu marcador temporário.
* `\`: Pula para seu marcador temporário.
* `Shift+N`: Nota anterior.
* `N`: Próxima nota.
* `Ctrl+B`: Pula para todos os marcadores e notas.
* `Ctrl+Alt+B`: Pula para marcadores apenas.
* `Ctrl+Alt+M`: Pula para notas apenas.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, ou seja, a tecla Control física em vez de Cmd): Visualiza o texto da nota na posição atual.
* `Shift+K`: Link anterior.
* `K`: Próximo link.
* `Shift+G`: Imagem anterior.
* `G`: Próxima imagem.
* `Shift+F`: Figura anterior.
* `F`: Próxima figura.
* `Shift+T`: Tabela anterior.
* `T`: Próxima tabela.
* `Shift+M`: Fórmula anterior.
* `M`: Próxima fórmula.
* `Shift+S`: Separador anterior.
* `S`: Próximo separador.
* `Shift+L`: Lista anterior.
* `L`: Próxima lista.
* `Shift+I`: Item de lista anterior.
* `I`: Próximo item de lista.
* `Shift+,`: Vai para o início do contêiner atual (lista ou tabela).
* `,`: Vai além do fim do contêiner atual (lista ou tabela).

### Menu Ferramentas

* `Ctrl+W` (macOS: `RawCtrl+W`, ou seja, a tecla Control física em vez de Cmd): Mostra a contagem de palavras do documento atual.
* `Ctrl+I`: Mostra informações do documento.
* `Ctrl+T`: Mostra sumário.
* `F7`: Mostra lista de elementos.
* `Ctrl+Shift+C`: Abre pasta contendo.
* `Ctrl+Shift+V`: Abre conteúdo atual em Visualização Web.
* `Ctrl+U`: Visualiza a fonte do documento em uma nova aba.
* `Ctrl+Shift+E`: Exporta dados do documento (`.paperback`).
* `Ctrl+Shift+I`: Importa dados do documento (`.paperback`).
* `Ctrl+E`: Exporta o documento atual como texto simples.
* `Ctrl+Shift+B`: Alterna marcador na seleção/cursor atual.
* `Ctrl+Shift+N`: Adiciona ou edita nota de marcador na seleção/cursor atual.
* `Ctrl+Alt+W`: Alterna quebra de linha.
* `Ctrl+Space`: Reproduz/pausa narração de áudio.
* `'`: Avança narração de áudio.
* `;`: Retrocede narração de áudio.
* `Ctrl+'`: Aumenta a quantidade de busca de áudio.
* `Ctrl+;`: Diminui a quantidade de busca de áudio.
* `F11` (macOS: `RawCtrl+Ctrl+F`, ou seja, Control+Command+F): Alterna tela cheia.
* `Ctrl+,`: Abre opções (macOS: Preferências, no menu do aplicativo).
* `Ctrl+Shift+S`: Alterna temporizador de suspensão.
* `Alt+F9` (macOS: `Cmd+F9`): Marca o início de uma seleção, para que tudo a partir daqui até onde você chegar possa ser copiado de uma vez.
* `Alt+F10` (macOS: `Cmd+F10`): Copia tudo do início da seleção marcado até a posição atual.
* `Alt+Shift+F9` (macOS: `Cmd+Shift+F9`): Volta para o início da seleção marcado, mantendo a marca em seu lugar.

### Menu Ajuda

* `Ctrl+F1`: Mostra diálogo Sobre.
* `F1`: Visualiza ajuda no navegador padrão.
* `Shift+F1`: Visualiza ajuda no Paperback.
* `Ctrl+Shift+U`: Verifica se há atualizações.
* `Ctrl+D`: Abre a página de doação no navegador padrão.

### Teclas adicionais de exibição de documento

* `Delete` / `Numpad Delete` no controle de aba: Fecha a aba de documento selecionada.
* `Enter` ou `Space` no texto do documento: Segue um link ou abre visualização de tabela ou fórmula no cursor.
* `Shift+F10` ou a tecla Menu/Aplicativo no texto do documento: Abre o menu de contexto.

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

## Histórico de alterações

### Versão 0.9.2
* Audiolivros não fazem mais seu leitor de tela ler uma sequência de espaços quando você foca o campo de texto.
* Audiolivros agora nomeiam o arquivo conforme você percorre as seções.
* Audiolivros agora informam sua duração real, em vez de afirmar que todos os arquivos duram 24 horas.
* Fechar a Exibição da Web com Escape não mais exibe um alerta de depuração após você ter seguido um link nela.
* Copiar depois de Selecionar Tudo agora fornece o documento inteiro, em vez de apenas a parte carregada no momento.
* Localizar agora vai direto à linha encontrada, em vez de fazer você ouvir o leitor de tela ler a janela novamente quando o foco volta ao livro.
* Corrigidos EPUBs que carregam um bloco ZIP64 isolado recusando abrir com "Cabeçalho de arquivo local inválido".
* Corrigidos documentos longos retornando ao início enquanto um leitor de tela lia continuamente por eles.
* Links na Exibição da Web agora levam você à seção para a qual apontam, em vez de falhar com "Arquivo não encontrado".
* Marque o início de uma seleção com `Alt+F9`, copie tudo de lá até onde você chegou com `Alt+F10`, e retorne à marca com `Alt+Shift+F9`, para copiar um longo trecho de texto sem usar shift+seta. Os três estão em Ferramentas > Selecionar e copiar.
* O atalho `=` agora anuncia a página bem como a porcentagem, por exemplo, "15%, página 30", e permanece como estava para documentos sem números de página.
* O anúncio automático "Documento recarregado" não interrompe mais seu leitor de tela no meio da frase, esperando que ele termine de falar.
* A guia Geral do diálogo Configurações agora percorre suas opções na ordem em que aparecem na tela, com o canal de atualização logo após a opção verificar atualizações.
* Atualizar agora traz a janela relançada para o primeiro plano, em vez de deixá-la atrás de todas as outras janelas em Alt+Tab.
* Windows agora sempre mostrará "Paperback" no menu Abrir com, em vez da tagline completa do programa.
* Contagem de Palavras e Informações do Documento agora mostram quantos arquivos um audiolivro contém e sua duração total.

### Versão 0.9.1
* Sons de marcador e nota agora são reproduzidos no macOS.
* Livros DAISY agora reproduzem seu áudio no macOS, em vez de abrir e rastrear sua linha do tempo em silêncio.
* Corrigidas aspas curvas, travessões e caracteres similares desaparecendo de documentos RTF, unindo as palavras circundantes.
* Corrigidas imagens RTF vazando seus dados brutos no documento como texto corrompido.
* Corrigido o submenu Documentos Recentes mantendo entradas obsoletas até algo mais acontecer para reconstruí-lo.
* Aceleradores de teclado estão de volta em todas as traduções, então os menus do russo têm acesso ao teclado novamente.
* Documentos CHM grandes agora abrem até sete vezes mais rápido.
* Documentos abertos agora são registrados no Windows, então aparecem na jump list da barra de tarefas e na lista recente do menu Iniciar.
* Opções foi renomeado para Configurações, combinando com os aplicativos móveis e, no macOS, a convenção da plataforma.
* Paperback agora lembra sua posição de janela, tamanho e estado maximizado entre execuções.
* Formas plurais agora são traduzidas, então mensagens que contam coisas leem corretamente em idiomas que precisam de mais de um formato.
* Selecionar o ncc.html de um livro DAISY agora abre o audiolivro completo em vez de apenas seu texto.
* Os nomes de ação do diálogo Personalizar Atalhos de Teclado agora podem ser traduzidos.
* O título do documento agora vem em primeiro lugar na barra de título, então livros abertos podem ser distinguidos na barra de tarefas e Alt+Tab.
* O diálogo de atualização agora é traduzido.

### Versão 0.9.0

#### Adicionado

##### Geral
* Uma ferramenta CLI, chamada pb, para converter rapidamente qualquer um dos formatos suportados do Paperback para HTML, Markdown ou texto simples.
* Uma opção para recarregar documentos que foram modificados por outros programas no disco.
* Uma opção Ver Fonte para abrir a fonte de um documento em uma nova guia, útil para editar Markdown, por exemplo.
* O texto do documento agora é paginado, o que significa que você pode carregar livros com dezenas de milhões de palavras em apenas alguns segundos. Por favor, reporte qualquer estranheza encontrada.

##### Suporte de Plataforma
* Suporte Windows ARM64!
* Suporte nativo macOS!
* Um alternador de tela cheia.

##### Diálogo Todos os Documentos
* Um botão localizar para localizar livros ausentes que acabaram de mudar seu caminho.
* Um filtro de status e barra de status, para que você possa filtrar por status de documento e ver quantos documentos estão sendo exibidos e selecionados.
* O atalho `Ctrl+Shift+A` para desselecionar todos os documentos.

##### Opções e Legibilidade
* Uma guia de legibilidade, com as seguintes opções:
    * Quebra de linha (movida de geral);
    * Renderizar tabelas inline (novo nesta versão, veja abaixo);
    * Fonte;
    * Cor de fundo;
    * Espaçamento de linhas;
    * Espaçamento de parágrafos;
    * Espaçamento de letras;
    * Alinhamento de texto.
* Um item de menu de quebra de linha e subsequente tecla de atalho.
* Um alternador para determinar como você quer que as tabelas sejam exibidas, e unificou como as tabelas são exibidas em documentos.

##### Navegação
* Fórmulas MathML em EPUB e HTML são renderizadas como AsciiMath usando MathCAT. Use `M` ou `Shift+M` para navegar em fórmulas, então `Enter` ou `Space` para abrir o MathML original na Visualização de Fórmula.
* Suporte para navegação por contêiner.
* Uma opção para mover automaticamente o cursor para o início da linha ao navegar entre linhas, semelhante ao modo de navegação em leitores de tela.
* O atalho de teclado de igualdade para anunciar sua porcentagem atual em um documento.

##### Marcadores
* Marcadores temporários: você pode ter um por documento, e eles persistem. Use barra para definir um e barra invertida para pular para ele.

##### Contagem de Palavras
* Tempo de leitura estimado no diálogo de contagem de palavras, bem como a capacidade de definir sua velocidade de leitura para tornar essa métrica realmente útil.
* Se uma seleção estiver ativa quando você abre o diálogo de contagem de palavras, quantas palavras você selecionou agora será exibido.

##### Atalhos de Teclado
* A capacidade de personalizar todos os atalhos de teclado no aplicativo através de um diálogo simples.
* Um atalho de teclado configurável para restaurar Paperback da bandeja do sistema.

##### Idiomas
* Holandês, finlandês e polonês.

##### Exportar
* Expandido o item de menu de exportação para permitir exportação para HTML e Markdown, além de texto simples.

##### Atualizador
* Um botão cancelar para o diálogo atualização em andamento.
* O atualizador agora valida se o arquivo baixado não foi adulterado.

##### Exibição da Web
* A exibição web agora é aberta em sua posição de leitura atual.

##### Livros DAISY
* Suporte para livros DAISY 2.0.
* Suporte para reprodução de áudio DAISY 2.02.

##### Audiolivros
* A capacidade de reproduzir audiolivros, atualmente suportando tanto DAISY áudio (incluindo DAISY áudio + texto) quanto zips de arquivos de áudio.
* Atalhos de teclado e itens de menu para reproduzir/pausar narração, avançar e retroceder, e ajustar o valor de busca.
* Opções para sincronizar o cursor de leitura à reprodução de áudio, definir o valor de busca de áudio e escolher se buscar além do final de um capítulo continua no próximo.

##### Documentos CHM
* Suporte para listas, itens de lista, figuras e imagens.

##### PowerPoint
* Documentos PowerPoint agora suportam tabelas.

#### Corrigido

##### Geral
* Documentos codificados em codificações CJK legadas, como GBK, Big5 e Shift_JIS, agora são renderizados corretamente em vez de um monte de mojibake.
* "Reabrir último fechado" tentando reabrir o readme incluído.
* Sua guia selecionada não recebendo o foco adequadamente após reiniciar o Paperback.
* O tratamento de arquivos do Paperback em unidades de rede do Windows: pressionar mostrar arquivo na pasta agora foca adequadamente o arquivo no armazenamento de rede, e os caminhos não contêm mais caracteres estranhos.
* Arquivos .paperback não serão mais carregados forçadamente na restauração de documentos; em vez disso, você será solicitado a confirmação quando um for encontrado.
* Abrir pasta contendo agora foca o arquivo fornecido no explorador.
* Abrir o readme agora respeitará seu idioma selecionado.
* A interface do usuário do Paperback agora será dimensionada corretamente em exibições de alta DPI.
* O menu agora se atualiza corretamente, e o foco passa para o controle de texto, ao abrir ajuda no Paperback.
* Mudou para um método muito mais seguro de IPC no Windows.
* O título do documento ativo agora será lido ao alternar entre guias.
* Reduzido o uso de memória em documentos grandes pela metade do tamanho das tabelas de índice interno por caractere.

##### Diálogo Todos os Documentos
* Escape não fechando os diálogos Informações de Documento e Todos os Documentos.
* A barra de título não se atualizando após fechar um documento do diálogo todos os documentos.
* Readme.html não será mais adicionado à sua lista de todos os documentos quando aberto via Shift+F1.
* Remover documentos do diálogo recentes agora também fechará sua guia ativa.
* Seu filtro de pesquisa agora é preservado após remover um documento.

##### Navegação
* Navegação de página anunciando texto de linha incorreto em algumas situações.
* Ir para Linha, Ir para Página e Ir para Percentual colocando seu cursor na posição errada em documentos grandes.
* Localizar e Localizar Próximo não respeitando a janela de documento carregada em documentos grandes.

##### Marcadores
* Sons de marcador/nota agora devem ser reproduzidos exclusivamente quando você navega sobre uma palavra contendo um.

##### Legibilidade
* Aplicar quebra de linha deixando você no início de seu documento.

##### Exibição da Web
* O diálogo exibição web não sendo redimensionável e aparecendo com um tamanho inicial muito pequeno.
* As imagens agora devem ser exibidas adequadamente na exibição web incorporada.

##### Atualizador
* O atualizador agora mostra adequadamente o conteúdo de tags de código markdown em notas de versão.

##### Livros DAISY
* Livros DAISY mostrando informações incorretas na barra de status.
* Carregamento de livros DAISY com declarações de codificação bogus.

##### Documentos RTF
* Análise de documentos RTF com caracteres não-latinos.
* Grupos RTF `\pict` para que dados de imagem incorporados não vazem mais para o texto do documento.

##### Livros Mobi/AZW3
* Âncoras filepos em livros Mobi dividindo tags HTML e colocando lixo no texto do livro.
* Links em livros Mobi legados.
* Análise AZW3 significativamente melhorada.

##### Documentos Word
* Documentos Word com nomes de estilo específicos de localidade não renderizando seus títulos corretamente.

##### Documentos HTML/XHTML
* Elementos dl, dt e dd não produzindo quebras de linha em documentos XHTML.

##### Documentos PDF
* Paperback agora retorna para extração de texto simples para PDFs marcados falsamente.
* Documentos PDF contendo caracteres de controle em seus títulos e/ou marcadores não causarão mais falha no Paperback ao abrir.

### Versão 0.8.5
* Adicionado suporte de página para livros epub.
* Adicionado suporte para documentos Microsoft Office criptografados. Atualmente Word legado, Word moderno e Powerpoint moderno são suportados, com Powerpoint legado planejado para o futuro.
* Adicionado suporte para documentos Microsoft Word legados!
* Adicionado suporte para apresentações Powerpoint legadas!
* Adicionado suporte para livros mobi e AZW3!
* Adicionado suporte para arquivos PDF marcados!
* Adicionado o atalho ctrl+q para sair do aplicativo.
* Adicionado suporte para livros compactados da Bookshare (tanto DAISY quanto Word)!
* Texto alternativo para imagens incorporadas agora deve ser exibido corretamente.
* Documentos CHM agora suportam adequadamente navegação de links internos.
* Corrigido ir para página estar deslocado por 1.
* Corrigida a tecla escape não funcionando para fechar o diálogo abrir como.
* Corrigido o menu de contexto do leitor não aparecendo ao clicar com o botão direito ou pressionar a tecla Aplicativos.
* Corrigido o documento errado às vezes recebendo foco ao abrir documentos da linha de comando.
* PDFs somente de imagem novamente são detectados e alertam você de sua existência.
* Agora é possível navegar por imagens e figuras com g/shift+g e f/shift+f, respectivamente.
* Paperback agora respeitará sua configuração de modo escuro do aplicativo.
* Removido suporte DAISY XML, pois não é mais necessário.
* Voltado à navegação nativa de primeira letra Win32 na árvore de sumário.
* O diálogo de carregamento de erro agora mostra mensagens de erro mais detalhadas.
* A exibição web agora abrirá muito mais rápido e suavemente.

### Versão 0.8.2
* Adicionado suporte de página para documentos RTF!
* Corrigido um bug onde abrir a exibição web em epubs contendo links externos os ativaria automaticamente.
* Corrigido um bug onde o analisador RTF não colocaria um espaço entre palavras em casos raros.
* Corrigidos parágrafos sendo divididos em múltiplas linhas curtas em alguns documentos PDF.
* Documentos PDF agora têm suporte básico de navegação de links e títulos!
* Abas e alimentações de linha RTF agora são renderizadas exatamente como aparecem no documento.
* Voltado à biblioteca pdfium experiente para análise de PDFs, tornando a renderização PDF muito mais confiável novamente.

### Versão 0.8.1
* Adicionado Ctrl+Shift+T para reabrir o último documento fechado.
* O diálogo Todos os Documentos agora suporta selecionar vários documentos para abrir de uma vez.
* Corrigidos alguns bugs com o analisador RTF.
* Corrigidos caminhos de arquivo contendo caracteres não-ASCII (como bósnio š, č, ć, ž) sendo corrompidos ao abrir um arquivo através de uma segunda instância do Paperback.
* Corrigido texto PDF sendo lido na ordem errada e espaçamento incorreto ao redor de palavras em maiúsculas.
* Corrigido carregamento lento de documentos ao abrir arquivos grandes.
* Corrigida a localização dos botões Sim/Não em diálogos de confirmação.

### Versão 0.8.0
* Adicionadas traduções para japonês, chinês simplificado e vietnamita!
* Adicionado um atualizador automático que agora substituirá sua versão instalada atualmente do Paperback em vez de apenas baixar a nova versão!
* Adicionado feedback de som opcional ao atingir um marcador ou uma nota, obrigado Andre Louis pelos sons!
* Adicionado suporte para documentos RTF!
* Adicionado suporte para documentos DAISY XML.
* Adicionado suporte para arquivos Flat Open Document Text!
* Adicionado suporte para apresentações Flat Open Document!
* Adicionado suporte para separadores com s e shift+s.
* Qualquer movimento maior que 300 caracteres agora adicionará automaticamente ao seu histórico de navegação.
* Corrigida a restauração da janela do Paperback da bandeja do sistema.
* Corrigidos documentos Markdown mostrando texto bruto em vez de HTML renderizado na Exibição da Web.
* Corrigidas tabelas não sendo renderizadas adequadamente em arquivos Markdown.
* PDFs somente de imagem agora avisarão você de sua existência ao tentar carregar um.
* Incorporadas adequadamente informações de versão no executável do Paperback.
* Dividido o diálogo de opções em abas para facilitar o uso e navegação.
* Mudado para Hayro para análise de PDFs, levando a mais confiabilidade, velocidade e menos DLLs.
* Reescrito o aplicativo inteiro em Rust. A nova base de código é mais segura, carrega documentos mais rápido e é mais fácil de manter e estender.
* O menu de contexto de controle de texto agora incluirá ações específicas do leitor em vez de itens genéricos como cortar e colar.

### Versão 0.7.0
* Adicionado suporte a tabelas para documentos baseados em HTML e XHTML! Navegue entre tabelas usando T e Shift+T, e pressione Enter para visualizar uma em uma exibição web.
* Adicionado um recurso básico de renderização web! Pressione Ctrl+Shift+V para abrir a seção atual de seu documento em um renderizador baseado em web, útil para conteúdo como formatação complexa ou amostras de código.
* Adicionada tradução para russo, obrigado Ruslan Gulmagomedov!
* Adicionado um botão Limpar Tudo ao diálogo Todos os Documentos.
* O verificador de atualização agora exibe notas de versão quando uma nova versão está disponível.
* Corrigida a restauração da janela da bandeja do sistema.
* Corrigidas traduções de botões Sim/Não em diálogos de confirmação.
* Corrigido carregamento de configs ao executar como administrador.
* Corrigido tratamento de comentários em documentos XML e HTML.
* Corrigido análise de TOC em livros Epub 2.
* Corrigido navegar para o próximo item com a mesma letra no sumário.
* Corrigido o diálogo de pesquisa não se ocultando adequadamente ao usar os botões anterior/próximo.
* Corrigidos TOCs epub ocasionalmente levando você para o item errado.
* Corrigidos vários problemas de tratamento de espaços em branco em XML, HTML e tags pre.
* Corrigido erro off-by-one na navegação de links.
* Corrigidos alguns livros tendo espaço em branco à direita em suas linhas.
* Corrigidos vários problemas do analisador.
* Itens de menu relacionados a marcadores bem como a lista de elementos agora são desabilitados adequadamente quando nenhum documento está aberto.
* Melhorado o tratamento de listas em vários formatos de documento.
* Melhorado o fluxo de trabalho de tradução para colaboradores.
* Muitas refatorações internas, movendo a maioria da lógica de negócio do aplicativo de C++ para Rust para melhor desempenho e manutenibilidade.

### Versão 0.6.1
* Adicionado suporte a PDF protegido por senha!
* Adicionado um recurso muito básico ir para posição anterior/próxima. Se você pressionar enter em um link interno e ele mover seu cursor, essa posição agora será lembrada e pode ser navegada com setas alt+esquerda/direita.
* Adicionada uma lista de elementos! Atualmente apenas mostra uma árvore de todos os títulos em seu documento ou uma lista de links, mas há planos para expandi-la no futuro.
* Adicionada uma opção para iniciar o Paperback em modo maximizado por padrão.
* Corrigidos links em alguns documentos Epub não funcionando adequadamente.
* Corrigida análise de TOCs Epub contendo caminhos relativos.
* Corrigidos alguns documentos epub não mostrando título ou autor.
* Corrigidos os títulos de alguns capítulos epub não aparecendo adequadamente no diálogo TOC.
* Corrigida a impossibilidade de usar a barra de espaço para ativar os botões OK/cancelar no diálogo TOC.
* Melhorado o tratamento de títulos em documentos Word.
* Você receberá agora feedback falado se a lista de documentos recentes estiver vazia ao tentar abrir o diálogo.

### Versão 0.6.0
* Uma nova opção para mostrar o menu ir em uma forma muito mais compacta foi adicionada ao diálogo de opções, marcada por padrão.
* Adicionada uma opção para fazer a navegação por elementos estruturais quebrar.
* Adicionada uma opção ao menu ferramentas para abrir a pasta contendo do documento focado no momento.
* Adicionado um sistema de atualização bastante simples, mas muito eficaz.
* Adicionado um recurso básico de timer de sono, acessível com Ctrl+Shift+S.
* Adicionado suporte para análise de ebooks FB2!
* Adicionado suporte para análise de apresentações OpenDocument!
* Adicionado suporte para análise de arquivos OpenDocument Text!
* Marcadores agora podem marcar uma linha inteira ou marcar apenas algum texto especificado. Se você não tiver nenhuma seleção ativa ao colocar um marcador, o comportamento é como pré-0.6, e marcará a linha inteira. No entanto, se você selecionar algum texto, apenas esse texto será incluído no marcador.
* Marcadores agora podem ter notas de texto opcional anexadas a eles! Navegue entre marcadores contendo notas com N e Shift+N, ou abra o diálogo de marcadores com todos os marcadores, apenas notas ou apenas não-notas selecionados com teclas de atalho específicas.
* Marcadores no diálogo de marcadores não terão mais um prefixo irritante "marcador x".
* Livros Epub contendo conteúdo HTML pretendendo ser XML agora serão tratados adequadamente.
* Corrigido carregamento de documentos Markdown grandes.
* Corrigido pressionar espaço na árvore de sumário ativando o botão OK.
* Corrigido tratamento de espaços em branco no início de tags pre em documentos HTML e XHTML.
* Corrigido o campo de texto não recuperando o foco às vezes ao retornar à janela do Paperback.
* Corrigido o campo de texto no diálogo ir para percentual não atualizando o valor do slider.
* Corrigida a renderização de IDs HTML personalizados em documentos Markdown.
* HTML dentro de blocos de código Markdown agora será renderizado adequadamente.
* Se carregar um livro com um parâmetro de linha de comando enquanto uma instância Paperback existente está em execução, você não receberá mais um erro se carregar seu documento levar mais de 5 segundos.
* Se executar Paperback como administrador, a configuração agora será carregada e salva adequadamente.
* Agora é possível excluir um marcador diretamente do diálogo de marcadores.
* Agora é possível importar e exportar seus marcadores e posição de leitura para um documento específico. O arquivo gerado é nomeado após o arquivo com uma extensão .paperback. Se tal arquivo for encontrado no mesmo diretório que um arquivo ao carregá-lo, será carregado automaticamente. Caso contrário, você pode importá-los manualmente usando um item no menu ferramentas.
* Links dentro de documentos agora são totalmente suportados! Use k e shift+k para avançar e retroceder por eles, e pressione enter para abrir/ativar um.
* Muitas refatorações internas, tornando o aplicativo mais rápido e o binário menor.
* O conteúdo Markdown agora é pré-processado para ser compatível com CommonMark antes de ser renderizado.
* Navegação por listas e seus itens agora é totalmente suportada! Use L e Shift+L para ir por listas em si, e I e Shift+I para passar por itens de lista.
* Delete do numpad agora funciona para remover documentos da barra de guias além de delete normal.
* Paperback agora pode opcionalmente minimizar para sua bandeja do sistema! Esta opção está desligada por padrão, mas ativá-la fará com que a opção minimizar no menu do sistema coloque o Paperback em sua bandeja, podendo ser restaurado clicando no ícone gerado.
* Paperback agora é totalmente traduzível! A lista de idiomas que suporta é atualmente bastante pequena, mas está crescendo constantemente!
* Documentos PPTX agora mostram um sumário básico, contendo todos os slides.
* O caminho completo para o documento aberto agora será exibido no diálogo de informações do documento.
* O instalador agora inclui uma opção para visualizar o readme em seu navegador após a instalação.
* A lista de documentos recentes foi dramaticamente expandida! Em vez de simplesmente mostrar os últimos 10 documentos que você abriu, agora mostrará um número personalizável, com o resto dos documentos que você já abriu sendo acessível através de um pequeno diálogo.
* Várias pequenas melhorias nos analisadores em todo o conselho, incluindo colocar uma linha em branco entre slides em apresentações PPTX, corrigir o tratamento de nova linha dentro de parágrafos em documentos word e adicionar pontos de bala aos itens de lista.

### Versão 0.5.0
* Adicionado suporte para documentos Microsoft Word!
* Adicionado suporte para apresentações PowerPoint!
* Corrigidos certos itens de menu não sendo desabilitados sem documentos abertos.
* Corrigida a orientação do slider ir para percentual.
* Corrigido sumário em livros Epub com caminhos de arquivo e/ou IDs de fragmento codificados em URL.
* Corrigido espaço em branco sendo removido de títulos XHTML de maneiras estranhas.
* Corrigido tratamento de espaços em branco dentro de tags pre aninhadas em documentos HTML.
* Documentos HTML e Markdown agora suportam o recurso de sumário! Ao carregar um documento HTML/Markdown, o Paperback construirá seu próprio sumário fora da estrutura dos títulos em seu documento e o mostrará a você no diálogo ctrl+t.
* Documentos HTML agora terão o título conforme definido na tag título, se existir. Caso contrário, continuarão usando o nome do arquivo sem a extensão.
* Mudado de UniversalSpeech para usar uma região ao vivo para reportar fala. Isso significa que nenhuma DLL de leitor de tela é mais enviada junto com o programa, e mais leitores de tela agora serão suportados, como Microsoft Narrator.
* Mudado bibliotecas zip para permitir abrir uma gama mais ampla de livros epub.
* O diálogo pedindo se você quer abrir seu documento como texto simples foi completamente refeito, e agora permite que você abra seu documento como texto simples, HTML ou Markdown.
* O diálogo ir para percentual agora inclui um campo de texto permitindo que você insira manualmente uma percentagem para pular.
* O analisador HTML agora reconhecerá dd, dt e dl como elementos de lista.
* O sumário em livros Epub será novamente preservado exatamente.
* O espaço não-quebrável unicode agora é considerado ao remover linhas em branco.
* Você não será mais perguntado como deseja abrir um arquivo não reconhecido toda vez que o carrega, apenas na primeira vez.

### Versão 0.4.1
* Adicionado um ícone do menu iniciar opcional ao instalador.
* O sumário agora deve estar mais limpo em alguns casos, por exemplo, se você tiver um item filho e pai com o mesmo texto na mesma posição, você verá apenas o item pai.
* Corrigido sumário em certos documentos CHM.
* Corrigido sumário em livros Epub 3 com caminhos absolutos neles.
* Documentos CHM agora devem mostrar seu título conforme definido no arquivo de metadados.

### Versão 0.4.0
* Adicionado suporte para arquivos CHM!
* Adicionado suporte para marcadores! Você pode ter quantos marcadores quiser em quantos documentos quiser. Você pode pular para frente e para trás por eles com b e shift+b, definir um com control+shift+b, e abrir um diálogo para pular para um marcador específico com control+b.
* Adicionado um instalador junto com o arquivo zip portátil! O instalador instalará o Paperback no seu diretório Program Files e configurará automaticamente associações de arquivo para você.
* Arquivos de texto com BOMs agora devem ser decodificados adequadamente, e a BOM não será mais exibida no início do texto.
* Adicionadas muitas mais informações à barra de status. Agora mostrará sua linha, caractere e percentual de leitura atuais.
* Comentários HTML, bem como o conteúdo de tags script e style, não serão mais exibidos na saída de texto.
* Se passar um caminho relativo para o Paperback na linha de comando, agora será resolvido adequadamente.
* Movimento percentual agora é tratado por seu próprio diálogo baseado em slider, acessível com control+shift+g.
* Documentos sem títulos ou autores conhecidos agora sempre terão um padrão.
* A lógica de salvamento de posição agora é muito mais inteligente e deve gravar no disco apenas quando absolutamente necessário.
* O documento em que você tinha foco quando fechou o Paperback agora é lembrado nas reinicializações do aplicativo.
* Entrada nos diálogos ir para linha e ir para página agora deve ser mais rigorosamente higienizada.
* Corrigida navegação de sumário em livros epub 3 com caminhos relativos em seus manifestos.

### Versão 0.3.0
* Corrigido sumário em livros epub com manifestos codificados em URL.
* Corrigida navegação de títulos em documentos HTML contendo caracteres Unicode multibyte.
* Corrigido uso alto de CPU em documentos com títulos longos devido a uma regressão no wxWidgets.
* Corrigido carregamento de arquivos de texto UTF-8.
* Corrigidos itens TOC aninhados em livros Epub colocando seu cursor na posição errada.
* Corrigido um travamento ao sair do aplicativo em certos casos.
* Adicionada uma caixa de seleção no diálogo de opções para habilitar ou desabilitar quebra de linha!
* Agora é possível fazer uma doação para o desenvolvimento do Paperback, através do novo item doações no menu ajuda ou através do link patrocinar este projeto na parte inferior da página principal do repositório GitHub.
* Documentos Markdown agora sempre terão um título, e o Paperback agora deve ser capaz de carregar praticamente qualquer arquivo Markdown.
* Documentos PDF agora sempre terão um título, mesmo que os metadados estejam faltando.
* Mudado bibliotecas PDF para a usada no Chromium, levando a análise PDF muito mais confiável em todo o conselho.
* Você agora pode ter apenas uma instância do Paperback em execução por vez. Executar paperback.exe com um nome de arquivo enquanto já está em execução abrirá esse documento na instância já em execução.
* Você agora pode pressionar delete em um documento no controle de guia para fechá-lo.

### Versão 0.2.1
* Adicionado o número total de páginas ao rótulo de página no diálogo ir para página.
* Permita tabular do conteúdo do documento para sua lista de documentos abertos.
* Corrigidas pressionadas de tecla de título às vezes abrindo documentos recentes se você tivesse o suficiente deles.
* Paperback agora removerá hífens suaves desnecessários da saída de texto.
* Corrigida navegação de títulos às vezes colocando você no caractere errado.

### Versão 0.2.0
* Adicionado suporte para documentos markdown!
* Adicionado suporte para documentos PDF, incluindo a capacidade de navegar entre páginas!
* Adicionados atalhos de teclado para navegar por títulos em conteúdo HTML, incluindo livros epub e documentos markdown. Esses atalhos foram projetados para funcionar semelhante a um leitor de tela.
* Corrigido carregamento de epubs com nomes de arquivo codificados em URL em seus manifestos.
* Corrigido carregamento de livros epub 3 com XHTML incorporado neles.
* Uma mensagem agora é falada se o documento não suportar sumário ou seções, em oposição aos itens de menu sendo desabilitados.
* Adicionado um menu de documentos recentes! Atualmente armazena seus últimos 10 documentos abertos, e pressionar enter em um os abrirá para leitura.
* Completamente reescrito o diálogo Localizar, tornando-o muito mais simples de usar, enquanto também adicionava um histórico de seus últimas 25 pesquisas e suporte a expressões regulares!
* Documentos abertos anteriormente agora são lembrados nas reinicializações do aplicativo. Isso é configurável através do novo item de opções no menu ferramentas.
* Adicionado shift+f1 para abrir o readme diretamente no Paperback.

### Versão 0.1.0
* Lançamento inicial.
