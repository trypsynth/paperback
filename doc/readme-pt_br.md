<!-- machine-translated from doc/readme.md (source-hash: 06f1089b5f255d98; sections: 84030068,db723a70,df2f4c18,14335443,1387e8b7,3887c286,94527a25,ca4819ea,a9eba369,e9860ee8,80b9b9ca); please review and edit as needed -->

# Paperback - versão 1.0

## Introdução

Paperback é um leitor leve, rápido e acessível para livros eletrônicos, documentos e audiolivros, para todos, desde leitores ocasionais até usuários avançados. É projetado para acessibilidade com leitores de tela, velocidade rápida e uma experiência sem inchaços.

## Requisitos do Sistema

Paperback funciona no Windows 10/11, em todas as versões modernas de ARM macOS, Linux, iOS 17 e posterior, e Android 7 e posterior. Os aplicativos iOS e Android estão na App Store e Google Play.

## Recursos

* Completamente independente, não exigindo a instalação de nenhum software no seu computador para começar a ler.
* Incrivelmente rápido, mesmo em hardware antigo.
* Interface simples com abas, permitindo que você abra quantos documentos quiser lado a lado.
* Salva sua posição exata de leitura em todos os documentos que você abre.
* Opcionalmente lembra quais documentos você tinha abertos quando fechou o programa e os restaura no próximo lançamento.
* Inclui funcionalidade de navegação semelhante à encontrada no modo de navegação na web de muitos leitores de tela para navegar rápida e facilmente pelos documentos.
* Inclui um diálogo de busca robusto, com recursos como histórico e suporte a expressões regulares.
* Pode ser executado totalmente portátil ou instalado com associações de arquivo configuradas automaticamente.
* Suporta uma grande variedade de formatos de arquivo comuns.
* Reproduz audiolivros, com velocidade ajustável e marcadores que lembram o tempo exato.
* Lê páginas PDF digitalizadas com o OCR integrado no Windows e macOS.
* Marcadores e notas, para que você possa marcar seu lugar e voltar a ele.
* Todos os atalhos de teclado podem ser alterados.
* Vem com `pb`, uma ferramenta de linha de comando que converte qualquer documento suportado para HTML, Markdown ou texto simples.

## Compatibilidade com Leitores de Tela

O Paperback funciona bem com todos os principais leitores de tela. Existem, porém, dois problemas conhecidos para usuários de JAWS.

### JAWS e Displays Braille

Se você usar JAWS com um display Braille, pode acontecer de parágrafos longos serem truncados ao fazer navegação para frente com as teclas de navegação do seu display. O comando de ler parágrafo atual também é afetado. Trata-se de um bug no tratamento do JAWS do controle de texto RICHEDIT50W, não algo no próprio Paperback, e um bug que levou bastante tempo para uma correção surgir considerando o entusiasmo da Vispero em responder a problemas em software de código aberto.

A solução alternativa, eventualmente surgida através do grupo de discussão do JAWS após meses de espera, é editar `paperback.jcf` e definir "Braille Presentation and Panning" para "Always use DOM if available". Você também vai querer habilitar "Pan Text by Paragraph", caso contrário seu display permanecerá no parágrafo ativo em vez de avançar. Com ambas as configurações em vigor, a navegação deve funcionar corretamente.

### JAWS e as mensagens do Paperback

O Paperback diz coisas como "No pages." ou "This document has no audio." como notificações de acessibilidade, o que permite que um leitor de tela as pronuncie sobre o que quer que esteja dizendo. O JAWS só age sobre essas notificações quando "Enable accessible notification events" está ativado para o aplicativo, e em algumas máquinas não está.

Se o JAWS não disser nada quando você pressionar uma tecla que deveria relatar algo, abra o Settings Center com o Paperback em foco (`Insert+6`), procure por "notification" e marque "Enable accessible notification events". Isso escreve a configuração em `paperback.jcf`, então se aplica apenas ao Paperback.

## Tipos de arquivo atualmente suportados

O Paperback oferece suporte aos seguintes formatos e extensões:

* Arquivos de histórias em quadrinhos (`.cbz`)
* Arquivos de ajuda CHM (`.chm`)
* Livros DAISY (`.opf`, `.zip`)
* Livros EPUB (`.epub`)
* E-books FB2 (`.fb2`)
* Documentos HTML (`.htm`, `.html`, `.xhtml`)
* Páginas de manual, tanto `man` quanto BSD `mdoc` (`.1` a `.9`, `.man`, `.roff` e as formas compactadas de cada um)
* Documentos Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Documentos Microsoft Word (`.docx`, `.docm`, `.doc`)
* Audiolivros M4B (`.m4b`)
* Livros MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
* Audiolivros MP3 (`.mp3`)
* Apresentações OpenDocument (`.odp`, `.fodp`)
* Arquivos de texto OpenDocument (`.odt`, `.fodt`)
* Documentos PDF (`.pdf`)
* Apresentações PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Documentos reStructuredText (`.rst`, `.rest`)
* Documentos RTF (`.rtf`)
* Documentos Windows Write (`.wri`)
* Arquivos WinHelp (`.hlp`)
* Arquivos de texto simples e arquivos de log (`.txt`, `.log`)

## Atalhos de teclado

O Paperback foi projetado para uso com teclado em primeiro lugar. Aqui estão os atalhos atuais.

Os atalhos abaixo são para Windows. Quando o macOS difere, o equivalente é anotado entre parênteses — principalmente porque Ctrl+G, Ctrl+W e Alt+Left/Right já são utilizados por outras convenções de sistema ou aplicativos nessa plataforma.

### Menu Arquivo

* `Ctrl+O`: Abrir um documento.
* `Ctrl+F4` (macOS: `Cmd+W`): Fechar o documento atual.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Fechar todos os documentos abertos.
* `Ctrl+Shift+T`: Reabrir o último documento fechado.
* `Ctrl+R`: Mostrar o diálogo "Todos os Documentos" (de Documentos Recentes).
* `Ctrl+Q`: Sair (apenas Windows; no macOS está no menu do aplicativo).

### Menu Ir

* `Ctrl+F`: Mostrar o diálogo Localizar.
* `F3` (macOS: `Cmd+G`): Localizar próximo.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Localizar anterior.
* `Ctrl+G` (macOS: `Cmd+L`): Ir para linha.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Ir para percentual.
* `Ctrl+P`: Ir para página (quando suportado pelo documento atual).
* `=`: Anunciar seu percentual de leitura atual e página, por exemplo, "15%, página 30". A página é omitida para documentos que não possuem números de página.
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
* `Shift+M`: Fórmula anterior.
* `M`: Próxima fórmula.
* `Shift+S`: Separador anterior.
* `S`: Próximo separador.
* `Shift+L`: Lista anterior.
* `L`: Próxima lista.
* `Shift+I`: Item de lista anterior.
* `I`: Próximo item de lista.
* `Shift+,`: Ir para o início do contêiner atual (lista ou tabela).
* `,`: Ir após o fim do contêiner atual (lista ou tabela).

### Menu Ferramentas

