<!-- machine-translated from doc/readme.md (source-hash: bdf582cc25a739ea); please review and edit as needed -->

# Paperback - versão 0.9.0

## Introdução

O Paperback é um leitor de ebooks e documentos leve, rápido e acessível para todos, desde leitores casuais até usuários avançados. Foi projetado para acessibilidade com leitores de tela, alta velocidade e uma experiência livre de excessos.

## Requisitos do sistema

O Paperback atualmente funciona no Windows 10/11 e em todas as versões modernas do macOS ARM. Aplicativos nativos para iOS e Android estão em desenvolvimento ativo, com versões de teste públicas previstas logo após o lançamento da versão 0.9.0 para desktop, antes de um lançamento unificado 1.0 abrangendo todas as quatro plataformas.

## Funcionalidades

* Completamente independente, não exigindo a instalação de nenhum software no seu computador para começar a ler.
* Incrivelmente rápido, mesmo em hardware antigo.
* Interface simples com abas, permitindo abrir quantos documentos você quiser lado a lado.
* Salva sua posição exata de leitura em todos os documentos que você abre.
* Opcionalmente, lembra quais documentos estavam abertos quando você fechou o programa e os restaura na próxima inicialização.
* Inclui funcionalidades de navegação semelhantes às encontradas no modo de navegação web de muitos leitores de tela, para navegar pelos documentos de forma rápida e fácil.
* Inclui uma caixa de diálogo de busca robusta, com recursos como histórico e suporte a expressões regulares.
* Pode ser executado de forma totalmente portátil ou instalado com as associações de arquivos configuradas automaticamente.
* Suporta uma enorme variedade de formatos de arquivo comuns.

## Compatibilidade com leitores de tela

O Paperback funciona bem com todos os principais leitores de tela. Há, no entanto, um problema conhecido para usuários do JAWS.

### JAWS e linhas braille

Se você usa o JAWS com uma linha braille, pode notar que parágrafos longos são truncados ao avançar com as teclas de navegação do seu display. O comando de ler o parágrafo atual também é afetado. Isso é um bug na forma como o JAWS lida com o controle de texto RICHEDIT50W, não algo do próprio Paperback, e que levou bastante tempo para ter uma solução divulgada, dado o entusiasmo da Vispero em responder a problemas com software de código aberto.

A solução alternativa, finalmente divulgada no grupo de discussão do JAWS após meses de espera, é editar o `paperback.jcf` e definir "Braille Presentation and Panning" como "Always use DOM if available". Você também vai querer habilitar "Pan Text by Paragraph", caso contrário seu display permanecerá no parágrafo ativo em vez de avançar. Com ambas as configurações aplicadas, a navegação pelo display deve funcionar corretamente.

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
* Apresentações do PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Documentos RTF (`.rtf`)
* Arquivos de texto simples e de log (`.txt`, `.log`)

## Atalhos de teclado

O Paperback foi projetado para uso prioritário pelo teclado. Aqui estão os atalhos atuais.

Os atalhos abaixo são para Windows. Onde o macOS difere, o equivalente é indicado entre parênteses — principalmente porque Ctrl+G, Ctrl+W e Alt+Esquerda/Direita já são usados por outras convenções do sistema ou de aplicativos nessa plataforma.

### Menu Arquivo

