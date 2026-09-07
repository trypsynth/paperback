<!-- machine-translated from doc/readme.md (source-hash: c5a741eb100e6fbc; sections: f48ce952,d4d8160f,a02f4421,a4ffb7f7,91be3b41,55bac79e,a548b5d0,71df8e94,e9860ee8,c7735cbe); please review and edit as needed -->

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

## Types de fichiers actuellement pris en charge

Paperback prend en charge les formats et extensions suivants :

* Fichiers d'aide CHM (`.chm`)
* Livres DAISY (`.opf`, `.zip`)
* Livres EPUB (`.epub`)
* Livres électroniques FB2 (`.fb2`)
* Documents HTML (`.htm`, `.html`, `.xhtml`)
* Documents Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Documents Microsoft Word (`.docx`, `.docm`, `.doc`)
* Audiolivres M4B (`.m4b`)
* Livres MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
* Présentations OpenDocument (`.odp`, `.fodp`)
* Fichiers texte OpenDocument (`.odt`, `.fodt`)
* Documents PDF (`.pdf`)
* Présentations PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Documents RTF (`.rtf`)
* Fichiers texte brut et journaux (`.txt`, `.log`)

## Raccourcis clavier

Paperback est conçu pour une utilisation en priorité au clavier. Voici les raccourcis actuels.

Les raccourcis ci-dessous sont pour Windows. Lorsque macOS diffère, l'équivalent est noté entre parenthèses — principalement parce que Ctrl+G, Ctrl+W et Alt+Left/Right sont déjà utilisés par d'autres conventions système ou d'application sur cette plateforme.

### Menu Fichier

* `Ctrl+O` : Ouvrir un document.
* `Ctrl+F4` (macOS : `Cmd+W`) : Fermer le document actuel.
* `Ctrl+Shift+F4` (macOS : `Cmd+Shift+W`) : Fermer tous les documents ouverts.
* `Ctrl+Shift+T` : Rouvrir le dernier document fermé.
* `Ctrl+R` : Afficher la boîte de dialogue « Tous les documents » (à partir des documents récents).
* `Ctrl+Q` : Quitter (Windows uniquement ; sur macOS, c'est dans le menu de l'application).

### Menu Aller

* `Ctrl+F` : Afficher la boîte de dialogue Rechercher.
* `F3` (macOS : `Cmd+G`) : Rechercher le suivant.
* `Shift+F3` (macOS : `Cmd+Shift+G`) : Rechercher le précédent.
* `Ctrl+G` (macOS : `Cmd+L`) : Aller à la ligne.
* `Ctrl+Shift+G` (macOS : `Cmd+Shift+L`) : Aller au pourcentage.
* `Ctrl+P` : Aller à la page (lorsque supporté par le document actuel).
* `=` : Annoncer votre pourcentage de lecture actuel.
* `Alt+Left` (macOS : `Cmd+[`) : Revenir en arrière dans l'historique de navigation.
* `Alt+Right` (macOS : `Cmd+]`) : Avancer dans l'historique de navigation.
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
* `,` : Aller après la fin du conteneur actuel (liste ou tableau).

### Menu Outils

