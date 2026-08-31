<!-- machine-translated from doc/readme.md (source-hash: df18cffffe239932); please review and edit as needed -->

# Paperback - version 0.9.1

## Introduction

Paperback est un lecteur d'ebooks et de documents léger, rapide et accessible pour tous, des lecteurs occasionnels aux utilisateurs avancés. Il est conçu pour l'accessibilité aux lecteurs d'écran, la rapidité et une expérience sans surcharge.

## Configuration requise

Paperback fonctionne actuellement sur Windows 10/11 et toutes les versions modernes d'ARM macOS. Les applications natives iOS et Android sont en développement actif, avec des versions de test publiques prévues peu de temps après la sortie de la version 0.9.0 du bureau, avant une sortie unifiée 1.0 couvrant les quatre plates-formes.

## Fonctionnalités

* Complètement autonome, ne nécessitant aucun logiciel à installer sur votre ordinateur pour commencer à lire.
* Incroyablement rapide, même sur un ancien matériel.
* Interface à onglets simple, vous permettant d'ouvrir autant de documents que vous le souhaitez côte à côte.
* Enregistre votre position de lecture exacte dans chaque document que vous ouvrez.
* Mémorise facultativement les documents que vous aviez ouverts à la fermeture du programme et les restaure au lancement suivant.
* Inclut une fonctionnalité de navigation similaire à celle que l'on trouve dans le mode de navigation Web de nombreux lecteurs d'écran pour naviguer rapidement et facilement dans les documents.
* Inclut une boîte de dialogue de recherche robuste, avec des fonctionnalités telles que l'historique et le support des expressions régulières.
* Peut fonctionner entièrement de façon portable ou être installé avec les associations de fichiers configurées automatiquement.
* Prend en charge un vaste ensemble de formats de fichiers courants.

## Compatibilité avec les lecteurs d'écran

Paperback fonctionne bien avec tous les lecteurs d'écran majeurs. Il y a, cependant, un problème connu pour les utilisateurs de JAWS.

### JAWS et affichages Braille

Si vous utilisez JAWS avec un affichage Braille, vous constaterez peut-être que les longs paragraphes sont tronqués lors du panoramique vers l'avant avec les touches de navigation de votre affichage. La commande de lecture du paragraphe actuel est également affectée. C'est un bogue dans le traitement par JAWS du contrôle de texte RICHEDIT50W, et non quelque chose dans Paperback lui-même, et c'est un problème qui a pris un certain temps à trouver une correction étant donné l'enthousiasme de Vispero à répondre aux problèmes concernant les logiciels open source.

La solution de contournement, finalement révélée par le groupe de discussion JAWS après des mois d'attente, consiste à éditer `paperback.jcf` et à définir « Braille Presentation and Panning » sur « Always use DOM if available ». Vous voudrez également activer « Pan Text by Paragraph », sinon votre affichage restera sur le paragraphe actif plutôt que d'avancer. Avec les deux paramètres en place, le panoramique devrait fonctionner correctement.

## Types de fichiers actuellement pris en charge

Paperback prend en charge les formats et extensions suivants :

* Fichiers d'aide CHM (`.chm`)
* Livres DAISY (`.opf`, `.zip`)
* Livres EPUB (`.epub`)
* Ebooks FB2 (`.fb2`)
* Documents HTML (`.htm`, `.html`, `.xhtml`)
* Documents Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Documents Microsoft Word (`.docx`, `.docm`, `.doc`)
* Livres MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
* Présentations OpenDocument (`.odp`, `.fodp`)
* Fichiers texte OpenDocument (`.odt`, `.fodt`)
* Documents PDF (`.pdf`)
* Présentations PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Documents RTF (`.rtf`)
* Fichiers texte brut et journaux (`.txt`, `.log`)

## Raccourcis clavier

Paperback est conçu pour une utilisation en priorité au clavier. Voici les raccourcis actuels.

Les raccourcis ci-dessous sont pour Windows. Lorsque macOS diffère, l'équivalent est indiqué entre parenthèses — principalement parce que Ctrl+G, Ctrl+W et Alt+Left/Right sont déjà utilisés par d'autres conventions du système ou de l'application sur cette plateforme.

### Menu Fichier