* `Ctrl+W` (macOS: `RawCtrl+W`, ou seja, a tecla Control física em vez de Cmd): Mostrar contagem de palavras do documento atual.
* `Ctrl+I`: Mostrar informações do documento.
* `Ctrl+T`: Mostrar tabela de conteúdos.
* `F7`: Mostrar lista de elementos.
* `Ctrl+Shift+C`: Abrir pasta contendo.
* `Ctrl+Shift+V`: Abrir conteúdo atual na Visualização da Web.
* `Ctrl+U`: Ver a fonte do documento em uma nova aba.
* `Ctrl+Shift+E`: Exportar dados do documento (`.paperback`).
* `Ctrl+Shift+I`: Importar dados do documento (`.paperback`).
* `Ctrl+E`: Exportar o documento atual para texto simples.
* `Ctrl+Shift+B`: Alternar marcador na seleção/cursor atual.
* `Ctrl+Shift+N`: Adicionar ou editar nota de marcador na seleção/cursor atual.
* `Ctrl+Alt+W`: Alternar quebra de linha.
* `Ctrl+Space` (macOS: `RawCtrl+Space`, ou seja, a tecla Control física, pois Cmd+Space abre o Spotlight): Reproduzir/pausar narração de áudio.
* `'`: Avançar narração de áudio.
* `;`: Retroceder narração de áudio.
* `Shift+'`: Aumentar o valor de busca de áudio.
* `Shift+;`: Diminuir o valor de busca de áudio.
* `Ctrl+Shift+.`: Acelerar narração de áudio.
* `Ctrl+Shift+,`: Desacelerar narração de áudio.
* `F11` (macOS: `RawCtrl+Ctrl+F`, ou seja, Control+Command+F): Alternar tela cheia.
* `Ctrl+,`: Abrir Configurações (macOS: no menu do aplicativo).
* `Ctrl+Shift+S`: Alternar temporizador de repouso.
* `Ctrl+Shift+O`: Reconhecer um intervalo de páginas de PDF digitalizadas com OCR.
* `Alt+F9` (macOS: `Cmd+F9`): Marcar o início de uma seleção, para que tudo daqui até onde você chegar possa ser copiado de uma só vez.
* `Alt+F10` (macOS: `Cmd+F10`): Copiar tudo desde o início marcado da seleção até a posição atual.
* `Alt+Shift+F9` (macOS: `Cmd+Shift+F9`): Voltar ao início marcado da seleção, deixando a marca no lugar.

### Menu Ajuda

* `Ctrl+F1`: Mostrar diálogo Sobre.
* `F1`: Ver ajuda no navegador padrão.
* `Shift+F1`: Ver ajuda no Paperback.
* `Ctrl+Shift+U`: Verificar atualizações.
* `Ctrl+D`: Abrir a página de doação no navegador padrão.

### Teclas adicionais de visualização de documento

* `Delete` / `Numpad Delete` no controle de aba: Fechar a aba do documento selecionado.
* `Enter` ou `Space` no texto do documento: Seguir um link ou abrir visualização de tabela ou fórmula no cursor.
* `Enter` em uma página de PDF digitalizada: Reconhecer a página com OCR.
* `Shift+F10` ou a tecla Menu/Aplicativo no texto do documento: Abrir o menu de contexto.

## iOS e Android

Os aplicativos iOS e Android usam o mesmo mecanismo de leitura que o desktop, então abrem os mesmos formatos e lembram seu lugar da mesma forma. Eles são construídos para serem usados com VoiceOver no iOS e TalkBack no Android.

### Abrindo documentos

* Use o botão Open Book ou abra um documento do aplicativo Files ou outro aplicativo e escolha Paperback.
* No Android, você pode ativar o navegador de arquivos no aplicativo em Settings. Ele precisa da permissão All Files Access e abre arquivos grandes imediatamente em vez de copiá-los primeiro.
* Pressione e mantenha pressionado o botão Open Book para importar ou exportar dados de um documento (`.paperback`), os mesmos arquivos que o aplicativo desktop usa.

### Lendo e ouvindo

Cada aplicativo tem duas formas de ler um documento. No modo texto, você lê o texto com seu leitor de tela. No modo de leitura em voz alta, Paperback lê o texto para você com a voz que você escolher em Settings, e continua funcionando em segundo plano e na tela de bloqueio. Alterne entre eles no menu More Options.

Audiolivros, como DAISY, M4B e livros MP3, reproduzem sua própria gravação.

### A barra de leitura

A barra na parte inferior da tela tem, da esquerda para a direita:

* A unidade de navegação, como parágrafo, heading, página ou link. Deslize para cima ou para baixo nela para alterá-la.
* Botões Previous, play e next. Previous e next se movem pela unidade de navegação.
* A velocidade de fala. Deslize para cima ou para baixo nela para alterar a velocidade de leitura do Paperback.

Você também pode deslizar para cima ou para baixo no botão play para se mover pela unidade de navegação, sem alcançar os botões previous e next. Se é tudo o que você usa, a configuração Hide previous and next buttons os remove do caminho do seu leitor de tela. A configuração Swipe up moves forward escolhe para qual direção um deslize vai.

### Mais opções

O menu More Options é onde tudo mais fica. Alguns itens funcionam um pouco diferente em cada aplicativo.

* **Switch to TTS Mode ou Switch to Text Mode:** alterna entre modo de leitura em voz alta e modo texto, descrito acima. No modo texto, um item Read Aloud inicia e pausa a leitura em voz alta sem sair do modo texto.
* **Table of Contents:** os capítulos do livro, aberto no que você está lendo. Escolha um para ir direto a ele. Entradas com capítulos sob elas podem ser expandidas e recolhidas com as ações do leitor de tela.
* **Elements:** uma lista dos headings ou links do documento. Alterne entre os dois com o Type picker no iOS ou as abas no Android, depois escolha um para ir a ele.
* **Find:** digite o que procurar ou escolha uma busca anterior de Search History e escolha se deseja corresponder maiúsculas e minúsculas, corresponder apenas palavras inteiras ou usar uma expressão regular. Find Previous e Find Next saltam para uma correspondência e dizem onde caíram, e Find fica aberto para que você possa continuar. No modo de leitura em voz alta, Find também aparece como uma unidade de navegação na barra de leitura, para que você possa percorrer as correspondências de lá também.
* **Go To:** salte para uma linha, uma página ou uma porcentagem através do documento. Escolha qual com o Mode picker.
* **Recent Documents:** cada documento que você abriu, cada um marcado como atualmente aberto, fechado ou arquivo não encontrado. Cada um tem duas ações do leitor de tela: Remove o remove da lista e Locate permite encontrar um documento cujo arquivo foi movido. Clear Recent Documents esvazia a lista sem excluir nenhum documento.
* **Word Count:** o número de palavras no documento.
* **Document Info:** o título, o autor, o nome do arquivo e no iOS também as contagens de linhas e caracteres.
* **Export:** salva o documento como texto simples, HTML ou Markdown.
* **Sleep Timer:** para de ler após 5, 10, 15, 30, 45 ou 60 minutos, ou um tempo de sua escolha. Abra novamente enquanto está em execução para ver quanto tempo falta ou para cancelá-lo.
* **Help:** abre este readme.
* **Settings:**
    * **Text to speech:** a voz, a velocidade de fala e o pitch, um botão Play Sample para ouvi-los e a pausa entre parágrafos. Android também permite que você escolha o mecanismo de fala. No iOS, é também aqui que fica o dicionário de fala: regras que alteram como as palavras são pronunciadas, para cada voz ou apenas algumas.
    * **Readability:** tamanho do texto, espaçamento de linhas, espaçamento de parágrafos, alinhamento e texto de alto contraste. iOS também tem aparência clara e escura.
    * **Behavior:** se deseja reabrir seus documentos quando o aplicativo inicia, para qual direção um deslize no botão play se move e se deseja ocultar os botões previous e next. Android também tem o navegador de arquivos no aplicativo aqui.