* `Ctrl+W` (macOS : `RawCtrl+W`, c'est-à-dire la touche Contrôle physique plutôt que Cmd) : Afficher le nombre de mots du document actuel.
* `Ctrl+I` : Afficher les informations du document.
* `Ctrl+T` : Afficher la table des matières.
* `F7` : Afficher la liste des éléments.
* `Ctrl+Shift+C` : Ouvrir le dossier contenant.
* `Ctrl+Shift+V` : Ouvrir le contenu actuel dans l'affichage Web.
* `Ctrl+U` : Afficher la source du document dans un nouvel onglet.
* `Ctrl+Shift+E` : Exporter les données du document (`.paperback`).
* `Ctrl+Shift+I` : Importer les données du document (`.paperback`).
* `Ctrl+E` : Exporter le document actuel en texte brut.
* `Ctrl+Shift+B` : Basculer le signet à la sélection/curseur actuel.
* `Ctrl+Shift+N` : Ajouter ou modifier la note du signet à la sélection/curseur actuel.
* `Ctrl+Alt+W` : Basculer le retour à la ligne automatique.
* `Ctrl+Space` : Lire/pause la narration audio.
* `'` : Avancer dans la narration audio.
* `;` : Reculer dans la narration audio.
* `Ctrl+'` : Augmenter la durée de déplacement audio.
* `Ctrl+;` : Diminuer la durée de déplacement audio.
* `F11` (macOS : `RawCtrl+Ctrl+F`, c'est-à-dire Contrôle+Commande+F) : Basculer le mode plein écran.
* `Ctrl+,` : Ouvrir les options (macOS : Préférences, dans le menu de l'application).
* `Ctrl+Shift+S` : Basculer la minuterie de sommeil.

### Menu Aide

* `Ctrl+F1` : Afficher la boîte de dialogue À propos.
* `F1` : Afficher l'aide dans votre navigateur par défaut.
* `Shift+F1` : Afficher l'aide dans Paperback.
* `Ctrl+Shift+U` : Vérifier les mises à jour.
* `Ctrl+D` : Ouvrir la page de don dans votre navigateur par défaut.

### Touches supplémentaires de la vue document

* `Delete` / `Numpad Delete` sur le contrôle d'onglet : Fermer l'onglet du document sélectionné.
* `Enter` ou `Space` dans le texte du document : Activer le lien au curseur, ou ouvrir une vue de tableau lorsqu'on se trouve sur un marqueur de tableau.
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

## Historique des versions

### Version 0.9.2
* Les audiolivres ne font plus lire par le lecteur d'écran une suite d'espaces lorsque vous concentrez le champ de texte.
* Les audiolivres nomment maintenant le fichier à mesure que vous les parcourez section par section.
* Les audiolivres rapportent maintenant leur vraie durée, au lieu de prétendre que chaque fichier dure 24 heures.
* Fermer la Web View avec Échap ne déclenche plus une alerte de débogage après avoir suivi un lien à l'intérieur.
* Copier après Sélectionner tout vous donne maintenant l'intégralité du document, au lieu de seulement la partie actuellement chargée.
* Rechercher va maintenant directement à la ligne trouvée, au lieu de vous forcer à écouter le lecteur d'écran relire la fenêtre alors que le focus revient au livre.
* Correction des EPUB contenant un bloc ZIP64 égaré refusant de s'ouvrir avec « Invalid local file header ».
* Correction des longs documents revenant à leur début alors qu'un lecteur d'écran les lisait continuellement.
* Les liens dans la WebView vous mènent maintenant à la section vers laquelle ils pointent, au lieu d'échouer avec « File not found ».
* L'annonce automatique « Document reloaded » ne coupe plus votre lecteur d'écran en pleine phrase, attendant au lieu de cela qu'il finisse ce qu'il disait.
* L'onglet Général du dialogue Paramètres navigue maintenant dans ses options dans l'ordre où elles apparaissent à l'écran, avec le canal de mise à jour directement après l'option de vérification des mises à jour.
* Windows affichera maintenant toujours « Paperback » dans le menu Ouvrir avec, au lieu de la ligne d'accroche complète du programme.
* Le Nombre de mots et les Informations du document affichent maintenant le nombre de fichiers qu'un audiolivre contient et sa durée totale.

### Version 0.9.1
* Les sons des signets et des notes se jouent maintenant sur macOS.
* Les livres DAISY jouent maintenant leur audio sur macOS, au lieu de s'ouvrir et de suivre leur chronologie en silence.
* Correction des guillemets bouclés, tirets em et caractères similaires disparaissant des documents RTF, fusionnant les mots environnants.
* Correction des images RTF qui fuyaient leurs données brutes dans le document sous forme de texte brouillé.
* Correction du sous-menu Documents récents conservant les entrées obsolètes jusqu'à ce que quelque chose d'autre le reconstruise.
* Les accélérateurs clavier sont de retour dans chaque traduction, les menus russes ont à nouveau accès au clavier.
* Les gros documents CHM s'ouvrent maintenant jusqu'à sept fois plus vite.
* Les documents ouverts sont maintenant enregistrés auprès de Windows, ils apparaissent dans la liste de saut de la barre des tâches et dans la liste récente du menu Démarrer.
* Options a été renommé en Paramètres, correspondant aux applications mobiles et, sur macOS, à la convention de la plateforme.
* Paperback se souvient maintenant de sa position de fenêtre, sa taille et son état maximisé entre les exécutions.
* Les formes plurielles sont maintenant traduites, les messages qui comptent les choses se lisent correctement dans les langues qui ont besoin de plus d'une forme.
* Sélectionner le ncc.html d'un livre DAISY ouvre maintenant l'audiolivre complet au lieu de simplement son texte.
* Les noms d'actions du dialogue Personnaliser les raccourcis clavier peuvent maintenant être traduits.
* Le titre du document vient maintenant en premier dans la barre de titre, les livres ouverts peuvent être distingués dans la barre des tâches et Alt+Tab.
* Le dialogue de mise à jour est maintenant traduit.

### Version 0.9.0

#### Ajouté

##### Général
* Un outil CLI, appelé pb, pour convertir rapidement n'importe quel format supporté de Paperback en HTML, Markdown ou texte brut.
* Une option pour recharger les documents qui ont été modifiés par d'autres programmes sur le disque.
* Une option Afficher la source pour ouvrir le code source d'un document dans un nouvel onglet, utile par exemple pour éditer Markdown.
* Le texte du document est maintenant paginé, ce qui signifie que vous pouvez charger des livres avec des dizaines de millions de mots en seulement quelques secondes. Veuillez signaler toute bizarrerie trouvée avec cela.

##### Support de plateforme
* Support ARM64 Windows !
* Support natif macOS !
* Un basculement de plein écran.

##### Dialogue Tous les documents
* Un bouton de localisation pour localiser les livres manquants qui viennent de changer de chemin.
* Un filtre de statut et une barre de statut, vous pouvez filtrer par statut de document et voir combien de documents sont affichés et sélectionnés.
* Le raccourci `Ctrl+Shift+A` pour désélectionner tous les documents.

##### Options et lisibilité
* Un onglet de lisibilité, avec les options suivantes :
    * Retour à la ligne automatique (déplacé de général) ;
    * Rendu des tableaux en ligne (nouveau dans cette version, voir ci-dessous) ;
    * Police ;
    * Couleur de fond ;
    * Interligne ;
    * Espacement des paragraphes ;
    * Espacement des lettres ;
    * Alignement du texte.
* Un élément de menu de retour à la ligne automatique et une touche de raccourci ultérieure.
* Un basculement pour déterminer comment vous voulez que les tableaux soient affichés, et a unifié l'affichage des tableaux dans tous les documents.

##### Navigation
* Support de la navigation par conteneur.
* Une option pour déplacer automatiquement le curseur au début de la ligne lors de la navigation entre les lignes, similaire au mode parcours des lecteurs d'écran.
* Le raccourci clavier égal pour annoncer votre pourcentage actuel dans un document.

##### Signets
* Signets temporaires : vous pouvez en avoir un par document, et ils persistent. Utilisez la barre oblique pour en définir un et la barre oblique inverse pour y accéder.

##### Nombre de mots
* Temps de lecture estimé dans le dialogue de nombre de mots, ainsi que la possibilité de définir votre vitesse de lecture pour rendre cette métrique réellement utile.
* Si une sélection est active lorsque vous ouvrez le dialogue de nombre de mots, le nombre de mots sélectionnés sera maintenant affiché.

##### Raccourcis clavier
* La possibilité de personnaliser tous les raccourcis clavier de l'application via un dialogue simple.
* Un raccourci clavier configurable pour restaurer Paperback depuis le plateau système.

##### Langues
* Néerlandais, finnois et polonais.

##### Export
* Expansion de l'élément de menu d'exportation pour permettre l'exportation en HTML et Markdown, en plus du texte brut.

##### Mise à jour
* Un bouton d'annulation au dialogue de mise à jour en cours.
* La mise à jour valide maintenant que le fichier téléchargé n'a pas été falsifié.

##### Web View
* La webview s'ouvre maintenant à votre position de lecture actuelle.

##### Livres DAISY
* Support des livres DAISY 2.0.
* Support de la lecture audio DAISY 2.02.

##### Audiolivres
* La capacité de lire des audiolivres, supportant actuellement à la fois DAISY audio (y compris DAISY audio + texte) et les fichiers zippés audio.
* Raccourcis clavier et éléments de menu pour lire/mettre en pause la narration, avancer et reculer, et ajuster la quantité d'avance.
* Options pour synchroniser le curseur de lecture avec la lecture audio, définir la quantité d'avance audio et choisir si l'avance rapide à la fin d'un chapitre continue dans le suivant.

##### Documents CHM
* Support des listes, éléments de liste, figures et images.

##### PowerPoint
* Les documents PowerPoint supportent maintenant les tableaux.

#### Corrections

##### Général
* Les documents encodés dans des codages CJK hérités, tels que GBK, Big5 et Shift_JIS, s'affichent maintenant correctement au lieu d'une suite de caractères brisés.
* « Rouvrir le dernier fermé » tentant de rouvrir le fichier lisez-moi fourni.
* Votre onglet sélectionné ne se concentrant pas correctement après le redémarrage de Paperback.
* La gestion des fichiers de Paperback sur les lecteurs réseau Windows : appuyer sur afficher le fichier dans le dossier concentre correctement le fichier sur le stockage réseau, et les chemins ne contiennent plus de caractères étranges.
* Les fichiers .paperback ne seront plus forcément chargés lors de la restauration du document ; vous serez plutôt demandé une confirmation lorsqu'un est trouvé.
* Ouvrir le dossier contenant concentre maintenant le fichier donné dans l'explorateur.
* L'ouverture du fichier lisez-moi respecte maintenant votre langue sélectionnée.
* L'interface utilisateur de Paperback s'adapte maintenant correctement sur les écrans haute résolution.
* Le menu se met à jour correctement maintenant, et le focus se déplace vers le contrôle de texte, lors de l'ouverture de l'aide dans Paperback.
* Passage à une méthode beaucoup plus sécurisée de l'IPC sur Windows.
* Le titre du document actif sera maintenant lu lors du basculement entre les onglets.
* Réduction de l'utilisation de la mémoire sur les gros documents en réduisant de moitié la taille des tableaux d'index par caractère internes.

##### Dialogue Tous les documents
* Échap ne fermant pas les dialogues Document Info et Tous les documents.
* La barre de titre ne se mettant pas à jour après la fermeture d'un document à partir du dialogue tous les documents.
* Readme.html ne sera plus ajouté à votre liste tous les documents lorsqu'il est ouvert via Shift+F1.
* La suppression de documents du dialogue récents fermera maintenant aussi leurs onglets actifs.
* Votre filtre de recherche est maintenant préservé après la suppression d'un document.

##### Navigation
* La navigation par page annonçant un texte de ligne incorrect dans certaines situations.
* Aller à la ligne, Aller à la page et Aller au pourcentage plaçant votre curseur à la mauvaise position dans les gros documents.
* Rechercher et Rechercher suivant ne respectant pas la fenêtre de document chargée dans les gros documents.

##### Signets
* Les sons des signets/notes doivent maintenant se jouer exclusivement lorsque vous naviguez sur un mot en contenant un.

##### Lisibilité
* L'application du retour à la ligne automatique vous tirant au début de votre document.

##### Web View
* Le dialogue webview n'étant pas redimensionnable et s'affichant à une taille initiale très petite.
* Les images doivent maintenant s'afficher correctement dans la webview intégrée.

##### Mise à jour
* La mise à jour affiche maintenant correctement le contenu des balises de code markdown dans les notes de version.

##### Livres DAISY
* Les livres DAISY affichant des informations incorrectes dans la barre de statut.
* Chargement des livres DAISY avec des déclarations d'encodage incorrectes.

##### Documents RTF
* Analyse des documents RTF contenant des caractères non latins.
* Les groupes RTF `\pict` pour que les données d'image intégrées ne fuient plus dans le texte du document.

##### Livres Mobi/AZW3
* Les ancres filepos dans les livres Mobi divisant les balises HTML et mettant des ordures dans le texte du livre.
* Les liens dans les livres Mobi hérités.
* Analyse AZW3 considérablement améliorée.

##### Documents Word
* Les documents Word avec des noms de style spécifiques aux paramètres régionaux n'affichant pas correctement leurs titres.

##### Documents HTML/XHTML
* Les éléments dl, dt et dd ne produisant pas de sauts de ligne dans les documents XHTML.

##### Documents PDF
* Paperback revient maintenant à l'extraction de texte brut pour les PDF incorrectement étiquetés.
* Les documents PDF contenant des caractères de contrôle dans leurs titres et/ou signets n'crasheront plus Paperback à l'ouverture.

### Version 0.8.5
* Ajout du support de page aux livres epub.
* Ajout du support des documents Microsoft Office chiffrés. Actuellement Word hérité, PowerPoint moderne et Word moderne sont supportés, PowerPoint hérité étant prévu pour l'avenir.
* Ajout du support des documents Microsoft Word hérités !
* Ajout du support des présentations PowerPoint héritées !
* Ajout du support des livres mobi et AZW3 !
* Ajout du support des fichiers PDF étiquetés !
* Ajout du raccourci ctrl+q pour quitter l'application.
* Ajout du support des livres zippés de Bookshare (DAISY et Word) !
* Le texte alternatif pour les images intégrées devrait maintenant s'afficher correctement.
* Les documents CHM supportent maintenant correctement la navigation des liens internes.
* Correction de l'aller à la page étant décalé de 1.
* Correction de la touche Échap ne fonctionnant pas pour fermer le dialogue ouvrir en tant que.
* Correction du menu contextuel du lecteur ne s'affichant pas au clic droit ou à la touche Applications.
* Correction du mauvais document parfois concentré lors de l'ouverture de documents à partir de la ligne de commande.
* Les PDF contenant uniquement des images sont à nouveau détectés et vous alertent de leur existence.
* Il est maintenant possible de naviguer dans les images et les figures avec g/shift+g et f/shift+f, respectivement.
* Paperback respectera maintenant votre paramètre de mode sombre de l'application.
* Suppression du support DAISY XML, car il n'est plus nécessaire.
* Retour à la navigation par première lettre native Win32 dans l'arborescence de la table des matières.
* Le dialogue de chargement des erreurs affiche maintenant des messages d'erreur plus détaillés.
* La webview s'ouvrira maintenant beaucoup plus vite et en douceur.

### Version 0.8.2
* Ajout du support de page aux documents RTF !
* Correction d'un bug où l'ouverture de la webview dans les epub contenant des liens externes les activerait automatiquement.
* Correction d'un bug où l'analyseur RTF ne mettrait pas d'espace entre les mots dans des cas rares.
* Correction des paragraphes divisés en plusieurs lignes courtes dans certains documents PDF.
* Les documents PDF ont maintenant un support de navigation de liens et de titres de base !
* Les tabulations et les retours à la ligne RTF sont maintenant rendus exactement comme ils apparaissent dans le document.
* Retour à la bibliothèque pdfium éprouvée pour l'analyse des PDF, rendant l'analyse des PDF beaucoup plus fiable à nouveau.

### Version 0.8.1
* Ajout de Ctrl+Shift+T pour rouvrir le dernier document fermé.
* Le dialogue Tous les documents supporte maintenant la sélection de plusieurs documents à ouvrir à la fois.
* Correction de quelques bugs avec l'analyseur RTF.
* Correction des chemins de fichiers contenant des caractères non-ASCII (tels que le serbe š, č, ć, ž) devenant corrompus lors de l'ouverture d'un fichier via une deuxième instance de Paperback.
* Correction du texte PDF étant lu dans le mauvais ordre, et de l'espacement incorrect autour des mots en majuscules.
* Correction du chargement lent des documents lors de l'ouverture de gros fichiers.
* Correction de la localisation des boutons Oui/Non dans les dialogues de confirmation.

### Version 0.8.0
* Ajout des traductions en japonais, chinois simplifié et vietnamien !
* Ajout d'une mise à jour automatique qui remplacera maintenant votre version actuellement installée de Paperback au lieu de simplement télécharger la nouvelle version !
* Ajout du retour sonore facultatif pour atteindre un signet ou une note, merci Andre Louis pour les sons !
* Ajout du support des documents RTF !
* Ajout du support des documents DAISY XML.
* Ajout du support des fichiers Texte Open Document aplatis !
* Ajout du support des présentations Open Document aplaties !
* Ajout du support des séparateurs avec s et shift+s.
* Tout mouvement supérieur à 300 caractères ajoutera maintenant automatiquement à votre historique de navigation.
* Correction de la restauration de la fenêtre de Paperback depuis le plateau système.
* Correction des documents Markdown affichant du texte brut au lieu du HTML rendu dans la Web View.
* Correction des tableaux ne s'affichant pas correctement dans les fichiers Markdown.
* Les PDF contenant uniquement des images vous avertissent maintenant de leur existence lorsque vous tentez d'en charger un.
* Incorporation correcte des informations de version dans l'exécutable Paperback.
* Division du dialogue des options en onglets pour la facilité d'utilisation et la navigation.
* Passage à Hayro pour l'analyse des PDF, conduisant à plus de fiabilité, de vitesse et de moins de DLL.
* Réécriture de l'application entière en Rust. La nouvelle base de code est plus sûre, charge les documents plus vite, et est plus facile à maintenir et à étendre.
* Le menu contextuel du contrôle de texte inclura maintenant des actions spécifiques au lecteur au lieu d'éléments génériques tels que couper et coller.

### Version 0.7.0
* Ajout du support de tableau pour les documents basés sur HTML et XHTML ! Naviguez entre les tableaux en utilisant T et Shift+T, et appuyez sur Entrée pour en afficher un dans une webview.
* Ajout d'une fonction de rendu web de base ! Appuyez sur Ctrl+Shift+V pour ouvrir la section actuelle de votre document dans un moteur de rendu basé sur le web, utile pour le contenu comme le formatage complexe ou les exemples de code.
* Ajout d'une traduction en russe, merci Ruslan Gulmagomedov !
* Ajout d'un bouton Effacer tout au dialogue Tous les documents.
* Le vérificateur de mise à jour affiche maintenant les notes de version lorsqu'une nouvelle version est disponible.
* Correction de la restauration de la fenêtre depuis le plateau système.
* Correction de la traduction des boutons Oui/Non dans les dialogues de confirmation.
* Correction du chargement des configurations lors de l'exécution en tant qu'administrateur.
* Correction de la gestion des commentaires dans les documents XML et HTML.
* Correction de l'analyse TOC dans les livres Epub 2.
* Correction de la navigation vers l'élément suivant avec la même lettre dans la table des matières.
* Correction du dialogue de recherche ne se cachant pas correctement lors de l'utilisation des boutons suivant/précédent.
* Correction des TOC epub vous jetant occasionnellement au mauvais élément.
* Correction de divers problèmes de gestion des espaces blancs dans XML, HTML et les balises pre.
* Correction d'une erreur hors de 1 dans la navigation des liens.
* Correction de certains livres ayant des espaces blancs à la fin de leurs lignes.
* Correction de divers problèmes d'analyse.
* Les éléments de menu liés aux signets ainsi que la liste des éléments sont maintenant correctement désactivés lorsqu'aucun document n'est ouvert.
* Amélioration de la gestion des listes dans divers formats de document.
* Amélioration du flux de travail de traduction pour les contributeurs.
* De nombreuses refactorisations internes, déplaçant la majorité de la logique métier de l'application de C++ à Rust pour une meilleure performance et maintenabilité.

### Version 0.6.1
* Ajout du support des PDF protégés par mot de passe !
* Ajout d'une fonction très basique d'aller à la position précédente/suivante. Si vous appuyez sur Entrée sur un lien interne et que cela déplace votre curseur, cette position sera maintenant mémorisée, et peut être naviguée avec les flèches alt+gauche/droite.
* Ajout d'une liste d'éléments ! Actuellement, elle n'affiche qu'une arborescence de tous les titres de votre document ou une liste de liens, mais il y a des plans pour l'étendre à l'avenir.
* Ajout d'une option pour démarrer Paperback en mode maximisé par défaut.
* Correction des liens dans certains documents Epub ne fonctionnant pas correctement.
* Correction de l'analyse des TOC Epub contenant des chemins relatifs.
* Correction de certains documents epub n'affichant pas de titre ou d'auteur.
* Correction des titres de certains chapitres epub ne s'affichant pas correctement dans le dialogue TOC.
* Correction du fait que vous ne pouviez pas utiliser la barre d'espace pour activer les boutons OK/annulation dans le dialogue TOC.
* Amélioration de la gestion des titres dans les documents Word.
* Vous obtiendrez maintenant un retour parlé si la liste des documents récents est vide lorsque vous essayez d'afficher le dialogue.

### Version 0.6.0
* Une nouvelle option pour afficher le menu aller sous une forme beaucoup plus compacte a été ajoutée au dialogue des options, cochée par défaut.
* Ajout d'une option pour que la navigation par éléments structurels s'enroule.
* Ajout d'une option au menu outils pour ouvrir le dossier contenant du document actuellement concentré.
* Ajout d'un système de mise à jour assez simple mais très efficace.
* Ajout d'une fonction de minuterie de sommeil basique, accessible avec Ctrl+Shift+S.
* Ajout du support de l'analyse des livres FB2 !
* Ajout du support de l'analyse des présentations OpenDocument !
* Ajout du support de l'analyse des fichiers Texte OpenDocument !
* Les signets peuvent maintenant être faits pour marquer une ligne entière, ou pour marquer uniquement du texte spécifié. Si vous n'avez pas de sélection active lorsque vous placez un signet, le comportement est comme avant 0.6, et il marquera la ligne entière. Cependant, si vous sélectionnez du texte, seul ce texte sera inclus dans le signet.
* Les signets peuvent maintenant avoir des notes de texte optionnelles attachées ! Naviguez entre les signets contenant des notes avec N et Shift+N, ou affichez le dialogue des signets avec tous les signets, uniquement les notes ou uniquement les non-notes sélectionnés avec des touches de raccourci spécifiques.
* Les signets dans le dialogue des signets n'auront plus un préfixe ennuyeux « signet x ».
* Les livres Epub contenant du contenu HTML prétendant être du XML seront maintenant gérés correctement.
* Correction du chargement de gros documents Markdown.
* Correction de l'appui sur espace dans l'arborescence de vue d'arbre de la table des matières activant le bouton OK.
* Correction de la gestion des espaces blancs au début des balises pre dans les documents HTML et XHTML.
* Correction du champ de texte ne reprenant pas le focus parfois lors du retour à la fenêtre de Paperback.
* Correction du champ de texte du dialogue aller au pourcentage ne mettant pas à jour la valeur du curseur.
* Correction du rendu des ID HTML personnalisés dans les documents Markdown.
* Le HTML à l'intérieur des blocs de code Markdown sera maintenant rendu correctement.
* Si vous chargez un livre avec un paramètre de ligne de commande alors qu'une instance Paperback existante est en cours d'exécution, vous n'obtiendrez plus une erreur si le chargement de votre document prend plus de 5 secondes.
* Si vous exécutez Paperback en tant qu'administrateur, la configuration sera maintenant chargée et enregistrée correctement.
* Il est maintenant possible de supprimer un signet directement depuis le dialogue des signets.
* Il est maintenant possible d'importer et d'exporter vos signets et votre position de lecture pour un document particulier. Le fichier généré est nommé d'après le fichier avec une extension .paperback. Si un tel fichier est trouvé dans le même répertoire qu'un fichier lors de son chargement, il sera automatiquement chargé. Sinon, vous pouvez les importer manuellement en utilisant un élément dans le menu outils.
* Les liens à l'intérieur des documents sont maintenant entièrement supportés ! Utilisez k et shift+k pour avancer et reculer dans les liens, et appuyez sur Entrée pour en ouvrir/activer un.
* De nombreuses refactorisations internes, rendant l'application plus rapide et le binaire plus petit.
* Le contenu Markdown est maintenant prétraité pour être conforme à CommonMark avant le rendu.
* La navigation par listes et leurs éléments est maintenant entièrement supportée ! Utilisez L et Shift+L pour naviguer par les listes elles-mêmes, et I et Shift+I pour naviguer dans les éléments de liste.
* Supprimer sur le pavé numérique fonctionne maintenant aussi pour supprimer les documents de la barre d'onglets en plus du Supprimer normal.
* Paperback peut maintenant optionnellement se minimiser dans votre plateau système ! Cette option est désactivée par défaut, mais l'activation fera que l'option de minimisation dans le menu système mette Paperback dans votre plateau, capable d'être restauré en cliquant sur l'icône générée.
* Paperback est maintenant entièrement traduisible ! La liste des langues qu'il supporte est actuellement assez petite, mais elle grandit constamment !
* Paperback a maintenant un site Web officiel, sur [paperback.dev](https://paperback.dev) !
* Les documents PPTX affichent maintenant une table des matières de base, contenant toutes les diapositives.
* Le chemin complet vers le document ouvert sera maintenant affiché dans le dialogue des informations du document.
* L'installeur inclut maintenant une option pour afficher le fichier lisez-moi dans votre navigateur après l'installation.
* La liste des documents récents a été considérablement agrandie ! Au lieu de simplement vous afficher les 10 derniers documents que vous avez ouverts, elle affichera maintenant un nombre personnalisable, les documents restants que vous avez jamais ouverts étant accessibles via un petit dialogue.
* Diverses petites améliorations des analyseurs dans l'ensemble, y compris la mise d'une ligne vierge entre les diapositives dans les présentations PPTX, la correction de la gestion des retours à la ligne à l'intérieur des paragraphes dans les documents Word, et l'ajout de puces aux éléments de liste.

### Version 0.5.0
* Ajout du support des documents Microsoft Word !
* Ajout du support des présentations PowerPoint !
* Correction de certains éléments de menu ne being désactivés sans documents ouvert.
* Correction de l'orientation du curseur aller au pourcentage.
* Correction de la table des matières dans les livres Epub avec des chemins de fichier codés en URL et/ou des ID de fragment.
* Correction de l'espacement blanc étant supprimé des titres XHTML de manière étrange.
* Correction de la gestion des espaces blancs à l'intérieur des balises pre imbriquées dans les documents HTML.
* Les documents HTML et Markdown supportent maintenant la fonction de table des matières ! Lorsque vous chargez un document HTML/Markdown, Paperback construira sa propre table des matières à partir de la structure des titres de votre document, et il vous l'affichera dans le dialogue ctrl+t.
* Les documents HTML auront maintenant le titre tel que défini dans la balise titre, s'il existe. Sinon, ils continueront à utiliser le nom de fichier sans l'extension.
* Passage de UniversalSpeech à l'utilisation d'une région dynamique pour signaler la parole. Cela signifie qu'aucune DLL de lecteur d'écran ne sera expédiée avec le programme, et plus de lecteurs d'écran seront maintenant supportés, tels que Microsoft Narrator.
* Passage des bibliothèques de fichiers zippés pour permettre d'ouvrir un plus large éventail de livres epub.
* Le dialogue vous demandant si vous voulez ouvrir votre document en texte brut a été complètement refait, et il vous permet maintenant d'ouvrir votre document en texte brut, HTML ou Markdown.
* Le dialogue aller au pourcentage inclut maintenant un champ de texte vous permettant de saisir manuellement un pourcentage pour y accéder.
* L'analyseur HTML reconnaît maintenant dd, dt et dl comme éléments de liste.
* La table des matières dans les livres Epub sera maintenant préservée exactement.
* L'espace non-coupable unicode est maintenant considéré lors de la suppression de lignes vierges.
* Vous ne serez plus demandé comment vous voulez ouvrir un fichier non reconnu à chaque fois que vous le chargez, uniquement la première fois.

### Version 0.4.1
* Ajout d'une icône du menu Démarrer optionnelle à l'installeur.
* La table des matières devrait maintenant être plus propre dans quelques cas, par exemple si vous avez un élément enfant et parent avec le même texte à la même position, vous verrez maintenant uniquement l'élément parent.
* Correction de la table des matières dans certains documents CHM.
* Correction de la table des matières dans les livres Epub 3 avec des chemins absolus dedans.
* Les documents CHM devraient maintenant afficher leur titre tel que défini dans le fichier de métadonnées.

### Version 0.4.0
* Ajout du support des fichiers CHM !
* Ajout du support des signets ! Vous pouvez avoir autant de signets que vous le souhaitez dans autant de documents que vous le souhaitez. Vous pouvez naviguer vers l'avant et vers l'arrière avec b et shift+b, en définir un avec control+shift+b, et afficher un dialogue pour accéder à un signet spécifique avec control+b.
* Ajout d'un installeur aux côtés du fichier zip portable ! L'installeur installera Paperback dans votre répertoire Program Files, et configurera automatiquement les associations de fichiers pour vous.
* Les fichiers texte avec des BOM devraient maintenant être décodés correctement, et le BOM ne s'affichera plus au début du texte.
* Ajout d'informations bien plus détaillées à la barre de statut. Il affichera maintenant votre ligne actuelle, votre caractère et votre pourcentage de lecture.
* Les commentaires HTML, ainsi que le contenu des balises script et style, ne s'afficheront plus dans la sortie de texte.
* Si vous passez un chemin relatif à Paperback sur la ligne de commande, il le résoudra maintenant correctement.
* Le mouvement en pourcentage est maintenant géré par son propre dialogue basé sur un curseur, accessible avec control+shift+g.
* Les documents sans titres ou auteurs connus auront maintenant une valeur par défaut.
* La logique de sauvegarde de position est maintenant beaucoup plus intelligente et ne devrait écrire sur le disque que si c'est absolument nécessaire.
* Le document sur lequel vous vous concentriez lorsque vous avez fermé Paperback est maintenant mémorisé entre les redémarrages de l'application.
* L'entrée dans les dialogues aller à la ligne et aller à la page devrait maintenant être assainie plus strictement.
* Correction de la navigation de la table des matières dans les livres epub 3 avec des chemins relatifs dans leurs manifestes.

### Version 0.3.0
* Correction de la table des matières dans les livres epub avec des manifestes codés en URL.
* Correction de la navigation par titre dans les documents HTML contenant des caractères Unicode multi-octets.
* Correction de l'utilisation élevée du CPU dans les documents avec de longs titres en raison d'une régression dans wxWidgets.
* Correction du chargement des fichiers texte UTF-8.
* Correction des éléments de TOC imbriqués dans les livres Epub mettant votre curseur à la mauvaise position.
* Correction d'un crash à la sortie de l'application dans certains cas.
* Ajout d'une case à cocher dans le dialogue des options pour activer ou désactiver le retour à la ligne automatique !
* Il est maintenant possible de faire un don au développement de Paperback, soit via le nouvel élément de donation dans le menu aide, soit via le lien de parrainage du projet en bas de la page principale du référentiel GitHub.
* Les documents Markdown auront maintenant toujours un titre, et Paperback devrait maintenant être capable de charger pratiquement n'importe quel fichier Markdown.
* Les documents PDF auront maintenant toujours un titre, même si les métadonnées sont manquantes.
* Passage à la bibliothèque PDF utilisée dans Chromium, conduisant à une analyse PDF beaucoup plus fiable dans l'ensemble.
* Vous ne pouvez maintenant avoir qu'une seule instance de Paperback en cours d'exécution à la fois. L'exécution de paperback.exe avec un nom de fichier alors qu'il est déjà en cours d'exécution ouvrira ce document dans l'instance déjà en cours d'exécution.
* Vous pouvez maintenant appuyer sur Supprimer sur un document dans la commande de tabulation pour le fermer.

### Version 0.2.1
* Ajout du nombre total de pages à l'étiquette de page dans le dialogue aller à la page.
* Permettre la tabulation du contenu du document à votre liste de documents ouverts.
* Correction de l'appui sur les touches de titre ouvrant parfois les documents récents si vous en aviez assez.
* Paperback supprimera maintenant les traits d'union conditionnels inutiles du texte en sortie.
* Correction de la navigation par titre vous mettant parfois sur le mauvais caractère.

### Version 0.2.0
* Ajout du support des documents markdown !
* Ajout du support des documents PDF, y compris la possibilité de naviguer entre les pages !
* Ajout de touches de frappe pour naviguer par les titres dans le contenu HTML, y compris les livres epub et les documents markdown. Ces touches de frappe ont été conçues pour fonctionner de manière similaire à un lecteur d'écran.
* Correction du chargement des epub avec les noms de fichier codés en URL dans leurs manifestes.
* Correction du chargement des livres epub 3 avec XHTML intégré en eux.
* Un message est maintenant parlé si le document ne supporte pas une table des matières ou des sections, par opposition à la désactivation des éléments de menu.
* Ajout d'un menu de documents récents ! Il stocke actuellement vos 10 derniers documents ouverts, et appuyer sur Entrée sur l'un d'eux l'ouvrira pour la lecture.
* Réécriture complète du dialogue Rechercher, le rendant beaucoup plus simple à utiliser, tout en ajoutant un historique de vos 25 dernières recherches et un support des expressions régulières !
* Les documents précédemment ouverts sont maintenant mémorisés entre les redémarrages de l'application. Ceci est configurable via le nouvel élément des options dans le menu outils.
* Ajout de shift+f1 pour ouvrir le fichier lisez-moi directement dans Paperback lui-même.

### Version 0.1.0
* Version initiale.
