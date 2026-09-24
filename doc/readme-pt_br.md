<!-- machine-translated from doc/readme.md (source-hash: 6564745fd3218b1a; sections: 84030068,db723a70,df2f4c18,14335443,91be3b41,6c87c514,94527a25,ca4819ea,a9eba369,e9860ee8,3b8321f8); please review and edit as needed -->

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

Paperback funciona bem com todos os leitores de tela principais. Há, porém, um problema conhecido para usuários de JAWS.

### JAWS e Displays Braille

Se você usar JAWS com um display Braille, pode descobrir que parágrafos longos são truncados ao fazer panorâmica para frente com as teclas de navegação do seu display. O comando de leitura do parágrafo atual também é afetado. Este é um bug no tratamento do JAWS do controle de texto RICHEDIT50W, não algo no próprio Paperback, e um que levou bastante tempo para deixar uma correção aparecer, dado o entusiasmo da Vispero em responder a problemas com software de código aberto.

A solução alternativa, eventualmente encontrada através do grupo de discussão do JAWS após meses de espera, é editar `paperback.jcf` e definir "Braille Presentation and Panning" como "Always use DOM if available". Você também vai querer ativar "Pan Text by Paragraph", caso contrário seu display permanecerá no parágrafo ativo em vez de avançar. Com ambas as configurações em vigor, a panorâmica deve funcionar corretamente.

## Tipos de arquivo atualmente suportados

Paperback suporta os seguintes formatos e extensões:

* Arquivos de histórias em quadrinhos (`.cbz`)
* Arquivos de ajuda CHM (`.chm`)
* Livros DAISY (`.opf`, `.zip`)
* Livros EPUB (`.epub`)
* Livros eletrônicos FB2 (`.fb2`)
* Documentos HTML (`.htm`, `.html`, `.xhtml`)
* Páginas de manual, tanto `man` quanto BSD `mdoc` (`.1` a `.9`, `.man`, `.roff` e as formas compactadas de cada uma)
* Documentos Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Documentos Microsoft Word (`.docx`, `.docm`, `.doc`)
* Audiolivros M4B (`.m4b`)
* Livros MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
* Audiolivros MP3 (`.mp3`)
* Apresentações OpenDocument (`.odp`, `.fodp`)
* Arquivos de texto OpenDocument (`.odt`, `.fodt`)
* Documentos PDF (`.pdf`)
* Apresentações PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Documentos RTF (`.rtf`)
* Documentos Windows Write (`.wri`)
* Arquivos WinHelp (`.hlp`)
* Arquivos de texto simples e de log (`.txt`, `.log`)

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

1.0 é o primeiro lançamento em todas as cinco plataformas: Windows, macOS, Linux, iOS e Android, com os aplicativos iOS e Android disponíveis na App Store e Google Play.

#### Adicionado

##### Geral
* Suporte a Linux, como AppImage ou tar.gz, com integração de desktop para que documentos sejam abertos pelo seu gerenciador de arquivos.
* Marque o início de uma seleção com `Alt+F9`, copie tudo dali até onde você chegou com `Alt+F10`, e volte à marca com `Alt+Shift+F9`, para copiar um longo trecho de texto sem usar shift+seta. Os três estão em Tools > Select and copy.
* O atalho `=` agora anuncia a página bem como a porcentagem, por exemplo "15%, página 30", e permanece como estava para documentos sem números de página.
* A caixa About agora mostra a licença do Paperback e todos os tradutores.
* Uma tradução para o ucraniano.

##### Novos Formatos
* Arquivos de quadrinhos (`.cbz`).
* Audiolivros M4B, divididos em seus capítulos.
* Páginas de manual, tanto `man` quanto `mdoc` BSD, compactadas ou não.
* Audiolivros MP3, divididos em capítulos quando o arquivo possui.
* Arquivos Windows Write (`.wri`).
* Arquivos WinHelp (`.hlp`).
* Documentos Word 6 e Word 95.

##### OCR
* Páginas de PDF digitalizado agora podem ser reconhecidas com o OCR integrado ao Windows e macOS. Pressione `Enter` em uma página digitalizada para reconhecê-la, ou use Batch OCR (`Ctrl+Shift+O`) para um intervalo de páginas.

##### Navegação
* Fórmulas MathML em EPUB e HTML são renderizadas como AsciiMath usando MathCAT. Use `M` ou `Shift+M` para navegar pelas fórmulas, depois `Enter` ou `Space` para abrir o MathML original em Formula View.
* Um botão Find All na caixa de diálogo Find, listando cada linha com uma correspondência para que você possa pular direto para a que deseja.
* Visualizações Tables, Lists e Pages na lista de elementos (`F7`).
* Go to Line, Go to Page e Go to Percent agora aceitam `+n` e `-n` para se mover relativamente a onde você está.
* Livros EPUB, MOBI e CHM sem seus próprios títulos agora recebem navegação de títulos de seu índice.
* Livros KF8 (AZW3) agora oferecem suporte à navegação de seções.
* Páginas EPUB que são apenas uma imagem agora mostram uma linha para ela, para que você possa pousar nelas em vez de pular direto.