### Teclados e headsets

Com um teclado, os atalhos do desktop para abrir livros, documentos recentes, Find, Go To, table of contents, word count, document info, export e sleep timer funcionam, usando `Cmd` em vez de `Ctrl` no iOS. O mesmo acontece com as teclas de letra única para se mover por heading, página, link e o resto, e `Space` reproduz e pausa. No iOS, as teclas de letra única apenas alcançam Paperback enquanto o Quick Nav de letra única do VoiceOver estiver desligado.

No Android, um botão de headset reproduz e pausa com um pressionamento, avança com dois e volta com três.

## Idiomas suportados

O Paperback é traduzido para muitos idiomas diferentes, com mais sendo adicionados o tempo todo. Uma lista completa segue abaixo.

Para aprender como contribuir, por favor leia nosso [Guia de Tradução](translating.md).

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
* Ucraniano
* Vietnamita

## Créditos
### Desenvolvimento
* Quin Gillespie: desenvolvedor principal e fundador do projeto.
* Aryan Choudhary: principal contribuidor.

### Doações
As pessoas a seguir fizeram doações de algum valor para o desenvolvimento do Paperback. Se você fizer uma doação, seu nome não será adicionado automaticamente aqui; eu apenas adiciono pessoas que desejam que sua doação seja pública.

Nota: Considero um patrocinador público no GitHub motivo suficiente para inclusão automática nesta lista.

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

### Versão 1.0

1.0 é o primeiro lançamento em todas as cinco plataformas: Windows, macOS, Linux, iOS e Android, com os apps iOS e Android na App Store e Google Play.

#### Adicionado

##### Geral
* Suporte a Linux, como AppImage ou tar.gz, com integração de desktop para que documentos se abram a partir do seu gerenciador de arquivos.
* Marque o início de uma seleção com `Alt+F9`, copie tudo dali até onde você chegou com `Alt+F10`, e volte à marca com `Alt+Shift+F9`, para copiar um longo trecho de texto sem usar shift+seta. Os três atalhos estão em Tools > Select and copy.
* O atalho `=` agora anuncia a página, bem como o percentual, por exemplo "15%, página 30", e permanece como era para documentos sem números de página.
* A caixa Sobre agora mostra a licença do Paperback e todos os tradutores.
* Uma tradução para ucraniano.

##### Novos Formatos
* Arquivos de histórias em quadrinhos (`.cbz`).
* Audiolivros M4B, divididos em seus capítulos.
* Páginas de manual, tanto `man` quanto `mdoc` BSD, compactadas ou não.
* Audiolivros MP3, divididos em capítulos quando o arquivo os contém.
* Documentos reStructuredText.
* Arquivos Windows Write (`.wri`).
* Arquivos WinHelp (`.hlp`).
* Documentos Word 6 e Word 95.

##### OCR
* Páginas de PDF digitalizadas agora podem ser reconhecidas com o OCR integrado ao Windows e macOS. Pressione `Enter` em uma página digitalizada para reconhecê-la, ou use Batch OCR (`Ctrl+Shift+O`) para uma série de páginas.

##### Navegação
* Fórmulas MathML em EPUB e HTML são renderizadas como AsciiMath usando MathCAT. Use `M` ou `Shift+M` para navegar pelas fórmulas, depois `Enter` ou `Space` para abrir o MathML original na Visualização de Fórmulas.
* Um botão Find All na caixa de diálogo Find, listando cada linha com uma correspondência para que você possa pular direto para a desejada.
* Visualizações de Tables, Lists e Pages na lista de elementos (`F7`).
* Go to Line, Go to Page e Go to Percent agora aceitam `+n` e `-n` para se mover relativamente ao lugar onde você está.
* Livros EPUB, MOBI e CHM sem títulos próprios agora recebem navegação por títulos a partir de seu sumário.
* Livros KF8 (AZW3) agora suportam navegação por seções.
* Páginas de EPUB que contêm apenas uma imagem agora mostram uma linha para ela, para que você possa pousar nelas em vez de pular direto.

##### Audiolivros
* Controles de velocidade de reprodução, de meia velocidade até três vezes mais rápido. Use `Ctrl+Shift+.` e `Ctrl+Shift+,`, ou o menu Tools.
* Marcadores e notas em audiolivros somente de áudio agora lembram o tempo exato em que você os definiu.
* Próxima e anterior posição (`Alt+Left` e `Alt+Right`) agora funcionam em audiolivros.
* O progresso através de um audiolivro agora é medido por sua gravação, portanto Go to Percent e a barra de status correspondem ao quanto você realmente avançou.

##### Documentos Recentes
* Um item Clear Recent Documents no submenu Documentos Recentes.

##### Documentos PDF
* Uma configuração para manter cada linha de um PDF separada, em vez de uni-las em parágrafos.
* Imagens e figuras em PDFs agora são anunciadas.
* PDFs que carregam estrutura de leitura mas não marcam nenhuma de suas imagens agora anunciam essas imagens, em vez de deixá-las de fora do livro completamente.

##### Web View
* Qualquer documento agora pode ser aberto na visualização web, não apenas EPUB, HTML e Markdown.

##### Legibilidade
* Os títulos agora são desenhados em um tamanho que corresponde ao seu nível, e imagens e tabelas são separadas do texto ao redor delas.

##### pb
* `pb --list-formats` lista cada formato que pb pode ler.
* pb agora diz qual arquivo não conseguiu ler e por quê.

#### Corrigido

##### Geral
* Um livro reaberto na inicialização agora é lido imediatamente, em vez de ficar em silêncio até ser fechado e aberto novamente.
* Um documento cujo arquivo desapareceu agora pode ser removido de Todos os Documentos, em vez de permanecer na lista por mais que você confirme.
* Corrigido um travamento ao fechar o Paperback.
* Fechar o Paperback agora oculta a janela imediatamente, em vez de deixá-la na tela enquanto salva.
* Livros grandes com pouca formatação agora abrem em cerca de metade do tempo.
* Mensagens escolhidas de um menu, como "Este documento não tem áudio", não são mais interrompidas pelo leitor de tela antes de você ouvi-las.
* Abrir um documento não deixa mais Reabrir Último Fechado ativado quando não há nada para reabrir.
* O Paperback não tenta mais repetidamente documentos em sua lista recente que desapareceram, e limita quantos documentos recentes armazena.
* O arquivo de configurações INI antigo agora é excluído após ser movido para o novo formato.
* Os títulos dos diálogos de fonte e cor, e o menu Exportar Como em vietnamita, agora estão traduzidos.
* A atualização agora traz a janela relançada para a frente, em vez de deixá-la atrás de todas as outras janelas em `Alt+Tab`.
* A quebra de linha agora se aplica imediatamente em documentos grandes, em vez de recarregar tudo.