* `Ctrl+O`: Abrir um documento.
* `Ctrl+F4` (macOS: `Cmd+W`): Fechar o documento atual.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Fechar todos os documentos abertos.
* `Ctrl+Shift+T`: Reabrir o último documento fechado.
* `Ctrl+R`: Mostrar a caixa de diálogo "Todos os Documentos" (a partir de Documentos Recentes).
* `Ctrl+Q`: Sair (somente Windows; no macOS isso fica no menu do aplicativo).

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
* `Shift+1` até `Shift+6`: Título anterior de nível 1-6.
* `1` até `6`: Próximo título de nível 1-6.
* `Shift+P`: Página anterior.
* `P`: Próxima página.
* `Shift+B`: Marcador anterior.
* `B`: Próximo marcador.
* `/`: Definir seu marcador temporário.
* `\`: Ir para seu marcador temporário.
* `Shift+N`: Nota anterior.
* `N`: Próxima nota.
* `Ctrl+B`: Ir para todos os marcadores e notas.
* `Ctrl+Alt+B`: Ir apenas para os marcadores.
* `Ctrl+Alt+M`: Ir apenas para as notas.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, isto é, a tecla Control física em vez de Cmd): Ver o texto da nota na posição atual.
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
* `,`: Ir para depois do fim do contêiner atual (lista ou tabela).

### Menu Ferramentas

* `Ctrl+W` (macOS: `RawCtrl+W`, isto é, a tecla Control física em vez de Cmd): Mostrar a contagem de palavras do documento atual.
* `Ctrl+I`: Mostrar informações do documento.
* `Ctrl+T`: Mostrar sumário.
* `F7`: Mostrar lista de elementos.
* `Ctrl+Shift+C`: Abrir a pasta que contém o arquivo.
* `Ctrl+Shift+V`: Abrir o conteúdo atual na Visualização Web.
* `Ctrl+U`: Ver o código-fonte do documento em uma nova aba.
* `Ctrl+Shift+E`: Exportar dados do documento (`.paperback`).
* `Ctrl+Shift+I`: Importar dados do documento (`.paperback`).
* `Ctrl+E`: Exportar o documento atual como texto simples.
* `Ctrl+Shift+B`: Alternar marcador na seleção/cursor atual.
* `Ctrl+Shift+N`: Adicionar ou editar a nota do marcador na seleção/cursor atual.
* `Ctrl+Alt+W`: Alternar quebra automática de linha.
* `Ctrl+Space`: Reproduzir/pausar a narração em áudio.
* `'`: Avançar na narração em áudio.
* `;`: Retroceder na narração em áudio.
* `Ctrl+'`: Aumentar o intervalo de avanço do áudio.
* `Ctrl+;`: Diminuir o intervalo de avanço do áudio.
* `F11` (macOS: `RawCtrl+Ctrl+F`, isto é, Control+Command+F): Alternar tela cheia.
* `Ctrl+,`: Abrir as opções (macOS: Preferências, no menu do aplicativo).
* `Ctrl+Shift+S`: Alternar o temporizador de dormir.

### Menu Ajuda

* `Ctrl+F1`: Mostrar a caixa de diálogo Sobre.
* `F1`: Ver a ajuda no seu navegador padrão.
* `Shift+F1`: Ver a ajuda no Paperback.
* `Ctrl+Shift+U`: Verificar atualizações.
* `Ctrl+D`: Abrir a página de doações no seu navegador padrão.

### Teclas adicionais da visualização de documentos

* `Delete` / `Numpad Delete` no controle de abas: Fechar a aba do documento selecionado.
* `Enter` ou `Space` no texto do documento: Ativar o link no cursor ou abrir uma visualização de tabela quando estiver em um marcador de tabela.
* `Shift+F10` ou a tecla Menu/Aplicativo no texto do documento: Abrir o menu de contexto.

## Idiomas suportados

O Paperback é traduzido para muitos idiomas diferentes, e mais são adicionados constantemente. Uma lista completa segue abaixo.

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
* Chinês simplificado
* Sérvio
* Espanhol
* Vietnamita

## Créditos
### Desenvolvimento
* Quin Gillespie: desenvolvedor principal e fundador do projeto.
* Aryan Choudhary: contribuidor principal.

### Doações
As pessoas a seguir fizeram doações de algum valor para o desenvolvimento do Paperback. Se você fizer uma doação, seu nome não será adicionado aqui automaticamente; eu só adiciono pessoas que desejam que sua doação seja divulgada.

Observação: considero que ser patrocinador público no GitHub é motivo para inclusão automática nesta lista.

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

### Versão 0.9.0

#### Adicionado

##### Geral
* Uma ferramenta de linha de comando, chamada pb, para converter rapidamente qualquer um dos formatos suportados pelo Paperback para HTML, Markdown ou texto simples.
* Uma opção para recarregar documentos que foram modificados por outros programas no disco.
* Uma opção Ver Código-Fonte para abrir o código-fonte de um documento em uma nova aba, útil para editar Markdown, por exemplo.
* O texto dos documentos agora é paginado, o que significa que você pode carregar livros com dezenas de milhões de palavras em apenas alguns segundos. Por favor, relate qualquer estranheza encontrada nisso.

##### Suporte a plataformas
* Suporte ao Windows ARM64!
* Suporte nativo ao macOS!
* Uma alternância de tela cheia.

##### Caixa de diálogo Todos os Documentos
* Um botão para localizar livros ausentes que apenas mudaram de caminho.
* Um filtro de status e uma barra de status, para que você possa filtrar por status do documento e ver quantos documentos estão exibidos e selecionados.
* O atalho `Ctrl+Shift+A` para desmarcar todos os documentos.