* `Ctrl+O` : Ouvrir un document.
* `Ctrl+F4` (macOS : `Cmd+W`) : Fermer le document courant.
* `Ctrl+Shift+F4` (macOS : `Cmd+Shift+W`) : Fermer tous les documents ouverts.
* `Ctrl+Shift+T` : Rouvrir le dernier document fermé.
* `Ctrl+R` : Afficher la boîte de dialogue « Tous les documents » (à partir des documents récents).
* `Ctrl+Q` : Quitter (Windows uniquement ; sur macOS, cela se trouve dans le menu de l'application).

### Menu Aller

* `Ctrl+F` : Afficher la boîte de dialogue Rechercher.
* `F3` (macOS : `Cmd+G`) : Trouver le suivant.
* `Shift+F3` (macOS : `Cmd+Shift+G`) : Trouver le précédent.
* `Ctrl+G` (macOS : `Cmd+L`) : Aller à la ligne.
* `Ctrl+Shift+G` (macOS : `Cmd+Shift+L`) : Aller au pourcentage.
* `Ctrl+P` : Aller à la page (si supporté par le document courant).
* `=` : Annoncer votre pourcentage de lecture courant.
* `Alt+Left` (macOS : `Cmd+[`) : Aller en arrière dans l'historique de navigation.
* `Alt+Right` (macOS : `Cmd+]`) : Aller en avant dans l'historique de navigation.
* `[` : Section précédente.
* `]` : Section suivante.
* `Shift+H` : Titre précédent.
* `H` : Titre suivant.
* `Shift+1` à `Shift+6` : Titre précédent au niveau 1-6.
* `1` à `6` : Titre suivant au niveau 1-6.
* `Shift+P` : Page précédente.
* `P` : Page suivante.
* `Shift+B` : Signet précédent.
* `B` : Signet suivant.
* `/` : Définir votre signet temporaire.
* `\` : Aller à votre signet temporaire.
* `Shift+N` : Note précédente.
* `N` : Note suivante.
* `Ctrl+B` : Aller à tous les signets et notes.
* `Ctrl+Alt+B` : Aller aux signets uniquement.
* `Ctrl+Alt+M` : Aller aux notes uniquement.
* `Ctrl+Shift+W` (macOS : `RawCtrl+Shift+W`, c'est-à-dire la touche Control physique plutôt que Cmd) : Afficher le texte de la note à la position courante.
* `Shift+K` : Lien précédent.
* `K` : Lien suivant.
* `Shift+G` : Image précédente.
* `G` : Image suivante.
* `Shift+F` : Figure précédente.
* `F` : Figure suivante.
* `Shift+T` : Tableau précédent.
* `T` : Tableau suivant.
* `Shift+S` : Séparateur précédent.
* `S` : Séparateur suivant.
* `Shift+L` : Liste précédente.
* `L` : Liste suivante.
* `Shift+I` : Élément de liste précédent.
* `I` : Élément de liste suivant.
* `Shift+,` : Aller au début du conteneur courant (liste ou tableau).
* `,` : Aller après la fin du conteneur courant (liste ou tableau).

### Menu Outils

* `Ctrl+W` (macOS : `RawCtrl+W`, c'est-à-dire la touche Control physique plutôt que Cmd) : Afficher le nombre de mots du document courant.
* `Ctrl+I` : Afficher les informations du document.
* `Ctrl+T` : Afficher la table des matières.
* `F7` : Afficher la liste des éléments.
* `Ctrl+Shift+C` : Ouvrir le dossier contenant.
* `Ctrl+Shift+V` : Ouvrir le contenu courant dans la vue Web.
* `Ctrl+U` : Afficher la source du document dans un nouvel onglet.
* `Ctrl+Shift+E` : Exporter les données du document (`.paperback`).
* `Ctrl+Shift+I` : Importer les données du document (`.paperback`).
* `Ctrl+E` : Exporter le document courant en texte brut.
* `Ctrl+Shift+B` : Basculer le signet à la sélection/position courante.
* `Ctrl+Shift+N` : Ajouter ou modifier la note du signet à la sélection/position courante.
* `Ctrl+Alt+W` : Basculer le retour à la ligne automatique.
* `Ctrl+Space` : Lire/Pause la narration audio.
* `'` : Avancer la narration audio.
* `;` : Reculer la narration audio.
* `Ctrl+'` : Augmenter le décalage de recherche audio.
* `Ctrl+;` : Diminuer le décalage de recherche audio.
* `F11` (macOS : `RawCtrl+Ctrl+F`, c'est-à-dire Control+Command+F) : Basculer le plein écran.
* `Ctrl+,` : Ouvrir les options (macOS : Préférences, dans le menu de l'application).
* `Ctrl+Shift+S` : Basculer la minuterie de mise en veille.

### Menu Aide

* `Ctrl+F1` : Afficher la boîte de dialogue À propos.
* `F1` : Afficher l'aide dans votre navigateur par défaut.
* `Shift+F1` : Afficher l'aide dans Paperback.
* `Ctrl+Shift+U` : Vérifier les mises à jour.
* `Ctrl+D` : Ouvrir la page de don dans votre navigateur par défaut.

### Touches supplémentaires de vue de document

* `Delete` / `Numpad Delete` sur le contrôle d'onglet : Fermer l'onglet du document sélectionné.
* `Enter` ou `Space` dans le texte du document : Activer le lien au curseur, ou ouvrir une vue de tableau lorsque vous êtes sur un marqueur de tableau.
* `Shift+F10` ou la touche Menu/Application dans le texte du document : Ouvrir le menu contextuel.

## Langues supportées

Paperback est traduit dans de nombreuses langues différentes, et d'autres sont ajoutées tout le temps. Une liste complète suit ci-dessous.

Pour en savoir plus sur la contribution, veuillez consulter notre [Guide de traduction](translating.md).

* Bosniaque
* Tchèque
* Néerlandais
* Finnois
* Français
* Allemand
* Japonais
* Polonais
* Portugais (Brésil)
* Russe
* Chinois simplifié
* Serbe
* Espagnol
* Vietnamien

## Crédits
### Développement
* Quin Gillespie : développeur principal et fondateur du projet.
* Aryan Choudhary : contributeur principal.

### Dons
Les personnes suivantes ont fait des dons de quelque envergure au développement de Paperback. Si vous faites un don, votre nom ne sera pas automatiquement ajouté ici, je n'ajoute que les personnes qui souhaitent que leur don soit public.

Remarque : Je considère qu'être un sponsor public sur GitHub justifie une inclusion automatique dans cette liste.

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

## Journal des modifications

### Version 0.9.2
* Les livres audio ne font plus lire au lecteur d'écran une série d'espaces lors de la mise au point sur le champ de texte.
* Les livres audio nomment maintenant le fichier au fur et à mesure que vous les parcourez par section.
* Les livres audio rapportent maintenant leur vraie durée, au lieu de prétendre que chaque fichier dure 24 heures.
* Fermer Web View avec Échap ne lève plus une alerte de débogage après avoir suivi un lien à l'intérieur.
* Copier après Sélectionner tout vous donne maintenant le document entier, au lieu de seulement la partie actuellement chargée.
* Chercher va maintenant directement à la ligne trouvée, au lieu de vous faire écouter le lecteur d'écran relire la fenêtre alors que le focus revient au livre.
* Corrigé les EPUB comportant un bloc ZIP64 isolé refusant de s'ouvrir avec "En-tête de fichier local invalide".
* Corrigé les longs documents revenant à leur début pendant qu'un lecteur d'écran les lisait continuellement.
* Les liens dans Web View vous conduisent maintenant à la section vers laquelle ils pointent, au lieu d'échouer avec "Fichier non trouvé".
* L'annonce automatique "Document rechargé" ne coupe plus votre lecteur d'écran en pleine phrase, attendant plutôt qu'il finisse ce qu'il disait.
* L'onglet Général du dialogue Paramètres parcourt maintenant ses options dans l'ordre où elles apparaissent à l'écran, avec le canal de mise à jour directement après l'option de vérification des mises à jour.
* Windows affichera maintenant toujours "Paperback" dans le menu Ouvrir avec, au lieu du slogan complet du programme.
* Word Count et Document Info montrent maintenant combien de fichiers un livre audio contient, et combien de temps il dure au total.

### Version 0.9.1
* Les sons de signets et de notes jouent maintenant sur macOS.
* Les livres DAISY jouent maintenant leur audio sur macOS, au lieu de s'ouvrir et de suivre leur chronologie en silence.
* Corrigé les guillemets courbes, tirets longs et caractères similaires disparaissant des documents RTF, fusionnant les mots environnants.
* Corrigé les images RTF fuyant leurs données brutes dans le document sous forme de texte brouillé.
* Corrigé le sous-menu Documents récents gardant les anciennes entrées jusqu'à ce que quelque chose d'autre le reconstruise.
* Les accélérateurs clavier sont de retour dans chaque traduction, donc les menus russes ont à nouveau accès au clavier.
* Les grands documents CHM s'ouvrent maintenant jusqu'à sept fois plus vite.
* Les documents ouverts sont maintenant enregistrés auprès de Windows, donc ils apparaissent dans la liste de sauts de la barre des tâches et dans la liste récente du menu Démarrer.
* Options a été renommé en Paramètres, correspondant aux applications mobiles et, sur macOS, à la convention de la plateforme.
* Paperback mémorise maintenant sa position de fenêtre, sa taille et son état maximisé entre les exécutions.
* Les formes plurielles sont maintenant traduites, donc les messages qui comptent les choses se lisent correctement dans les langues qui ont besoin de plus d'une forme.
* Sélectionner le ncc.html d'un livre DAISY ouvre maintenant le livre audio complet au lieu de son seul texte.
* Les noms d'actions du dialogue Personnaliser les raccourcis clavier peuvent maintenant être traduits.
* Le titre du document vient maintenant en premier dans la barre de titre, donc les livres ouverts peuvent être distingués dans la barre des tâches et Alt+Tab.
* Le dialogue de mise à jour est maintenant traduit.

### Version 0.9.0

#### Ajouté

##### Général
* Un outil CLI, appelé pb, pour convertir rapidement l'un des formats pris en charge par Paperback en HTML, Markdown ou texte brut.
* Une option pour recharger les documents qui ont été modifiés par d'autres programmes sur le disque.
* Une option Afficher la source pour ouvrir la source d'un document dans un nouvel onglet, utile par exemple pour éditer du Markdown.
* Le texte du document est maintenant paginé, ce qui signifie que vous pouvez charger des livres avec des dizaines de millions de mots en quelques secondes seulement. Veuillez signaler toute bizarrerie trouvée avec cela.

##### Support de plateforme
* Support de Windows ARM64 !
* Support natif de macOS !
* Une bascule de plein écran.

##### Dialogue Tous les documents
* Un bouton localiser pour localiser les livres manquants qui viennent de changer de chemin.
* Un filtre de statut et une barre de statut, ce qui vous permet de filtrer par statut du document et de voir combien de documents sont affichés et sélectionnés.
* Le raccourci `Ctrl+Shift+A` pour désélectionner tous les documents.

##### Options et lisibilité
* Un onglet lisibilité, avec les options suivantes :
    * Retour à la ligne (déplacé de général) ;
    * Rendre les tableaux en ligne (nouveau dans cette version, voir ci-dessous) ;
    * Police ;
    * Couleur de fond ;
    * Interligne ;
    * Espacement des paragraphes ;
    * Espacement des lettres ;
    * Alignement du texte.
* Un élément de menu de retour à la ligne et un raccourci clavier ultérieur.
* Une bascule pour déterminer comment vous voulez que les tableaux soient affichés, et uniformiser l'affichage des tableaux dans tous les documents.

##### Navigation
* Support de la navigation par conteneur.
* Une option pour déplacer automatiquement le curseur au début de la ligne lors de la navigation entre les lignes, similaire au mode naviguation dans les lecteurs d'écran.
* Le raccourci clavier égal pour annoncer votre pourcentage actuel dans un document.

##### Signets
* Signets temporaires : vous pouvez en avoir un par document, et ils persistent. Utilisez la barre oblique pour en définir un et la barre oblique inversée pour y accéder.

##### Décompte des mots
* Temps de lecture estimé dans le dialogue de décompte des mots, ainsi que la possibilité de définir votre vitesse de lecture pour rendre cette métrique réellement utile.
* Si une sélection est active lorsque vous ouvrez le dialogue de décompte des mots, le nombre de mots que vous avez sélectionnés s'affichera maintenant.

##### Raccourcis clavier
* La possibilité de personnaliser chaque raccourci clavier de l'application via un simple dialogue.
* Un raccourci clavier configurable pour restaurer Paperback à partir du plateau système.

##### Langues
* Néerlandais, finnois et polonais.

##### Export
* Extension de l'élément de menu d'export pour permettre l'export en HTML et Markdown, en plus du texte brut.

##### Mise à jour
* Un bouton d'annulation au dialogue de mise à jour en cours.
* La mise à jour valide maintenant que le fichier téléchargé n'a pas été altéré.

##### Web View
* La webview est maintenant ouverte à votre position de lecture actuelle.

##### Livres DAISY
* Support des livres DAISY 2.0.
* Support de la lecture audio DAISY 2.02.

##### Livres audio
* La possibilité de lire des livres audio, prenant actuellement en charge les audio DAISY (y compris l'audio DAISY + texte) et les zips de fichiers audio.
* Des raccourcis clavier et des éléments de menu pour jouer/mettre en pause la narration, avancer et reculer, et ajuster le montant de la recherche.
* Des options pour synchroniser le curseur de lecture à la lecture audio, définir le montant de la recherche audio et choisir si la recherche passée la fin d'un chapitre continue dans le suivant.

##### Documents CHM
* Support des listes, des éléments de liste, des figures et des images.

##### PowerPoint
* Les documents PowerPoint prennent maintenant en charge les tableaux.

#### Corrigé

##### Général
* Les documents codés en anciens encodages CJK, tels que GBK, Big5 et Shift_JIS, s'affichent maintenant correctement au lieu d'un tas de mojibake.
* "Rouvrir le dernier fermé" tentant de rouvrir le readme inclus.
* Votre onglet sélectionné ne recevant pas correctement le focus après le redémarrage de Paperback.
* La gestion par Paperback des fichiers sur les lecteurs réseau Windows : appuyer sur afficher le fichier dans le dossier met maintenant correctement l'accent sur le fichier sur le stockage réseau, et les chemins ne contiennent plus de caractères étranges.
* Les fichiers .paperback ne seront plus forcément chargés lors de la restauration du document ; au lieu de cela, vous serez invité à confirmer lorsque vous en trouverez un.
* Open containing folder met maintenant l'accent sur le fichier donné dans l'explorateur.
* L'ouverture du readme respectera maintenant votre langue sélectionnée.
* L'interface utilisateur de Paperback s'adaptera maintenant correctement sur les écrans haute DPI.
* Le menu se met maintenant à jour correctement et le focus se déplace vers le contrôle de texte lors de l'ouverture de l'aide dans Paperback.
* Passer à une méthode IPC beaucoup plus sécurisée sur Windows.
* Le titre du document actif sera maintenant lu lors du changement d'onglets.
* Réduction de l'utilisation de la mémoire sur les grands documents en réduisant de moitié la taille des tableaux d'index internes par caractère.

##### Dialogue Tous les documents
* Échap ne fermant pas les dialogues Document Info et Tous les documents.
* La barre de titre ne se mettant pas à jour après la fermeture d'un document depuis le dialogue tous les documents.
* Readme.html ne sera plus ajouté à votre liste de tous les documents lors de l'ouverture via Shift+F1.
* La suppression de documents du dialogue récents fermera maintenant également leurs onglets actifs.
* Votre filtre de recherche est maintenant préservé après la suppression d'un document.

##### Navigation
* La navigation de page annonçant un texte de ligne incorrect dans certaines situations.
* Go to Line, Go to Page et Go to Percent plaçant votre curseur à la mauvaise position dans les grands documents.
* Find et Find Next ne respectant pas la fenêtre de document chargée dans les grands documents.

##### Signets
* Les sons des signets/notes doivent maintenant jouer exclusivement lorsque vous naviguez sur un mot contenant un.

##### Lisibilité
* L'application du retour à la ligne vous tirant au début de votre document.

##### Web View
* Le dialogue webview n'étant pas redimensionnable et surgissant à une très petite taille initiale.
* Les images doivent maintenant s'afficher correctement dans la webview intégrée.

##### Mise à jour
* La mise à jour affiche maintenant correctement le contenu des balises de code markdown dans les notes de version.

##### Livres DAISY
* Les livres DAISY montrant des informations incorrectes dans la barre de statut.
* Chargement de livres DAISY avec des déclarations d'encodage fantaisistes.

##### Documents RTF
* Analyse des documents RTF contenant des caractères non-latins.
* Les groupes RTF `\pict` afin que les données d'image intégrées ne fuient plus dans le texte du document.

##### Livres Mobi/AZW3
* Les ancres filepos dans les livres Mobi divisant les balises HTML et mettant des ordures dans le texte du livre.
* Les liens dans les livres Mobi hérités.
* Analyse AZW3 considérablement améliorée.

##### Documents Word
* Les documents Word avec des noms de styles spécifiques aux paramètres régionaux ne rendant pas correctement leurs en-têtes.

##### Documents HTML/XHTML
* Les éléments dl, dt et dd ne produisant pas de sauts de ligne dans les documents XHTML.

##### Documents PDF
* Paperback revient maintenant à l'extraction de texte brut pour les PDF mal étiquetés.
* Les documents PDF contenant des caractères de contrôle dans leurs titres et/ou signets ne bloqueront plus Paperback lors de l'ouverture.

### Version 0.8.5
* Support des pages ajouté aux livres epub.
* Ajout du support des documents Microsoft Office chiffrés. Actuellement, Word hérité, Word moderne et Powerpoint moderne sont pris en charge, avec Powerpoint hérité prévu pour l'avenir.
* Ajout du support des documents Microsoft Word hérités (*.doc) !
* Ajout du support des présentations Powerpoint hérités (*.ppt) !
* Ajout du support des livres mobi et AZW3 !
* Ajout du support des fichiers PDF balisés !
* Ajout du raccourci ctrl+q pour quitter l'application.
* Ajout du support des livres zippés de Bookshare (DAISY et Word) !
* Le texte alternatif pour les images intégrées devrait maintenant s'afficher correctement.
* Les documents CHM supportent maintenant correctement la navigation par lien interne.
* Corrigé les sons de signets se déclenchant au début du paragraphe au lieu de la position du signet.
* Corrigé l'aller à page décalé de 1.
* Corrigé la touche d'échappement ne fonctionnant pas pour fermer le dialogue d'ouverture en tant que.
* Corrigé le menu contextuel du lecteur ne s'affichant pas au clic droit ou à la touche Applis.
* Corrigé le mauvais document parfois mis au point lors de l'ouverture de documents depuis la ligne de commande.
* Les PDF contenant uniquement des images sont à nouveau détectés et vous alertent de leur existence.
* Il est maintenant possible de naviguer dans les images et les figures avec g/shift+g et f/shift+f, respectivement.
* Paperback respectera maintenant votre paramètre de mode sombre de l'application.
* Suppression du support DAISY XML, car il n'est plus nécessaire.
* Retour à la navigation native Win32 par première lettre dans l'arborescence de la table des matières.
* Le dialogue d'erreur de chargement affiche maintenant des messages d'erreur plus détaillés.
* La webview s'ouvrira maintenant beaucoup plus vite et en douceur.

### Version 0.8.2
* Support des pages ajouté aux documents RTF !
* Corrigé un bogue où l'ouverture de la webview dans les epub contenant des liens externes les activerait automatiquement.
* Corrigé un bogue où l'analyseur RTF ne mettrait pas d'espace entre les mots dans les cas rares.
* Corrigé les paragraphes étant divisés en plusieurs lignes courtes dans certains documents PDF.
* Les documents PDF ont maintenant un support de base de la navigation par lien et en-tête !
* Les onglets et les sauts de ligne RTF sont maintenant rendus exactement comme ils apparaissent dans le document.
* Retour à la bibliothèque pdfium éprouvée pour l'analyse des PDF, rendant l'analyse des PDF beaucoup plus fiable à nouveau.

### Version 0.8.1
* Ajout de Ctrl+Shift+T pour rouvrir le dernier document fermé.
* Le dialogue Tous les documents prend maintenant en charge la sélection de plusieurs documents à ouvrir à la fois.
* Corrigé quelques bogues avec l'analyseur RTF.
* Corrigé les chemins de fichiers contenant des caractères non-ASCII (tels que le bosniaque š, č, ć, ž) étant corrompus lors de l'ouverture d'un fichier via une deuxième instance de Paperback.
* Corrigé le texte PDF étant lu dans le mauvais ordre et l'espacement incorrect autour des mots en majuscules.
* Corrigé le chargement lent des documents lors de l'ouverture de grands fichiers.
* Corrigé la localisation des boutons Oui/Non dans les dialogues de confirmation.

### Version 0.8.0
* Ajout des traductions japonaise, chinoise simplifiée et vietnamienne !
* Ajout d'un programme de mise à jour automatique qui remplacera maintenant votre version actuellement installée de Paperback au lieu de simplement télécharger la nouvelle version !
* Ajout d'un retour sonore facultatif pour atteindre un signet ou une note, merci Andre Louis pour les sons !
* Ajout du support des documents RTF !
* Ajout du support des documents DAISY XML.
* Ajout du support des fichiers Flat Open Document Text !
* Ajout du support des présentations Flat Open Document !
* Ajout du support des séparateurs avec s et shift+s.
* Tout mouvement supérieur à 300 caractères ajoutera maintenant automatiquement à votre historique de navigation.
* Corrigé la restauration de la fenêtre de Paperback à partir du plateau système.
* Corrigé les documents Markdown affichant du texte brut au lieu du HTML rendu dans la Web View.
* Corrigé les tableaux ne s'affichant pas correctement dans les fichiers Markdown.
* Les PDF contenant uniquement des images vous avertissent maintenant de leur existence lorsque vous tentez d'en charger un.
* Il est maintenant possible de vérifier les nouvelles versions de développement au lieu des versions stables lors de la vérification des mises à jour.
* Intégration correcte des informations de version dans l'exécutable Paperback.
* Division du dialogue des options en onglets pour faciliter l'utilisation et la navigation.
* Passage à Hayro pour l'analyse des PDF, ce qui améliore la fiabilité, la vitesse et réduit les DLL.
* Réécriture de toute l'application en Rust. La nouvelle base de code est plus sûre, charge les documents plus vite et est plus facile à maintenir et à étendre.
* Le menu contextuel du contrôle de texte inclura maintenant des actions spécifiques au lecteur au lieu d'éléments génériques tels que couper et coller.

### Version 0.7.0
* Ajout du support des tableaux pour les documents basés sur HTML et XHTML ! Naviguez entre les tableaux en utilisant T et Shift+T, et appuyez sur Entrée pour en voir un dans une webview.
* Ajout d'une fonction de rendu web basique ! Appuyez sur Ctrl+Shift+V pour ouvrir la section actuelle de votre document dans un moteur de rendu basé sur le web, utile pour le contenu comme le formatage complexe ou les exemples de code.
* Ajout d'une traduction russe, merci Ruslan Gulmagomedov !
* Ajout d'un bouton Effacer tout au dialogue Tous les documents.
* Le vérificateur de mise à jour affiche maintenant les notes de version lorsqu'une nouvelle version est disponible.
* Corrigé la restauration de la fenêtre à partir du plateau système.
* Corrigé les traductions des boutons Oui/Non dans les dialogues de confirmation.
* Corrigé le chargement des configurations lors de l'exécution en tant qu'administrateur.
* Corrigé la gestion des commentaires dans les documents XML et HTML.
* Corrigé l'analyse de la table des matières dans les livres Epub 2.
* Corrigé la navigation vers l'élément suivant avec la même lettre dans la table des matières.
* Corrigé le dialogue de recherche ne se cachant pas correctement lors de l'utilisation des boutons suivant/précédent.
* Corrigé les tables des matières epub vous jetant occasionnellement au mauvais élément.
* Corrigé divers problèmes de gestion des espaces blancs dans les balises XML, HTML et pre.
* Corrigé l'erreur hors d'un dans la navigation des liens.
* Corrigé certains livres ayant des espaces blancs à la fin de leurs lignes.
* Corrigé divers problèmes d'analyseur.
* Les éléments du menu liés aux signets ainsi que la liste des éléments sont maintenant correctement désactivés lorsqu'aucun document n'est ouvert.
* Amélioration de la gestion des listes dans divers formats de document.
* Amélioration du flux de travail de traduction pour les contributeurs.
* Nombreuses refactorisations internes, déplaçant la majorité de la logique métier de l'application de C++ vers Rust pour améliorer les performances et la maintenabilité.

### Version 0.6.1
* Ajout du support PDF protégé par mot de passe !
* Ajout d'une fonction très basique d'accès à la position précédente/suivante. Si vous appuyez sur entrée sur un lien interne et que cela déplace votre curseur, cette position sera maintenant mémorisée, et peut être navigué avec les flèches alt+gauche/droite.
* Ajout d'une liste d'éléments ! Actuellement, il affiche uniquement un arborescence de tous les en-têtes de votre document ou une liste de liens, mais il y a des plans pour l'étendre à l'avenir.
* Ajout d'une option pour démarrer Paperback en mode maximisé par défaut.
* Corrigé les liens dans certains documents Epub ne fonctionnant pas correctement.
* Corrigé l'analyse des tables des matières Epub contenant des chemins relatifs.
* Corrigé certains documents epub ne montrant pas de titre ou d'auteur.
* Corrigé les titres de certains chapitres epub n'apparaissant pas correctement dans le dialogue de la table des matières.
* Corrigé de ne pas pouvoir utiliser la barre d'espace pour activer les boutons OK/annuler dans le dialogue de la table des matières.
* Amélioration de la gestion des en-têtes dans les documents Word.
* Vous obtiendrez maintenant une rétroaction parlée si la liste des documents récents est vide lorsque vous essayez d'afficher le dialogue.

### Version 0.6.0
* Une nouvelle option pour afficher le menu de navigation sous une forme beaucoup plus compacte a été ajoutée au dialogue des options, cochée par défaut.
* Ajout d'une option pour que la navigation par éléments structurels s'enroule.
* Ajout d'une option au menu Outils pour ouvrir le dossier contenant le document actuellement mis au point.
* Ajout d'un système de mise à jour assez simple, mais très efficace.
* Ajout d'une fonction de minuterie de sommeil basique, accessible avec Ctrl+Shift+S.
* Ajout du support pour l'analyse des livres électroniques FB2 !
* Ajout du support pour l'analyse des présentations OpenDocument !
* Ajout du support pour l'analyse des fichiers Texte OpenDocument !
* Les signets peuvent maintenant être faits pour mettre en signet une ligne entière ou pour marquer seulement du texte spécifié. Si vous n'avez pas de sélection active lors du placement d'un signet, le comportement est comme la pré-0.6, et il marquera la ligne entière. Cependant, si vous sélectionnez du texte, seul ce texte sera inclus dans le signet.
* Les signets peuvent maintenant avoir des notes de texte optionnelles attachées ! Naviguez entre les signets contenant des notes avec N et Shift+N, ou affichez le dialogue des signets avec seulement les signets, seulement les notes ou seulement les non-notes sélectionnés avec des raccourcis spécifiques.
* Les signets dans le dialogue des signets n'auront plus le préfixe "signet x" ennuyeux.
* Les livres Epub contenant du contenu HTML prétendant être XML seront maintenant gérés correctement.
* Corrigé le chargement de grands documents Markdown.
* Corrigé la barre d'espace dans l'arborescence de la table des matières activant le bouton OK.
* Corrigé la gestion des espaces blancs au début des balises pre dans les documents HTML et XHTML.
* Corrigé le contrôle de texte ne regagnant pas le focus parfois lors du retour à la fenêtre de Paperback.
* Corrigé le champ de texte dans le dialogue de pourcentage d'accès ne mettant pas à jour la valeur du curseur.
* Corrigé le rendu des identifiants HTML personnalisés dans les documents Markdown.
* Le HTML à l'intérieur des blocs de code Markdown sera maintenant rendu correctement.
* Si le chargement d'un livre avec un paramètre de ligne de commande prend plus de 5 secondes alors qu'une instance Paperback existante est en cours d'exécution, vous n'obtiendrez plus d'erreur.
* Si vous exécutez Paperback en tant qu'administrateur, la configuration sera maintenant correctement chargée et enregistrée.
* Il est maintenant possible de supprimer un signet directement depuis le dialogue des signets.
* Il est maintenant possible d'importer et d'exporter vos signets et votre position de lecture pour un document particulier. Le fichier généré porte le nom du fichier avec une extension .paperback. Si un tel fichier est trouvé dans le même répertoire qu'un fichier lors de son chargement, il sera automatiquement chargé. Sinon, vous pouvez les importer manuellement à l'aide d'un élément du menu Outils.
* Les liens à l'intérieur des documents sont maintenant entièrement pris en charge ! Utilisez k et shift+k pour avancer et reculer dans les liens, et appuyez sur entrée pour ouvrir/activer un.
* Nombreuses refactorisations internes, rendant l'application plus rapide et le binaire plus petit.
* Le contenu Markdown est maintenant prétraité pour être conforme à CommonMark avant le rendu.
* La navigation par listes et leurs éléments est maintenant entièrement prise en charge ! Utilisez L et Shift+L pour aller par les listes elles-mêmes, et I et Shift+I pour parcourir les éléments de la liste.
* La suppression du pavé numérique fonctionne maintenant pour supprimer les documents de la barre d'onglets en plus de la suppression normale.
* Paperback peut maintenant optionnellement se minimiser sur votre plateau système ! Cette option est désactivée par défaut, mais son activation fera que l'option de minimisation dans le menu système place Paperback dans votre barre d'état système, capable d'être restaurée en cliquant sur l'icône générée.
* Paperback est maintenant entièrement translatable ! La liste des langues qu'il prend en charge est actuellement assez petite, mais elle s'agrandit constamment !
* Paperback a maintenant un site Web officiel, à l'adresse [paperback.dev](https://paperback.dev) !
* Les documents PPTX affichent maintenant une table des matières basique, contenant toutes les diapositives.
* Le chemin complet du document ouvert sera maintenant affiché dans le dialogue d'informations du document.
* Le programme d'installation inclut maintenant une option pour afficher le fichier lisezmoi dans votre navigateur après l'installation.
* La liste des documents récents a été considérablement développée ! Au lieu de simplement vous montrer les 10 derniers documents que vous avez ouverts, elle affiche maintenant un nombre personnalisable, les autres documents que vous avez jamais ouverts étant accessibles via un petit dialogue.
* Diverses petites améliorations aux analyseurs à tous les niveaux, y compris la mise d'une ligne vide entre les diapositives dans les présentations PPTX, la correction de la gestion des sauts de ligne à l'intérieur des paragraphes dans les documents Word, et l'ajout de puces aux éléments de la liste.

### Version 0.5.0
* Ajout du support des documents Microsoft Word !
* Ajout du support des présentations PowerPoint !
* Corrigé certains éléments de menu n'étant pas désactivés sans documents ouverts.
* Corrigé l'orientation du curseur de pourcentage d'accès.
* Corrigé la table des matières dans les livres Epub avec des chemins de fichiers et/ou des identifiants de fragment codés en URL.
* Corrigé les espaces blancs étant supprimés des en-têtes XHTML de façons bizarres.
* Corrigé la gestion des espaces blancs à l'intérieur des balises pre imbriquées dans les documents HTML.
* Les documents HTML et Markdown supportent maintenant la fonction de table des matières ! Lorsque vous chargez un document HTML/Markdown, Paperback construit sa propre table des matières à partir de la structure des en-têtes de votre document, et elle s'affiche dans le dialogue ctrl+t.
* Les documents HTML auront maintenant le titre tel que défini dans la balise de titre, s'il existe. Sinon, ils continueront à utiliser le nom de fichier sans l'extension.
* Passage de UniversalSpeech à l'utilisation d'une région en direct pour signaler la parole. Cela signifie qu'aucune DLL de lecteur d'écran n'est expédiée aux côtés du programme, et plus de lecteurs d'écran seront maintenant pris en charge, tels que le Narrateur Microsoft.
* Changement des bibliothèques zip pour permettre l'ouverture d'un plus large éventail de livres epub.
* Le dialogue vous demandant si vous voulez ouvrir votre document en tant que texte brut a été complètement refait, et permet maintenant d'ouvrir votre document en tant que texte brut, HTML ou Markdown.
* Le dialogue de pourcentage d'accès inclut maintenant un champ de texte vous permettant d'entrer manuellement un pourcentage pour y accéder.
* L'analyseur HTML reconnaîtra maintenant dd, dt et dl comme éléments de liste.
* La table des matières dans les livres Epub sera à nouveau préservée exactement.
* L'espace insécable Unicode est maintenant pris en compte lors de la suppression des lignes vides.
* Vous ne serez plus interrogé sur la façon d'ouvrir un fichier non reconnu chaque fois que vous le chargez, seulement la première fois.

### Version 0.4.1
* Ajout d'une icône du menu Démarrer optionnelle au programme d'installation.
* La table des matières devrait maintenant être plus propre dans certains cas, par exemple si vous avez un élément enfant et parent avec le même texte à la même position, vous ne verrez maintenant que l'élément parent.
* Corrigé la table des matières dans certains documents CHM.
* Corrigé la table des matières dans les livres Epub 3 avec des chemins absolus.
* Les documents CHM devraient maintenant afficher leur titre tel que défini dans le fichier de métadonnées.

### Version 0.4.0
* Ajout du support des fichiers CHM !
* Ajout du support des signets ! Vous pouvez avoir autant de signets dans autant de documents que vous le souhaitez. Vous pouvez y accéder en avant et en arrière avec b et shift+b, en définir un avec control+shift+b, et afficher un dialogue pour y accéder à un signet spécifique avec control+b.
* Ajout d'un programme d'installation aux côtés du fichier zip portable ! Le programme d'installation installera Paperback dans votre répertoire des fichiers de programme et configurera automatiquement les associations de fichiers pour vous.
* Les fichiers texte avec des BOM doivent maintenant être décodés correctement, et le BOM ne s'affichera plus au début du texte non plus.
* Ajout d'informations bien plus importantes à la barre de statut. Il affichera maintenant votre ligne actuelle, votre caractère et votre pourcentage de lecture.
* Les commentaires HTML, ainsi que le contenu des balises script et style, ne seront plus affichés dans la sortie de texte.
* Si vous transmettez un chemin relatif à Paperback sur la ligne de commande, il le résoudra correctement.
* Le mouvement en pourcentage est maintenant géré par son propre dialogue basé sur le curseur, accessible avec control+shift+g.
* Les documents sans titres ou auteurs connus auront maintenant un défaut.
* La logique d'économie de position est maintenant bien plus intelligente et ne devrait écrire sur le disque que si nécessaire.
* Le document sur lequel vous aviez le focus lorsque vous avez fermé Paperback est maintenant mémorisé lors des redémarrages de l'application.
* L'entrée dans les dialogues de numéro de ligne et de numéro de page doit maintenant être assainie plus strictement.
* Corrigé la navigation de la table des matières dans les livres epub 3 avec des chemins relatifs dans leurs manifestes.

### Version 0.3.0
* Corrigé la table des matières dans les livres epub avec des manifestes codés en URL.
* Corrigé la navigation des en-têtes dans les documents HTML contenant des caractères Unicode multi-octets.
* Corrigé l'utilisation élevée du processeur dans les documents avec de longs titres en raison d'une régression dans wxWidgets.
* Corrigé le chargement des fichiers texte UTF-8.
* Corrigé les éléments de table des matières imbriqués dans les livres Epub mettant votre curseur à la mauvaise position.
* Corrigé un crash à la sortie de l'application dans certains cas.
* Ajout d'une case à cocher dans le dialogue des options pour activer ou désactiver le retour à la ligne !
* Il est maintenant possible de faire un don au développement de Paperback, soit par le nouvel élément de don du menu d'aide, soit par le lien du sponsor ce projet en bas de la page principale du référentiel GitHub.
* Les documents Markdown auront maintenant toujours un titre, et Paperback devrait maintenant être capable de charger virtuellement n'importe quel fichier Markdown.
* Les documents PDF auront maintenant toujours un titre, même si les métadonnées manquent.
* Changement des bibliothèques PDF à celle utilisée dans Chromium, ce qui entraîne une analyse PDF bien plus fiable à tous les niveaux.
* Vous ne pouvez maintenant avoir qu'une seule instance de Paperback en cours d'exécution à la fois. Exécuter paperback.exe avec un nom de fichier alors qu'il est déjà en cours d'exécution ouvrira ce document dans l'instance déjà en cours d'exécution.
* Vous pouvez maintenant appuyer sur supprimer sur un document dans le contrôle d'onglet pour le fermer.

### Version 0.2.1
* Ajout du nombre total de pages à l'étiquette de page dans le dialogue d'accès à la page.
* Autoriser la tabulation du contenu du document à votre liste de documents ouverts.
* Corrigé les raccourcis de titre ouvrant parfois les documents récents si vous en aviez suffisamment.
* Paperback supprimera maintenant les traits d'union doux inutiles du résultat textuel.
* Corrigé la navigation des en-têtes vous mettant parfois au mauvais caractère.

### Version 0.2.0
* Ajout du support des documents Markdown !
* Ajout du support des documents PDF, y compris la possibilité de naviguer entre les pages !
* Ajout de touches de raccourci pour naviguer par les en-têtes dans le contenu HTML, y compris les livres epub et les documents Markdown. Ces touches ont été conçues pour fonctionner de manière similaire à un lecteur d'écran.
* Corrigé le chargement des épubs avec des noms de fichiers codés en URL dans leurs manifestes.
* Corrigé le chargement des livres epub 3 avec du XHTML intégré à l'intérieur.
* Un message est maintenant prononcé si le document ne prend pas en charge une table des matières ou des sections, par opposition à la désactivation des éléments du menu.
* Ajout d'un menu de documents récents ! Il stocke actuellement vos 10 derniers documents ouverts, et appuyer sur entrée sur un les ouvrira pour la lecture.
* Réécriture complète du dialogue de recherche, le rendant beaucoup plus simple à utiliser, tout en ajoutant un historique de vos 25 dernières recherches et un support d'expression régulière !
* Les documents ouverts précédemment sont maintenant mémorisés lors des redémarrages de l'application. Ceci est configurable via le nouvel élément des options dans le menu Outils.
* Ajout de shift+f1 pour ouvrir le fichier lisezmoi directement dans Paperback lui-même.

### Version 0.1.0
* Version initiale.