##### Navegação
* `Alt+Left` agora volta para onde você pulou, em vez de ir para uma posição mais antiga.
* Sons de marcador agora tocam apenas quando você passa sobre um marcador, não quando você para na linha em que está.
* Fechar a tabela de conteúdos, a lista de elementos e os diálogos Ir agora o leva direto para a linha em que você para, em vez de fazer você ouvir o leitor de tela ler a janela novamente.
* Ir para Linha, Ir para Página e Ir para Percentual agora recusam números fora do documento em vez de ir silenciosamente para outro lugar.
* NVDA não interrompe mais o anúncio quando um documento não tem páginas.
* Pressionar OK na tabela de conteúdos sem se mover agora vai para a entrada já selecionada.
* A tabela de conteúdos, a lista de elementos e a lista de marcadores não travam ou congelam mais em livros com milhares de entradas.
* As setas Para Cima e Para Baixo agora lembram sua coluna por documento, em vez de levá-la quando você muda abas.

##### Audiolivros
* A reprodução de áudio agora usa `Control+Space` no macOS, já que `Command+Space` pertence ao Spotlight.

##### Documentos PDF
* Corrigido PDFs exportados do Apple Pages sendo lidos como texto simples, sem nenhum dos títulos e listas com que foram escritos.
* Corrigido parágrafos e títulos em PDF se dividindo em cada linha, e palavras se dividindo em espaços.
* Corrigido títulos PDF numerados se juntando em um único título.
* Corrigido PDFs cuja árvore de estrutura não leva a nenhum texto abrindo vazio.
* Linhas definidas em fonte monoespacial, como código, não são mais unidas em parágrafos.
* Cabeçalhos e rodapés de página não são mais lidos em cada página de PDFs sem marcas.
* PDFs que marcam seus cabeçalhos e rodapés de página como texto comum não repetem mais o título e o número da página entre dois parágrafos em cada página.
* PDFs agora mostram seu título real, em vez do nome do arquivo.

##### Livros MOBI/AZW3
* Livros MOBI grandes não ficam mais sem memória, e não são mais interrompidos após 20 MB.
* Livros MOBI e AZW3 agora abrem muito mais rápido.
* Corrigido livros MOBI perdendo sua lista de capítulos.
* Corrigido texto corrompido onde livros MOBI passam de um registro para o próximo.

##### Visualização Web
* A visualização da web não carrega mais todo um livro enorme de uma vez.
* A visualização da web agora mostra documentos inteiros quando o leitor os mostra inteiros, em vez de apenas uma fatia deles.

##### Outros Formatos
* Livros FictionBook (.fb2) escritos em windows-1251, que é a maioria deles, agora abrem em vez de falhar completamente na leitura.
* Livros FictionBook que usam um namespace ou entidade HTML que nunca declararam agora abrem, em vez de serem recusados como quebrados.
* Livros em codificações legadas agora abrem muito mais rápido.
* Corrigido alguns arquivos de texto chinês abrindo como texto corrompido.
* Arquivos OpenDocument protegidos por senha agora pedem sua senha, em vez de serem relatados como quebrados.
* Arquivos PowerPoint legados protegidos por senha agora abrem, e slides PowerPoint legados não perdem mais seu texto.
* Arquivos de texto simples salvos com extensão `.rtf` agora abrem como texto, em vez de falhar com um erro.
* Palavras-chave de controle RTF não aparecem mais como texto.

#### iOS e Android

Os aplicativos iOS e Android abrem todos os formatos que o desktop suporta, e incluem:

* Leitura em voz alta, com sua escolha de voz, velocidade e tom, um controle de taxa de fala direto na barra de leitura e uma pausa opcional entre parágrafos.
* Reprodução de audiolivros DAISY, M4B e MP3, que continua em segundo plano e na tela de bloqueio.
* Navegação por títulos, páginas, links, tabelas, listas e muito mais pela barra de leitura, além da tabela de conteúdos e Localizar.
* Um temporizador, contagem de palavras e exportação de documentos, além de um dicionário de fala no iOS. No iOS, a exportação funciona através da folha de compartilhamento, para que um livro possa ir para outro aplicativo ou para Arquivos, em outro formato ou exatamente como está.
* Opções de tamanho de texto, espaçamento e texto de alto contraste.
* Atalhos de teclado que correspondem ao desktop.

### Versão 0.9.2
* Audiolivros não fazem mais seu leitor de tela ler uma série de espaços quando você foca o campo de texto.
* Audiolivros agora nomeiam o arquivo conforme você avança por seção.
* Audiolivros agora relatam seu comprimento real, em vez de afirmar que todos os arquivos neles duram 24 horas.
* Fechar a Web View com Escape não lança mais um alerta de debug depois que você seguiu um link dentro dela.
* Copiar após Select All agora fornece o documento completo, em vez de apenas a parte carregada no momento.
* Find agora vai direto para a linha encontrada, em vez de fazer você esperar o leitor de tela ler a janela novamente conforme o foco retorna ao livro.
* EPUBs que carregam um bloco ZIP64 extravagante recusando abrir com "Invalid local file header" foram corrigidos.
* Documentos longos voltando ao início enquanto um leitor de tela lia continuamente através deles foram corrigidos.
* Links no WebView agora o levam para a seção para a qual apontam, em vez de falhar com "File not found".
* O anúncio automático "Document reloaded" não corta mais seu leitor de tela no meio da frase, esperando em vez disso que termine o que estava dizendo.
* A aba General do diálogo Settings agora percorre suas opções na ordem em que aparecem na tela, com o canal de atualização diretamente após a opção de verificar atualizações.
* Windows agora sempre mostrará "Paperback" no menu Open With, em vez do tagline completo do programa.
* Word Count e Document Info agora mostram quantos arquivos um audiolivro possui e quanto tempo ele dura no total.

