<!-- machine-translated from doc/readme.md (source-hash: 13c58fb50049f608); please review and edit as needed -->

# Paperback - version 0.9.1

## Introduction

Paperback est un lecteur d'ebooks et de documents léger, rapide et accessible pour tous, des lecteurs occasionnels aux utilisateurs avertis. Il est conçu pour l'accessibilité aux lecteurs d'écran, la rapidité et une expérience sans surcharge.

## Configuration système requise

Paperback fonctionne actuellement sur Windows 10/11 et toutes les versions modernes d'ARM macOS. Les applications natives iOS et Android sont en développement actif, avec des versions bêta publiques prévues peu après la sortie de la version 0.9.0 du bureau, avant une sortie unifiée 1.0 couvrant les quatre plates-formes.

## Caractéristiques

* Complètement autonome, ne nécessitant l'installation d'aucun logiciel sur votre ordinateur pour commencer à lire.
* Incroyablement rapide, même sur du matériel ancien.
* Interface à onglets simple, vous permettant d'ouvrir autant de documents que vous le souhaitez côte à côte.
* Enregistre votre position de lecture exacte dans chaque document que vous ouvrez.
* Mémorise éventuellement les documents que vous aviez ouverts à la fermeture du programme et les restaure au prochain lancement.
* Inclut une fonctionnalité de navigation similaire à celle trouvée en mode navigation Web de nombreux lecteurs d'écran pour naviguer rapidement et facilement dans les documents.
* Inclut une solide boîte de dialogue de recherche, avec des fonctionnalités telles que l'historique et la prise en charge des expressions régulières.
* Peut être exécuté entièrement de manière portable, ou installé avec les associations de fichiers configurées automatiquement.
* Supporte un large éventail de formats de fichiers courants.

## Compatibilité avec les lecteurs d'écran

Paperback fonctionne bien avec tous les principaux lecteurs d'écran. Il existe cependant un problème connu pour les utilisateurs de JAWS.

### JAWS et afficheurs Braille

Si vous utilisez JAWS avec un afficheur Braille, vous constaterez peut-être que les paragraphes longs sont tronqués lors du défilement vers l'avant avec les touches de navigation de votre afficheur. La commande de lecture du paragraphe actuel est également affectée. Il s'agit d'un bug dans la gestion par JAWS du contrôle de texte RICHEDIT50W, et non quelque chose dans Paperback lui-même, et un bug dont la correction a pris du temps à émerger compte tenu de l'enthousiasme de Vispero à répondre aux problèmes des logiciels open source.

La solution de contournement, finalement mise à la surface à travers le groupe de discussion JAWS après des mois d'attente, consiste à éditer `paperback.jcf` et à définir « Braille Presentation and Panning » sur « Always use DOM if available ». Vous voudrez également activer « Pan Text by Paragraph », sinon votre afficheur restera sur le paragraphe actif plutôt que d'avancer. Avec les deux paramètres en place, le défilement devrait fonctionner correctement.

## Types de fichiers actuellement supportés

Paperback supporte les formats et extensions suivants :

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

Paperback est conçu pour une utilisation orientée vers le clavier. Voici les raccourcis actuels.

Les raccourcis ci-dessous sont pour Windows. Lorsque macOS diffère, l'équivalent est noté entre parenthèses — principalement parce que `Ctrl+G`, `Ctrl+W` et `Alt+Left`/`Alt+Right` sont déjà utilisés par d'autres conventions système ou d'application sur cette plateforme.

### Menu Fichier