##### Opções e legibilidade
* Uma aba de legibilidade, com as seguintes opções:
    * Quebra automática de linha (movida de geral);
    * Renderizar tabelas em linha (novidade nesta versão, veja abaixo);
    * Fonte;
    * Cor de fundo;
    * Espaçamento entre linhas;
    * Espaçamento entre parágrafos;
    * Espaçamento entre letras;
    * Alinhamento do texto.
* Um item de menu para quebra automática de linha e a respectiva tecla de atalho.
* Uma alternância para determinar como você quer que as tabelas sejam exibidas, além da unificação da exibição de tabelas entre os documentos.

##### Navegação
* Suporte para navegação por contêiner.
* Uma opção para mover automaticamente o cursor para o início da linha ao navegar entre linhas, semelhante ao modo de navegação dos leitores de tela.
* O atalho de teclado igual para anunciar sua porcentagem atual dentro de um documento.

##### Marcadores
* Marcadores temporários: você pode ter um por documento, e eles são preservados. Use a barra para definir um e a barra invertida para saltar até ele.

##### Contagem de palavras
* Tempo estimado de leitura na caixa de diálogo de contagem de palavras, além da possibilidade de definir sua velocidade de leitura para tornar essa métrica realmente útil.
* Se uma seleção estiver ativa quando você abrir a caixa de diálogo de contagem de palavras, agora será exibido quantas palavras você selecionou.

##### Atalhos de teclado
* A possibilidade de personalizar todos os atalhos de teclado do aplicativo por meio de uma caixa de diálogo simples.
* Um atalho de teclado configurável para restaurar o Paperback da bandeja do sistema.

##### Idiomas
* Holandês, finlandês e polonês.

##### Exportação
* Expandido o item de menu de exportação para permitir exportar para HTML e Markdown, além de texto simples.

##### Atualizador
* Um botão de cancelar na caixa de diálogo de atualização em andamento.
* O atualizador agora verifica se o arquivo baixado não foi alterado.

##### Visualização web
* A visualização web agora é aberta na sua posição atual de leitura.

##### Livros DAISY
* Suporte para livros DAISY 2.0.
* Suporte para reprodução de áudio DAISY 2.02.

##### Audiolivros
* A possibilidade de reproduzir audiolivros, atualmente com suporte tanto para áudio DAISY (incluindo áudio + texto DAISY) quanto para arquivos zip de arquivos de áudio.
* Atalhos de teclado e itens de menu para reproduzir/pausar a narração, avançar e retroceder, e ajustar o intervalo de busca.
* Opções para sincronizar o cursor de leitura com a reprodução de áudio, definir o intervalo de busca do áudio e escolher se a busca além do final de um capítulo continua no próximo.

##### Documentos CHM
* Suporte para listas, itens de lista, figuras e imagens.

##### PowerPoint
* Documentos do PowerPoint agora suportam tabelas.

#### Corrigido

##### Geral
* Documentos codificados em codificações CJK legadas, como GBK, Big5 e Shift_JIS, agora serão renderizados corretamente em vez de aparecerem como um monte de mojibake.
* "Reabrir último fechado" tentando reabrir o readme incluído.
* Sua aba selecionada não recebendo o foco corretamente após reiniciar o Paperback.
* O tratamento de arquivos em unidades de rede do Windows pelo Paperback: pressionar mostrar arquivo na pasta agora foca corretamente o arquivo no armazenamento de rede, e os caminhos não contêm mais caracteres estranhos.
* Arquivos .paperback não serão mais carregados à força na restauração de documentos; em vez disso, você será solicitado a confirmar quando um for encontrado.
* Abrir pasta que contém o arquivo agora foca o arquivo indicado no explorador.
* Abrir o readme agora respeitará o idioma selecionado.
* A interface do usuário do Paperback agora será dimensionada corretamente em telas de alta DPI.
* O menu agora é atualizado corretamente, e o foco vai para o controle de texto, ao abrir a ajuda no Paperback.
* Mudança para um método muito mais seguro de IPC no Windows.
* O título do documento ativo agora será lido ao alternar entre abas.
* Redução do uso de memória em documentos grandes ao reduzir pela metade o tamanho das tabelas internas de índice por caractere.

##### Caixa de diálogo Todos os Documentos
* Escape não fechando as caixas de diálogo Informações do Documento e Todos os Documentos.
* A barra de título não sendo atualizada após fechar um documento pela caixa de diálogo Todos os Documentos.
* Readme.html não será mais adicionado à sua lista de todos os documentos quando aberto via Shift+F1.
* Remover documentos da caixa de diálogo de recentes agora também fecha sua aba ativa.
* Seu filtro de pesquisa agora é preservado após remover um documento.

