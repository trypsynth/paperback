<!-- machine-translated from doc/readme.md (source-hash: 88d5313cd5871ed4); please review and edit as needed -->

# Brochura - versão 0.8.5 {#paperback---version-0.8.5}

## Introdução {#introduction}

O Paperback é um leitor de e-books e documentos leve, rápido e acessível
para todos, desde leitores casuais até usuários avançados. Ele foi
projetado para oferecer acessibilidade a leitores de tela, alta
velocidade e uma experiência sem excessos.

## Requisitos do sistema {#system-requirements}

Atualmente, o Paperback roda no Windows, macOS, iOS e Android.

## Recursos {#features}

-   Totalmente independente, não requer a instalação de nenhum software
    no seu computador para começar a ler.
-   Incrivelmente rápido, mesmo em equipamentos antigos.
-   Interface simples com abas, permitindo que você abra quantos
    documentos quiser lado a lado.
-   Salva sua posição exata de leitura em todos os documentos que você
    abrir.
-   Opcionalmente, lembra quais documentos você tinha abertos quando
    fechou o programa e os restaura na próxima vez que for iniciado.
-   Inclui funcionalidades de navegação semelhantes às encontradas no
    modo de navegação na web de muitos leitores de tela, para navegar de
    forma rápida e fácil pelos documentos.
-   Inclui uma caixa de diálogo de busca robusta, com recursos como
    histórico e suporte a expressões regulares.
-   Pode ser executado de forma totalmente portátil ou instalado com
    associações de arquivos configuradas automaticamente.
-   Oferece suporte a uma ampla variedade de formatos de arquivo comuns.

## Compatibilidade com leitores de tela {#screen-reader-compatibility}

O Paperback funciona bem com todos os principais leitores de tela. Há,
no entanto, um problema conhecido para usuários do JAWS.

### JAWS e visores de braille {#jaws-and-braille-displays}

Se você usa o JAWS com um visor Braille, poderá perceber que parágrafos
longos são truncados ao avançar com as teclas de navegação do seu visor.
O comando "ler o parágrafo atual" também é afetado. Trata-se de um bug
no tratamento do controle de texto RICHEDIT50W pelo JAWS, e não de algo
no próprio Paperback, e que demorou bastante tempo para que uma correção
fosse disponibilizada, considerando o entusiasmo da Vispero em responder
a problemas relacionados a software de código aberto.

A solução alternativa, que finalmente surgiu no grupo de discussão do
JAWS após meses de espera, é editar `paperback.jcf` e definir
"Apresentação e deslocamento em braille" como "Sempre usar DOM, se
disponível". Você também deve ativar "Deslocar texto por parágrafo";
caso contrário, seu monitor permanecerá no parágrafo ativo em vez de
avançar. Com ambas as configurações ativadas, o deslocamento deve
funcionar corretamente.

## Tipos de arquivos atualmente suportados {#currently-supported-file-types}

O Paperback suporta os seguintes formatos e extensões:

-   Arquivos de ajuda CHM (`.chm`)
-   livros DAISY (`.opf`, `.zip`)
-   Livros EPUB (`.epub`)
-   e-books FB2 (`.fb2`)
-   Documentos HTML (`.htm`, `.html`, `.xhtml`)
-   Documentos Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`,
    `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
-   Documentos do Microsoft Word (`.docx`, `.docm`, `.doc`)
-   Livros MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
-   Apresentações OpenDocument (`.odp`, `.fodp`)
-   Arquivos de texto do OpenDocument (`.odt`, `.fodt`)
-   Documentos em PDF (`.pdf`)
-   Apresentações do PowerPoint (`.pptx`, `.pptm`, `.ppt`)
-   Documentos RTF (`.rtf`)
-   Textos simples e arquivos de log (`.txt`, `.log`)

## Atalhos de teclado {#keyboard-shortcuts}

O Paperback foi projetado para uso prioritário do teclado. Aqui estão os
atalhos atuais.

Os atalhos abaixo são para o Windows. Nos casos em que o macOS difere, o
equivalente é indicado entre parênteses --- principalmente porque
Ctrl+G, Ctrl+W e Alt+Seta para a Esquerda/Direita já são utilizados por
outras convenções do sistema ou de aplicativos nessa plataforma.

### Menu Arquivo {#file-menu}

-   `Ctrl+O`: Abre um documento.
-   `Ctrl+F4` (macOS: `Cmd+W`): Fechar o documento atual.
-   `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Fechar todos os documentos
    abertos.
-   `Ctrl+Shift+T`: Reabrir o último documento fechado.
-   `Ctrl+R`: Exibir a caixa de diálogo "Todos os documentos" (a partir
    de "Documentos recentes" ).
-   `Ctrl+Q`: Sair (somente no Windows; no macOS, essa opção fica no
    menu do aplicativo).

### Menu "Ir" {#go-menu}

-   `Ctrl+F`: Exibir a caixa de diálogo "Localizar".
-   `F3` (macOS: `Cmd+G`): Localizar o próximo.
-   `Shift+F3` (macOS: `Cmd+Shift+G`): Localizar anterior.
-   `Ctrl+G` (macOS: `Cmd+L`): Ir para a linha.
-   `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Ir para porcentual.
-   `Ctrl+P`: Ir para a página (quando compatível com o documento
    atual).
