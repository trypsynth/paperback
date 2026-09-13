<!-- machine-translated from doc/readme.md (source-hash: d583a89d8ac391f5; sections: f48ce952,d4d8160f,a02f4421,a4ffb7f7,91be3b41,a028b7f6,af36028a,71df8e94,e9860ee8,93dd8dd6); please review and edit as needed -->

# Paperback - version 0.9.2

## Introduction

Paperback est un lecteur léger, rapide et accessible de livres électroniques et de documents pour tous, des lecteurs occasionnels aux utilisateurs avancés. Il est conçu pour l'accessibilité aux lecteurs d'écran, la rapidité et une expérience sans fioritures.

## Configuration requise

Paperback fonctionne actuellement sur Windows 10/11 et toutes les versions modernes de macOS ARM. Les applications natives iOS et Android sont en développement actif, avec des versions de test publiques prévues peu après la version 0.9.0 pour le bureau, en amont d'une version unifiée 1.0 couvrant les quatre plates-formes.

## Fonctionnalités

* Complètement autonome, ne nécessitant aucun logiciel à installer sur votre ordinateur pour commencer à lire.
* Incroyablement rapide, même sur du matériel ancien.
* Interface à onglets simple, vous permettant d'ouvrir autant de documents que vous le souhaitez côte à côte.
* Enregistre votre position de lecture exacte dans tous les documents que vous ouvrez.
* Mémorise éventuellement les documents que vous aviez ouverts lorsque vous avez fermé le programme et les restaure au lancement suivant.
* Inclut une fonctionnalité de navigation similaire à celle que l'on trouve dans le mode de navigation web de nombreux lecteurs d'écran pour naviguer rapidement et facilement dans les documents.
* Inclut un dialogue de recherche robuste, avec des fonctionnalités telles que l'historique et la prise en charge des expressions régulières.
* Peut être exécuté entièrement de manière portable ou installé avec les associations de fichiers configurées automatiquement.
* Prend en charge un grand nombre de formats de fichiers courants.

## Compatibilité avec les lecteurs d'écran

Paperback fonctionne bien avec tous les lecteurs d'écran majeurs. Il existe cependant un problème connu pour les utilisateurs de JAWS.

### JAWS et afficheurs braille

Si vous utilisez JAWS avec un afficheur braille, vous constaterez peut-être que les paragraphes longs sont tronqués lors du balayage vers l'avant avec les touches de navigation de votre afficheur. La commande de lecture du paragraphe actuel est également affectée. Il s'agit d'un bogue dans le traitement par JAWS du contrôle de texte RICHEDIT50W, et non quelque chose dans Paperback lui-même, et celui-ci a pris un certain temps à faire surface étant donné l'enthousiasme de Vispero à répondre aux problèmes des logiciels open source.

La solution de contournement, finalement révélée par le groupe de discussion JAWS après des mois d'attente, consiste à modifier `paperback.jcf` et à définir « Braille Presentation and Panning » sur « Always use DOM if available ». Vous souhaiterez également activer « Pan Text by Paragraph », sinon votre afficheur restera sur le paragraphe actif plutôt que d'avancer. Avec les deux paramètres en place, le balayage devrait fonctionner correctement.

## Formats de fichiers actuellement pris en charge

Paperback prend en charge les formats et extensions suivants :

* Archives de bandes dessinées (`.cbz`, `.cbr`)
* Fichiers d'aide CHM (`.chm`)
* Livres DAISY (`.opf`, `.zip`)
* Livres EPUB (`.epub`)
* Livres électroniques FB2 (`.fb2`)
* Documents HTML (`.htm`, `.html`, `.xhtml`)
* Pages de manuel, tant `man` que BSD `mdoc` (`.1` à `.9`, `.man`, `.roff`, et les formes compressées de chacun)
* Documents Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Documents Microsoft Word (`.docx`, `.docm`, `.doc`)
* Livres audio M4B (`.m4b`)
* Livres MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
* Présentations OpenDocument (`.odp`, `.fodp`)
* Fichiers texte OpenDocument (`.odt`, `.fodt`)
* Documents PDF (`.pdf`)
* Présentations PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Documents RTF (`.rtf`)
* Fichiers WinHelp (`.hlp`)
* Fichiers texte brut et journaux (`.txt`, `.log`)

## Raccourcis clavier

Paperback est conçu pour une utilisation en priorité au clavier. Voici les raccourcis actuels.

Les raccourcis ci-dessous sont pour Windows. Lorsque macOS diffère, l'équivalent est noté entre parenthèses — principalement parce que Ctrl+G, Ctrl+W, et Alt+Left/Right sont déjà utilisés par d'autres conventions système ou d'application sur cette plateforme.

### Menu Fichier