##### Navegação
* A navegação por páginas anunciando o texto de linha incorreto em algumas situações.
* Ir para Linha, Ir para Página e Ir para Porcentagem posicionando seu cursor no lugar errado em documentos grandes.
* Localizar e Localizar Próximo não respeitando a janela de documento carregada em documentos grandes.

##### Marcadores
* Os sons de marcador/nota agora devem ser reproduzidos exclusivamente quando você navegar sobre uma palavra que contenha um.

##### Legibilidade
* Aplicar a quebra automática de linha levando você para o início do documento.

##### Visualização web
* A caixa de diálogo da visualização web não podendo ser redimensionada e aparecendo com um tamanho inicial muito pequeno.
* As imagens agora devem ser exibidas corretamente na visualização web incorporada.

##### Atualizador
* O atualizador agora mostra corretamente o conteúdo das tags de código em Markdown nas notas de lançamento.

##### Livros DAISY
* Livros DAISY mostrando informações incorretas na barra de status.
* Carregamento de livros DAISY com declarações de codificação inválidas.

##### Documentos RTF
* A análise de documentos RTF com caracteres não latinos.
* Grupos `\pict` de RTF, para que os dados de imagem incorporados não vazem mais para o texto do documento.

##### Livros Mobi/AZW3
* Âncoras filepos em livros Mobi dividindo tags HTML e colocando lixo no texto do livro.
* Links em livros Mobi legados.
* Grande melhoria na análise de AZW3.

##### Documentos do Word
* Documentos do Word com nomes de estilo específicos de localidade não renderizando seus títulos corretamente.

##### Documentos HTML/XHTML
* Elementos dl, dt e dd não produzindo quebras de linha em documentos XHTML.

##### Documentos PDF
* O Paperback agora recorre à extração de texto simples para PDFs marcados incorretamente.
* Documentos PDF contendo caracteres de controle em seus títulos e/ou marcadores não travarão mais o Paperback ao abrir.

### Versão 0.8.5
* Adicionado suporte a páginas em livros epub.
* Adicionado suporte a documentos do Microsoft Office criptografados. Atualmente são suportados Word legado, Word moderno e PowerPoint moderno, com PowerPoint legado planejado para o futuro.
* Adicionado suporte a documentos legados do Microsoft Word (*.doc)!
* Adicionado suporte a apresentações legadas do PowerPoint (*.ppt)!
* Adicionado suporte a livros mobi e AZW3!
* Adicionado suporte a arquivos PDF marcados!
* Adicionado o atalho ctrl+q para sair do aplicativo.
* Adicionado suporte a livros compactados do Bookshare (tanto DAISY quanto Word)!
* O texto alternativo de imagens incorporadas agora deve ser exibido corretamente.
* Documentos CHM agora suportam corretamente a navegação por links internos.
* Corrigido os sons de marcador sendo acionados no início do parágrafo em vez da posição do marcador.
* Corrigido o ir para página estar deslocado em 1.
* Corrigido a tecla escape não funcionando para fechar a caixa de diálogo abrir como.
* Corrigido o menu de contexto do leitor não aparecendo com o clique direito ou a tecla Aplicativos.
* Corrigido o documento errado às vezes receber o foco ao abrir documentos pela linha de comando.
* PDFs apenas com imagens voltam a ser detectados e alertam sobre sua existência.
* Agora é possível navegar por imagens e figuras com g/shift+g e f/shift+f, respectivamente.
* O Paperback agora respeitará a configuração de modo escuro do aplicativo.
* Removido o suporte a DAISY XML, pois não é mais necessário.
* Retorno à navegação nativa do Win32 pela primeira letra na árvore do sumário.
* A caixa de diálogo de erro ao carregar agora mostra mensagens de erro mais detalhadas.
* A visualização web agora abrirá muito mais rápido e suavemente.

### Versão 0.8.2
* Adicionado suporte a páginas em documentos RTF!
* Corrigido um bug em que abrir a visualização web em epubs contendo links externos os ativava automaticamente.
* Corrigido um bug em que o analisador de RTF não colocava um espaço entre palavras em casos raros.
* Corrigido parágrafos sendo divididos em várias linhas curtas em alguns documentos PDF.
* Documentos PDF agora têm suporte básico à navegação por links e títulos!
* Tabulações e saltos de linha em RTF agora são renderizados exatamente como aparecem no documento.
* Retorno à consagrada biblioteca pdfium para analisar PDFs, tornando a renderização de PDF muito mais confiável novamente.