-   `Alt+Left` (macOS: `Cmd+[`): Voltar no histórico de navegação.
-   `Alt+Right` (macOS: `Cmd+]`): Avançar no histórico de navegação.
-   `[`: Seção anterior.
-   `]`: Próxima seção.
-   `Shift+H`: Título anterior.
-   `H`: Título seguinte.
-   `Shift+1` até `Shift+6`: Título anterior nos níveis 1 a 6.
-   `1` até `6`: Próximo título nos níveis 1 a 6.
-   `Shift+P`: Página anterior.
-   `P`: Próxima página.
-   `Shift+B`: Marcador anterior.
-   `B`: Próximo marcador.
-   `Shift+N`: Nota anterior.
-   `N`: Próxima nota.
-   `Ctrl+B`: Ir para todos os marcadores e notas.
-   `Ctrl+Alt+B`: Ir apenas para os marcadores.
-   `Ctrl+Alt+M`: Ir diretamente para as notas.
-   `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, ou seja, a tecla Control
    física em vez da tecla Cmd): Exibir o texto da nota na posição
    atual.
-   `Shift+K`: Link anterior.
-   `K`: Link seguinte.
-   `Shift+G`: Imagem anterior.
-   `G`: Próxima imagem.
-   `Shift+F`: Figura anterior.
-   `F`: Próxima figura.
-   `Shift+T`: Tabela anterior.
-   `T`: Próxima tabela.
-   `Shift+S`: Separador anterior.
-   `S`: Separador seguinte.
-   `Shift+L`: Lista anterior.
-   `L`: Próxima lista.
-   `Shift+I`: Item anterior da lista.
-   `I`: Próximo item da lista.
-   `Shift+,`: Ir para o início do contêiner atual (lista ou tabela).
-   `,`: Ir além do final do contêiner atual (lista ou tabela).

### Menu Ferramentas {#tools-menu}

-   `Ctrl+W` (macOS: `RawCtrl+W`, ou seja, a tecla Control física em vez
    da tecla Cmd): Mostrar a contagem de palavras do documento atual.
-   `Ctrl+I`: Mostrar informações do documento.
-   `Ctrl+T`: Mostrar índice.
-   `F7`: Mostrar lista de elementos.
-   `Ctrl+Shift+C`: Abrir a pasta contida.
-   `Ctrl+Shift+V`: Abrir o conteúdo atual na Visualização da Web.
-   `Ctrl+U`: Visualizar o código-fonte do documento em uma nova aba.
-   `Ctrl+Shift+E`: Exportar dados do documento (`.paperback`).
-   `Ctrl+Shift+I`: Importar dados do documento (`.paperback`).
-   `Ctrl+E`: Exportar o documento atual como texto simples.
-   `Ctrl+Shift+B`: Marcar ou desmarcar como favorito na seleção/cursor
    atual.
-   `Ctrl+Shift+N`: Adicionar ou editar nota de marcador na
    seleção/cursor atual.
-   `Ctrl+Alt+W`: Ativar/desativar quebra automática de linha.
-   `Ctrl+,`: Abrir opções (macOS: Preferências, no menu do aplicativo
    ).
-   `Ctrl+Shift+S`: Ativar/desativar o temporizador de suspensão.

### Menu Ajuda {#help-menu}

-   `Ctrl+F1`: Exibir a caixa de diálogo "Sobre".
-   `F1`: Visualizar a ajuda no seu navegador padrão.
-   `Shift+F1`: Visualizar a ajuda no Paperback.
-   `Ctrl+Shift+U`: Verificar se há atualizações.
-   `Ctrl+D`: Abrir a página de doações no seu navegador padrão .

### Teclas adicionais para visualização de documentos {#additional-document-view-keys}

-   `Delete` / `Numpad Delete` no controle de abas: Fechar a aba do
    documento selecionado.
-   `Enter` ou `Space` no texto do documento: Ativar o link no cursor ou
    abrir uma visualização de tabela quando estiver sobre um marcador de
    tabela.
-   `Shift+F10` ou a tecla Menu/Aplicativo no texto do documento : Abre
    o menu de contexto.

## Idiomas suportados {#supported-languages}

O Paperback está traduzido para diversos idiomas, com mais sendo
adicionados constantemente. Segue abaixo a lista completa.

Para saber como contribuir, leia nosso [Guia de
Tradução](translating.md).

-   Bósnio
-   Tcheco
-   Holandês
-   Finlandês
-   Francês
-   Alemão
-   Japonês
-   Polonês
-   Português (Brasil)
-   Russo
-   Chinês simplificado
-   Sérvio
-   Espanhol
-   Vietnamita

## Créditos {#credits}

### Desenvolvimento {#development}

-   Quin Gillespie: desenvolvedor principal e fundador do projeto.
-   Aryan Choudhary: colaborador principal.

### Doações {#donations}

As seguintes pessoas fizeram doações de valor significativo para o
desenvolvimento do Paperback. Se você fizer uma doação, seu nome não
será automaticamente adicionado aqui; eu só incluo pessoas que desejam
que sua doação seja divulgada publicamente.

Observação: considero um patrocínio público no GitHub motivo para
inclusão automática nesta lista.

-   Alex Hall
-   Brandon McGinty
-   Brian Hartgen
-   Debbie Yuille
-   Devin Prater
-   Felix Steindorff
-   Hamish Mackenzie
-   James Scholes
-   Jayson Smith
-   Jonathan Rodriguez
-   Jonathan Schuster
-   Keao Wright
-   Michael Marshall
-   Pratik Patel
-   Roberto Perez
-   Sean Randall
-   Timothy Wynn
-   Tyler Rodick

## Registro de alterações {#changelog}

### Versão 0.9.0 (ainda não lançada) {#version-0.9.0-unreleased}

-   Adicionado um botão "Cancelar" à caixa de diálogo de atualização em
    andamento.
-   Adicionada uma ferramenta de linha de comando, chamada pb, para
    converter rapidamente qualquer um dos formatos suportados pelo
    Paperback para HTML, Markdown ou texto simples.
-   Adicionado um atalho de teclado configurável para restaurar o
    Paperback a partir da bandeja do sistema.
-   Adicionado um botão "Localizar" à caixa de diálogo "Todos os
    documentos" para localizar livros ausentes cujo caminho tenha sido
    alterado recentemente.
-   Adicionado um filtro de status e uma barra de status à caixa de
    diálogo "Todos os documentos", para que você possa filtrar por
    status do documento e ver quantos documentos estão sendo exibidos e
    selecionados.
-   Adicionado o `Ctrl+Shift+A` atalho para desmarcar todos os
    documentos na caixa de diálogo "Todos os documentos".
-   Adicionada uma guia "Legibilidade" à caixa de diálogo "Opções", com
    as seguintes opções:
    -   Quebra automática de linha (transferida da seção "Geral");
    -   Exibir tabelas em linha (novidade nesta versão, veja abaixo);
    -   Fonte;
    -   Cor de fundo;
    -   Espaçamento entre linhas;
    -   Espaçamento entre parágrafos;
    -   Espaçamento entre letras;
    -   Alinhamento do texto.
-   Adicionamos um botão para definir como você deseja que as tabelas
    sejam exibidas e unificamos a forma como as tabelas são exibidas em
    todos os documentos.
-   Adicionada a opção "Exibir código-fonte" para abrir o código-fonte
    de um documento em uma nova aba, útil para editar Markdown, por
    exemplo.
-   Adicionado o tempo estimado de leitura à caixa de diálogo de
    contagem de palavras, bem como a possibilidade de definir sua
    velocidade de leitura para tornar essa métrica realmente útil.
-   Adicionado suporte ao Windows ARM64!
-   Adicionado suporte ao Android!
-   Adicionado suporte ao iOS!
-   Adicionado suporte ao macOS!
-   Adicionados novos idiomas: holandês, finlandês e polonês.
-   Adicionado suporte para navegação por contêiner.
-   Adicionado suporte para listas, itens de lista, figuras e imagens em
    documentos CHM .
-   Adicionado um item de menu para quebra automática de linha e a
    respectiva tecla de atalho.
-   Os sons de marcadores/notas agora devem ser reproduzidos
    corretamente e exclusivamente quando você navegar sobre uma palavra
    que contenha um deles.
-   Documentos codificados em codificações CJK legadas, como GBK, Big5 e
    Shift_JIS, agora serão exibidos corretamente, em vez de aparecerem
    como um monte de caracteres ilegíveis.
-   Expandimos o item de menu de exportação para permitir a exportação
    para HTML e Markdown, além de texto simples.
-   Corrigimos o problema em que a aplicação do quebra automático de
    linha levava você de volta ao início do documento.
-   Corrigido o problema de livros em formato Daisy exibirem informações
    incorretas na barra de status.
-   Corrigido o problema em que os elementos dl, dt e dd não geravam
    quebras de linha em documentos XHTML .
-   Corrigido o problema em que a tecla Escape não fechava as caixas de
    diálogo "Informações do Documento" e "Todos os Documentos".
    Corrigido o problema em que as âncoras \`filepos\` em livros Mobi
    dividiam tags HTML e inseriam
-   Corrigido o problema em que âncoras filepos em livros Mobi dividiam
    tags HTML e inseriam caracteres indesejados no texto do livro.
-   Corrigido o atraso ao chegar perto do final do campo de texto em
    documentos grandes .
-   Corrigidos os links em livros Mobi antigos.
-   Corrigido o carregamento de livros DAISY com declarações de
    codificação incorretas.
-   Corrigida a navegação por páginas que anunciava texto de linha
    incorreto em algumas situações.
-   Corrigida a análise de documentos RTF contendo caracteres não
    latinos.
-   Corrigida a função "Reabrir o último fechado", que tentava reabrir o
    arquivo readme incluído.
-   Corrigida a falha na atualização da barra de título após o
    fechamento de um documento a partir da caixa de diálogo "Todos os
    documentos".
-   Corrigida a falha na qual a caixa de diálogo do WebView não era
    redimensionável e aparecia com um tamanho inicial muito pequeno.
-   Corrigida a exibição incorreta de títulos em documentos do Word com
    nomes de estilos específicos da localidade. Corrigido o problema em
    que a guia selecionada não recebia o foco corretamente após
-   Corrigido o problema em que a guia selecionada não recebia o foco
    corretamente após reiniciar o Paperback.
-   Se uma seleção estiver ativa ao abrir a caixa de diálogo de contagem
    de palavras, o número de palavras selecionadas agora será exibido.
-   As imagens agora devem ser exibidas corretamente na visualização da
    web incorporada.
-   Melhoramos o tratamento do Paperback em relação a arquivos em
    unidades de rede do Windows: ao pressionar "Mostrar arquivo na
    pasta", o foco agora é direcionado corretamente para o arquivo no
    armazenamento de rede, e os caminhos não contêm mais caracteres
    estranhos.
-   A análise de AZW3 foi significativamente aprimorada.
-   Migramos do chmlib para nosso próprio leitor de arquivos CHM em Rust
    puro.
-   No desktop, os arquivos .paperback não serão mais carregados à força
    durante a restauração de documentos. Em vez disso, será solicitada
    uma confirmação quando o arquivo for encontrado.
-   O Paperback agora recorre à extração de texto simples para PDFs com
    marcações incorretas. A
-   A opção "Abrir pasta contida" agora seleciona o arquivo especificado
    no Explorador.
-   Ao abrir o arquivo "readme", o idioma selecionado agora será
    respeitado.
-   Documentos do PowerPoint agora suportam tabelas.
-   Atualize corretamente o menu e defina o foco no campo de texto ao
    abrir a ajuda no Paperback.
-   O arquivo "readme.html" não será mais adicionado à sua lista de
    todos os documentos quando aberto por meio de Shift+F1.
-   Remover documentos da caixa de diálogo "Recentes" agora também
    fechará a aba ativa.
-   Mudamos para um método muito mais seguro de IPC no Windows.
-   O título do documento ativo agora será lido ao alternar entre abas.
-   O atualizador agora exibe corretamente o conteúdo das tags de código
    Markdown nas notas de lançamento.
-   O atualizador agora verifica se o arquivo baixado não foi adulterado
    .
-   A visualização da web agora é aberta na sua posição atual de
    leitura.
-   Seu filtro de pesquisa na caixa de diálogo "Todos os documentos"
    agora é preservado após a remoção de um documento.

### Versão 0.8.5 {#version-0.8.5}

-   Adicionado suporte a páginas para livros em formato ePub.
-   Adicionado suporte a documentos criptografados do Microsoft Office.
    Atualmente, o Word antigo, o Word moderno e o PowerPoint moderno são
    suportados, com o PowerPoint antigo previsto para o futuro.
-   Adicionado suporte para documentos antigos do Microsoft Word
    (\*.doc)!
-   Adicionado suporte para apresentações do PowerPoint antigo (\*.ppt)!
-   Adicionado suporte para livros nos formatos mobi e AZW3!
-   Adicionado suporte a arquivos PDF com tags!
-   Adicionado o atalho Ctrl+Q para sair do aplicativo.
-   Adicionado suporte para livros compactados do Bookshare (tanto DAISY
    quanto Word)!
-   O texto alternativo para imagens incorporadas agora deve ser exibido
    corretamente.
-   Documentos CHM agora oferecem suporte adequado à navegação por links
    internos.
-   Corrigido o problema em que os sons dos marcadores eram acionados no
    início do parágrafo, em vez de na posição do marcador.
-   Corrigido o deslocamento de 1 página na função "Ir para a página".
-   Corrigido o problema em que a tecla Escape não funcionava para
    fechar a caixa de diálogo "Abrir como".
-   Corrigido o problema em que o menu de contexto do leitor não
    aparecia ao clicar com o botão direito ou ao pressionar a tecla
    "Aplicativos".
-   Corrigido o problema em que, às vezes, o documento errado recebia o
    foco ao abrir documentos a partir da linha de comando.
-   PDFs compostos apenas por imagens são novamente detectados e você é
    alertado sobre sua existência.
-   Agora é possível navegar pelas imagens e figuras com g/Shift+g e
    f/Shift+f, respectivamente.
-   O Paperback agora respeita a configuração de modo escuro do seu
    aplicativo.
-   Removido o suporte a DAISY XML, já que não é mais necessário.
-   Voltou-se à navegação nativa do Win32 por primeira letra na árvore
    do índice.
-   A caixa de diálogo de erro ao carregar agora exibe mensagens de erro
    mais detalhadas.
-   A visualização da web agora abre de forma muito mais rápida e suave.

### Versão 0.8.2 {#version-0.8.2}

-   Adicionado suporte a páginas para documentos RTF!
-   Corrigido um bug em que a abertura da visualização da web em ePUBs
    contendo links externos os ativava automaticamente.
-   Corrigido um bug em que o analisador RTF não inseria um espaço entre
    as palavras em casos raros.
-   Corrigida a divisão de parágrafos em várias linhas curtas em alguns
    documentos PDF .
-   Os documentos PDF agora têm suporte básico para navegação por links
    e títulos !
-   As tabulações e avanços de linha do RTF agora são renderizados
    exatamente como aparecem no documento.
-   Voltamos a usar a comprovada biblioteca pdfium para analisar PDFs,
    tornando a renderização de PDFs muito mais confiável novamente.

### Versão 0.8.1 {#version-0.8.1}

-   Adicionado Ctrl+Shift+T para reabrir o último documento fechado.
-   A caixa de diálogo "Todos os Documentos" agora permite selecionar
    vários documentos para abrir de uma só vez.
-   Corrigimos alguns bugs no analisador RTF.
-   Corrigimos o problema de caminhos de arquivo contendo caracteres não
    ASCII (como os bósnios š, č, ć, ž) ficarem corrompidos ao abrir um
    arquivo por meio de uma segunda instância do Paperback .
-   Corrigida a leitura do texto do PDF na ordem errada e o espaçamento
    incorreto em torno de palavras em maiúsculas.
-   Corrigido o carregamento lento de documentos ao abrir arquivos
    grandes.
-   Corrigida a localização dos botões "Sim"/"Não" nas caixas de diálogo
    de confirmação .

### Versão 0.8.0 {#version-0.8.0}

-   Adicionadas traduções em japonês, chinês simplificado e vietnamita !
-   Adicionado um atualizador automático que agora substituirá a versão
    atualmente instalada do Paperback, em vez de apenas baixar a nova
    versão!
-   Adicionado feedback sonoro opcional ao acessar um marcador ou uma
    nota; agradecimentos a Andre Louis pelos sons!
-   Adicionado suporte a documentos RTF!
-   Adicionado suporte para documentos DAISY XML.
-   Adicionado suporte para arquivos de texto Flat Open Document!
-   Adicionado suporte para apresentações em formato Flat Open Document!
-   Adicionado suporte para separadores com "s" e "Shift+s".
-   Qualquer deslocamento superior a 300 caracteres agora será
    automaticamente adicionado ao seu histórico de navegação.
-   Corrigida a restauração da janela do Paperback a partir da bandeja
    do sistema.
-   Corrigido o problema em que documentos Markdown exibiam texto bruto
    em vez de HTML renderizado na Visualização da Web.
-   Corrigida a exibição incorreta de tabelas em arquivos Markdown.
-   PDFs compostos apenas por imagens agora exibirão um aviso sobre sua
    existência quando você tentar carregar um.
-   Agora é possível verificar se há novas compilações de
    desenvolvimento em vez de versões estáveis ao verificar se há
    atualizações.
-   Incorporadas corretamente as informações de versão no executável do
    Paperback.
-   Dividimos a caixa de diálogo de opções em abas para facilitar o uso
    e a navegação.
-   Mudamos para o Hayro para a análise de PDFs, o que resultou em maior
    confiabilidade, velocidade e menos DLLs.
-   Reescrevemos todo o aplicativo em Rust. A nova base de código é mais
    segura, carrega documentos mais rapidamente e é mais fácil de manter
    e ampliar.
-   O menu de contexto do controle de texto agora incluirá ações
    específicas do leitor, em vez de itens genéricos, como cortar e
    colar.

### Versão 0.7.0 {#version-0.7.0}

-   Adicionado suporte a tabelas para documentos baseados em HTML e
    XHTML! Navegue entre tabelas usando T e Shift+T, e pressione Enter
    para visualizar uma delas em um visualizador da web.
-   Adicionado um recurso básico de renderização na web! Pressione
    Ctrl+Shift+V para abrir a seção atual do seu documento em um
    renderizador baseado na web, útil para conteúdos como formatação
    complexa ou exemplos de código.
-   Adicionada uma tradução para o russo, obrigado, Ruslan Gulmagomedov!
-   Adicionado um botão "Limpar tudo" à caixa de diálogo "Todos os
    documentos".
-   O verificador de atualizações agora exibe notas de lançamento quando
    uma nova versão está disponível.
-   Corrigida a restauração da janela a partir da bandeja do sistema.
-   Corrigidas as traduções dos botões "Sim"/"Não" nas caixas de diálogo
    de confirmação.
-   Corrigido o carregamento de configurações ao executar como
    administrador.
-   Corrigido o tratamento de comentários em documentos XML e HTML.
-   Corrigida a análise do índice (TOC) em livros no formato Epub 2.
-   Corrigida a navegação para o próximo item com a mesma letra no
    índice.
-   Corrigido o problema em que a caixa de diálogo de localização não se
    ocultava corretamente ao usar os botões "próximo" e "anterior".
-   Corrigido o problema em que os índices de ePub ocasionalmente
    levavam ao item errado.
-   Corrigidos vários problemas de tratamento de espaços em branco em
    XML, HTML e nas tags pre.
-   Corrigido o erro "off-by-one" na navegação por links.
-   Corrigido o problema de alguns livros apresentarem espaços em branco
    no final das linhas.
-   Corrigidos vários problemas no analisador sintático.
-   Os itens de menu relacionados a marcadores, bem como a lista de
    elementos, agora são desativados corretamente quando nenhum
    documento está aberto.
-   Melhorou-se o tratamento de listas em vários formatos de documento.
-   Melhoramos o fluxo de trabalho de tradução para colaboradores.
-   Foram realizadas muitas refatorações internas, transferindo a maior
    parte da lógica de negócios do aplicativo de C++ para Rust, a fim de
    melhorar o desempenho e a manutenção.

### Versão 0.6.1 {#version-0.6.1}

-   Adicionado suporte a PDFs protegidos por senha!
-   Adicionado um recurso muito básico para ir para a posição
    anterior/próxima. Se você pressionar Enter em um link interno e isso
    mover o cursor, essa posição agora será lembrada e poderá ser
    acessada com as setas Alt+esquerda/direita .
-   Adicionada uma lista de elementos! Atualmente, ela mostra apenas uma
    árvore com todos os títulos do seu documento ou uma lista de links,
    mas há planos para expandi-la no futuro.
-   Adicionada uma opção para iniciar o Paperback no modo maximizado por
    padrão.
-   Corrigimos o problema de links em alguns documentos EPUB que não
    funcionavam corretamente.
-   Corrigida a análise de índices de EPUB contendo caminhos relativos.
-   Corrigido o problema de alguns documentos ePub não exibirem título
    ou autor.
-   Corrigido o problema de alguns títulos de capítulos de ePub não
    aparecerem corretamente na caixa de diálogo do índice.
-   Corrigida a impossibilidade de usar a barra de espaço para ativar os
    botões OK/Cancelar na caixa de diálogo do índice.
-   Melhorou-se o tratamento de títulos em documentos do Word.
-   Agora você receberá um feedback falado se a lista de documentos
    recentes estiver vazia ao tentar abrir a caixa de diálogo.

### Versão 0.6.0 {#version-0.6.0}

-   Uma nova opção para exibir o menu "Ir para" de forma muito mais
    compacta foi adicionada à caixa de diálogo de opções, marcada por
    padrão.
-   Adicionada uma opção para que a navegação por elementos estruturais
    quebre a linha.
-   Adicionada uma opção ao menu "Ferramentas" para abrir a pasta que
    contém o documento atualmente em foco.
-   Foi adicionado um sistema de atualização bastante simples, mas muito
    eficaz.
-   Adicionado um recurso básico de temporizador de suspensão, acessível
    com Ctrl+Shift+S.
-   Adicionado suporte para análise de e-books no formato FB2!
-   Adicionado suporte para análise de apresentações OpenDocument!
-   Adicionado suporte para análise de arquivos de texto OpenDocument!
-   Agora é possível criar marcadores para marcar uma linha inteira ou
    apenas um trecho específico de texto. Se não houver nenhuma seleção
    ativa ao inserir um marcador, o comportamento será igual ao das
    versões anteriores à 0.6, e a linha inteira será marcada. No
    entanto, se você selecionar algum texto, apenas esse texto será
    incluído no marcador.
-   Agora é possível anexar notas de texto opcionais aos marcadores!
    Navegue entre marcadores que contenham notas com N e Shift+N, ou
    abra a caixa de diálogo de marcadores com todos os marcadores,
    apenas notas ou apenas marcadores sem notas selecionados por meio de
    teclas de atalho específicas.
-   Os marcadores na caixa de diálogo de marcadores não terão mais o
    incômodo prefixo "marcador x".
-   Livros em Epub que contêm conteúdo HTML disfarçado de XML agora
    serão tratados corretamente.
-   Corrigido o carregamento de documentos Markdown grandes.
-   Corrigido o problema em que pressionar a barra de espaço na
    visualização em árvore do índice ativava o botão OK.
-   Corrigido o tratamento de espaços em branco no início das tags
    \`pre\` em documentos HTML e XHTML.
-   Corrigido o problema em que o controle de texto às vezes não
    recuperava o foco ao retornar à janela do Paperback.
-   Corrigida a falha em que o campo de texto na caixa de diálogo "Ir
    para %" não atualizava o valor do controle deslizante.
-   Corrigida a renderização de IDs HTML personalizados em documentos
    Markdown.
-   O HTML dentro de blocos de código Markdown agora será renderizado
    corretamente.
-   Ao carregar um livro com um parâmetro de linha de comando enquanto
    uma instância existente do Paperback estiver em execução, você não
    receberá mais um erro se o carregamento do seu documento demorar
    mais de 5 segundos.
-   Se o Paperback for executado como administrador, a configuração
    agora será carregada e salva corretamente.
-   Agora é possível excluir um marcador diretamente da caixa de diálogo
    de marcadores.
-   Agora é possível importar e exportar seus marcadores e a posição de
    leitura de um documento específico. O arquivo gerado recebe o nome
    do arquivo com a extensão .paperback. Se tal arquivo for encontrado
    no mesmo diretório que o arquivo durante o carregamento, ele será
    automaticamente carregado. Caso contrário, você pode importá-los
    manualmente usando um item no menu "Ferramentas".
-   Links dentro de documentos agora são totalmente suportados! Use k e
    shift+k para avançar e retroceder entre eles e pressione Enter para
    abrir/ativar um deles.
-   Muitas refatorações internas, tornando o aplicativo mais rápido e o
    binário menor.
-   O conteúdo em Markdown agora é pré-processado para estar em
    conformidade com o CommonMark antes da renderização.
-   A navegação por listas e seus itens agora é totalmente suportada!
    Use L e Shift+L para navegar pelas próprias listas, e I e Shift+I
    para percorrer os itens da lista.
-   A tecla Delete do teclado numérico agora funciona para remover
    documentos da barra de abas, além da função normal de excluir.
-   O Paperback agora pode, opcionalmente, ser minimizado para a bandeja
    do sistema! Essa opção está desativada por padrão, mas ativá-la fará
    com que a opção de minimizar no menu do sistema coloque o Paperback
    na bandeja, podendo ser restaurado clicando no ícone exibido.
-   O Paperback agora é totalmente traduzível! A lista de idiomas que
    ele suporta ainda é bem pequena, mas está crescendo constantemente!
-   O Paperback agora tem um site oficial, em
    [paperback.dev](https://paperback.dev)!
-   Os documentos PPTX agora exibirão um índice básico, contendo todos
    os slides.
-   O caminho completo para o documento aberto agora será exibido na
    caixa de diálogo de informações do documento.
-   O instalador agora inclui uma opção para visualizar o arquivo
    "Leia-me" no seu navegador após a instalação.
-   A lista de documentos recentes foi ampliada significativamente! Em
    vez de simplesmente mostrar os últimos 10 documentos abertos, ela
    agora exibe um número personalizável, com o restante dos documentos
    que você já abriu disponíveis por meio de uma pequena caixa de
    diálogo.
-   Várias pequenas melhorias nos analisadores em geral, incluindo a
    inserção de uma linha em branco entre os slides em apresentações
    PPTX, a correção do tratamento de quebras de linha dentro de
    parágrafos em documentos do Word e a adição de marcadores aos itens
    de lista.

### Versão 0.5.0 {#version-0.5.0}

-   Adicionado suporte a documentos do Microsoft Word!
-   Adicionado suporte para apresentações do PowerPoint!
-   Corrigimos o problema em que certos itens do menu não eram
    desativados quando não havia documentos abertos.
-   Corrigida a orientação do controle deslizante de porcentagem "Ir
    para".
-   Corrigido o índice em livros EPUB com caminhos de arquivo
    codificados por URL e/ou IDs de fragmentos.
-   Corrigida a remoção de espaços em branco dos títulos XHTML de
    maneiras estranhas.
-   Corrigido o tratamento de espaços em branco dentro de tags \`pre\`
    aninhadas em documentos HTML.
-   Documentos HTML e Markdown agora suportam o recurso de índice ! Ao
    carregar um documento HTML/Markdown, o Paperback criará seu próprio
    índice a partir da estrutura dos títulos do seu documento e o
    exibirá na caixa de diálogo acessível com Ctrl+T.
-   Os documentos HTML agora terão o título definido na tag title, se
    ela existir. Caso contrário, continuarão a usar o nome do arquivo
    sem a extensão.
-   Mudamos do UniversalSpeech para o uso de uma região ativa para gerar
    a leitura em voz alta. Isso significa que as DLLs de leitores de
    tela não são mais fornecidas junto com o programa, e agora mais
    leitores de tela serão suportados, como o Microsoft Narrator.
-   Mudamos as bibliotecas ZIP para permitir a abertura de uma variedade
    maior de livros em ePub .
-   A caixa de diálogo que pergunta se você deseja abrir seu documento
    como texto simples foi totalmente reformulada e agora permite que
    você abra seu documento como texto simples, HTML ou Markdown.
-   A caixa de diálogo "Ir para a porcentagem" agora inclui um campo de
    texto que permite que você insira manualmente uma porcentagem para a
    qual deseja saltar.
-   O analisador de HTML agora reconhece dd, dt e dl como elementos de
    lista.
-   O índice dos livros em ePub será novamente preservado exatamente.
-   O espaço não separável Unicode agora é levado em consideração ao
    remover linhas em branco.
-   Você não será mais questionado sobre como deseja abrir um arquivo
    não reconhecido toda vez que o carregar, apenas na primeira vez.

### Versão 0.4.1 {#version-0.4.1}

-   Adicionado um ícone opcional no menu Iniciar ao instalador.
-   O índice agora deve ficar mais organizado em alguns casos; por
    exemplo, se você tiver um item filho e um item pai com o mesmo texto
    na mesma posição, agora você verá apenas o item pai.
-   Corrigido o índice em determinados documentos CHM.
-   Corrigido o índice em livros Epub 3 que contivessem caminhos
    absolutos neles.
-   Os documentos CHM agora devem exibir o título conforme definido no
    arquivo de metadados .

### Versão 0.4.0 {#version-0.4.0}

-   Adicionado suporte a arquivos CHM!
-   Adicionado suporte a marcadores! Você pode ter quantos marcadores
    quiser em quantos documentos desejar. É possível avançar e
    retroceder entre eles com as teclas b e shift+b, definir um marcador
    com control+shift+b e abrir uma caixa de diálogo para saltar para um
    marcador específico com control+b.
-   Adicionado um instalador junto com o arquivo ZIP portátil! O
    instalador instalará o Paperback no diretório "Arquivos de
    Programas" e configurará automaticamente as associações de arquivos
    para você.
-   Arquivos de texto com BOMs agora devem ser decodificados
    corretamente, e o BOM também não será mais exibido no início do
    texto.
-   Adicionamos muito mais informações à barra de status. Agora ela
    mostrará sua linha atual, caractere e porcentagem de leitura.
-   Comentários HTML, bem como o conteúdo das tags de script e estilo,
    não serão mais exibidos na saída de texto.
-   Se você passar um caminho relativo para o Paperback na linha de
    comando, ele agora o resolverá corretamente.
-   O deslocamento por porcentagem agora é controlado por uma caixa de
    diálogo própria baseada em controle deslizante, acessível com
    Control+Shift+G.
-   Documentos sem títulos ou autores conhecidos agora sempre terão um
    padrão.
-   A lógica de salvamento da posição agora é muito mais inteligente e
    só deve gravar no disco quando for absolutamente necessário.
-   O documento em que você estava com o foco ao fechar o Paperback
    agora é lembrado mesmo após reinicializações do aplicativo.
-   As entradas nas caixas de diálogo "Ir para a linha" e "Ir para a
    página" agora devem ser validadas de forma mais rigorosa.
-   Corrigida a navegação no índice de livros em ePub 3 com caminhos
    relativos em seus manifestos.

### Versão 0.3.0 {#version-0.3.0}

-   Corrigido o índice em livros ePub com manifestos codificados por
    URL. Corrigida a navegação por títulos em documentos HTML que contêm
    caracteres Unicode multibyte.
-   Corrigida a navegação por títulos em documentos HTML contendo
    caracteres Unicode multibyte.
-   Corrigido o alto consumo de CPU em documentos com títulos longos
    devido a uma regressão no wxWidgets.
-   Corrigido o carregamento de arquivos de texto UTF-8.
-   Corrigido o problema em que itens aninhados do índice em livros ePub
    posicionavam o cursor na posição errada.
-   Corrigida uma falha ao encerrar o aplicativo em certos casos.
-   Adicionada uma caixa de seleção na caixa de diálogo de opções para
    ativar ou desativar o quebra automático de linha!
-   Agora é possível fazer doações para o desenvolvimento do Paperback,
    seja por meio do novo item "Doar" no menu de ajuda ou pelo link
    "Patrocine este projeto" na parte inferior da página principal do
    repositório do GitHub.
-   Documentos em Markdown agora sempre terão um título, e o Paperback
    deve agora ser capaz de carregar praticamente qualquer arquivo
    Markdown.
-   Os documentos PDF agora sempre terão um título, mesmo que os
    metadados estejam ausentes.
-   Mudamos as bibliotecas de PDF para a utilizada no Chromium, o que
    resultou em uma análise de PDF muito mais confiável em todos os
    aspectos.
-   Agora, só é possível ter uma instância do Paperback em execução por
    vez. Executar o paperback.exe com um nome de arquivo enquanto ele já
    estiver em execução irá abrir esse documento na instância já em
    execução.
-   Agora você pode pressionar a tecla Delete em um documento no
    controle de abas para fechá-lo. Versão 0.2.1

### Versão 0.2.1 {#version-0.2.1}

-   Adicionamos o número total de páginas ao rótulo da página na caixa
    de diálogo "Ir para a página".
-   Permite navegar com a tecla Tab do conteúdo do documento para a sua
    lista de documentos abertos.
-   Corrigimos o problema em que as teclas de cabeçalho às vezes abriam
    documentos recentes se você tivesse um número suficiente deles.
-   O Paperback agora remove hífens de separação desnecessários da saída
    de texto .
-   Corrigido o problema em que a navegação por títulos às vezes levava
    você ao caractere errado.

### Versão 0.2.0 {#version-0.2.0}

-   Adicionado suporte a documentos Markdown!
-   Adicionado suporte a documentos PDF, incluindo a capacidade de
    navegar entre páginas!
-   Adicionados atalhos de teclado para navegar por títulos em conteúdo
    HTML, incluindo livros em ePub e documentos Markdown. Esses atalhos
    foram projetados para funcionar de maneira semelhante a um leitor de
    tela.
-   Corrigido o carregamento de ePUBs com nomes de arquivos codificados
    por URL em seus manifestos.
-   Corrigido o carregamento de livros em ePub 3 com XHTML incorporado
    neles.
-   Agora, uma mensagem é lida em voz alta se o documento não suportar
    um índice ou seções, em vez de os itens do menu ficarem desativados.
-   Adicionado um menu de documentos recentes! Atualmente, ele armazena
    seus últimos 10 documentos abertos, e pressionar Enter em um deles o
    abrirá para leitura.
-   A caixa de diálogo "Localizar" foi totalmente reescrita, tornando-a
    muito mais simples de usar, ao mesmo tempo em que adicionou um
    histórico das suas últimas 25 pesquisas e suporte a expressões
    regulares!
-   Os documentos abertos anteriormente agora são lembrados mesmo após o
    reinício do aplicativo. Isso pode ser configurado por meio do novo
    item de opções no menu "Ferramentas".
-   Adicionamos a combinação Shift+F1 para abrir o arquivo "Leia-me"
    diretamente no próprio Paperback.

### Versão 0.1.0 {#version-0.1.0}

-   Lançamento inicial.