##### Audiolivros
* Controles de velocidade de reprodução, de meia velocidade a três vezes mais rápido. Use `Ctrl+Shift+.` e `Ctrl+Shift+,`, ou o menu Tools.
* Marcadores e notas em livros apenas de áudio agora lembram a hora exata em que você os definiu.
* Próxima e posição anterior (`Alt+Left` e `Alt+Right`) agora funcionam em audiolivros.
* O progresso através de um audiolivro agora é medido por sua gravação, para que Go to Percent e a barra de status correspondam a quão longe você realmente está.

##### Documentos Recentes
* Um item Clear Recent Documents no submenu Recent Documents.

##### Documentos PDF
* Uma configuração para manter cada linha de um PDF separada, em vez de juntá-las em parágrafos.
* Imagens e figuras em PDFs agora são anunciadas.
* PDFs que possuem estrutura de leitura mas não marcam nenhuma de suas imagens agora anunciam essas imagens, em vez de omiti-las completamente do livro.

##### Web View
* Qualquer documento agora pode ser aberto na visualização da web, não apenas EPUB, HTML e Markdown.

##### Legibilidade
* Os títulos agora são desenhados em um tamanho que corresponde ao seu nível, e imagens e tabelas são separadas do texto ao seu redor.

##### pb
* `pb --list-formats` lista todos os formatos que pb pode ler.
* pb agora diz qual arquivo não conseguiu ler e por quê.

#### Corrigido

##### Geral
* Corrigido um travamento ao fechar o Paperback.
* Fechar o Paperback agora oculta a janela imediatamente, em vez de deixá-la na tela enquanto salva.
* Abrir um documento não deixa mais "Reabrir Último Fechado" ativado quando não há nada para reabrir.
* O Paperback não tenta mais abrir documentos desaparecidos da lista recente e limita quantos documentos recentes armazena.
* O arquivo antigo de configurações INI agora é excluído quando movido para o novo formato.
* Os títulos dos diálogos de fonte e cor, e o menu Exportar Como em vietnamita, agora estão traduzidos.
* A atualização agora traz a janela reiniciada para a frente, em vez de deixá-la atrás de todas as outras janelas no Alt+Tab.
* O ajuste de linha agora se aplica imediatamente em documentos grandes, em vez de recarregar tudo.

##### Navegação
* `Alt+Left` agora volta para o local de onde você saltou, em vez de ir para uma posição mais antiga.
* Sons de marcadores agora tocam apenas quando você se move sobre um marcador, não quando você chega na linha em que está.
* Fechar a tabela de conteúdo, a lista de elementos e os diálogos Ir agora o leva direto para a linha em que você chega, em vez de fazer você ouvir o leitor de tela ler a janela novamente.
* Ir para Linha, Ir para Página e Ir para Porcentagem agora recusam números fora do documento em vez de ir silenciosamente para outro lugar.
* O NVDA não corta mais o anúncio quando um documento não tem páginas.
* Pressionar OK na tabela de conteúdo sem se mover agora vai para a entrada já selecionada.
* A tabela de conteúdo, a lista de elementos e a lista de marcadores não têm mais atraso ou congelamento em livros com milhares de entradas.
* As setas Para Cima e Para Baixo agora lembram sua coluna por documento, em vez de levar para outra quando você muda de abas.

##### Audiolivros
* A reprodução de áudio agora usa `Control+Space` no macOS, já que `Command+Space` pertence ao Spotlight.

##### Documentos PDF
* Corrigido PDFs exportados do Apple Pages serem lidos como texto simples, sem os títulos e listas com os quais foram escritos.
* Corrigido parágrafos e títulos em PDF dividindo-se em cada linha, e palavras separando-se em espaços.
* Corrigido títulos PDF numerados sendo executados juntos em um título.
* Corrigido PDFs cuja árvore de estrutura leva a nenhum texto abrindo vazios.
* Cabeçalhos e rodapés de página não são mais lidos em cada página de PDFs sem tag.
* PDFs que marcam seus cabeçalhos e rodapés de página como texto ordinário não repetem mais o título e número da página entre dois parágrafos em cada página.
* PDFs agora mostram seu título real, em vez do nome do arquivo.
* Linhas definidas em fonte monoespaçada, como código, não são mais unidas em parágrafos.

##### Livros MOBI/AZW3
* Livros MOBI grandes não ficam mais sem memória e não são mais cortados após 20 MB.
* Livros MOBI e AZW3 agora abrem muito mais rápido.
* Corrigido livros MOBI perdendo sua lista de capítulos.
* Corrigido texto distorcido onde livros MOBI atravessam de um registro para o próximo.

##### Visualização da Web
* A visualização da web não carrega mais um livro enorme inteiro de uma vez.
* A visualização da web agora mostra documentos inteiros quando o leitor os mostra inteiros, em vez de apenas uma fatia deles.

##### Outros Formatos
* Livros FictionBook (.fb2) escritos em windows-1251, que é a maioria deles, agora abrem em vez de falhar ao ler.
* Livros FictionBook que usam um namespace ou uma entidade HTML que nunca declararam agora abrem, em vez de serem recusados como quebrados.
* Livros em codificações legadas agora abrem muito mais rápido.
* Corrigido alguns arquivos de texto em chinês abrindo como texto distorcido.
* Arquivos OpenDocument protegidos por senha agora solicitam sua senha, em vez de serem reportados como quebrados.
* Arquivos PowerPoint legados protegidos por senha agora abrem, e slides PowerPoint legados não perdem mais seu texto.
* Arquivos de texto simples salvos com extensão `.rtf` agora abrem como texto, em vez de falhar com um erro.
* Palavras de controle RTF não aparecem mais como texto.