* `Ctrl+O` : Ouvrir un document.
* `Ctrl+F4` (macOS : `Cmd+W`) : Fermer le document actuel.
* `Ctrl+Shift+F4` (macOS : `Cmd+Shift+W`) : Fermer tous les documents ouverts.
* `Ctrl+Shift+T` : Rouvrir le dernier document fermé.
* `Ctrl+R` : Afficher la boîte de dialogue « Tous les documents » (à partir des Documents récents).
* `Ctrl+Q` : Quitter (Windows uniquement ; sur macOS, cela se trouve dans le menu de l'application).

### Menu Aller à

* `Ctrl+F` : Afficher la boîte de dialogue Rechercher.
* `F3` (macOS : `Cmd+G`) : Rechercher suivant.
* `Shift+F3` (macOS : `Cmd+Shift+G`) : Rechercher précédent.
* `Ctrl+G` (macOS : `Cmd+L`) : Aller à une ligne.
* `Ctrl+Shift+G` (macOS : `Cmd+Shift+L`) : Aller à un pourcentage.
* `Ctrl+P` : Aller à une page (si supporté par le document actuel).
* `=` : Annoncer votre pourcentage de lecture actuel et votre page, par ex. « 15%, page 30 ». La page est omise pour les documents qui n'ont pas de numéros de page.
* `Alt+Left` (macOS : `Cmd+[`) : Retour dans l'historique de navigation.
* `Alt+Right` (macOS : `Cmd+]`) : Avancer dans l'historique de navigation.
* `[` : Section précédente.
* `]` : Section suivante.
* `Shift+H` : Titre précédent.
* `H` : Titre suivant.
* `Shift+1` à `Shift+6` : Titre précédent de niveau 1-6.
* `1` à `6` : Titre suivant de niveau 1-6.
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
* `Ctrl+Shift+W` (macOS : `RawCtrl+Shift+W`, c'est-à-dire la touche Contrôle physique plutôt que Cmd) : Afficher le texte de la note à la position actuelle.
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
* `Shift+,` : Aller au début du conteneur actuel (liste ou tableau).
* `,` : Aller au-delà de la fin du conteneur actuel (liste ou tableau).

### Menu Outils

* `Ctrl+W` (macOS : `RawCtrl+W`, c'est-à-dire la touche Contrôle physique plutôt que Cmd) : Afficher le nombre de mots pour le document actuel.
* `Ctrl+I` : Afficher les informations du document.
* `Ctrl+T` : Afficher la table des matières.
* `F7` : Afficher la liste des éléments.
* `Ctrl+Shift+C` : Ouvrir le dossier contenant.
* `Ctrl+Shift+V` : Ouvrir le contenu actuel dans Web View.
* `Ctrl+U` : Afficher la source du document dans un nouvel onglet.
* `Ctrl+Shift+E` : Exporter les données du document (`.paperback`).
* `Ctrl+Shift+I` : Importer les données du document (`.paperback`).
* `Ctrl+E` : Exporter le document actuel en texte brut.
* `Ctrl+Shift+B` : Basculer le signet à la sélection/curseur actuel.
* `Ctrl+Shift+N` : Ajouter ou modifier la note du signet à la sélection/curseur actuel.
* `Ctrl+Alt+W` : Basculer le retour à la ligne.
* `Ctrl+Space` : Lire/Pause la narration audio.
* `'` : Avancer la narration audio.
* `;` : Reculer la narration audio.
* `Ctrl+'` : Augmenter la quantité de recherche audio.
* `Ctrl+;` : Diminuer la quantité de recherche audio.
* `F11` (macOS : `RawCtrl+Ctrl+F`, c'est-à-dire Contrôle+Commande+F) : Basculer le plein écran.
* `Ctrl+,` : Ouvrir les options (macOS : Préférences, sous le menu de l'application).
* `Ctrl+Shift+S` : Basculer la minuterie de veille.

### Menu Aide

* `Ctrl+F1` : Afficher la boîte de dialogue À propos.
* `F1` : Afficher l'aide dans votre navigateur par défaut.
* `Shift+F1` : Afficher l'aide dans Paperback.
* `Ctrl+Shift+U` : Vérifier les mises à jour.
* `Ctrl+D` : Ouvrir la page de donation dans votre navigateur par défaut.

### Touches supplémentaires pour la vue document

* `Delete` / `Numpad Delete` sur le contrôle d'onglet : Fermer l'onglet du document sélectionné.
* `Enter` ou `Space` dans le texte du document : Activer le lien au curseur, ou ouvrir une vue de tableau lorsque vous êtes sur un marqueur de tableau.
* `Shift+F10` ou la touche Menu/Application dans le texte du document : Ouvrir le menu contextuel.

## Langues supportées

Paperback est traduit dans de nombreuses langues différentes, et d'autres s'ajoutent tout le temps. Une liste complète suit ci-dessous.

Pour apprendre à contribuer, veuillez lire notre [Guide de traduction](translating.md).

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

### Donations
Les personnes suivantes ont fait des dons de diverses tailles au développement de Paperback. Si vous faites un don, votre nom ne sera pas automatiquement ajouté ici, j'ajoute seulement les personnes qui souhaitent que leur donation soit publique.

Remarque : Je considère un parrainage GitHub public comme des motifs d'inclusion automatique dans cette liste.

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
* Les audiolivres ne font plus lire à votre lecteur d'écran une suite d'espaces lorsque vous focalisez le champ de texte.
* Les audiolivres nomment désormais le fichier au fur et à mesure que vous les parcourez par section.
* Les audiolivres rapportent désormais leur vraie longueur, au lieu de prétendre que chaque fichier dure 24 heures.
* Fermer la Web View avec Échap ne lève plus une alerte de débogage après avoir suivi un lien à l'intérieur.
* Copier après Sélectionner tout vous donne désormais le document complet, au lieu de seulement la partie actuellement chargée.
* Find va maintenant droit à la ligne trouvée, au lieu de vous faire traverser le lecteur d'écran relisant la fenêtre tandis que le focus revient au livre.
* Correction des EPUB contenant un bloc ZIP64 isolé qui refusaient de s'ouvrir avec « Invalid local file header ».
* Correction des longs documents revenant à leur début pendant qu'un lecteur d'écran les lisait continuellement.
* Les liens dans la WebView vous mènent désormais à la section vers laquelle ils pointent, au lieu d'échouer avec « File not found ».
* Le raccourci `=` annonce maintenant la page ainsi que le pourcentage, par exemple « 15%, page 30 », et reste comme avant pour les documents sans numéro de page.
* L'annonce automatique « Document reloaded » ne coupe plus votre lecteur d'écran en pleine phrase, attendant à la place qu'il finisse ce qu'il disait.
* L'onglet General de la boîte de dialogue Settings parcourt maintenant ses options dans l'ordre où elles apparaissent à l'écran, le canal de mise à jour venant directement après l'option de vérification des mises à jour.
* La mise à jour apporte désormais la fenêtre relancée à l'avant, au lieu de la laisser derrière toutes les autres fenêtres dans `Alt+Tab`.
* Windows affichera désormais toujours « Paperback » dans le menu Ouvrir avec, au lieu du tagline complet du programme.
* Word Count et Document Info affichent maintenant combien de fichiers un audiolivre contient, et combien de temps il dure au total.

### Version 0.9.1
* Les sons de signet et de note jouent maintenant sur macOS.
* Les livres DAISY jouent maintenant leur audio sur macOS, au lieu de s'ouvrir et de suivre leur chronologie en silence.
* Correction des guillemets courbés, tirets demi-cadratin et caractères similaires disparaissant des documents RTF, les mots environnants fusionnant au passage.
* Correction des images RTF fuyant leurs données brutes dans le document sous forme de texte brouillé.
* Correction du sous-menu Documents récents conservant des entrées obsolètes jusqu'à ce que quelque chose d'autre le reconstruise.
* Les accélérateurs de clavier sont de retour dans chaque traduction, donc les menus du russe ont à nouveau accès au clavier.
* Les grands documents CHM s'ouvrent maintenant jusqu'à sept fois plus rapidement.
* Les documents ouverts sont maintenant enregistrés auprès de Windows, donc ils apparaissent dans la liste de saut de la barre des tâches et dans la liste récente du menu Démarrer.
* Options a été renommé en Settings, correspondant aux applications mobiles et, sur macOS, à la convention de la plateforme.
* Paperback se souvient maintenant de la position, de la taille et de l'état maximisé de sa fenêtre entre les exécutions.
* Les formes plurielles sont maintenant traduites, donc les messages qui comptent les choses se lisent correctement dans les langues qui ont besoin de plus d'une forme.
* Sélectionner le ncc.html d'un livre DAISY ouvre maintenant l'audiolivre complet au lieu de seulement son texte.
* Les noms d'actions de la boîte de dialogue Customize Keyboard Shortcuts peuvent maintenant être traduits.
* Le titre du document vient maintenant en premier dans la barre de titre, donc les livres ouverts peuvent être distingués dans la barre des tâches et `Alt+Tab`.
* La boîte de dialogue de mise à jour est maintenant traduite.

### Version 0.9.0

#### Ajouts

##### Général
* Un outil CLI, appelé pb, pour convertir rapidement n'importe lequel des formats supportés par Paperback en HTML, Markdown ou texte brut.
* Une option pour recharger les documents qui ont été modifiés par d'autres programmes sur le disque.
* Une option View Source pour ouvrir la source d'un document dans un nouvel onglet, utile par exemple pour éditer du Markdown.
* Le texte du document est maintenant paginé, ce qui signifie que vous pouvez charger des livres avec des dizaines de millions de mots en seulement quelques secondes. Veuillez signaler toute bizarrerie trouvée avec ceci.

##### Support de plateforme
* Support ARM64 Windows !
* Support natif macOS !
* Un bouton pour basculer le plein écran.

##### Boîte de dialogue Tous les documents
* Un bouton localiser pour localiser les livres manquants qui viennent de changer de chemin.
* Un filtre d'état et une barre d'état, vous permettant de filtrer par statut de document et de voir combien de documents sont affichés et sélectionnés.
* Le raccourci `Ctrl+Shift+A` pour désélectionner tous les documents.

##### Options et lisibilité
* Un onglet de lisibilité, avec les options suivantes :
    * Retour à la ligne automatique (déplacé de la section générale) ;
    * Rendre les tableaux en ligne (nouveau dans cette version, voir ci-dessous) ;
    * Police ;
    * Couleur de fond ;
    * Interligne ;
    * Espacement des paragraphes ;
    * Espacement des lettres ;
    * Alignement du texte.
* Un élément de menu Retour à la ligne automatique et touches de raccourci ultérieures.
* Un bouton pour déterminer comment vous voulez que les tableaux soient affichés, et unification de la façon dont les tableaux sont affichés dans les documents.

##### Navigation
* Support de la navigation par conteneur.
* Une option pour déplacer automatiquement le curseur au début de la ligne lors de la navigation entre les lignes, similaire au mode de navigation des lecteurs d'écran.
* Le raccourci clavier égal pour annoncer votre pourcentage actuel dans un document.

##### Signets
* Signets temporaires : vous pouvez en avoir un par document, et ils persisten. Utilisez la barre oblique pour en définir un et la barre oblique inverse pour y accéder.

##### Nombre de mots
* Temps de lecture estimé dans la boîte de dialogue du nombre de mots, ainsi que la possibilité de définir votre vitesse de lecture pour rendre cette métrique réellement utile.
* Si une sélection est active lorsque vous ouvrez la boîte de dialogue du nombre de mots, le nombre de mots sélectionnés s'affichera maintenant.

##### Raccourcis clavier
* La possibilité de personnaliser chaque raccourci clavier de l'application via une simple boîte de dialogue.
* Un raccourci clavier configurable pour restaurer Paperback depuis le plateau système.

##### Langues
* Néerlandais, finnois et polonais.

##### Export
* Extension de l'élément de menu d'export pour permettre l'export en HTML et Markdown, en plus du texte brut.

##### Mises à jour
* Un bouton Annuler pour la boîte de dialogue de mise à jour en cours.
* La mise à jour valide maintenant que le fichier téléchargé n'a pas été altéré.

##### Web View
* La webview est maintenant ouverte à votre position de lecture actuelle.

##### Livres DAISY
* Support des livres DAISY 2.0.
* Support de la lecture audio DAISY 2.02.

##### Audiolivres
* La capacité de lire des audiolivres, supportant actuellement à la fois l'audio DAISY (y compris audio DAISY + texte) et les archives de fichiers audio.
* Raccourcis clavier et éléments de menu pour lire/mettre en pause la narration, avancer et reculer, et ajuster la quantité de recherche.
* Options pour synchroniser le curseur de lecture à la lecture audio, définir la quantité de recherche audio et choisir si la recherche au-delà de la fin d'un chapitre continue dans le suivant.

##### Documents CHM
* Support des listes, éléments de liste, figures et images.

##### PowerPoint
* Les documents PowerPoint supportent maintenant les tableaux.

#### Corrections

##### Général
* Les documents codés dans les encodages CJK hérités, tels que GBK, Big5 et Shift_JIS, s'affichent maintenant correctement au lieu de s'afficher comme un tas de mojibake.
* « Rouvrir le dernier fermé » tentant de rouvrir le fichier readme fourni.
* Votre onglet sélectionné ne recevant pas correctement le focus après le redémarrage de Paperback.
* Gestion par Paperback des fichiers sur les lecteurs réseau Windows : appuyer sur show file in folder focalise maintenant correctement le fichier sur le stockage réseau, et les chemins ne contiennent plus de caractères étranges.
* Les fichiers .paperback ne seront plus forcément chargés lors de la restauration des documents ; à la place, vous serez invité à confirmer lorsqu'un est trouvé.
* Open containing folder focalise maintenant le fichier donné dans l'explorateur.
* L'ouverture du fichier readme respecte maintenant votre langue sélectionnée.
* L'interface utilisateur de Paperback s'adapte maintenant correctement aux écrans haute résolution.
* Le menu se met maintenant à jour correctement, et le focus passe au contrôle de texte, lors de l'ouverture de l'aide dans Paperback.
* Passage à une méthode beaucoup plus sécurisée de l'IPC sur Windows.
* Le titre du document actif sera maintenant lu lors du changement entre les onglets.
* Réduction de l'utilisation de la mémoire sur les grands documents en réduisant de moitié la taille des tableaux d'index par caractère interne.

##### Boîte de dialogue Tous les documents
* Échap ne fermant pas les boîtes de dialogue Document Info et All Documents.
* La barre de titre ne se mettant pas à jour après la fermeture d'un document à partir de la boîte de dialogue de tous les documents.
* Readme.html ne sera plus ajouté à votre liste de tous les documents lorsqu'il est ouvert via `Shift+F1`.
* La suppression de documents de la boîte de dialogue recents fermera maintenant aussi leur onglet actif.
* Votre filtre de recherche est maintenant conservé après la suppression d'un document.

##### Navigation
* La navigation par page annonçant un texte de ligne incorrect dans certaines situations.
* Go to Line, Go to Page et Go to Percent plaçant votre curseur à la mauvaise position dans les grands documents.
* Find et Find Next ne respectant pas la fenêtre du document chargé dans les grands documents.

##### Signets
* Les sons de signet/note devraient maintenant se lire correctement exclusivement lorsque vous naviguez sur un mot en contenant un.

##### Lisibilité
* Appliquer le retour à la ligne automatique vous envoyant au début de votre document.

##### Web View
* La boîte de dialogue webview n'étant pas redimensionnable et s'affichant à une très petite taille initiale.
* Les images devraient maintenant s'afficher correctement dans la webview intégrée.

##### Mises à jour
* La mise à jour affiche maintenant correctement le contenu des balises de code markdown dans les notes de version.

##### Livres DAISY
* Les livres DAISY affichant des informations incorrectes dans la barre d'état.
* Chargement de livres DAISY avec des déclarations d'encodage bogues.

##### Documents RTF
* Analyse des documents RTF contenant des caractères non-latins.
* Les groupes RTF `\pict` afin que les données d'image intégrées ne s'échappent plus dans le texte du document.

##### Livres Mobi/AZW3
* Les ancres filepos dans les livres Mobi divisant les balises HTML et mettant les ordures dans le texte du livre.
* Liens dans les livres Mobi hérités.
* Analyse AZW3 considérablement améliorée.

##### Documents Word
* Les documents Word avec des noms de style spécifiques aux paramètres régionaux ne rendant pas correctement leurs en-têtes.

##### Documents HTML/XHTML
* Les éléments dl, dt et dd ne produisant pas de sauts de ligne dans les documents XHTML.

##### Documents PDF
* Paperback se rabat maintenant sur l'extraction de texte brut pour les PDF incorrectement balisés.
* Les documents PDF contenant des caractères de contrôle dans leurs titres et/ou signets ne feront plus planter Paperback à l'ouverture.

### Version 0.8.5
* Ajout du support des pages aux livres epub.
* Ajout du support des documents Microsoft Office chiffrés. Actuellement, Word hérité, Word moderne et Powerpoint moderne sont supportés, avec Powerpoint hérité prévu pour l'avenir.
* Ajout du support des documents Microsoft Word hérités !
* Ajout du support des présentations Powerpoint héritées !
* Ajout du support des livres mobi et AZW3 !
* Ajout du support des fichiers PDF balisés !
* Ajout du raccourci `Ctrl+Q` pour quitter l'application.
* Ajout du support des livres compressés de Bookshare (à la fois DAISY et Word) !
* Le texte alternatif pour les images intégrées devrait maintenant être correctement affiché.
* Les documents CHM supportent maintenant correctement la navigation des liens internes.
* Correction de go to page étant décalé de 1.
* Correction de la touche Échap ne fonctionnant pas pour fermer la boîte de dialogue ouvrir sous.
* Correction du menu contextuel du lecteur ne s'affichant pas au clic droit ou à la touche Applications.
* Correction du mauvais document étant parfois focalisé lors de l'ouverture de documents à partir de la ligne de commande.
* Les PDF images uniquement sont à nouveau détectés et vous alertent de leur existence.
* Il est maintenant possible de naviguer dans les images et figures avec g/`Shift+G` et f/`Shift+F`, respectivement.
* Paperback respecte maintenant votre paramètre de mode sombre de l'application.
* Suppression du support DAISY XML, car ce n'est plus nécessaire.
* Passage à la première lettre de navigation Win32 native dans l'arborescence de la table des matières.
* La boîte de dialogue d'erreur de chargement affiche maintenant des messages d'erreur plus détaillés.
* La webview s'ouvre maintenant beaucoup plus rapidement et en douceur.

### Version 0.8.2
* Ajout du support des pages aux documents RTF !
* Correction d'un bogue où l'ouverture de la webview dans les epub contenant des liens externes les activerait automatiquement.
* Correction d'un bogue où l'analyseur RTF ne mettrait pas d'espace entre les mots dans les rares cas.
* Correction des paragraphes étant divisés en plusieurs lignes courtes dans certains documents PDF.
* Les documents PDF ont maintenant un support basique de navigation par lien et en-tête !
* Les onglets RTF et les sauts de ligne sont maintenant rendus exactement comme ils apparaissent dans le document.
* Passage à la bibliothèque pdfium éprouvée pour l'analyse des PDF, rendant l'analyse PDF beaucoup plus fiable à nouveau.

### Version 0.8.1
* Ajout de `Ctrl+Shift+T` pour rouvrir le dernier document fermé.
* La boîte de dialogue All Documents supporte maintenant la sélection de plusieurs documents à ouvrir à la fois.
* Correction de quelques bogues avec l'analyseur RTF.
* Correction des chemins de fichiers contenant des caractères non-ASCII (tels que le serbe š, č, ć, ž) devenant corrompus lors de l'ouverture d'un fichier via une deuxième instance de Paperback.
* Correction du texte PDF étant lu dans le mauvais ordre, et un espacement incorrect autour des mots en majuscules.
* Correction du chargement lent des documents lors de l'ouverture de gros fichiers.
* Correction de la localisation des boutons Oui/Non dans les boîtes de dialogue de confirmation.

### Version 0.8.0
* Ajout des traductions japonaise, chinoise simplifiée et vietnamienne !
* Ajout d'une mise à jour automatique qui remplacera maintenant votre version actuellement installée de Paperback au lieu de simplement télécharger la nouvelle version !
* Ajout d'une rétroaction sonore optionnelle pour atteindre un signet ou une note, merci Andre Louis pour les sons !
* Ajout du support des documents RTF !
* Ajout du support des documents DAISY XML.
* Ajout du support des fichiers Flat Open Document Text !
* Ajout du support des présentations Flat Open Document !
* Ajout du support des séparateurs avec s et `Shift+S`.
* Tout mouvement supérieur à 300 caractères ajoutera maintenant automatiquement à votre historique de navigation.
* Correction de la restauration de la fenêtre de Paperback depuis le plateau système.
* Correction des documents Markdown affichant du texte brut au lieu du HTML rendu dans la Web View.
* Correction des tableaux ne se rendant pas correctement dans les fichiers Markdown.
* Les PDF images uniquement vous avertissent maintenant de leur existence lorsque vous tentez d'en charger un.
* Incorporer correctement les informations de version dans l'exécutable Paperback.
* Division de la boîte de dialogue des options en onglets pour faciliter l'utilisation et la navigation.
* Passage à Hayro pour l'analyse des PDF, conduisant à plus de fiabilité, de vitesse et moins de DLL.
* Réécriture de l'application entière en Rust. La nouvelle base de code est plus sûre, charge les documents plus rapidement et est plus facile à maintenir et à étendre.
* Le menu contextuel du contrôle de texte inclura maintenant des actions spécifiques au lecteur au lieu d'éléments génériques tels que couper et coller.

### Version 0.7.0
* Ajout du support des tableaux pour les documents basés sur HTML et XHTML ! Naviguez entre les tableaux en utilisant T et `Shift+T`, et appuyez sur Entrée pour en afficher un dans une webview.
* Ajout d'une fonction de rendu web basique ! Appuyez sur `Ctrl+Shift+V` pour ouvrir la section actuelle de votre document dans un moteur de rendu web, utile pour les contenus comme le formatage complexe ou les exemples de code.
* Ajout d'une traduction russe, merci Ruslan Gulmagomedov !
* Ajout d'un bouton Clear All à la boîte de dialogue All Documents.
* Le vérificateur de mise à jour affiche maintenant les notes de version lorsqu'une nouvelle version est disponible.
* Correction de la restauration de la fenêtre depuis le plateau système.
* Correction de la traduction des boutons Oui/Non dans les boîtes de dialogue de confirmation.
* Correction du chargement des configs lors de l'exécution en tant qu'administrateur.
* Correction de la gestion des commentaires dans les documents XML et HTML.
* Correction de l'analyse de la table des matières dans les livres Epub 2.
* Correction de la navigation vers l'élément suivant avec la même lettre dans la table des matières.
* Correction de la boîte de dialogue find ne se cachant pas correctement lors de l'utilisation des boutons suivant/précédent.
* Correction de la table des matières epub vous envoyant occasionnellement au mauvais élément.
* Correction de diverses problèmes de gestion de l'espace blanc dans les balises XML, HTML et pre.
* Correction de l'erreur off-by-one dans la navigation des liens.
* Correction de certains livres ayant des espaces blancs à la fin de leurs lignes.
* Correction de diverses problèmes d'analyseur.
* Les éléments de menu liés aux signets ainsi que la liste des éléments sont maintenant correctement désactivés lorsqu'aucun document n'est ouvert.
* Amélioration de la gestion des listes dans divers formats de documents.
* Amélioration du flux de travail de traduction pour les contributeurs.
* De nombreux refactorisations internes, déplaçant la majorité de la logique métier de l'application de C++ vers Rust pour améliorer les performances et la maintenabilité.

### Version 0.6.1
* Ajout du support des PDF protégés par mot de passe !
* Ajout d'une fonction très basique go to previous/next position. Si vous appuyez sur Entrée sur un lien interne et que cela déplace votre curseur, cette position sera maintenant mémorisée, et peut être naviguée avec les flèches gauche/droite `Alt+Left`/`Alt+Right`.
* Ajout d'une liste d'éléments ! Actuellement, elle affiche seulement une arborescence de tous les en-têtes de votre document ou une liste de liens, mais il y a des plans pour l'étendre à l'avenir.
* Ajout d'une option pour démarrer Paperback en mode maximisé par défaut.
* Correction des liens dans certains documents Epub ne fonctionnant pas correctement.
* Correction de l'analyse des tables des matières Epub contenant des chemins relatifs.
* Correction de certains documents epub n'affichant pas de titre ou d'auteur.
* Correction des titres de certains chapitres epub ne s'affichant pas correctement dans la boîte de dialogue de la table des matières.
* Correction du fait que vous ne pouviez pas utiliser la barre d'espace pour activer les boutons OK/annuler dans la boîte de dialogue de la table des matières.
* Amélioration de la gestion des en-têtes dans les documents Word.
* Vous recevrez maintenant une rétroaction parlée si la liste des documents récents est vide lorsque vous essayez d'afficher la boîte de dialogue.

### Version 0.6.0
* Une nouvelle option pour afficher le menu go sous une forme beaucoup plus compacte a été ajoutée à la boîte de dialogue des options, cochée par défaut.
* Ajout d'une option pour que la navigation par éléments structurels s'enroule.
* Ajout d'une option au menu tools pour ouvrir le dossier contenant du document actuellement focalisé.
* Ajout d'un système de mise à jour assez simple, mais très efficace.
* Ajout d'une fonction de minuteur de sommeil basique, accessible avec `Ctrl+Shift+S`.
* Ajout du support de l'analyse des livres FB2 !
* Ajout du support de l'analyse des présentations OpenDocument !
* Ajout du support de l'analyse des fichiers Texte OpenDocument !
* Les signets peuvent maintenant être définis pour marquer une ligne entière ou pour ne marquer que du texte spécifié. Si vous n'avez pas de sélection active lors du placement d'un signet, le comportement est comme avant 0.6, et il marquera la ligne entière. Cependant, si vous sélectionnez du texte, seul ce texte sera inclus dans le signet.
* Les signets peuvent maintenant avoir des notes de texte optionnelles attachées !
* Naviguez entre les signets contenant des notes avec N et `Shift+N`, ou ouvrez la boîte de dialogue des signets avec tous les signets, seulement les notes ou seulement les non-notes sélectionnées avec des touches d'accès spécifiques.
* Les signets dans la boîte de dialogue des signets n'auront plus le préfixe ennuyeux « bookmark x ».
* Les livres Epub contenant du contenu HTML prétendant être XML seront maintenant gérés correctement.
* Correction du chargement de grands documents Markdown.
* Correction de l'appui sur l'espace dans l'arborescence de la vue de la table des matières activant le bouton OK.
* Correction de la gestion de l'espace blanc au début des balises pre dans les documents HTML et XHTML.
* Correction du contrôle de texte ne récupérant pas correctement le focus parfois lors du retour à la fenêtre de Paperback.
* Correction du champ de texte dans la boîte de dialogue go to percent ne mettant pas à jour la valeur du curseur.
* Correction du rendu des ID HTML personnalisés dans les documents Markdown.
* Le HTML à l'intérieur des blocs de code Markdown s'affiche maintenant correctement.
* Si vous chargez un livre avec un paramètre de ligne de commande tandis qu'une instance Paperback existante est en cours d'exécution, vous ne recevrez plus d'erreur si le chargement de votre document prend plus de 5 secondes.
* Si vous exécutez Paperback en tant qu'administrateur, la configuration sera maintenant correctement chargée et sauvegardée.
* Il est maintenant possible de supprimer un signet directement à partir de la boîte de dialogue des signets.
* Il est maintenant possible d'importer et d'exporter vos signets et votre position de lecture pour un document particulier. Le fichier généré est nommé d'après le fichier avec une extension .paperback. Si un tel fichier est trouvé dans le même répertoire qu'un fichier lors de son chargement, il sera automatiquement chargé. Sinon, vous pouvez les importer manuellement en utilisant un élément du menu tools.
* Les liens à l'intérieur des documents sont maintenant entièrement supportés ! Utilisez k et `Shift+K` pour vous déplacer en avant et en arrière à travers eux, et appuyez sur Entrée pour en ouvrir/activer un.
* De nombreuses refactorisations internes, rendant l'application plus rapide et le binaire plus petit.
* Le contenu Markdown est maintenant prétraité pour être conforme à CommonMark avant le rendu.
* La navigation par listes et leurs éléments est maintenant entièrement supportée ! Utilisez L et `Shift+L` pour aller par les listes elles-mêmes, et I et `Shift+I` pour parcourir les éléments de liste.
* La touche Supprimer du clavier numérique fonctionne maintenant pour supprimer les documents de la barre des onglets en plus de la touche Supprimer normale.
* Paperback peut maintenant éventuellement se minimiser dans votre plateau système ! Cette option est désactivée par défaut, mais l'activation la fera mettre Paperback dans votre plateau, pouvant être restaurée en cliquant sur l'icône générée.
* Paperback est maintenant entièrement traduisible ! La liste des langues qu'il supporte est actuellement assez petite, mais elle grandit constamment !
* Paperback a maintenant un site web officiel, à [paperback.dev](https://paperback.dev) !
* Les documents PPTX affichent maintenant une table des matières basique, contenant toutes les diapositives.
* Le chemin complet du document ouvert s'affichera maintenant dans la boîte de dialogue document info.
* L'installateur inclut maintenant une option pour afficher le fichier readme dans votre navigateur après l'installation.
* La liste des documents récents a été considérablement agrandie ! Au lieu de simplement afficher les 10 derniers documents que vous avez ouverts, elle affichera un nombre personnalisable, les autres documents que vous avez jamais ouverts étant accessibles via une petite boîte de dialogue.
* Diverses petites améliorations aux analyseurs dans tous les domaines, y compris la mise d'une ligne vierge entre les diapositives dans les présentations PPTX, la correction de la gestion des nouvelles lignes à l'intérieur des paragraphes dans les documents word et l'ajout de puces aux éléments de liste.

### Version 0.5.0
* Ajout du support des documents Microsoft Word !
* Ajout du support des présentations PowerPoint !
* Correction de certains éléments de menu n'étant pas désactivés sans documents ouverts.
* Correction de l'orientation du curseur go to percent.
* Correction de la table des matières dans les livres Epub avec chemins de fichiers encodés en URL et/ou ID de fragments.
* Correction de l'espace blanc étant supprimé des en-têtes XHTML de façons bizarres.
* Correction de la gestion de l'espace blanc à l'intérieur des balises pre imbriquées dans les documents HTML.
* Les documents HTML et Markdown supportent maintenant la fonction de table des matières ! Lorsque vous chargez un document HTML/Markdown, Paperback construit sa propre table des matières à partir de la structure des en-têtes de votre document, et il vous l'affiche dans la boîte de dialogue `Ctrl+T`.
* Les documents HTML auront maintenant le titre tel que défini dans la balise titre, s'il existe. Sinon, ils continueront à utiliser le nom de fichier sans l'extension.
* Passage de UniversalSpeech à l'utilisation d'une région active pour rapporter la parole. Cela signifie qu'aucune DLL de lecteur d'écran n'est expédiée avec le programme, et plus de lecteurs d'écran seront maintenant supportés, comme Microsoft Narrator.
* Passage des bibliothèques zip pour permettre l'ouverture d'un large éventail de livres epub.
* La boîte de dialogue vous demandant si vous voulez ouvrir votre document en tant que texte brut a été complètement refaite, et elle vous permet maintenant d'ouvrir votre document en tant que texte brut, HTML ou Markdown.
* La boîte de dialogue go to percent inclut maintenant un champ de texte vous permettant d'entrer manuellement un pourcentage pour y accéder.
* L'analyseur HTML reconnaît maintenant dd, dt et dl comme des éléments de liste.
* La table des matières dans les livres Epub sera à nouveau préservée exactement.
* L'espace insécable unicode est maintenant considéré lors de la suppression des lignes vierges.
* Vous ne serez plus demandé comment vous voulez ouvrir un fichier non reconnu à chaque fois que vous le chargez, seulement la première fois.

### Version 0.4.1
* Ajout d'une icône de menu Démarrer optionnelle à l'installateur.
* La table des matières devrait maintenant être plus propre dans quelques cas, par exemple si vous avez un élément enfant et parent avec le même texte à la même position, vous ne verrez maintenant que l'élément parent.
* Correction de la table des matières dans certains documents CHM.
* Correction de la table des matières dans les livres Epub 3 avec des chemins absolus.
* Les documents CHM devraient maintenant afficher leur titre tel que défini dans le fichier de métadonnées.

### Version 0.4.0
* Ajout du support des fichiers CHM !
* Ajout du support des signets ! Vous pouvez avoir autant de signets que vous le souhaitez dans autant de documents que vous le souhaitez. Vous pouvez naviguer en avant et en arrière à travers eux avec b et `Shift+B`, en définir un avec `Ctrl+Shift+B`, et ouvrir une boîte de dialogue pour accéder à un signet spécifique avec `Ctrl+B`.
* Ajout d'un installateur à côté du fichier zip portable ! L'installateur installera Paperback dans votre répertoire Program Files et configurera automatiquement les associations de fichiers pour vous.
* Les fichiers texte avec BOM devraient maintenant être décodés correctement, et le BOM ne s'affichera plus au début du texte non plus.
* Ajout de bien plus d'informations à la barre d'état. Elle affichera maintenant votre ligne actuelle, caractère et pourcentage de lecture.
* Les commentaires HTML, ainsi que le contenu des balises script et style, ne s'afficheront plus dans la sortie texte.
* Si vous passez un chemin relatif à Paperback sur la ligne de commande, il le résoudra maintenant correctement.
* Le mouvement en pourcentage est maintenant géré par sa propre boîte de dialogue basée sur un curseur, accessible avec `Ctrl+Shift+G`.
* Les documents sans titres ou auteurs connus auront maintenant un défaut.
* La logique de sauvegarde de position est maintenant beaucoup plus intelligente et devrait seulement écrire sur le disque en cas d'absolue nécessité.
* Le document auquel vous aviez accès lorsque vous avez fermé Paperback est maintenant mémorisé lors de redémarrages d'applications.
* L'entrée dans les boîtes de dialogue go to line et go to page devrait maintenant être plus strictement désinfectée.
* Correction de la navigation de la table des matières dans les livres epub 3 avec des chemins relatifs dans leurs manifestes.

### Version 0.3.0
* Correction de la table des matières dans les livres epub avec des manifestes codés en URL.
* Correction de la navigation des en-têtes dans les documents HTML contenant des caractères Unicode multi-octets.
* Correction de l'utilisation élevée du CPU dans les documents avec de longs titres en raison d'une régression dans wxWidgets.
* Correction du chargement des fichiers texte UTF-8.
* Correction des éléments de table des matières imbriqués dans les livres Epub mettant votre curseur à la mauvaise position.
* Correction d'un plantage à la sortie de l'application dans certains cas.
* Ajout d'une case à cocher dans la boîte de dialogue des options pour activer ou désactiver le retour à la ligne automatique !
* Il est maintenant possible de faire un don au développement de Paperback, soit via le nouvel élément donate du menu help, soit via le lien sponsor this project en bas de la page principale du référentiel GitHub.
* Les documents Markdown auront maintenant toujours un titre, et Paperback devrait maintenant pouvoir charger pratiquement n'importe quel fichier Markdown.
* Les documents PDF auront maintenant toujours un titre, même si les métadonnées sont manquantes.
* Passage aux bibliothèques PDF utilisées dans Chromium, conduisant à une analyse PDF beaucoup plus fiable dans tous les domaines.
* Vous ne pouvez maintenant avoir qu'une seule instance de Paperback en cours d'exécution à la fois. L'exécution de paperback.exe avec un nom de fichier tandis qu'elle est déjà en cours d'exécution ouvrira ce document dans l'instance déjà en cours d'exécution.
* Vous pouvez maintenant appuyer sur Supprimer sur un document dans le contrôle de tabulation pour le fermer.

### Version 0.2.1
* Ajout du nombre total de pages à l'étiquette de page dans la boîte de dialogue go to page.
* Autoriser le passage de l'onglet du contenu du document à votre liste de documents ouverts.
* Correction des appuis clavier sur les en-têtes ouvrant parfois des documents récents si vous en aviez assez.
* Paperback supprimera maintenant les traits d'union souples inutiles de la sortie texte.
* Correction de la navigation des en-têtes vous mettant parfois sur le mauvais caractère.

### Version 0.2.0
* Ajout du support des documents Markdown !
* Ajout du support des documents PDF, y compris la possibilité de naviguer entre les pages !
* Ajout de raccourcis clavier pour naviguer par des en-têtes dans le contenu HTML, y compris les livres epub et les documents Markdown. Ces raccourcis ont été conçus pour fonctionner de manière similaire à un lecteur d'écran.
* Correction du chargement des epubs avec des noms de fichiers codés en URL dans leurs manifestes.
* Correction du chargement des livres epub 3 avec du XHTML intégré à l'intérieur.
* Un message est maintenant prononcé si le document ne supporte pas une table des matières ou des sections, au lieu que les éléments de menu soient désactivés.
* Ajout d'un menu de documents récents ! Il stocke actuellement vos 10 derniers documents ouverts, et appuyer sur Entrée sur l'un d'eux l'ouvrira pour la lecture.
* Réécriture complète de la boîte de dialogue Find, la rendant beaucoup plus simple à utiliser, tout en ajoutant un historique de vos 25 dernières recherches et le support des expressions régulières !
* Les documents ouverts auparavant sont maintenant mémorisés lors de redémarrages d'applications. Ceci est configurable via le nouvel élément options du menu tools.
* Ajout de `Shift+F1` pour ouvrir le fichier readme directement dans Paperback lui-même.

### Version 0.1.0
* Version initiale.