### Versão 0.8.1
* Adicionado Ctrl+Shift+T para reabrir o último documento fechado.
* A caixa de diálogo Todos os Documentos agora suporta a seleção de vários documentos para abrir de uma vez.
* Corrigidos alguns bugs no analisador de RTF.
* Corrigido caminhos de arquivo contendo caracteres não ASCII (como š, č, ć, ž do bósnio) sendo corrompidos ao abrir um arquivo por meio de uma segunda instância do Paperback.
* Corrigido o texto de PDF sendo lido na ordem errada e o espaçamento incorreto em torno de palavras em maiúsculas.
* Corrigido o carregamento lento de documentos ao abrir arquivos grandes.
* Corrigida a localização dos botões Sim/Não nas caixas de diálogo de confirmação.

### Versão 0.8.0
* Adicionadas traduções para japonês, chinês simplificado e vietnamita!
* Adicionado um atualizador automático que agora substituirá a versão do Paperback instalada em vez de apenas baixar a nova versão!
* Adicionado retorno sonoro opcional ao alcançar um marcador ou uma nota, obrigado a Andre Louis pelos sons!
* Adicionado suporte a documentos RTF!
* Adicionado suporte a documentos DAISY XML.
* Adicionado suporte a arquivos Flat Open Document Text!
* Adicionado suporte a apresentações Flat Open Document!
* Adicionado suporte a separadores com s e shift+s.
* Qualquer movimento maior que 300 caracteres agora será adicionado automaticamente ao seu histórico de navegação.
* Corrigida a restauração da janela do Paperback a partir da bandeja do sistema.
* Corrigido documentos Markdown mostrando texto bruto em vez de HTML renderizado na Visualização Web.
* Corrigido tabelas não sendo renderizadas corretamente em arquivos Markdown.
* PDFs apenas com imagens agora avisarão sobre sua existência quando você tentar carregar um.
* Agora é possível verificar novas versões de desenvolvimento em vez de lançamentos estáveis ao procurar atualizações.
* Informações de versão devidamente incorporadas ao executável do Paperback.
* A caixa de diálogo de opções foi dividida em abas para facilitar o uso e a navegação.
* Mudança para o Hayro na análise de PDFs, resultando em mais confiabilidade, velocidade e menos DLLs.
* Todo o aplicativo foi reescrito em Rust. A nova base de código é mais segura, carrega documentos mais rapidamente e é mais fácil de manter e estender.
* O menu de contexto do controle de texto agora incluirá ações específicas do leitor em vez de itens genéricos como cortar e colar.

### Versão 0.7.0
* Adicionado suporte a tabelas em documentos baseados em HTML e XHTML! Navegue entre tabelas usando T e Shift+T, e pressione Enter para ver uma em uma visualização web.
* Adicionado um recurso básico de renderização web! Pressione Ctrl+Shift+V para abrir a seção atual do seu documento em um renderizador baseado na web, útil para conteúdos como formatação complexa ou exemplos de código.
* Adicionada uma tradução para russo, obrigado Ruslan Gulmagomedov!
* Adicionado um botão Limpar Tudo à caixa de diálogo Todos os Documentos.
* O verificador de atualizações agora exibe as notas de lançamento quando uma nova versão está disponível.
* Corrigida a restauração da janela a partir da bandeja do sistema.
* Corrigidas as traduções dos botões Sim/Não nas caixas de diálogo de confirmação.
* Corrigido o carregamento das configurações ao executar como administrador.
* Corrigido o tratamento de comentários em documentos XML e HTML.
* Corrigida a análise do sumário em livros Epub 2.
* Corrigida a navegação para o próximo item com a mesma letra no sumário.
* Corrigida a caixa de diálogo de localizar não sendo ocultada corretamente ao usar os botões próximo/anterior.
* Corrigidos sumários de epub que ocasionalmente levavam você ao item errado.
* Corrigidos diversos problemas de tratamento de espaços em branco em XML, HTML e tags pre.
* Corrigido erro de deslocamento de um na navegação por links.
* Corrigido alguns livros terem espaços em branco no final de suas linhas.
* Corrigidos vários problemas nos analisadores.
* Itens de menu relacionados a marcadores, bem como a lista de elementos, agora são corretamente desativados quando nenhum documento está aberto.
* Melhorado o tratamento de listas em vários formatos de documento.
* Melhorado o fluxo de trabalho de tradução para colaboradores.
* Muitas refatorações internas, movendo a maior parte da lógica de negócios do aplicativo de C++ para Rust para melhorar o desempenho e a manutenibilidade.