#### iOS e Android

Os aplicativos iOS e Android abrem todos os formatos que o desktop faz, e incluem:

* Leitura em voz alta, com sua escolha de voz, taxa e tom, um controle de taxa de fala direto na barra de leitura e uma pausa opcional entre parágrafos.
* Reprodução de audiolivros DAISY, M4B e MP3, que continua em segundo plano e na tela de bloqueio.
* Navegação por títulos, páginas, links, tabelas, listas e muito mais da barra de leitura, além da tabela de conteúdo e Localizar.
* Um temporizador de sono, contagem de palavras e exportação de documento, além de um dicionário de fala no iOS.
* Opções de tamanho de texto, espaçamento e texto de alto contraste.
* Atalhos de teclado que correspondem ao desktop.

### Versão 0.9.2
* Audiolivros não fazem mais seu leitor de tela ler uma sequência de espaços quando você foca o campo de texto.
* Audiolivros agora nomeiam o arquivo conforme você passa por eles por seção.
* Audiolivros agora relatam seu comprimento real, em vez de afirmar que todos os arquivos neles duram 24 horas.
* Fechar a Visualização da Web com Escape não dispara mais um alerta de depuração depois que você seguiu um link dentro dele.
* Copiar após Selecionar Tudo agora fornece todo o documento, em vez de apenas a parte carregada no momento.
* Localizar agora vai direto para a linha em que foi encontrado, em vez de fazer você ouvir o leitor de tela ler a janela novamente conforme o foco retorna ao livro.
* Corrigido EPUB que carregam um bloco ZIP64 perdido recusando-se a abrir com "Cabeçalho de arquivo local inválido".
* Corrigido documentos longos voltando ao início enquanto um leitor de tela lia continuamente através deles.
* Links no WebView agora o levam para a seção para a qual apontam, em vez de falhar com "Arquivo não encontrado".
* O anúncio automático "Documento recarregado" não corta mais seu leitor de tela no meio de uma frase, esperando que ele termine o que estava dizendo.
* A aba Geral do diálogo Configurações agora passa por suas opções na ordem em que aparecem na tela, com o canal de atualização logo após a opção verificar atualizações.
* Windows agora sempre mostrará "Paperback" no menu Abrir Com, em vez da tagline completa do programa.
* Contagem de Palavras e Informações do Documento agora mostram quantos arquivos um audiolivro contém e quanto tempo ele dura no total.

### Versão 0.9.1
* Sons de marcadores e anotações agora são reproduzidos no macOS.
* Livros DAISY agora reproduzem seu áudio no macOS, em vez de abrir e rastrear sua linha do tempo em silêncio.
* Corrigido o desaparecimento de aspas curvas, travessões e caracteres similares em documentos RTF, unindo as palavras circundantes.
* Corrigidas imagens RTF vazando seus dados brutos no documento como texto corrompido.
* Corrigido o submenu Documentos Recentes mantendo entradas antigas até que algo acontecesse para reconstruí-lo.
* Aceleradores de teclado estão de volta em todas as traduções, então os menus do russo têm acesso ao teclado novamente.
* Documentos CHM grandes agora abrem até sete vezes mais rápido.
* Documentos abertos agora são registrados no Windows, então aparecem na lista de atalhos da barra de tarefas e na lista recente do menu Iniciar.
* Opções foi renomeado para Configurações, correspondendo aos aplicativos móveis e, no macOS, à convenção da plataforma.
* Paperback agora se lembra da posição, tamanho e estado maximizado da janela entre execuções.
* Formas plurais agora são traduzidas, então mensagens que contam coisas leem corretamente em idiomas que precisam de mais de uma forma.
* Selecionar o ncc.html de um livro DAISY agora abre o audiolivro completo em vez de apenas seu texto.
* Os nomes de ação do diálogo Personalizar Atalhos de Teclado agora podem ser traduzidos.
* O título do documento agora vem primeiro na barra de título, então livros abertos podem ser diferenciados na barra de tarefas e Alt+Tab.
* O diálogo de atualização agora é traduzido.

### Versão 0.9.0

#### Adicionado

##### Geral
* Uma ferramenta CLI, chamada pb, para converter rapidamente qualquer um dos formatos suportados pelo Paperback para HTML, Markdown ou texto simples.
* Uma opção para recarregar documentos que foram modificados por outros programas no disco.
* Uma opção Ver Fonte para abrir a fonte de um documento em uma nova aba, útil para editar Markdown, por exemplo.
* O texto do documento agora é paginado, o que significa que você pode carregar livros com dezenas de milhões de palavras em apenas alguns segundos. Por favor, reporte qualquer coisa estranha encontrada com isso.

##### Suporte à Plataforma
* Suporte ARM64 do Windows!
* Suporte nativo do macOS!
* Um botão de alternância de tela cheia.

##### Diálogo Todos os Documentos
* Um botão de localização para encontrar livros faltantes que acabaram de mudar seu caminho.
* Um filtro de status e barra de status, para que você possa filtrar por status do documento e ver quantos documentos são mostrados e selecionados.
* O atalho `Ctrl+Shift+A` para desselecionar todos os documentos.