### Versão 0.9.1
* Sons de marcador e nota agora funcionam no macOS.
* Livros DAISY agora reproduzem seu áudio no macOS, em vez de abrir e rastrear sua linha do tempo em silêncio.
* Aspas curvas, travessões e caracteres similares desaparecendo de documentos RTF, juntando as palavras ao redor conforme desapareciam foram corrigidos.
* Imagens RTF vazando seus dados brutos no documento como texto corrompido foram corrigidas.
* O submenu Recent Documents mantendo entradas obsoletas até que algo mais acontecesse para reconstruí-lo foi corrigido.
* Aceleradores de teclado estão de volta em todas as traduções, então os menus em russo têm acesso por teclado novamente.
* Documentos CHM grandes agora abrem até sete vezes mais rápido.
* Documentos abertos agora são registrados no Windows, para que apareçam na lista de jump list da barra de tarefas e na lista recente do menu Iniciar.
* Options foi renomeado para Settings, correspondendo aos aplicativos móveis e, no macOS, à convenção da plataforma.
* Paperback agora lembra da posição, tamanho e estado maximizado de sua janela entre execuções.
* Formas plurais agora são traduzidas, então mensagens que contam coisas são lidas corretamente em idiomas que precisam de mais de uma forma.
* Selecionar ncc.html de um livro DAISY agora abre o audiolivro completo em vez de apenas seu texto.
* Os nomes de ações do diálogo Customize Keyboard Shortcuts agora podem ser traduzidos.
* O título do documento agora vem primeiro na barra de título, para que livros abertos possam ser diferenciados na barra de tarefas e em Alt+Tab.
* O diálogo de atualização agora é traduzido.

### Versão 0.9.0

#### Adicionado

##### Geral
* Uma ferramenta CLI, chamada `pb`, para converter rapidamente qualquer um dos formatos suportados pelo Paperback para HTML, Markdown ou texto simples.
* Uma opção para recarregar documentos que foram modificados por outros programas no disco.
* Uma opção Ver Origem para abrir a origem de um documento em uma nova aba, útil para editar Markdown, por exemplo.
* O texto do documento agora é paginado, o que significa que você pode carregar livros com dezenas de milhões de palavras em apenas alguns segundos. Por favor, reporte qualquer comportamento estranho encontrado com isso.

##### Suporte de Plataforma
* Suporte para Windows ARM64!
* Suporte nativo para macOS!
* Um botão de alternância para tela cheia.

##### Diálogo Todos os Documentos
* Um botão localizar para encontrar livros que apenas mudaram de caminho.
* Um filtro de status e barra de status, para que você possa filtrar por status do documento e ver quantos documentos estão sendo exibidos e selecionados.
* O atalho `Ctrl+Shift+A` para desselecionar todos os documentos.

##### Opções e Legibilidade
* Uma aba de legibilidade, com as seguintes opções:
    * Quebra de linha (movida de geral);
    * Renderizar tabelas inline (novo nesta versão, veja abaixo);
    * Fonte;
    * Cor de fundo;
    * Espaçamento de linha;
    * Espaçamento de parágrafo;
    * Espaçamento de letra;
    * Alinhamento de texto.
* Um item de menu de quebra de linha e respectiva tecla de atalho.
* Um botão de alternância para determinar como você quer que as tabelas sejam exibidas, e unificou como as tabelas são exibidas em todos os documentos.

##### Navegação
* Suporte para navegação por contêiner.
* Uma opção para mover automaticamente o cursor para o início da linha ao navegar entre linhas, semelhante ao modo de navegação em leitores de tela.
* O atalho de teclado igual para anunciar sua porcentagem atual em um documento.

##### Marcadores
* Marcadores temporários: você pode ter um por documento, e eles persistem. Use barra para definir um e barra invertida para pular para ele.

##### Contagem de Palavras
* Tempo de leitura estimado no diálogo de contagem de palavras, bem como a capacidade de definir sua velocidade de leitura para tornar essa métrica realmente útil.
* Se uma seleção estiver ativa quando você abrir o diálogo de contagem de palavras, quantas palavras você selecionou será exibido agora.

##### Atalhos de Teclado
* A capacidade de personalizar todos os atalhos de teclado do aplicativo através de um diálogo simples.
* Um atalho de teclado configurável para restaurar o Paperback da bandeja do sistema.

##### Idiomas
* Holandês, Finlandês e Polonês.

##### Exportar
* Expandiu o item de menu de exportação para permitir a exportação para HTML e Markdown, além de texto simples.

##### Atualizador
* Um botão cancelar para o diálogo de atualização em andamento.
* O atualizador agora valida que o arquivo baixado não foi alterado.

##### Visualizador da Web
* A visualização da web agora é aberta na sua posição de leitura atual.

##### Livros DAISY
* Suporte para livros DAISY 2.0.
* Suporte para reprodução de áudio DAISY 2.02.

##### Audiolivros
* A capacidade de reproduzir audiolivros, atualmente suportando DAISY áudio (incluindo DAISY áudio + texto) e zips de arquivos de áudio.
* Atalhos de teclado e itens de menu para reproduzir/pausar narração, buscar para frente e para trás, e ajustar a quantidade de busca.
* Opções para sincronizar o cursor de leitura com a reprodução de áudio, definir a quantidade de busca de áudio e escolher se buscar após o final de um capítulo continua no próximo.

##### Documentos CHM
* Suporte para listas, itens de lista, figuras e imagens.

##### PowerPoint
* Documentos PowerPoint agora suportam tabelas.

#### Corrigido

##### Geral
* Documentos codificados em codificações CJK legadas, como GBK, Big5 e Shift_JIS, agora serão renderizados corretamente em vez de exibir uma enxurrada de mojibake.
* "Reabrir último fechado" tentando reabrir o readme agrupado.
* Sua aba selecionada não ficando adequadamente focada após reiniciar o Paperback.
* Tratamento de arquivos do Paperback em unidades de rede do Windows: pressionar mostrar arquivo na pasta agora foca corretamente o arquivo no armazenamento de rede, e os caminhos não contêm mais caracteres estranhos.
* Arquivos .paperback não serão mais carregados à força na restauração de documentos; em vez disso, você será solicitado a confirmar quando um for encontrado.
* Abrir pasta contendo agora foca o arquivo fornecido no explorador.
* Abrir o readme agora respeitará seu idioma selecionado.
* A interface do usuário do Paperback agora será dimensionada corretamente em exibições de alta DPI.
* O menu agora é atualizado corretamente e o foco se move para o controle de texto ao abrir ajuda no Paperback.
* Mudado para um método muito mais seguro de IPC no Windows.
* O título do documento ativo agora será lido ao alternar entre abas.
* Reduzido o uso de memória em documentos grandes reduzindo pela metade o tamanho das tabelas internas de índice por caractere.

##### Diálogo Todos os Documentos
* Escape não fechando os diálogos Informações do Documento e Todos os Documentos.
* A barra de título não sendo atualizada após fechar um documento do diálogo de todos os documentos.
* Readme.html não será mais adicionado à sua lista de todos os documentos quando aberto via Shift+F1.
* Remover documentos do diálogo recentes agora também fechará sua aba ativa.
* Seu filtro de pesquisa agora é preservado após remover um documento.

##### Navegação
* Navegação de página anunciando texto de linha incorreto em algumas situações.
* Ir para Linha, Ir para Página e Ir para Percentual colocando seu cursor na posição errada em documentos grandes.
* Localizar e Localizar Próximo não respeitando a janela do documento carregado em documentos grandes.

##### Marcadores
* Sons de marcador/nota agora devem ser reproduzidos exclusivamente quando você navega sobre uma palavra contendo um.