* `Ctrl+O` : Ouvrir un document.
* `Ctrl+F4` (macOS : `Cmd+W`) : Fermer le document actuel.
* `Ctrl+Shift+F4` (macOS : `Cmd+Shift+W`) : Fermer tous les documents ouverts.
* `Ctrl+Shift+T` : Rouvrir le dernier document fermé.
* `Ctrl+R` : Afficher la boîte de dialogue « Tous les documents » (à partir des documents récents).
* `Ctrl+Q` : Quitter (Windows uniquement ; sur macOS, c'est dans le menu app).

### Menu Aller

* `Ctrl+F` : Afficher la boîte de dialogue Rechercher.
* `F3` (macOS : `Cmd+G`) : Trouver suivant.
* `Shift+F3` (macOS : `Cmd+Shift+G`) : Trouver précédent.
* `Ctrl+G` (macOS : `Cmd+L`) : Aller à la ligne.
* `Ctrl+Shift+G` (macOS : `Cmd+Shift+L`) : Aller au pourcentage.
* `Ctrl+P` : Aller à la page (si pris en charge par le document actuel).
* `=` : Annoncer votre pourcentage de lecture actuel.
* `Alt+Left` (macOS : `Cmd+[`) : Retour dans l'historique de navigation.
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
* `\` : Accéder à votre signet temporaire.
* `Shift+N` : Note précédente.
* `N` : Note suivante.
* `Ctrl+B` : Aller à tous les signets et notes.
* `Ctrl+Alt+B` : Aller aux signets uniquement.
* `Ctrl+Alt+M` : Aller aux notes uniquement.
* `Ctrl+Shift+W` (macOS : `RawCtrl+Shift+W`, c'est-à-dire la touche Control physique plutôt que Cmd) : Afficher le texte de la note à la position actuelle.
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

* `Ctrl+W` (macOS : `RawCtrl+W`, c'est-à-dire la touche Control physique plutôt que Cmd) : Afficher le nombre de mots pour le document actuel.
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
* `Ctrl+Shift+N` : Ajouter ou modifier la note de signet à la sélection/curseur actuel.
* `Ctrl+Alt+W` : Basculer l'habillage du texte.
* `Ctrl+Space` : Lire/suspendre la narration audio.
* `'` : Avancer la narration audio.
* `;` : Reculer la narration audio.
* `Ctrl+'` : Augmenter la quantité de recherche audio.
* `Ctrl+;` : Diminuer la quantité de recherche audio.
* `F11` (macOS : `RawCtrl+Ctrl+F`, c'est-à-dire Control+Command+F) : Basculer le plein écran.
* `Ctrl+,` : Ouvrir les options (macOS : Préférences, sous le menu app).
* `Ctrl+Shift+S` : Basculer la minuterie de sommeil.

### Menu Aide

* `Ctrl+F1` : Afficher la boîte de dialogue À propos.
* `F1` : Afficher l'aide dans votre navigateur par défaut.
* `Shift+F1` : Afficher l'aide dans Paperback.
* `Ctrl+Shift+U` : Vérifier les mises à jour.
* `Ctrl+D` : Ouvrir la page de don dans votre navigateur par défaut.

### Touches supplémentaires pour la vue de document

* `Delete` / `Numpad Delete` sur le contrôle d'onglet : Fermer l'onglet de document sélectionné.
* `Enter` ou `Space` dans le texte du document : Activer le lien au curseur, ou ouvrir une vue de tableau lorsque vous êtes sur un marqueur de tableau.
* `Shift+F10` ou la touche Menu/Application dans le texte du document : Ouvrir le menu contextuel.

## Langues prises en charge

Paperback est traduit dans de nombreuses langues différentes, et d'autres sont ajoutées tout le temps. Une liste complète suit ci-dessous.

Pour savoir comment contribuer, veuillez lire notre [Guide de traduction](translating.md).

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
Les personnes suivantes ont fait des donations de certaine importance au développement de Paperback. Si vous faites une donation, votre nom ne sera pas automatiquement ajouté ici, je n'ajoute que les personnes qui souhaitent que leur donation soit publique.

Remarque : Je considère un parrainage public sur GitHub comme une raison d'inclusion automatique dans cette liste.

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
* Les audiolivres ne font plus lire une suite d'espaces par votre lecteur d'écran lorsque vous focalisez le champ de texte.
* Les audiolivres nomment désormais le fichier à mesure que vous les parcourez section par section.
* Les audiolivres signalent maintenant leur durée réelle, au lieu de prétendre que chaque fichier dure 24 heures.
* Fermer la Web View avec Échap ne lève plus une alerte de débogage après avoir suivi un lien à l'intérieur.
* Copier après Sélectionner tout donne maintenant l'intégralité du document, au lieu de seulement la partie actuellement chargée.
* Trouver va maintenant directement à la ligne trouvée, au lieu de vous faire écouter le lecteur d'écran relire la fenêtre alors que le focus revient au livre.
* Correction des fichiers EPUB comportant un bloc ZIP64 parasite refusant de s'ouvrir avec « Invalid local file header ».
* Correction des documents longs revenant à leur début pendant qu'un lecteur d'écran les lisait de façon continue.
* Les liens dans la WebView vous amènent maintenant à la section vers laquelle ils pointent, au lieu d'échouer avec « File not found ».
* L'annonce automatique « Document rechargé » ne coupe plus votre lecteur d'écran en pleine phrase, attendant plutôt qu'il finisse ce qu'il était en train de dire.
* L'onglet Général de la boîte de dialogue Paramètres passe maintenant en revue ses options dans l'ordre dans lequel elles apparaissent à l'écran, avec le canal de mise à jour directement après l'option de vérification des mises à jour.
* Windows affichera désormais toujours « Paperback » dans le menu Ouvrir avec, au lieu de la ligne d'accroche complète du programme.
* Le comptage de mots et les informations sur le document indiquent maintenant combien de fichiers contient un audiolivre et sa durée totale.

### Version 0.9.1
* Les sons des signets et des notes se jouent maintenant sur macOS.
* Les livres DAISY jouent maintenant leur audio sur macOS, au lieu de s'ouvrir et de suivre leur chronologie en silence.
* Correction de la disparition des guillemets courbes, tirets en cadratin et caractères similaires dans les documents RTF, les mots environnants se fusionnant au passage.
* Correction des images RTF fuyant leurs données brutes dans le document sous forme de texte brouillé.
* Correction du sous-menu Documents récents conservant les entrées périmées jusqu'à ce que quelque chose d'autre ne le reconstruise.
* Les accélérateurs clavier sont de retour dans chaque traduction, de sorte que les menus russes ont accès au clavier.
* Les grands documents CHM s'ouvrent maintenant jusqu'à sept fois plus rapidement.
* Les documents ouverts sont maintenant enregistrés auprès de Windows, de sorte qu'ils apparaissent dans la liste de saut de la barre des tâches et dans la liste récente du menu Démarrer.
* Options a été renommé en Paramètres, ce qui correspond aux applications mobiles et, sur macOS, à la convention de la plateforme.
* Paperback se souvient maintenant de la position, de la taille et de l'état maximisé de sa fenêtre entre les exécutions.
* Les formes plurielles sont maintenant traduites, de sorte que les messages qui comptent les choses se lisent correctement dans les langues qui ont besoin de plus d'une forme.
* La sélection du fichier ncc.html d'un livre DAISY ouvre maintenant l'audiolivre complet au lieu de son seul texte.
* Les noms d'actions de la boîte de dialogue Personnaliser les raccourcis clavier peuvent maintenant être traduits.
* Le titre du document apparaît maintenant en premier dans la barre de titre, de sorte que les livres ouverts peuvent être distingués dans la barre des tâches et Alt+Tab.
* La boîte de dialogue de mise à jour est maintenant traduite.

### Version 0.9.0

#### Ajouts

##### Général
* Un outil CLI, appelé pb, pour convertir rapidement n'importe quel format pris en charge par Paperback en HTML, Markdown ou texte brut.
* Une option pour recharger les documents qui ont été modifiés par d'autres programmes sur le disque.
* Une option Afficher la source pour ouvrir la source d'un document dans un nouvel onglet, utile par exemple pour éditer Markdown.
* Le texte du document est maintenant paginé, ce qui signifie que vous pouvez charger des livres avec des dizaines de millions de mots en seulement quelques secondes. Veuillez signaler toute bizarrerie trouvée avec cela.

##### Support de plateforme
* Support ARM64 Windows !
* Support natif macOS !
* Un basculement en plein écran.

##### Boîte de dialogue Tous les documents
* Un bouton Localiser pour localiser les livres manquants qui viennent de changer de chemin.
* Un filtre d'état et une barre d'état, de sorte que vous pouvez filtrer par état du document et voir combien de documents sont affichés et sélectionnés.
* Le raccourci `Ctrl+Maj+A` pour désélectionner tous les documents.

##### Options et lisibilité
* Un onglet Lisibilité, avec les options suivantes :
    * Retour à la ligne automatique (déplacé depuis le menu général) ;
    * Rendre les tableaux en ligne (nouveau dans cette version, voir ci-dessous) ;
    * Police ;
    * Couleur de fond ;
    * Interligne ;
    * Espacement des paragraphes ;
    * Espacement des lettres ;
    * Alignement du texte.
* Un élément de menu de retour à la ligne automatique et un raccourci clavier suivant.
* Un basculement pour déterminer comment vous voulez que les tableaux soient affichés, et unification de la façon dont les tableaux sont affichés dans les documents.

##### Navigation
* Support pour naviguer par conteneur.
* Une option pour déplacer automatiquement le curseur au début de la ligne lors de la navigation entre les lignes, similaire au mode parcours dans les lecteurs d'écran.
* Le raccourci clavier égal pour annoncer votre pourcentage actuel dans un document.

##### Signets
* Signets temporaires : vous pouvez en avoir un par document, et ils persistent. Utilisez la barre oblique pour en définir un et la barre oblique inversée pour y accéder.

##### Comptage de mots
* Temps de lecture estimé dans la boîte de dialogue du comptage de mots, ainsi que la possibilité de définir votre vitesse de lecture pour rendre cette mesure réellement utile.
* Si une sélection est active lorsque vous ouvrez la boîte de dialogue du comptage de mots, le nombre de mots que vous avez sélectionnés s'affichera maintenant.

##### Raccourcis clavier
* La possibilité de personnaliser chaque raccourci clavier de l'application via une boîte de dialogue simple.
* Un raccourci clavier configurable pour restaurer Paperback à partir du plateau système.

##### Langues
* Néerlandais, finnois et polonais.

##### Export
* Expansion de l'élément de menu d'export pour permettre l'exportation en HTML et Markdown, en plus du texte brut.

##### Gestionnaire de mises à jour
* Un bouton Annuler à la boîte de dialogue mise à jour en cours.
* Le gestionnaire de mises à jour valide maintenant que le fichier téléchargé n'a pas été modifié.

##### Web View
* La webview est maintenant ouverte à votre position de lecture actuelle.

##### Livres DAISY
* Support des livres DAISY 2.0.
* Support de la lecture audio DAISY 2.02.

##### Audiolivres
* La possibilité de lire des audiolivres, prenant actuellement en charge à la fois l'audio DAISY (y compris l'audio DAISY + texte) et les archives de fichiers audio.
* Raccourcis clavier et éléments de menu pour lire/arrêter la narration, avancer et reculer, et ajuster la quantité de recherche.
* Options pour synchroniser le curseur de lecture à la lecture audio, définir la quantité de recherche audio et choisir si la recherche au-delà de la fin d'un chapitre continue dans le suivant.

##### Documents CHM
* Support des listes, des éléments de liste, des figures et des images.

##### PowerPoint
* Les documents PowerPoint prennent maintenant en charge les tableaux.

#### Corrections

##### Général
* Les documents encodés dans les encodages CJK hérités, tels que GBK, Big5 et Shift_JIS, s'affichent maintenant correctement au lieu de s'afficher sous forme de mojibake.
* « Rouvrir le dernier fermé » tentant de rouvrir le fichier readme fourni.
* Votre onglet sélectionné ne recevait pas correctement le focus après le redémarrage de Paperback.
* La gestion par Paperback des fichiers sur les lecteurs réseau Windows : appuyer sur afficher le fichier dans le dossier focalise maintenant correctement le fichier sur le stockage réseau, et les chemins ne contiennent plus de caractères étranges.
* Les fichiers .paperback ne seront plus forcément chargés lors de la restauration du document ; à la place, vous serez invité à confirmer si vous en trouvez un.
* Ouvrir le dossier conteneur focalise maintenant le fichier donné dans l'explorateur.
* L'ouverture du fichier readme respecte maintenant votre langue sélectionnée.
* L'interface utilisateur de Paperback s'adapte maintenant correctement sur les écrans haute résolution.
* Le menu se met maintenant à jour correctement, et le focus passe au contrôle de texte, lors de l'ouverture de l'aide dans Paperback.
* Passage à une méthode beaucoup plus sécurisée de l'IPC sur Windows.
* Le titre du document actif sera maintenant lu lors du basculement entre les onglets.
* Réduction de l'utilisation de la mémoire sur les grands documents en réduisant de moitié la taille des tableaux d'index internes par caractère.

##### Boîte de dialogue Tous les documents
* Échap ne fermant pas les boîtes de dialogue Informations sur le document et Tous les documents.
* La barre de titre ne se mettait pas à jour après la fermeture d'un document à partir de la boîte de dialogue tous les documents.
* Readme.html ne sera plus ajouté à votre liste de tous les documents lorsqu'il est ouvert via Maj+F1.
* La suppression de documents de la boîte de dialogue des récents ferme maintenant aussi leurs onglets actifs.
* Votre filtre de recherche est maintenant conservé après la suppression d'un document.

##### Navigation
* La navigation de page annonçant le texte de ligne incorrect dans certaines situations.
* Aller à la ligne, Aller à la page, et Aller au pourcentage plaçant votre curseur à la mauvaise position dans les grands documents.
* Trouver et Trouver suivant ne respectant pas la fenêtre du document chargé dans les grands documents.

##### Signets
* Les sons des signets/notes doivent maintenant se jouer exclusivement lorsque vous naviguez sur un mot en contenant un.

##### Lisibilité
* L'application du retour à la ligne automatique vous propulsait au début de votre document.

##### Web View
* La boîte de dialogue webview n'était pas redimensionnable et apparaissait avec une taille initiale très petite.
* Les images doivent maintenant s'afficher correctement dans la webview intégrée.

##### Gestionnaire de mises à jour
* Le gestionnaire de mises à jour affiche maintenant correctement le contenu des balises de code markdown dans les notes de version.

##### Livres DAISY
* Les livres DAISY affichaient les informations incorrectes dans la barre d'état.
* Chargement des livres DAISY avec des déclarations d'encodage erronées.

##### Documents RTF
* Analyse des documents RTF contenant des caractères non-latins.
* Groupes RTF `\pict` de sorte que les données d'image intégrées ne fuient plus dans le texte du document.

##### Livres Mobi/AZW3
* Les ancres filepos dans les livres Mobi divisant les balises HTML et mettant des ordures dans le texte du livre.
* Liens dans les livres Mobi hérités.
* Analyse AZW3 considérablement améliorée.

##### Documents Word
* Les documents Word avec des noms de styles spécifiques aux paramètres régionaux ne rendaient pas correctement leurs titres.

##### Documents HTML/XHTML
* Les éléments dl, dt, et dd ne produisant pas de sauts de ligne dans les documents XHTML.

##### Documents PDF
* Paperback revient maintenant à l'extraction de texte brut pour les PDF mal étiquetés.
* Les documents PDF contenant des caractères de contrôle dans leurs titres et/ou signets n'arrêtent plus Paperback à l'ouverture.

### Version 0.8.5
* Support de page ajouté aux livres epub.
* Support ajouté pour les documents Microsoft Office chiffrés. Actuellement, les versions antérieures et modernes de Word et PowerPoint modernes sont prises en charge, avec PowerPoint antérieur prévu pour l'avenir.
* Support ajouté pour les documents Microsoft Word hérités !
* Support ajouté pour les présentations PowerPoint hérités !
* Support ajouté pour les livres mobi et AZW3 !
* Support ajouté pour les fichiers PDF étiquetés !
* Ajout du raccourci ctrl+q pour quitter l'application.
* Support ajouté pour les livres zippés de Bookshare (à la fois DAISY et Word) !
* Le texte alt pour les images intégrées doit maintenant être correctement affiché.
* Les documents CHM prennent maintenant correctement en charge la navigation des liens internes.
* Correction de l'aller à la page étant décalé de 1.
* Correction de la touche Échap ne fonctionnant pas pour fermer le dialogue Ouvrir en tant que.
* Correction du menu contextuel du lecteur ne s'affichant pas au clic droit ou sur la touche Applications.
* Correction du mauvais document étant parfois focalisé lors de l'ouverture de documents depuis la ligne de commande.
* Les PDF contenant uniquement des images sont à nouveau détectés et vous avertissent de leur existence.
* Il est maintenant possible de naviguer à travers les images et les figures avec g/maj+g et f/maj+f, respectivement.
* Paperback respecte maintenant votre paramètre du mode sombre de l'application.
* Suppression du support DAISY XML, car ce n'est plus nécessaire.
* Retour à la première lettre native Win32 dans l'arborescence de la table des matières.
* La boîte de dialogue de chargement d'erreur affiche maintenant des messages d'erreur plus détaillés.
* La webview s'ouvrira maintenant beaucoup plus rapidement et en douceur.

### Version 0.8.2
* Support de page ajouté aux documents RTF !
* Correction d'un bogue dans lequel l'ouverture de la webview dans les epubs contenant des liens externes les activait automatiquement.
* Correction d'un bogue dans lequel l'analyseur RTF ne mettrait pas d'espace entre les mots dans de rares cas.
* Correction de la division des paragraphes en plusieurs lignes courtes dans certains documents PDF.
* Les documents PDF ont maintenant un support de navigation basique par lien et titre !
* Les tabulations et sauts de ligne RTF sont maintenant rendus exactement comme ils apparaissent dans le document.
* Retour à la bibliothèque pdfium éprouvée pour l'analyse des PDF, rendant l'analyse PDF beaucoup plus fiable une fois de plus.

### Version 0.8.1
* Ajout de Ctrl+Maj+T pour rouvrir le dernier document fermé.
* La boîte de dialogue Tous les documents prend maintenant en charge la sélection de plusieurs documents à ouvrir à la fois.
* Correction de quelques bogues avec l'analyseur RTF.
* Correction de la corruption des chemins de fichiers contenant des caractères non-ASCII (tels que le bosniaque š, č, ć, ž) lors de l'ouverture d'un fichier via une deuxième instance de Paperback.
* Correction du texte PDF étant lu dans le mauvais ordre et l'espacement incorrect autour des mots en majuscules.
* Correction du chargement lent des documents lors de l'ouverture de grands fichiers.
* Correction de la localisation des boutons Oui/Non dans les boîtes de dialogue de confirmation.

### Version 0.8.0
* Ajout des traductions en japonais, chinois simplifié et vietnamien !
* Ajout d'un gestionnaire de mises à jour automatique qui remplacera maintenant votre version actuellement installée de Paperback au lieu de simplement télécharger la nouvelle version !
* Ajout d'une rétroaction sonore facultative pour atteindre un signet ou une note, merci à Andre Louis pour les sons !
* Support des documents RTF ajouté !
* Support ajouté pour les documents DAISY XML.
* Support ajouté pour les fichiers de texte Open Document plats !
* Support ajouté pour les présentations Open Document plates !
* Support ajouté pour les séparateurs avec s et maj+s.
* Tout mouvement supérieur à 300 caractères ajoutera maintenant automatiquement à votre historique de navigation.
* Correction de la restauration de la fenêtre de Paperback depuis le plateau système.
* Correction de l'affichage du texte brut au lieu du rendu HTML dans la Web View pour les documents Markdown.
* Correction de l'affichage incorrect des tableaux dans les fichiers Markdown.
* Les PDF contenant uniquement des images vous avertissent maintenant de leur existence lorsque vous en chargez un.
* Intégration correcte des informations de version dans l'exécutable Paperback.
* Division de la boîte de dialogue des options en onglets pour la facilité d'utilisation et la navigation.
* Passage à Hayro pour l'analyse des PDF, ce qui améliore la fiabilité, la vitesse et réduit les DLL.
* Réécriture complète de l'application en Rust. La nouvelle base de code est plus sûre, charge les documents plus rapidement et est plus facile à maintenir et à étendre.
* Le menu contextuel du contrôle de texte inclura maintenant des actions spécifiques au lecteur au lieu d'éléments génériques tels que couper et coller.

### Version 0.7.0
* Support de table ajouté pour les documents basés sur HTML et XHTML ! Naviguez entre les tableaux en utilisant T et Maj+T, et appuyez sur Entrée pour en consulter un dans une webview.
* Fonctionnalité de rendu web basique ajoutée ! Appuyez sur Ctrl+Maj+V pour ouvrir la section actuelle de votre document dans un moteur de rendu basé sur le web, utile pour du contenu comme la mise en forme complexe ou les exemples de code.
* Ajout d'une traduction russe, merci Ruslan Gulmagomedov !
* Ajout d'un bouton Tout effacer à la boîte de dialogue Tous les documents.
* Le vérificateur de mise à jour affiche maintenant les notes de version lorsqu'une nouvelle version est disponible.
* Correction de la restauration de la fenêtre depuis le plateau système.
* Correction des traductions des boutons Oui/Non dans les boîtes de dialogue de confirmation.
* Correction du chargement des configurations lors de l'exécution en tant qu'administrateur.
* Correction de la gestion des commentaires dans les documents XML et HTML.
* Correction de l'analyse TOC dans les livres Epub 2.
* Correction de la navigation vers l'élément suivant avec la même lettre dans la table des matières.
* Correction de la boîte de dialogue Rechercher ne se cachant pas correctement lors de l'utilisation des boutons suivant/précédent.
* Correction des TOC épub vous jetant occasionnellement au mauvais élément.
* Correction de divers problèmes de gestion des espaces blancs dans XML, HTML et les balises pre.
* Correction d'erreur décalée de 1 dans la navigation des liens.
* Correction de certains livres ayant des espaces blancs à la fin de leurs lignes.
* Correction de divers problèmes d'analyseur.
* Les éléments de menu liés aux signets ainsi que la liste des éléments sont maintenant correctement désactivés lorsqu'aucun document n'est ouvert.
* Amélioration de la gestion des listes dans divers formats de document.
* Amélioration du flux de travail de traduction pour les contributeurs.
* De nombreuses refactorisations internes, déplaçant la majorité de la logique métier de l'application de C++ à Rust pour améliorer les performances et la maintenabilité.

### Version 0.6.1
* Support ajouté pour les PDF protégés par mot de passe !
* Fonctionnalité très basique d'aller à la position précédente/suivante ajoutée. Si vous appuyez sur Entrée sur un lien interne et qu'il déplace votre curseur, cette position sera maintenant mémorisée et peut être naviguée avec les flèches alt+gauche/droite.
* Liste d'éléments ajoutée ! Actuellement, il affiche uniquement une arborescence de tous les titres de votre document ou une liste de liens, mais il y a des plans pour l'étendre à l'avenir.
* Option ajoutée pour démarrer Paperback en mode maximisé par défaut.
* Correction des liens dans certains documents Epub ne fonctionnant pas correctement.
* Correction de l'analyse des TOC Epub contenant des chemins relatifs.
* Correction de certains documents epub n'affichant pas de titre ou d'auteur.
* Correction de l'absence d'affichage des titres de certains chapitres epub dans la boîte de dialogue TOC.
* Correction de l'impossibilité d'utiliser la barre d'espace pour activer les boutons OK/annuler dans la boîte de dialogue TOC.
* Amélioration de la gestion des titres dans les documents Word.
* Vous obtiendrez maintenant une rétroaction parlée si la liste des documents récents est vide lorsque vous essayez d'afficher la boîte de dialogue.

### Version 0.6.0
* Une nouvelle option pour afficher le menu aller sous une forme beaucoup plus compacte a été ajoutée à la boîte de dialogue des options, cochée par défaut.
* Option ajoutée pour que la navigation par éléments structurels s'enroule.
* Option ajoutée au menu outils pour ouvrir le dossier conteneur du document actuellement focalisé.
* Ajout d'un système de mise à jour assez simple mais très efficace.
* Fonctionnalité de minuteur de veille basique ajoutée, accessible avec Ctrl+Maj+S.
* Support ajouté pour l'analyse des livres électroniques FB2 !
* Support ajouté pour l'analyse des présentations OpenDocument !
* Support ajouté pour l'analyse des fichiers de texte OpenDocument !
* Les signets peuvent maintenant créer un signet d'une ligne entière, ou marquer uniquement un texte spécifié. Si vous n'avez pas de sélection active lors du placement d'un signet, le comportement est comme avant 0.6, et il marquera la ligne entière. Cependant, si vous sélectionnez un texte, seul ce texte sera inclus dans le signet.
* Les signets peuvent maintenant avoir des notes de texte optionnelles attachées ! Naviguez entre les signets contenant des notes avec N et Maj+N, ou ouvrez la boîte de dialogue des signets avec tous les signets, uniquement les notes, ou uniquement les non-notes sélectionnées avec des raccourcis spécifiques.
* Les signets dans la boîte de dialogue des signets n'auront plus de préfixe annoying « signet x ».
* Les livres Epub contenant du contenu HTML se faisant passer pour du XML seront maintenant gérés correctement.
* Correction du chargement de grands documents Markdown.
* Correction de l'appui sur l'espace dans l'arborescence de la table des matières activant le bouton OK.
* Correction de la gestion des espaces blancs au début des balises pre dans les documents HTML et XHTML.
* Correction du contrôle de texte ne reprenant pas le focus parfois lors du retour à la fenêtre de Paperback.
* Correction du champ de texte dans la boîte de dialogue aller au pourcentage ne mettant pas à jour la valeur du curseur.
* Correction du rendu des ID HTML personnalisés dans les documents Markdown.
* Le HTML à l'intérieur des blocs de code Markdown sera maintenant restitué correctement.
* Si vous chargez un livre avec un paramètre de ligne de commande alors qu'une instance Paperback existante est en cours d'exécution, vous n'aurez plus d'erreur si le chargement de votre document prend plus de 5 secondes.
* Si vous exécutez Paperback en tant qu'administrateur, la configuration sera maintenant correctement chargée et enregistrée.
* Il est maintenant possible de supprimer un signet directement à partir de la boîte de dialogue des signets.
* Il est maintenant possible d'importer et d'exporter vos signets et votre position de lecture pour un document particulier. Le fichier généré porte le nom du fichier avec une extension .paperback. Si un tel fichier est trouvé dans le même répertoire qu'un fichier lors du chargement, il sera automatiquement chargé. Sinon, vous pouvez les importer manuellement en utilisant un élément du menu outils.
* Les liens à l'intérieur des documents sont maintenant entièrement pris en charge ! Utilisez k et maj+k pour vous déplacer en avant et en arrière à travers eux, et appuyez sur Entrée pour ouvrir/activer l'un d'eux.
* De nombreuses refactorisations internes, rendant l'application plus rapide et le binaire plus petit.
* Le contenu Markdown est maintenant prétraité pour être conforme à CommonMark avant le rendu.
* La navigation par listes et leurs éléments est maintenant entièrement prise en charge ! Utilisez L et Maj+L pour aller par les listes elles-mêmes, et I et Maj+I pour parcourir les éléments de liste.
* Numpad supprimer fonctionne maintenant pour supprimer les documents de la barre des onglets en plus de la suppression normale.
* Paperback peut maintenant opter pour minimiser dans votre barre d'état système ! Cette option est désactivée par défaut, mais l'activer fera que l'option minimiser du menu système met Paperback dans votre barre d'état système, capable d'être restaurée en cliquant sur l'icône générée.
* Paperback est maintenant entièrement traductible ! La liste des langues qu'il prend en charge est actuellement assez petite, mais elle augmente constamment !
* Paperback a maintenant un site officiel, à [paperback.dev](https://paperback.dev) !
* Les documents PPTX affichent maintenant une table des matières basique, contenant toutes les diapositives.
* Le chemin complet du document ouvert s'affiche maintenant dans la boîte de dialogue d'informations sur le document.
* Le programme d'installation inclut maintenant une option pour afficher le fichier readme dans votre navigateur après l'installation.
* La liste des documents récents a été considérablement agrandie ! Au lieu de simplement vous montrer les 10 derniers documents que vous avez ouverts, elle vous montre maintenant un nombre personnalisable, le reste des documents que vous avez jamais ouverts étant accessible via une petite boîte de dialogue.
* Diverses petites améliorations apportées aux analyseurs dans l'ensemble, notamment en plaçant une ligne vierge entre les diapositives dans les présentations PPTX, en corrigeant la gestion des sauts de ligne à l'intérieur des paragraphes dans les documents Word, et en ajoutant des puces aux éléments de liste.

### Version 0.5.0
* Support ajouté pour les documents Microsoft Word !
* Support ajouté pour les présentations PowerPoint !
* Correction de certains éléments de menu ne étant pas désactivés sans documents ouverts.
* Correction de l'orientation du curseur du pourcentage d'aller à.
* Correction de la table des matières dans les livres Epub avec chemins de fichiers codés en URL et/ou ID de fragment.
* Correction de l'épuration des espaces blancs des titres XHTML de manière bizarre.
* Correction de la gestion des espaces blancs à l'intérieur des balises pre imbriquées dans les documents HTML.
* Les documents HTML et Markdown prennent maintenant en charge la fonctionnalité de table des matières ! Lorsque vous chargez un document HTML/Markdown, Paperback construira sa propre table des matières à partir de la structure des titres de votre document, et il vous la montrera dans la boîte de dialogue ctrl+t.
* Les documents HTML auront maintenant le titre tel que défini dans la balise de titre, s'il existe. Sinon, ils continueront à utiliser le nom de fichier sans l'extension.
* Passage de UniversalSpeech à l'utilisation d'une région active pour signaler la parole. Cela signifie qu'aucune DLL de lecteur d'écran n'est expédiée aux côtés du programme, et plus de lecteurs d'écran seront désormais pris en charge, tels que le Narrateur Microsoft.
* Passage à des bibliothèques zip pour permettre l'ouverture d'un plus large éventail de livres epub.
* La boîte de dialogue vous demandant si vous souhaitez ouvrir votre document en tant que texte brut a été complètement refaite, et elle vous permet maintenant d'ouvrir votre document en tant que texte brut, HTML ou Markdown.
* La boîte de dialogue Aller au pourcentage inclut maintenant un champ de texte vous permettant d'entrer manuellement un pourcentage pour sauter.
* L'analyseur HTML reconnaîtra maintenant dd, dt et dl comme éléments de liste.
* La table des matières dans les livres Epub sera à nouveau préservée exactement.
* L'espace blanc non-rompant Unicode est maintenant pris en compte lors de la suppression des lignes vierges.
* Vous ne serez plus demandé comment vous souhaitez ouvrir un fichier non reconnu chaque fois que vous le chargez, seulement la première fois.

### Version 0.4.1
* Ajout d'une icône de menu Démarrer en option au programme d'installation.
* La table des matières devrait maintenant être plus propre dans quelques cas, par exemple si vous avez un élément enfant et parent avec le même texte à la même position, vous ne verrez maintenant que l'élément parent.
* Correction de la table des matières dans certains documents CHM.
* Correction de la table des matières dans les livres Epub 3 avec chemins absolus.
* Les documents CHM devraient maintenant afficher leur titre tel que défini dans le fichier de métadonnées.

### Version 0.4.0
* Support de fichier CHM ajouté !
* Support des signets ajouté ! Vous pouvez avoir autant de signets que vous le souhaitez dans autant de documents que vous le souhaitez. Vous pouvez sauter en avant et en arrière à travers eux avec b et maj+b, en définir un avec ctrl+maj+b, et apporter une boîte de dialogue pour sauter à un signet spécifique avec ctrl+b.
* Un programme d'installation à côté du fichier zip portable ajouté ! Le programme d'installation installera Paperback dans votre répertoire Program Files et configurera automatiquement les associations de fichiers pour vous.
* Les fichiers texte avec BOM doivent maintenant être décodés correctement, et le BOM ne s'affichera plus au début du texte non plus.
* Ajout de beaucoup plus d'informations à la barre d'état. Il affichera maintenant votre ligne actuelle, votre caractère et votre pourcentage de lecture.
* Les commentaires HTML, ainsi que le contenu des balises de script et de style, ne s'affichent plus dans la sortie texte.
* Si vous transmettez un chemin relatif à Paperback en ligne de commande, il résoudra maintenant correctement.
* Le mouvement en pourcentage est maintenant géré par sa propre boîte de dialogue basée sur curseur, accessible avec ctrl+maj+g.
* Les documents sans titres ou auteurs connus auront maintenant une valeur par défaut.
* La logique de sauvegarde de la position est maintenant beaucoup plus intelligente et doit être écrite sur le disque uniquement en cas de besoin absolu.
* Le document sur lequel vous aviez focalisé lorsque vous avez fermé Paperback est maintenant mémorisé entre les redémarrages de l'application.
* L'entrée dans les boîtes de dialogue Aller à la ligne et Aller à la page doit maintenant être assainie plus strictement.
* Correction de la navigation de la table des matières dans les livres epub 3 avec chemins relatifs dans leurs manifestes.

### Version 0.3.0
* Correction de la table des matières dans les livres epub avec manifestes codés en URL.
* Correction de la navigation des titres dans les documents HTML contenant des caractères Unicode multi-octets.
* Correction de l'utilisation élevée du CPU dans les documents avec de longs titres en raison d'une régression dans wxWidgets.
* Correction du chargement des fichiers texte UTF-8.
* Correction de l'imbrication des éléments TOC dans les livres Epub mettant votre curseur à la mauvaise position.
* Correction d'un plantage à la fermeture de l'application dans certains cas.
* Ajout d'une case à cocher dans la boîte de dialogue des options pour activer ou désactiver le retour à la ligne automatique !
* Il est maintenant possible de faire un don au développement de Paperback, soit par le nouvel élément de don du menu Aide, soit par le lien du projet de parrainage en bas de la page principale du référentiel GitHub.
* Les documents Markdown auront maintenant toujours un titre, et Paperback devrait maintenant être capable de charger pratiquement n'importe quel fichier Markdown.
* Les documents PDF auront maintenant toujours un titre, même si les métadonnées manquent.
* Passage à la bibliothèque PDF utilisée dans Chromium, conduisant à une analyse PDF beaucoup plus fiable dans l'ensemble.
* Vous ne pouvez avoir qu'une seule instance de Paperback en cours d'exécution à la fois. L'exécution de paperback.exe avec un nom de fichier alors qu'il est déjà en cours d'exécution ouvrira ce document dans l'instance déjà en cours d'exécution.
* Vous pouvez maintenant appuyer sur Supprimer sur un document dans le contrôle des onglets pour le fermer.

### Version 0.2.1
* Ajout du nombre total de pages à l'étiquette de page dans la boîte de dialogue Aller à la page.
* Autoriser le passage en revue du contenu du document à votre liste de documents ouverts.
* Correction de l'activation parfois des frappes de titre après les documents récents si vous en aviez assez.
* Paperback supprimera maintenant les traits d'union conditionnels inutiles du texte de sortie.
* Correction de la navigation des titres vous mettant parfois sur le mauvais caractère.

### Version 0.2.0
* Support des documents Markdown ajouté !
* Support des documents PDF ajouté, y compris la capacité de naviguer entre les pages !
* Touches ajoutées pour naviguer par titres dans le contenu HTML, y compris les livres epub et les documents Markdown. Ces touches ont été conçues pour fonctionner de manière similaire à un lecteur d'écran.
* Correction du chargement des epubs avec des noms de fichiers codés en URL dans leurs manifestes.
* Correction du chargement des livres epub 3 avec XHTML intégré.
* Un message est maintenant parlé si le document ne prend pas en charge une table des matières ou des sections, par rapport au désactif des éléments de menu.
* Menu des documents récents ajouté ! Il stocke actuellement vos 10 derniers documents ouverts, et appuyer sur Entrée sur l'un d'eux l'ouvrira pour la lecture.
* Réécriture complète de la boîte de dialogue Rechercher, la rendant beaucoup plus simple à utiliser, tout en ajoutant un historique de vos 25 dernières recherches et un support des expressions régulières !
* Les documents précédemment ouverts sont maintenant mémorisés entre les redémarrages de l'application. Ceci est configurable via le nouvel élément options du menu outils.
* Ajout de maj+f1 pour ouvrir le fichier readme directement dans Paperback lui-même.

### Version 0.1.0
* Version initiale.