##### Opções e Legibilidade
* Uma aba de legibilidade, com as seguintes opções:
    * Quebra de linha (movida de geral);
    * Renderizar tabelas em linha (nova nesta versão, veja abaixo);
    * Fonte;
    * Cor de fundo;
    * Espaçamento de linhas;
    * Espaçamento de parágrafos;
    * Espaçamento de letras;
    * Alinhamento de texto.
* Um item de menu de quebra de linha e um atalho subsequente.
* Uma alternância para determinar como você deseja que as tabelas sejam exibidas, e unificou como as tabelas são exibidas em documentos.

##### Navegação
* Suporte para navegação por contêiner.
* Uma opção para mover automaticamente o cursor para o início da linha ao navegar entre linhas, semelhante ao modo de navegação em leitores de tela.
* O atalho de teclado com sinal de igual para anunciar sua porcentagem atual por meio de um documento.

##### Marcadores
* Marcadores temporários: você pode ter um por documento, e eles persistem. Use barra invertida para definir um e barra invertida invertida para pular para ele.

##### Contagem de Palavras
* Tempo de leitura estimado no diálogo de contagem de palavras, bem como a capacidade de definir sua velocidade de leitura para tornar essa métrica realmente útil.
* Se uma seleção está ativa quando você abre o diálogo de contagem de palavras, quantas palavras você tem selecionadas agora será mostrado.

##### Atalhos de Teclado
* A capacidade de personalizar cada atalho de teclado no aplicativo através de um diálogo simples.
* Um atalho de teclado configurável para restaurar Paperback da bandeja do sistema.

##### Idiomas
* Holandês, Finlandês e Polonês.

##### Exportar
* Expandido o item de menu de exportação para permitir exportar para HTML e Markdown, além de texto simples.

##### Atualizador
* Um botão de cancelamento para o diálogo de atualização em andamento.
* O atualizador agora valida que o arquivo baixado não foi adulterado.

##### Visualização Web
* A visualização web agora é aberta em sua posição de leitura atual.

##### Livros DAISY
* Suporte para livros DAISY 2.0.
* Suporte para reprodução de áudio DAISY 2.02.

##### Audiolivros
* A capacidade de reproduzir audiolivros, atualmente suportando áudio DAISY (incluindo áudio DAISY + texto) e zips de arquivos de áudio.
* Atalhos de teclado e itens de menu para reproduzir/pausar narração, buscar para frente e para trás, e ajustar o montante de busca.
* Opções para sincronizar o cursor de leitura com reprodução de áudio, definir o montante de busca de áudio e escolher se a busca além do final de um capítulo continua para o próximo.

##### Documentos CHM
* Suporte para listas, itens de lista, figuras e imagens.

##### PowerPoint
* Documentos PowerPoint agora suportam tabelas.

#### Corrigido

##### Geral
* Documentos codificados em codificações CJK legadas, como GBK, Big5 e Shift_JIS, agora serão renderizados corretamente em vez de aparecerem como mojibake.
* "Reabrir último fechado" tentando reabrir o readme agrupado.
* Sua guia selecionada não estava recebendo o foco adequado após reiniciar o Paperback.
* Tratamento do Paperback para arquivos em unidades de rede do Windows: pressionar mostrar arquivo em pasta agora foca adequadamente o arquivo no armazenamento em rede, e os caminhos não contêm mais caracteres estranhos.
* Arquivos .paperback não serão mais carregados forçadamente na restauração do documento; em vez disso, você será solicitado a confirmar quando um for encontrado.
* Abrir pasta contendo agora foca o arquivo fornecido no explorador.
* Abrir o readme agora respeitará seu idioma selecionado.
* A interface do usuário do Paperback agora será dimensionada corretamente em displays com alto DPI.
* O menu agora é atualizado corretamente e o foco passa para o controle de texto ao abrir ajuda no Paperback.
* Alterado para um método muito mais seguro de IPC no Windows.
* O título do documento ativo agora será lido ao alternar entre guias.
* Uso de memória reduzido em documentos grandes ao reduzir pela metade o tamanho das tabelas de índice interno por caractere.

##### Diálogo Todos os Documentos
* Escape não fechando os diálogos Informações do Documento e Todos os Documentos.
* A barra de título não sendo atualizada após fechar um documento do diálogo de todos os documentos.
* Readme.html não será mais adicionado à sua lista de todos os documentos quando aberto via `Shift+F1`.
* Remover documentos do diálogo de recentes agora também fechará sua guia ativa.
* Seu filtro de pesquisa agora é preservado após remover um documento.

##### Navegação
* Navegação de página anunciando texto de linha incorreto em algumas situações.
* Ir para Linha, Ir para Página e Ir para Percentual colocando seu cursor na posição errada em documentos grandes.
* Encontrar e Encontrar Próximo não respeitando a janela do documento carregado em documentos grandes.

##### Marcadores
* Os sons de marcador/nota agora devem ser reproduzidos corretamente exclusivamente quando você navega sobre uma palavra contendo um.

##### Legibilidade
* Aplicar quebra de linha enviando você para o início do seu documento.

##### Web View
* O diálogo de visualização da web não era redimensionável e abria em um tamanho inicial muito pequeno.
* Imagens agora devem ser exibidas corretamente na visualização da web incorporada.