##### Legibilidade
* Aplicar quebra de linha o enviando para o início do seu documento.

##### Visualização da Web
* O diálogo webview não sendo redimensionável e aparecendo em um tamanho inicial muito pequeno.
* Imagens agora devem ser exibidas corretamente na webview incorporada.

##### Atualizador
* O atualizador agora exibe corretamente o conteúdo de tags de código markdown nas notas de lançamento.

##### Livros DAISY
* Livros DAISY mostrando informações incorretas na barra de status.
* Carregando livros DAISY com declarações de codificação falsas.

##### Documentos RTF
* Análise de documentos RTF com caracteres não-latinos neles.
* Grupos RTF `\pict` para que dados de imagem incorporados não vaze mais no texto do documento.

##### Livros Mobi/AZW3
* Âncoras filepos em livros Mobi dividindo tags HTML e colocando lixo no texto do livro.
* Links em livros Mobi legados.
* Análise de AZW3 muito melhorada.

##### Documentos Word
* Documentos Word com nomes de estilo específicos de localidade não renderizando seus títulos corretamente.

##### Documentos HTML/XHTML
* Elementos dl, dt e dd não produzindo quebras de linha em documentos XHTML.

##### Documentos PDF
* O Paperback agora volta para extração de texto simples para PDFs marcados falsamente.
* Documentos PDF contendo caracteres de controle em seus títulos e/ou marcadores não travará mais o Paperback ao abrir.

### Versão 0.8.5
* Adicionado suporte a página para livros epub.
* Adicionado suporte para documentos Microsoft Office criptografados. Atualmente Word legado, Word moderno e Powerpoint moderno são suportados, com Powerpoint legado planejado para o futuro.
* Adicionado suporte para documentos Microsoft Word legados!
* Adicionado suporte para apresentações Powerpoint legadas!
* Adicionado suporte para livros mobi e AZW3!
* Adicionado suporte para arquivos PDF marcados!
* Adicionado o atalho ctrl+q para sair do aplicativo.
* Adicionado suporte para livros compactados do Bookshare (DAISY e Word)!
* Texto alternativo para imagens incorporadas agora deve ser exibido corretamente.
* Documentos CHM agora suportam adequadamente navegação por link interno.
* Corrigido ir para página estar desativado em 1.
* Corrigido a tecla escape não funcionando para fechar o diálogo abrir como.
* Corrigido o menu de contexto do leitor não aparecendo ao clicar com o botão direito ou pressionar a tecla Aplicativos.
* Corrigido o documento errado às vezes sendo focado ao abrir documentos da linha de comando.
* PDFs somente de imagem são novamente detectados e alertam sobre sua existência.
* Agora é possível navegar por imagens e figuras com g/shift+g e f/shift+f, respectivamente.
* O Paperback agora respeitará sua configuração de modo escuro do aplicativo.
* Removido suporte a DAISY XML, pois não é mais necessário.
* Voltado para a navegação nativa da primeira letra do Win32 na árvore do sumário.
* O diálogo de erro ao carregar agora mostra mensagens de erro mais detalhadas.
* A webview agora será aberta muito mais rápido e suavemente.

### Versão 0.8.2
* Adicionado suporte a página para documentos RTF!
* Corrigido um bug onde abrir a webview em epubs contendo links externos os ativaria automaticamente.
* Corrigido um bug onde o analisador RTF não colocaria um espaço entre palavras em casos raros.
* Corrigidos parágrafos sendo divididos em múltiplas linhas curtas em alguns documentos PDF.
* Documentos PDF agora têm suporte básico a navegação de link e título!
* Abas RTF e alimentações de linha agora são renderizadas exatamente como aparecem no documento.
* Voltado para a confiável biblioteca pdfium para análise de PDFs, tornando a renderização de PDF muito mais confiável novamente.

### Versão 0.8.1
* Adicionado Ctrl+Shift+T para reabrir o último documento fechado.
* O diálogo Todos os Documentos agora suporta selecionar múltiplos documentos para abrir de uma vez.
* Corrigidos alguns bugs com o analisador RTF.
* Corrigidos caminhos de arquivo contendo caracteres não-ASCII (como š, č, ć, ž bósnios) ficando corrompidos ao abrir um arquivo via uma segunda instância do Paperback.
* Corrigido texto PDF sendo lido na ordem errada e espaçamento incorreto em torno de palavras capitalizadas.
* Corrigido carregamento lento de documento ao abrir arquivos grandes.
* Corrigida a localização dos botões Sim/Não em diálogos de confirmação.

### Versão 0.8.0
* Adicionadas traduções para japonês, chinês simplificado e vietnamita!
* Adicionado um atualizador automático que agora substituirá sua versão atualmente instalada do Paperback em vez de apenas baixar a nova versão!
* Adicionado feedback sonoro opcional ao atingir um marcador ou uma nota, obrigado Andre Louis pelos sons!
* Adicionado suporte para documentos RTF!
* Adicionado suporte para documentos DAISY XML.
* Adicionado suporte para arquivos Flat Open Document Text!
* Adicionado suporte para apresentações Flat Open Document!
* Adicionado suporte para separadores com `s` e `shift+s`.
* Qualquer movimento superior a 300 caracteres agora adicionará automaticamente ao seu histórico de navegação.
* Corrigida a restauração da janela do Paperback da bandeja do sistema.
* Corrigidos documentos Markdown mostrando texto bruto em vez de HTML renderizado na Visualização da Web.
* Corrigidas tabelas não sendo renderizadas adequadamente em arquivos Markdown.
* PDFs somente com imagem agora o avisarão de sua existência quando você tentar carregar um.
* Informações de versão adequadamente incorporadas no executável do Paperback.
* Diálogo de opções dividido em abas para facilidade de uso e navegação.
* Mudança para Hayro para análise de PDFs, resultando em mais confiabilidade, velocidade e menos DLLs.
* Reescrita de todo o aplicativo em Rust. A nova base de código é mais segura, carrega documentos mais rapidamente e é mais fácil de manter e estender.
* O menu de contexto do controle de texto agora incluirá ações específicas do leitor em vez de itens genéricos como cortar e colar.