### Versão 0.6.1
* Adicionado suporte a PDFs protegidos por senha!
* Adicionado um recurso bem básico de ir para a posição anterior/seguinte. Se você pressionar enter em um link interno e ele mover seu cursor, essa posição agora será lembrada e poderá ser acessada com alt+setas esquerda/direita.
* Adicionada uma lista de elementos! Atualmente ela mostra apenas uma árvore de todos os títulos do seu documento ou uma lista de links, mas há planos de expandi-la no futuro.
* Adicionada uma opção para iniciar o Paperback maximizado por padrão.
* Corrigidos links em alguns documentos Epub que não funcionavam corretamente.
* Corrigida a análise de sumários de Epub contendo caminhos relativos.
* Corrigido alguns documentos epub não mostrarem título ou autor.
* Corrigido os títulos de alguns capítulos de epub não aparecerem corretamente na caixa de diálogo do sumário.
* Corrigido você não conseguir usar a barra de espaço para acionar os botões OK/cancelar na caixa de diálogo do sumário.
* Melhorado o tratamento de títulos em documentos do Word.
* Você agora receberá um retorno falado se a lista de documentos recentes estiver vazia ao tentar abrir a caixa de diálogo.

### Versão 0.6.0
* Uma nova opção para mostrar o menu ir de forma bem mais compacta foi adicionada à caixa de diálogo de opções, marcada por padrão.
* Adicionada uma opção para fazer a navegação por elementos estruturais dar a volta.
* Adicionada uma opção no menu de ferramentas para abrir a pasta que contém o documento atualmente em foco.
* Adicionado um sistema de atualização bem simples, mas muito eficaz.
* Adicionado um recurso básico de temporizador de sono, acessível com Ctrl+Shift+S.
* Adicionado suporte à análise de ebooks FB2!
* Adicionado suporte à análise de apresentações OpenDocument!
* Adicionado suporte à análise de arquivos OpenDocument Text!
* Marcadores agora podem ser feitos para marcar uma linha inteira ou apenas algum texto específico. Se você não tiver nenhuma seleção ativa ao criar um marcador, o comportamento é como antes da 0.6, e ele marcará a linha inteira. No entanto, se você selecionar algum texto, apenas esse texto será incluído no marcador.
* Marcadores agora podem ter notas de texto opcionais anexadas a eles! Navegue entre marcadores com notas usando N e Shift+N, ou abra a caixa de diálogo de marcadores com todos os marcadores, apenas notas ou apenas não notas selecionados com teclas de atalho específicas.
* Os marcadores na caixa de diálogo de marcadores não terão mais um irritante prefixo "marcador x".
* Livros Epub contendo conteúdo HTML que finge ser XML agora serão tratados corretamente.
* Corrigido o carregamento de documentos Markdown grandes.
* Corrigido pressionar espaço na árvore do sumário acionando o botão OK.
* Corrigido o tratamento de espaços em branco no início de tags pre em documentos HTML e XHTML.
* Corrigido o controle de texto não recuperando o foco às vezes ao retornar à janela do Paperback.
* Corrigido o campo de texto da caixa de diálogo ir para porcentagem não atualizando o valor do controle deslizante.
* Corrigida a renderização de IDs HTML personalizados em documentos Markdown.
* HTML dentro de blocos de código Markdown agora será renderizado corretamente.
* Se você carregar um livro com um parâmetro de linha de comando enquanto uma instância do Paperback já estiver em execução, não receberá mais um erro se o carregamento do documento levar mais de 5 segundos.
* Se o Paperback estiver sendo executado como administrador, a configuração agora será carregada e salva corretamente.
* Agora é possível excluir um marcador diretamente pela caixa de diálogo de marcadores.
* Agora é possível importar e exportar seus marcadores e a posição de leitura de um documento específico. O arquivo gerado recebe o nome do arquivo com a extensão .paperback. Se esse arquivo for encontrado no mesmo diretório de um arquivo ao carregá-lo, será carregado automaticamente. Caso contrário, você pode importá-los manualmente usando um item no menu de ferramentas.
* Links dentro de documentos agora são totalmente suportados! Use k e shift+k para avançar e retroceder por eles, e pressione enter para abrir/ativar um.
* Muitas refatorações internas, tornando o aplicativo mais rápido e o binário menor.
* O conteúdo Markdown agora é pré-processado para ficar em conformidade com o CommonMark antes da renderização.
* A navegação por listas e seus itens agora é totalmente suportada! Use L e Shift+L para navegar pelas listas em si, e I e Shift+I para percorrer os itens de lista.
* O delete do teclado numérico agora funciona para remover documentos da barra de abas, além do delete normal.
* O Paperback agora pode opcionalmente minimizar para a bandeja do sistema! Essa opção está desativada por padrão, mas ativá-la fará com que a opção minimizar no menu do sistema coloque o Paperback na bandeja, podendo ser restaurado clicando no ícone criado.
* O Paperback agora é totalmente traduzível! A lista de idiomas suportados é atualmente bem pequena, mas está crescendo constantemente!
* O Paperback agora tem um site oficial, em [paperback.dev](https://paperback.dev)!
* Documentos PPTX agora exibirão um sumário básico, contendo todos os slides.
* O caminho completo do documento aberto agora será exibido na caixa de diálogo de informações do documento.
* O instalador agora inclui uma opção para ver o readme no seu navegador após a instalação.
* A lista de documentos recentes foi ampliada drasticamente! Em vez de mostrar simplesmente os últimos 10 documentos abertos, ela agora mostrará um número personalizável, com os demais documentos que você já abriu acessíveis por meio de uma pequena caixa de diálogo.
* Várias pequenas melhorias nos analisadores em geral, incluindo colocar uma linha em branco entre slides em apresentações PPTX, corrigir o tratamento de novas linhas dentro de parágrafos em documentos do Word e adicionar marcadores aos itens de lista.

### Versão 0.5.0
* Adicionado suporte a documentos do Microsoft Word!
* Adicionado suporte a apresentações do PowerPoint!
* Corrigido certos itens de menu não sendo desativados sem documentos abertos.
* Corrigida a orientação do controle deslizante de ir para porcentagem.
* Corrigido o sumário em livros Epub com caminhos de arquivo e/ou IDs de fragmento codificados em URL.
* Corrigido espaços em branco sendo removidos de títulos XHTML de formas estranhas.
* Corrigido o tratamento de espaços em branco dentro de tags pre aninhadas em documentos HTML.
* Documentos HTML e Markdown agora suportam o recurso de sumário! Quando você carrega um documento HTML/Markdown, o Paperback construirá seu próprio sumário a partir da estrutura dos títulos do seu documento e o exibirá na caixa de diálogo ctrl+t.
* Documentos HTML agora terão o título definido na tag title, se existir. Caso contrário, continuarão usando o nome do arquivo sem a extensão.
* Mudança do UniversalSpeech para o uso de uma região dinâmica para reportar a fala. Isso significa que nenhuma DLL de leitor de tela é mais distribuída junto com o programa, e mais leitores de tela agora serão suportados, como o Narrador da Microsoft.
* Troca de bibliotecas zip para permitir a abertura de uma gama maior de livros epub.
* A caixa de diálogo que pergunta se você quer abrir o documento como texto simples foi completamente refeita, e agora permite abrir seu documento como texto simples, HTML ou Markdown.
* A caixa de diálogo ir para porcentagem agora inclui um campo de texto que permite inserir manualmente uma porcentagem para saltar.
* O analisador de HTML agora reconhecerá dd, dt e dl como elementos de lista.
* O sumário em livros Epub voltará a ser preservado exatamente.
* O espaço não separável Unicode agora é considerado ao remover linhas em branco.
* Você não será mais perguntado como quer abrir um arquivo não reconhecido a cada vez que o carregar, apenas na primeira vez.

### Versão 0.4.1
* Adicionado um ícone opcional no menu iniciar ao instalador.
* O sumário agora deve estar mais limpo em alguns casos, por exemplo, se você tiver um item filho e um pai com o mesmo texto na mesma posição, agora verá apenas o item pai.
* Corrigido o sumário em certos documentos CHM.
* Corrigido o sumário em livros Epub 3 com caminhos absolutos.
* Documentos CHM agora devem mostrar o título definido no arquivo de metadados.

### Versão 0.4.0
* Adicionado suporte a arquivos CHM!
* Adicionado suporte a marcadores! Você pode ter quantos marcadores quiser em quantos documentos quiser. Você pode saltar para frente e para trás entre eles com b e shift+b, definir um com control+shift+b e abrir uma caixa de diálogo para ir a um marcador específico com control+b.
* Adicionado um instalador junto com o arquivo zip portátil! O instalador instalará o Paperback no seu diretório Program Files e configurará automaticamente as associações de arquivos para você.
* Arquivos de texto com BOMs agora devem ser decodificados corretamente, e o BOM também não será mais exibido no início do texto.
* Adicionadas muito mais informações à barra de status. Ela agora mostrará sua linha, caractere e porcentagem de leitura atuais.
* Comentários HTML, bem como o conteúdo das tags script e style, não serão mais exibidos na saída de texto.
* Se você passar um caminho relativo ao Paperback na linha de comando, ele agora o resolverá corretamente.
* O movimento por porcentagem agora é tratado por sua própria caixa de diálogo baseada em controle deslizante, acessível com control+shift+g.
* Documentos sem títulos ou autores conhecidos agora sempre terão um padrão.
* A lógica de salvamento de posição agora é muito mais inteligente e deve gravar no disco apenas quando absolutamente necessário.
* O documento que estava em foco quando você fechou o Paperback agora é lembrado entre reinicializações do aplicativo.
* A entrada nas caixas de diálogo ir para linha e ir para página agora deve ser validada de forma mais estrita.
* Corrigida a navegação pelo sumário em livros epub 3 com caminhos relativos em seus manifestos.

### Versão 0.3.0
* Corrigido o sumário em livros epub com manifestos codificados em URL.
* Corrigida a navegação por títulos em documentos HTML contendo caracteres Unicode de múltiplos bytes.
* Corrigido o alto uso de CPU em documentos com títulos longos devido a uma regressão no wxWidgets.
* Corrigido o carregamento de arquivos de texto UTF-8.
* Corrigido itens de sumário aninhados em livros Epub colocando seu cursor na posição errada.
* Corrigido um travamento ao sair do aplicativo em certos casos.
* Adicionada uma caixa de seleção na caixa de diálogo de opções para ativar ou desativar a quebra automática de linha!
* Agora é possível doar para o desenvolvimento do Paperback, seja pelo novo item de doação no menu de ajuda ou pelo link de patrocinar este projeto no final da página principal do repositório do GitHub.
* Documentos Markdown agora sempre terão um título, e o Paperback agora deve ser capaz de carregar praticamente qualquer arquivo Markdown.
* Documentos PDF agora sempre terão um título, mesmo se os metadados estiverem ausentes.
* Mudança das bibliotecas de PDF para a usada no Chromium, resultando em uma análise de PDF muito mais confiável em geral.
* Agora você só pode ter uma instância do Paperback em execução por vez. Executar paperback.exe com um nome de arquivo enquanto ele já está em execução abrirá esse documento na instância já em execução.
* Agora você pode pressionar delete em um documento no controle de abas para fechá-lo.

### Versão 0.2.1
* Adicionado o número total de páginas ao rótulo de página na caixa de diálogo ir para página.
* Permitido tabular do conteúdo do documento para a sua lista de documentos abertos.
* Corrigido as teclas de navegação por títulos às vezes abrirem documentos recentes se você tivesse muitos deles.
* O Paperback agora removerá hifens opcionais desnecessários da saída de texto.
* Corrigida a navegação por títulos às vezes colocando você no caractere errado.

### Versão 0.2.0
* Adicionado suporte a documentos markdown!
* Adicionado suporte a documentos PDF, incluindo a possibilidade de navegar entre páginas!
* Adicionadas teclas de atalho para navegar por títulos em conteúdo HTML, incluindo livros epub e documentos markdown. Essas teclas foram projetadas para funcionar de forma semelhante a um leitor de tela.
* Corrigido o carregamento de epubs com nomes de arquivo codificados em URL em seus manifestos.
* Corrigido o carregamento de livros epub 3 com XHTML incorporado neles.
* Uma mensagem agora é falada se o documento não suportar sumário ou seções, em vez de os itens de menu serem desativados.
* Adicionado um menu de documentos recentes! Atualmente ele armazena seus últimos 10 documentos abertos, e pressionar enter em um deles o abrirá para leitura.
* A caixa de diálogo Localizar foi completamente reescrita, tornando-a muito mais simples de usar, além de adicionar um histórico das suas últimas 25 pesquisas e suporte a expressões regulares!
* Documentos abertos anteriormente agora são lembrados entre reinicializações do aplicativo. Isso é configurável pelo novo item de opções no menu de ferramentas.
* Adicionado shift+f1 para abrir o readme diretamente no próprio Paperback.

### Versão 0.1.0
* Lançamento inicial.