##### Atualizador
* O atualizador agora exibe corretamente o conteúdo de tags de código markdown nas notas de versão.

##### Livros DAISY
* Livros DAISY mostrando informações incorretas na barra de status.
* Carregamento de livros DAISY com declarações de codificação falsas.

##### Documentos RTF
* Análise de documentos RTF com caracteres não-latinos neles.
* Grupos RTF `\pict` para que dados de imagem incorporados não vazem mais para o texto do documento.

##### Livros Mobi/AZW3
* Âncoras filepos em livros Mobi dividindo tags HTML e colocando lixo no texto do livro.
* Links em livros Mobi legados.
* Análise AZW3 muito melhorada.

##### Documentos Word
* Documentos Word com nomes de estilo específicos de locale não renderizando seus títulos corretamente.

##### Documentos HTML/XHTML
* Elementos dl, dt e dd não produzindo quebras de linha em documentos XHTML.

##### Documentos PDF
* Paperback agora retorna para extração de texto simples para PDFs falsamente marcados.
* Documentos PDF contendo caracteres de controle em seus títulos e/ou marcadores não causarão mais crash do Paperback ao abrir.

### Versão 0.8.5
* Adicionado suporte a páginas para livros epub.
* Adicionado suporte para documentos Microsoft Office criptografados. Atualmente, Word legado, Word moderno e Powerpoint moderno são suportados, com Powerpoint legado planejado para o futuro.
* Adicionado suporte para documentos Microsoft Word legados!
* Adicionado suporte para apresentações Powerpoint legadas!
* Adicionado suporte para livros mobi e AZW3!
* Adicionado suporte para arquivos PDF marcados!
* Adicionado o atalho `ctrl+q` para sair do aplicativo.
* Adicionado suporte para livros compactados do Bookshare (tanto DAISY quanto Word)!
* Texto alternativo para imagens incorporadas agora deve ser exibido corretamente.
* Documentos CHM agora suportam adequadamente a navegação de links internos.
* Corrigido ir para página estar deslocado em 1.
* Corrigido a tecla escape não funcionando para fechar o diálogo abrir como.
* Corrigido o menu de contexto do leitor não aparecendo ao clicar com o botão direito ou pressionar a tecla Aplicações.
* Corrigido o documento errado às vezes recebendo foco ao abrir documentos da linha de comando.
* PDFs somente com imagem são novamente detectados e o alertam sobre sua existência.
* Agora é possível navegar através de imagens e figuras com `g`/`Shift+G` e `f`/`Shift+F`, respectivamente.
* Paperback agora respeitará sua configuração de modo escuro de aplicação.
* Removido suporte a DAISY XML, pois não é mais necessário.
* Voltou a usar a navegação nativa de primeira letra Win32 na árvore de conteúdo.
* O diálogo de erro ao carregar agora mostra mensagens de erro mais detalhadas.
* A visualização da web agora será aberta muito mais rápido e suavemente.

### Versão 0.8.2
* Adicionado suporte a páginas para documentos RTF!
* Corrigido um bug onde abrir a visualização da web em epubs contendo links externos os ativaria automaticamente.
* Corrigido um bug onde o analisador RTF não colocaria um espaço entre palavras em casos raros.
* Corrigido parágrafos sendo divididos em múltiplas linhas curtas em alguns documentos PDF.
* Documentos PDF agora têm suporte básico a navegação de links e títulos!
* Abas RTF e alimentações de linha agora são renderizadas exatamente como aparecem no documento.
* Voltou a usar a biblioteca pdfium confiável e comprovada para análise de PDFs, tornando a renderização de PDF muito mais confiável novamente.

### Versão 0.8.1
* Adicionado `Ctrl+Shift+T` para reabrir o último documento fechado.
* O diálogo Todos os Documentos agora suporta seleção de múltiplos documentos para abrir de uma vez.
* Corrigido alguns bugs com o analisador RTF.
* Corrigido caminhos de arquivo contendo caracteres não-ASCII (como š, č, ć, ž bósnios) ficando corrompidos ao abrir um arquivo via uma segunda instância do Paperback.
* Corrigido texto de PDF sendo lido na ordem errada e espaçamento incorreto ao redor de palavras capitalizadas.
* Corrigido carregamento lento de documento ao abrir arquivos grandes.
* Corrigido a localização dos botões Sim/Não em diálogos de confirmação.

### Versão 0.8.0
* Adicionadas traduções para japonês, chinês simplificado e vietnamita!
* Adicionado um atualizador automático que agora substituirá sua versão instalada do Paperback em vez de apenas baixar a nova versão!
* Adicionado feedback de som opcional ao atingir um marcador ou uma nota, obrigado Andre Louis pelos sons!
* Adicionado suporte para documentos RTF!
* Adicionado suporte para documentos DAISY XML.
* Adicionado suporte para arquivos Flat Open Document Text!
* Adicionado suporte para apresentações Flat Open Document!
* Adicionado suporte para separadores com `s` e `shift+s`.
* Qualquer movimento maior que 300 caracteres agora adicionará automaticamente ao seu histórico de navegação.
* Corrigida a restauração da janela do Paperback pela bandeja do sistema.
* Corrigidos documentos Markdown mostrando texto bruto em vez de HTML renderizado na Web View.
* Corrigidas tabelas não renderizando corretamente em arquivos Markdown.
* PDFs contendo apenas imagens agora avisarão sobre sua existência quando você tentar carregar um.
* Informações de versão corretamente incorporadas no executável do Paperback.
* Dividido o diálogo de opções em abas para facilitar o uso e navegação.
* Alterado para Hayro para análise de PDFs, resultando em mais confiabilidade, velocidade e menos DLLs.
* Reescrito o aplicativo inteiro em Rust. A nova base de código é mais segura, carrega documentos mais rápido e é mais fácil de manter e expandir.
* O menu de contexto do controle de texto agora incluirá ações específicas do leitor em vez de itens genéricos como cortar e colar.