### Versão 0.7.0
* Adicionado suporte a tabelas para documentos baseados em HTML e XHTML! Navegue entre tabelas usando `T` e `Shift+T`, e pressione `Enter` para visualizar uma no navegador.
* Adicionado um recurso básico de renderização web! Pressione `Ctrl+Shift+V` para abrir a seção atual do seu documento em um renderizador baseado na web, útil para conteúdo como formatação complexa ou exemplos de código.
* Adicionada uma tradução em russo, obrigado Ruslan Gulmagomedov!
* Adicionado um botão Limpar Tudo ao diálogo Todos os Documentos.
* O verificador de atualização agora exibe notas de lançamento quando uma nova versão está disponível.
* Corrigida a restauração da janela da bandeja do sistema.
* Corrigidas as traduções dos botões Sim/Não em diálogos de confirmação.
* Corrigido o carregamento de configurações ao executar como administrador.
* Corrigido o tratamento de comentários em documentos XML e HTML.
* Corrigida a análise do TOC em livros Epub 2.
* Corrigida a navegação para o próximo item com a mesma letra no índice.
* Corrigido o diálogo de busca não se fechando adequadamente ao usar os botões próximo/anterior.
* Corrigido o TOC do epub ocasionalmente o levando ao item errado.
* Corrigidos vários problemas de tratamento de espaçamento em XML, HTML e tags pre.
* Corrigido erro de deslocamento em um na navegação de links.
* Corrigidos alguns livros tendo espaçamento em branco à direita em suas linhas.
* Corrigidos vários problemas do analisador.
* Itens de menu relacionados a marcadores e a lista de elementos agora estão devidamente desativados quando nenhum documento está aberto.
* Melhorado o tratamento de listas em vários formatos de documentos.
* Melhorado o fluxo de trabalho de tradução para colaboradores.
* Muitas refatorações internas, movendo a maioria da lógica de negócios do aplicativo de C++ para Rust para melhor desempenho e manutenibilidade.

### Versão 0.6.1
* Adicionado suporte para PDF protegido por senha!
* Adicionado um recurso muito básico de ir para posição anterior/próxima. Se você pressionar `Enter` em um link interno e isso mover seu cursor, essa posição será agora lembrada e poderá ser navegada com as setas `Alt+Left`/`Right`.
* Adicionada uma lista de elementos! Atualmente apenas mostra uma árvore de todos os títulos no seu documento ou uma lista de links, mas há planos para expandi-la no futuro.
* Adicionada uma opção para iniciar o Paperback em modo maximizado por padrão.
* Corrigidos links em alguns documentos Epub não funcionando adequadamente.
* Corrigida a análise de TOCs Epub contendo caminhos relativos.
* Corrigidos alguns documentos epub não exibindo título ou autor.
* Corrigidos os títulos de alguns capítulos epub não aparecendo adequadamente no diálogo de TOC.
* Corrigido você não ser capaz de usar a barra de espaço para ativar os botões OK/cancelar no diálogo de TOC.
* Melhorado o tratamento de títulos em documentos Word.
* Você agora receberá feedback falado se a lista de documentos recentes estiver vazia quando tentar abrir o diálogo.