### Versão 0.7.0
* Adicionado suporte de tabelas para documentos baseados em HTML e XHTML! Navegue entre tabelas usando `T` e `Shift+T`, e pressione `Enter` para visualizar uma em um webview.
* Adicionado um recurso básico de renderização web! Pressione `Ctrl+Shift+V` para abrir a seção atual do seu documento em um renderizador baseado na web, útil para conteúdo como formatação complexa ou exemplos de código.
* Adicionada uma tradução para russo, obrigado Ruslan Gulmagomedov!
* Adicionado um botão Limpar Tudo ao diálogo Todos os Documentos.
* O verificador de atualizações agora exibe notas de lançamento quando uma nova versão está disponível.
* Corrigida a restauração da janela pela bandeja do sistema.
* Corrigidas as traduções dos botões Sim/Não em diálogos de confirmação.
* Corrigido o carregamento de configurações ao executar como administrador.
* Corrigido o tratamento de comentários em documentos XML e HTML.
* Corrigida análise do TOC em livros Epub 2.
* Corrigida navegação para o próximo item com a mesma letra no sumário.
* Corrigido o diálogo de busca não ocultando corretamente ao usar os botões próximo/anterior.
* Corrigido o TOC epub ocasionalmente levando você para o item errado.
* Corrigidos vários problemas de tratamento de espaçamento em XML, HTML e tags pre.
* Corrigido erro off-by-one na navegação de links.
* Corrigidos alguns livros com espaçamento em branco à direita nas linhas.
* Corrigidos vários problemas de análise.
* Itens de menu relacionados a marcadores e a lista de elementos agora estão corretamente desabilitados quando nenhum documento está aberto.
* Melhorado o tratamento de listas em vários formatos de documentos.
* Melhorado o fluxo de trabalho de tradução para colaboradores.
* Muitas refatorações internas, movendo a maioria da lógica de negócios do aplicativo de C++ para Rust para melhor desempenho e manutenibilidade.

### Versão 0.6.1
* Adicionado suporte para PDF protegido por senha!
* Adicionado um recurso muito básico para ir para a posição anterior/próxima. Se você pressionar `enter` em um link interno e isso mover seu cursor, essa posição agora será lembrada e poderá ser navegada com `alt+left/right`.
* Adicionada uma lista de elementos! No momento, ela mostra apenas uma árvore de todos os títulos no seu documento ou uma lista de links, mas há planos para expandi-la no futuro.
* Adicionada uma opção para iniciar o Paperback no modo maximizado por padrão.
* Corrigidos links em alguns documentos Epub não funcionando corretamente.
* Corrigida análise de TOCs Epub contendo caminhos relativos.
* Corrigidos alguns documentos epub não mostrando um título ou autor.
* Corrigidos os títulos de alguns capítulos epub não aparecendo corretamente no diálogo TOC.
* Corrigido você não conseguir usar a barra de espaço para ativar os botões OK/cancelar no diálogo TOC.
* Melhorado o tratamento de títulos em documentos Word.
* Você agora receberá feedback de voz se a lista de documentos recentes estiver vazia quando tentar abrir o diálogo.