### Versão 0.6.0
* Uma nova opção para mostrar o menu de navegação em uma forma muito mais compacta foi adicionada ao diálogo de opções, marcada por padrão.
* Adicionada uma opção para fazer a navegação por elementos estruturais envolver-se.
* Adicionada uma opção ao menu de ferramentas para abrir a pasta contendo o documento atualmente em foco.
* Adicionado um sistema de atualização bastante simples, mas muito eficaz.
* Adicionado um recurso básico de temporizador de descanso, acessível com `Ctrl+Shift+S`.
* Adicionado suporte para análise de ebooks FB2!
* Adicionado suporte para análise de apresentações OpenDocument!
* Adicionado suporte para análise de arquivos OpenDocument Text!
* Os marcadores agora podem marcar uma linha inteira ou marcar apenas um texto especificado. Se você não tiver uma seleção ativa ao colocar um marcador, o comportamento será como pré-0.6 e marcará a linha inteira. No entanto, se você selecionar algum texto, apenas esse texto será incluído no marcador.
* Os marcadores agora podem ter notas de texto opcionais anexadas a eles! Navegue entre marcadores contendo notas com N e `Shift+N`, ou abra o diálogo de marcadores com todos os marcadores, apenas notas ou apenas não-notas selecionados com teclas de atalho específicas.
* Os marcadores no diálogo de marcadores não terão mais um prefixo irritante "marcador x".
* Livros Epub contendo conteúdo HTML fingindo ser XML agora serão tratados corretamente.
* Corrigido o carregamento de documentos Markdown grandes.
* Corrigido pressionar espaço no painel de árvore do índice ativando o botão OK.
* Corrigido o tratamento de espaços em branco no início de tags pre em documentos HTML e XHTML.
* Corrigido o controle de texto não recuperando o foco às vezes ao retornar à janela do Paperback.
* Corrigido o campo de texto no diálogo de ir para percentual não atualizando o valor do controle deslizante.
* Corrigida a renderização de IDs HTML personalizados em documentos Markdown.
* HTML dentro de blocos de código Markdown agora será renderizado corretamente.
* Se carregar um livro com um parâmetro de linha de comando enquanto uma instância existente do Paperback estiver em execução, você não receberá mais um erro se o carregamento do seu documento levar mais de 5 segundos.
* Se executar o Paperback como administrador, a configuração agora será carregada e salva corretamente.
* Agora é possível deletar um marcador diretamente no diálogo de marcadores.
* Agora é possível importar e exportar seus marcadores e posição de leitura para um documento específico. O arquivo gerado é nomeado após o arquivo com uma extensão `.paperback`. Se tal arquivo for encontrado no mesmo diretório de um arquivo ao carregá-lo, ele será carregado automaticamente. Caso contrário, você pode importá-los manualmente usando um item no menu de ferramentas.
* Links dentro de documentos agora são totalmente suportados! Use k e `shift+k` para mover para frente e para trás através deles, e pressione enter para abrir/ativar um.
* Muitas refatorações internas, tornando o aplicativo mais rápido e o binário menor.
* O conteúdo Markdown agora é pré-processado para estar em conformidade com CommonMark antes de ser renderizado.
* A navegação por listas e seus itens agora é totalmente suportada! Use L e `Shift+L` para ir por listas em si, e I e `Shift+I` para percorrer itens de lista.
* Numpad delete agora funciona para remover documentos da barra de abas além do delete normal.
* Paperback agora pode opcionalmente minimizar para sua bandeja do sistema! Esta opção está desativada por padrão, mas ativá-la fará com que a opção minimizar no menu do sistema coloque Paperback em sua bandeja, podendo ser restaurado clicando no ícone criado.
* Paperback agora é totalmente traduzível! A lista de idiomas que ele suporta é atualmente bastante pequena, mas está crescendo constantemente!
* Paperback agora tem um site oficial, em [paperback.dev](https://paperback.dev)!
* Documentos PPTX agora mostrarão um índice básico, contendo todos os slides.
* O caminho completo para o documento aberto agora será mostrado no diálogo de informações do documento.
* O instalador agora inclui uma opção para visualizar o readme em seu navegador após a instalação.
* A lista de documentos recentes foi dramaticamente expandida! Em vez de simplesmente mostrar os últimos 10 documentos que você abriu, agora mostrará um número personalizável, com o resto dos documentos que você já abriu sendo acessíveis através de um pequeno diálogo.
* Várias pequenas melhorias nos analisadores em geral, incluindo colocar uma linha em branco entre slides em apresentações PPTX, corrigir o tratamento de quebra de linha dentro de parágrafos em documentos do Word e adicionar pontos de marcação aos itens de lista.

### Versão 0.5.0
* Suporte adicionado para documentos do Microsoft Word!
* Suporte adicionado para apresentações do PowerPoint!
* Corrigidos certos itens de menu que não eram desabilitados quando nenhum documento estava aberto.
* Corrigida a orientação do controle deslizante de porcentagem de navegação.
* Corrigido o índice em livros Epub com caminhos de arquivo codificados em URL e/ou IDs de fragmento.
* Corrigido espaço em branco sendo removido de títulos XHTML de formas estranhas.
* Corrigido o tratamento de espaço em branco dentro de tags pre aninhadas em documentos HTML.
* Documentos HTML e Markdown agora suportam o recurso de índice! Quando você carrega um documento HTML/Markdown, o Paperback criará seu próprio índice a partir da estrutura dos títulos em seu documento, e o mostrará a você na caixa de diálogo `ctrl+t`.
* Documentos HTML agora terão o título conforme definido na tag title, se existir. Caso contrário, continuarão usando o nome do arquivo sem a extensão.
* Mudança de UniversalSpeech para usar uma live region para reportar fala. Isso significa que nenhuma DLL de leitor de tela é mais enviada junto com o programa, e mais leitores de tela agora serão suportados, como o Microsoft Narrator.
* Mudança de bibliotecas zip para permitir a abertura de uma variedade maior de livros epub.
* A caixa de diálogo que pergunta se você deseja abrir seu documento como texto simples foi completamente refeita, e agora permite que você abra seu documento como texto simples, HTML ou Markdown.
* A caixa de diálogo de porcentagem de navegação agora inclui um campo de texto que permite digitar manualmente uma porcentagem para pular.
* O analisador HTML agora reconhecerá dd, dt e dl como elementos de lista.
* O índice em livros Epub será preservado novamente exatamente como era.
* O espaço não-quebrável Unicode agora é considerado ao remover linhas em branco.
* Você não será mais perguntado como deseja abrir um arquivo não reconhecido toda vez que carrega, apenas na primeira vez.

### Versão 0.4.1
* Ícone opcional do menu Iniciar adicionado ao instalador.
* O índice deve estar mais limpo em alguns casos, por exemplo, se você tiver um item filho e um item pai com o mesmo texto na mesma posição, você verá apenas o item pai.
* Corrigido o índice em certos documentos CHM.
* Corrigido o índice em livros Epub 3 com caminhos absolutos neles.
* Documentos CHM agora devem mostrar seu título conforme definido no arquivo de metadados.

### Versão 0.4.0
* Suporte adicionado para arquivos CHM!
* Suporte adicionado para marcadores! Você pode ter quantos marcadores quiser em quantos documentos quiser. Você pode navegar para frente e para trás entre eles com `b` e `shift+b`, definir um com `control+shift+b`, e abrir uma caixa de diálogo para pular para um marcador específico com `control+b`.
* Um instalador adicionado junto com o arquivo zip portátil! O instalador instalará o Paperback em seu diretório Program Files e configurará automaticamente as associações de arquivo para você.
* Arquivos de texto com BOMs agora devem ser decodificados corretamente, e o BOM não será mais exibido no início do texto.
* Muitas mais informações adicionadas à barra de status. Agora mostrará sua linha atual, caractere e porcentagem de leitura.
* Comentários HTML, bem como o conteúdo de tags script e style, não serão mais mostrados na saída de texto.
* Se passar um caminho relativo para o Paperback na linha de comando, ele agora o resolverá corretamente.
* O movimento de porcentagem agora é tratado por sua própria caixa de diálogo baseada em controle deslizante, acessível com `control+shift+g`.
* Documentos sem títulos ou autores conhecidos agora sempre terão um padrão.
* A lógica de salvamento de posição agora é muito mais inteligente e deve escrever no disco apenas quando absolutamente necessário.
* O documento que você tinha em foco quando fechou o Paperback é agora lembrado ao reiniciar a aplicação.
* A entrada nas caixas de diálogo de ir para linha e ir para página agora deve ser mais rigorosamente sanitizada.
* Corrigida a navegação de índice em livros epub 3 com caminhos relativos em seus manifestos.

### Versão 0.3.0
* Corrigido o índice em livros epub com manifestos codificados em URL.
* Corrigida a navegação de títulos em documentos HTML contendo caracteres Unicode multi-byte.
* Corrigido alto uso de CPU em documentos com títulos longos devido a uma regressão no wxWidgets.
* Corrigido carregamento de arquivos de texto UTF-8.
* Corrigidos itens TOC aninhados em livros Epub colocando o cursor na posição errada.
* Corrigido um travamento na saída da aplicação em certos casos.
* Uma caixa de seleção adicionada na caixa de diálogo de opções para habilitar ou desabilitar quebra automática de linha!
* Agora é possível doar para o desenvolvimento do Paperback, seja através do novo item de doação no menu de ajuda ou através do link patrocinar este projeto na parte inferior da página principal do repositório GitHub.
* Documentos Markdown agora sempre terão um título, e o Paperback agora deve ser capaz de carregar praticamente qualquer arquivo Markdown.
* Documentos PDF agora sempre terão um título, mesmo que os metadados estejam ausentes.
* Mudança de bibliotecas PDF para a utilizada no Chromium, levando a uma análise de PDF muito mais confiável em todos os aspectos.
* Agora você pode ter apenas uma instância do Paperback em execução ao mesmo tempo. Executar paperback.exe com um nome de arquivo enquanto já está em execução abrirá esse documento na instância já em execução.
* Você agora pode pressionar delete em um documento no controle de abas para fechá-lo.

### Versão 0.2.1
* O número total de páginas adicionado ao rótulo de página na caixa de diálogo ir para página.
* Permitir abas do conteúdo do documento para sua lista de documentos abertos.
* Corrigidos os pressionamentos de teclas de título que às vezes abriam documentos recentes se você tivesse o suficiente.
* Paperback agora removerá hífens leves desnecessários da saída de texto.
* Corrigida a navegação de título às vezes colocando você no caractere errado.

### Versão 0.2.0
* Adicionado suporte a documentos markdown!
* Adicionado suporte a documentos PDF, incluindo a capacidade de navegar entre páginas!
* Adicionados atalhos de teclado para navegar por títulos em conteúdo HTML, incluindo livros epub e documentos markdown. Esses atalhos foram projetados para funcionar de forma semelhante a um leitor de tela.
* Corrigido carregamento de epubs com nomes de arquivo codificados por URL em seus manifestos.
* Corrigido carregamento de livros epub 3 com XHTML incorporado neles.
* Uma mensagem agora é reproduzida se o documento não suportar um índice ou seções, em vez dos itens do menu serem desabilitados.
* Adicionado menu de documentos recentes! Ele atualmente armazena seus últimos 10 documentos abertos, e pressionar enter em um abrirá para leitura.
* Reescrita completa do diálogo Localizar, tornando muito mais simples de usar, além de adicionar um histórico de suas últimas 25 buscas e suporte a expressões regulares!
* Documentos abertos anteriormente agora são lembrados entre reinicializações da aplicação. Isso é configurável através do novo item de opções no menu ferramentas.
* Adicionado `shift+f1` para abrir o readme diretamente no Paperback.

### Versão 0.1.0
* Lançamento inicial.