### Versão 0.6.0
* Uma nova opção para mostrar o menu de navegação em uma forma muito mais compacta foi adicionada ao diálogo de opções, marcada por padrão.
* Adicionada uma opção para fazer a navegação por elementos estruturais envolver.
* Adicionada uma opção ao menu de ferramentas para abrir a pasta contendo o documento atualmente focado.
* Adicionado um sistema de atualização bastante simples, mas muito eficaz.
* Adicionado um recurso básico de temporizador de sono, acessível com Ctrl+Shift+S.
* Adicionado suporte para análise de ebooks FB2!
* Adicionado suporte para análise de apresentações OpenDocument!
* Adicionado suporte para análise de arquivos OpenDocument Text!
* Marcadores agora podem ser feitos para marcar uma linha inteira, ou para marcar apenas algum texto especificado. Se você não tiver uma seleção ativa ao colocar um marcador, o comportamento é como pré-0.6, e ele marcará a linha inteira. No entanto, se você selecionar algum texto, apenas esse texto será incluído no marcador.
* Marcadores agora podem ter notas de texto opcionais anexadas a eles! Navegue entre marcadores contendo notas com N e Shift+N, ou abra o diálogo de marcadores com todos os marcadores, apenas notas, ou apenas não-notas selecionados com teclas de atalho específicas.
* Marcadores no diálogo de marcadores não terão mais um incômodo prefixo "marcador x".
* Livros Epub contendo conteúdo HTML fingindo ser XML agora serão tratados adequadamente.
* Corrigido o carregamento de documentos Markdown grandes.
* Corrigido o pressionamento de espaço na visualização em árvore do sumário ativando o botão OK.
* Corrigido o tratamento de espaço em branco no início de tags pre em documentos HTML e XHTML.
* Corrigido o controle de texto não recuperando o foco às vezes ao retornar à janela do Paperback.
* Corrigido o campo de texto no diálogo ir para percentual não atualizando o valor do controle deslizante.
* Corrigida a renderização de IDs HTML personalizados em documentos Markdown.
* HTML dentro de blocos de código Markdown agora será renderizado adequadamente.
* Se carregar um livro com um parâmetro de linha de comando enquanto uma instância existente do Paperback estiver em execução, você não receberá mais um erro se o carregamento do seu documento levar mais de 5 segundos.
* Se executar o Paperback como administrador, a configuração agora será carregada e salva adequadamente.
* Agora é possível excluir um marcador diretamente no diálogo de marcadores.
* Agora é possível importar e exportar seus marcadores e posição de leitura para um documento específico. O arquivo gerado recebe o nome do arquivo com uma extensão .paperback. Se tal arquivo for encontrado no mesmo diretório de um arquivo durante o carregamento, ele será carregado automaticamente. Caso contrário, você pode importá-los manualmente usando um item no menu de ferramentas.
* Links dentro de documentos agora são totalmente suportados! Use k e shift+k para se mover para frente e para trás através deles, e pressione enter para abrir/ativar um.
* Muitas refatorações internas, tornando o aplicativo mais rápido e o binário menor.
* O conteúdo Markdown agora é pré-processado para ser compatível com CommonMark antes de ser renderizado.
* Navegação por listas e seus itens agora é totalmente suportada! Use L e Shift+L para ir por listas em si, e I e Shift+I para ir através de itens de lista.
* Numpad delete agora funciona para remover documentos da barra de abas além do delete normal.
* Paperback agora pode opcionalmente minimizar para a sua bandeja do sistema! Esta opção está desativada por padrão, mas ativá-la fará com que a opção minimizar no menu do sistema coloque o Paperback em sua bandeja, podendo ser restaurado clicando no ícone gerado.
* Paperback agora é totalmente traduzível! A lista de idiomas que ele suporta é atualmente bem pequena, mas está crescendo constantemente!
* Paperback agora tem um site oficial, em [paperback.dev](https://paperback.dev)!
* Documentos PPTX agora mostrarão um sumário básico, contendo todos os slides.
* O caminho completo para o documento aberto agora será mostrado no diálogo de informações do documento.
* O instalador agora inclui uma opção para visualizar o readme no seu navegador após a instalação.
* A lista de documentos recentes foi dramaticamente expandida! Em vez de simplesmente mostrar os últimos 10 documentos que você abriu, ela agora mostrará um número personalizável, com o resto dos documentos que você já abriu sendo acessível através de um pequeno diálogo.
* Várias pequenas melhorias nos analisadores em geral, incluindo colocar uma linha em branco entre slides em apresentações PPTX, corrigir o tratamento de quebras de linha dentro de parágrafos em documentos do Word, e adicionar pontos de bala aos itens de lista.

### Versão 0.5.0
* Adicionado suporte para documentos do Microsoft Word!
* Adicionado suporte para apresentações do PowerPoint!
* Corrigido o problema de certos itens de menu não serem desativados quando nenhum documento estava aberto.
* Corrigida a orientação do controle deslizante de ir para porcentagem.
* Corrigida a tabela de conteúdos em livros Epub com caminhos de arquivo codificados em URL e/ou IDs de fragmento.
* Corrigido o espaçamento em branco sendo removido de títulos XHTML de maneiras estranhas.
* Corrigido o tratamento de espaçamento em branco dentro de tags pre aninhadas em documentos HTML.
* Documentos HTML e Markdown agora suportam o recurso de tabela de conteúdos! Quando você carrega um documento HTML/Markdown, o Paperback criará sua própria tabela de conteúdos baseada na estrutura dos títulos do seu documento, e a mostrará para você no diálogo `ctrl+t`.
* Documentos HTML agora terão o título definido na tag title, se existir. Caso contrário, continuarão a usar o nome do arquivo sem a extensão.
* Mudança de UniversalSpeech para usar uma região dinâmica para relatar a fala. Isso significa que nenhuma DLL de leitor de tela é enviada junto com o programa, e mais leitores de tela serão suportados, como o Microsoft Narrator.
* Mudança das bibliotecas zip para permitir abrir uma gama mais ampla de livros epub.
* O diálogo que pergunta se você deseja abrir seu documento como texto simples foi completamente reformulado e agora permite que você abra seu documento como texto simples, HTML ou Markdown.
* O diálogo de ir para porcentagem agora inclui um campo de texto que permite inserir manualmente uma porcentagem para saltar.
* O analisador HTML agora reconhecerá dd, dt e dl como elementos de lista.
* A tabela de conteúdos em livros Epub será preservada exatamente mais uma vez.
* O espaço não quebrável Unicode agora é considerado ao remover linhas em branco.
* Você não será mais perguntado como deseja abrir um arquivo não reconhecido toda vez que o carrega, apenas na primeira vez.

### Versão 0.4.1
* Adicionado um ícone de menu inicial opcional ao instalador.
* A tabela de conteúdos deve estar mais limpa em alguns casos. Por exemplo, se você tiver um item filho e pai com o mesmo texto na mesma posição, você verá apenas o item pai.
* Corrigida a tabela de conteúdos em certos documentos CHM.
* Corrigida a tabela de conteúdos em livros Epub 3 com caminhos absolutos.
* Documentos CHM agora devem mostrar seu título conforme definido no arquivo de metadados.

### Versão 0.4.0
* Adicionado suporte para arquivos CHM!
* Adicionado suporte para marcadores! Você pode ter quantos marcadores quiser em quantos documentos quiser. Você pode avançar e retroceder através deles com `b` e `shift+b`, definir um com `control+shift+b` e abrir um diálogo para saltar para um marcador específico com `control+b`.
* Adicionado um instalador junto com o arquivo zip portátil! O instalador instalará o Paperback em seu diretório Program Files e configurará automaticamente as associações de arquivo para você.
* Arquivos de texto com BOMs agora devem ser decodificados corretamente, e o BOM não será mais exibido no início do texto.
* Adicionado muito mais informações à barra de status. Agora mostrará sua linha atual, caractere e porcentagem de leitura.
* Comentários HTML, bem como o conteúdo de tags script e style, não serão mais mostrados na saída de texto.
* Se passar um caminho relativo para o Paperback na linha de comando, ele agora o resolverá corretamente.
* O movimento de porcentagem agora é tratado por seu próprio diálogo baseado em controle deslizante, acessível com `control+shift+g`.
* Documentos sem títulos ou autores conhecidos agora sempre terão um padrão.
* A lógica de salvamento de posição é muito mais inteligente e deve escrever no disco apenas quando absolutamente necessário.
* O documento que você tinha em foco quando fechou o Paperback agora é lembrado entre reinicializações do aplicativo.
* A entrada nos diálogos de ir para linha e ir para página agora deve ser sanitizada mais rigorosamente.
* Corrigida a navegação da tabela de conteúdos em livros epub 3 com caminhos relativos em seus manifestos.

### Versão 0.3.0
* Corrigida a tabela de conteúdos em livros epub com manifestos codificados em URL.
* Corrigida a navegação de títulos em documentos HTML contendo caracteres Unicode multi-byte.
* Corrigido o uso alto de CPU em documentos com títulos longos devido a uma regressão no wxWidgets.
* Corrigido o carregamento de arquivos de texto UTF-8.
* Corrigidos itens TOC aninhados em livros Epub colocando seu cursor na posição errada.
* Corrigido um travamento ao sair do aplicativo em certos casos.
* Adicionada uma caixa de seleção no diálogo de opções para ativar ou desativar o ajuste de linha!
* Agora é possível fazer uma doação para o desenvolvimento do Paperback, seja através do novo item doação no menu de ajuda ou através do link patrocinar este projeto na parte inferior da página principal do repositório do GitHub.
* Documentos Markdown agora sempre terão um título, e o Paperback agora deve ser capaz de carregar praticamente qualquer arquivo Markdown.
* Documentos PDF agora sempre terão um título, mesmo que os metadados estejam faltando.
* Mudança de bibliotecas PDF para a usada no Chromium, levando a uma análise de PDF muito mais confiável em geral.
* Você agora pode ter apenas uma instância do Paperback em execução por vez. Executar paperback.exe com um nome de arquivo enquanto já está em execução abrirá esse documento na instância já em execução.
* Você agora pode pressionar delete em um documento no controle de guia para fechá-lo.

### Versão 0.2.1
* Adicionado o número total de páginas ao rótulo de página no diálogo de ir para página.
* Permitir tabulação do conteúdo do documento para sua lista de documentos abertos.
* Corrigidos os pressionamentos de tecla de título às vezes abrindo documentos recentes se você tivesse o suficiente deles.
* O Paperback agora removerá hífens suaves desnecessários da saída de texto.
* Corrigida a navegação de títulos às vezes colocando você no caractere errado.

### Versão 0.2.0
* Adicionado suporte a documentos markdown!
* Adicionado suporte a documentos PDF, incluindo a capacidade de navegar entre páginas!
* Adicionadas teclas de atalho para navegar por títulos em conteúdo HTML, incluindo livros epub e documentos markdown. Essas teclas de atalho foram projetadas para funcionar de forma similar a um leitor de tela.
* Corrigido carregamento de epubs com nomes de arquivo codificados em URL em seus manifestos.
* Corrigido carregamento de livros epub 3 com XHTML incorporado neles.
* Uma mensagem é agora falada se o documento não suportar um sumário ou seções, em vez dos itens do menu serem desabilitados.
* Adicionado um menu de documentos recentes! Atualmente, ele armazena seus últimos 10 documentos abertos, e pressionar enter em um abrirá para leitura.
* Reescrita completa da caixa de diálogo Localizar, tornando-a muito mais simples de usar, além de adicionar um histórico de suas últimas 25 buscas e suporte a expressões regulares!
* Documentos abertos anteriormente agora são lembrados entre reinicializações da aplicação. Isso é configurável através do novo item de opções no menu ferramentas.
* Adicionado `shift+f1` para abrir o arquivo readme diretamente no Paperback.

### Versão 0.1.0
* Lançamento inicial.
