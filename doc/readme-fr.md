<!-- machine-translated from doc/readme.md (source-hash: efe922e94821c70e); please review and edit as needed -->

# Paperback - version 0.9.2

## Introduction

Paperback est un lecteur d'ebooks et de documents léger, rapide et accessible pour tous, des lecteurs occasionnels aux utilisateurs expérimentés. Il est conçu pour l'accessibilité aux lecteurs d'écran, la rapidité et une expérience sans encombrement.

## Configuration requise

Paperback fonctionne actuellement sur Windows 10/11 et toutes les versions modernes d'ARM macOS. Les applications natives iOS et Android sont en développement actif, avec des versions publiques de test prévues peu après la sortie de la version 0.9.0 du bureau, avant une sortie unifiée 1.0 couvrant les quatre plates-formes.

## Fonctionnalités

* Complètement autonome, ne nécessitant l'installation d'aucun logiciel sur votre ordinateur pour commencer à lire.
* Incroyablement rapide, même sur du matériel ancien.
* Interface à onglets simple, vous permettant d'ouvrir autant de documents que vous le souhaitez côte à côte.
* Enregistre votre position de lecture exacte dans tous les documents que vous ouvrez.
* Se souvient éventuellement des documents que vous aviez ouverts à la fermeture du programme et les restaure au lancement suivant.
* Inclut une fonctionnalité de navigation similaire à celle que l'on trouve en mode navigation web de nombreux lecteurs d'écran pour naviguer rapidement et facilement dans les documents.
* Inclut une boîte de dialogue de recherche robuste, avec des fonctionnalités telles que l'historique et la prise en charge des expressions régulières.
* Peut fonctionner entièrement en mode portable ou être installé avec les associations de fichiers configurées automatiquement.
* Prend en charge un large éventail de formats de fichiers courants.

## Compatibilité avec les lecteurs d'écran

Paperback fonctionne bien avec tous les lecteurs d'écran majeurs. Il existe cependant un problème connu pour les utilisateurs de JAWS.

### JAWS et afficheurs braille

Si vous utilisez JAWS avec un afficheur braille, vous pouvez constater que les longs paragraphes sont tronqués lors du panoramique vers l'avant avec les touches de navigation de votre afficheur. La commande de lecture du paragraphe actif est également affectée. C'est un bug dans la gestion par JAWS du contrôle de texte RICHEDIT50W, et non quelque chose dans Paperback lui-même, et c'est un problème pour lequel il a fallu un certain temps pour trouver une correction étant donné l'enthousiasme de Vispero à répondre aux problèmes des logiciels open source.

La solution de contournement, finalement révélée par le groupe de discussion JAWS après des mois d'attente, consiste à modifier `paperback.jcf` et à définir « Braille Presentation and Panning » sur « Always use DOM if available ». Vous voudrez également activer « Pan Text by Paragraph », sinon votre afficheur restera sur le paragraphe actif au lieu d'avancer. Avec les deux paramètres en place, le panoramique devrait fonctionner correctement.

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

Les raccourcis ci-dessous sont pour Windows. Lorsque macOS diffère, l'équivalent est noté entre parenthèses — principalement parce que Ctrl+G, Ctrl+W et Alt+Left/Right sont déjà utilisés par d'autres conventions système ou d'application sur cette plateforme.

### Menu Fichier

* `Ctrl+O` : Ouvrir un document.
* `Ctrl+F4` (macOS : `Cmd+W`) : Fermer le document actuel.
* `Ctrl+Shift+F4` (macOS : `Cmd+Shift+W`) : Fermer tous les documents ouverts.
* `Ctrl+Shift+T` : Rouvrir le dernier document fermé.
* `Ctrl+R` : Afficher la boîte de dialogue « Tous les documents » (parmi les documents récents).
* `Ctrl+Q` : Quitter (Windows uniquement ; sur macOS, cela se trouve dans le menu application).

### Menu Aller

* `Ctrl+F` : Afficher la boîte de dialogue Rechercher.
* `F3` (macOS : `Cmd+G`) : Trouver suivant.
* `Shift+F3` (macOS : `Cmd+Shift+G`) : Trouver précédent.
* `Ctrl+G` (macOS : `Cmd+L`) : Aller à la ligne.
* `Ctrl+Shift+G` (macOS : `Cmd+Shift+L`) : Aller au pourcentage.
* `Ctrl+P` : Aller à la page (lorsque supporté par le document actuel).
* `=` : Annoncer votre pourcentage de lecture actuel.
* `Alt+Left` (macOS : `Cmd+[`) : Revenir dans l'historique de navigation.
* `Alt+Right` (macOS : `Cmd+]`) : Avancer dans l'historique de navigation.
* `[` : Section précédente.
* `]` : Section suivante.
* `Shift+H` : En-tête précédent.
* `H` : En-tête suivant.
* `Shift+1` à `Shift+6` : En-tête précédent au niveau 1-6.
* `1` à `6` : En-tête suivant au niveau 1-6.
* `Shift+P` : Page précédente.
* `P` : Page suivante.
* `Shift+B` : Signet précédent.
* `B` : Signet suivant.
* `/` : Définir votre signet temporaire.
* `\` : Aller à votre signet temporaire.
* `Shift+N` : Note précédente.
* `N` : Note suivante.
* `Ctrl+B` : Accéder à tous les signets et notes.
* `Ctrl+Alt+B` : Accéder aux signets uniquement.
* `Ctrl+Alt+M` : Accéder aux notes uniquement.
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
* `,` : Aller après la fin du conteneur actuel (liste ou tableau).

### Menu Outils

* `Ctrl+W` (macOS : `RawCtrl+W`, c'est-à-dire la touche Control physique plutôt que Cmd) : Afficher le nombre de mots du document actuel.
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
* `Ctrl+Alt+W` : Basculer le retour à la ligne automatique.
* `Ctrl+Space` : Lire/pause la narration audio.
* `'` : Avancer la narration audio.
* `;` : Reculer la narration audio.
* `Ctrl+'` : Augmenter la durée de recherche audio.
* `Ctrl+;` : Diminuer la durée de recherche audio.
* `F11` (macOS : `RawCtrl+Ctrl+F`, c'est-à-dire Control+Command+F) : Basculer le mode plein écran.
* `Ctrl+,` : Ouvrir les options (macOS : Préférences, dans le menu application).
* `Ctrl+Shift+S` : Basculer la minuterie de sommeil.

### Menu Aide

* `Ctrl+F1` : Afficher la boîte de dialogue À propos.
* `F1` : Afficher l'aide dans votre navigateur par défaut.
* `Shift+F1` : Afficher l'aide dans Paperback.
* `Ctrl+Shift+U` : Vérifier les mises à jour.
* `Ctrl+D` : Ouvrir la page de don dans votre navigateur par défaut.

### Touches supplémentaires dans la vue document

* `Delete` / `Numpad Delete` sur le contrôle d'onglet : Fermer l'onglet du document sélectionné.
* `Enter` ou `Space` dans le texte du document : Activer le lien au curseur, ou ouvrir une vue de tableau lorsque vous êtes sur un marqueur de tableau.
* `Shift+F10` ou la touche Menu/Application dans le texte du document : Ouvrir le menu contextuel.

## Langues supportées

Paperback est traduit dans de nombreuses langues différentes, avec d'autres ajoutées en continu. Une liste complète suit ci-dessous.

Pour apprendre à contribuer, veuillez consulter notre [Guide de traduction](translating.md).

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
Les personnes suivantes ont fait des dons de toutes tailles pour le développement de Paperback. Si vous faites un don, votre nom ne sera pas automatiquement ajouté ici, je n'ajoute que les personnes qui souhaitent que leur don soit rendu public.

Remarque : Je considère un parrainage GitHub public comme suffisant pour une inclusion automatique dans cette liste.

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
* Les livres audio ne font plus lire à votre lecteur d'écran une série d'espaces lorsque vous focalisez le champ de texte.
* Les livres audio nomment maintenant le fichier au fur et à mesure que vous les parcourez section par section.
* Les livres audio signalent maintenant leur vraie durée, au lieu de prétendre que chaque fichier dure 24 heures.
* La fermeture de Web View avec Escape ne déclenche plus d'alerte de débogage après avoir suivi un lien à l'intérieur.
* La copie après Sélectionner tout vous donne maintenant le document complet, au lieu de seulement la partie actuellement chargée.
* La fonction Rechercher va maintenant directement à la ligne trouvée, au lieu de vous faire attendre que le lecteur d'écran relise la fenêtre lors du retour de la focalisation au livre.
* Correction des EPUB qui contenaient un bloc ZIP64 isolé refusant de s'ouvrir avec « En-tête de fichier local invalide ».
* Correction des longs documents qui revenaient au début pendant qu'un lecteur d'écran les lisait continuellement.
* Les liens dans WebView vous mènent maintenant à la section vers laquelle ils pointent, au lieu d'échouer avec « Fichier non trouvé ».
* L'annonce automatique « Document rechargé » ne coupe plus la parole à votre lecteur d'écran en pleine phrase, elle attend plutôt qu'il finisse ce qu'il disait.
* L'onglet Général de la boîte de dialogue Paramètres effectue maintenant un cycle à travers ses options dans l'ordre où elles apparaissent à l'écran, avec le canal de mise à jour directement après l'option de vérification des mises à jour.
* Windows affichera maintenant toujours « Paperback » dans le menu Ouvrir avec, au lieu de la ligne d'accroche complète du programme.
* Nombre de mots et Informations sur le document affichent maintenant le nombre de fichiers qu'un livre audio contient, et sa durée totale.

### Version 0.9.1
* Les sons des signets et des notes jouent maintenant sur macOS.
* Les livres DAISY jouent maintenant leur audio sur macOS, au lieu de s'ouvrir et de suivre leur chronologie en silence.
* Correction des guillemets courbes, des tirets cadratin et des caractères similaires disparaissant des documents RTF, les mots environnants s'assemblant lors de la disparition.
* Correction des images RTF qui fuyaient leurs données brutes dans le document sous forme de texte brouillé.
* Correction du sous-menu Documents récents gardant les anciennes entrées jusqu'à ce que quelque chose d'autre ne le reconstruit.
* Les accélérateurs clavier sont maintenant de retour dans chaque traduction, donc les menus russes ont à nouveau accès au clavier.
* Les grands documents CHM s'ouvrent maintenant jusqu'à sept fois plus rapidement.
* Les documents ouverts sont maintenant enregistrés auprès de Windows, de sorte qu'ils apparaissent dans la liste de sauts de la barre des tâches et dans la liste récente du menu Démarrer.
* Options a été renommé en Paramètres, ce qui correspond aux applications mobiles et, sur macOS, à la convention de plate-forme.
* Paperback se souvient maintenant de la position, de la taille et de l'état maximisé de sa fenêtre entre les exécutions.
* Les formes plurielles sont maintenant traduites, de sorte que les messages qui comptent les choses se lisent correctement dans les langues qui en ont besoin de plus d'une.
* La sélection du fichier ncc.html d'un livre DAISY ouvre maintenant le livre audio complet au lieu de seulement son texte.
* Les noms d'action de la boîte de dialogue Personnaliser les raccourcis clavier peuvent maintenant être traduits.
* Le titre du document vient maintenant en premier dans la barre de titre, de sorte que les livres ouverts peuvent être distingués dans la barre des tâches et `Alt+Tab`.
* La boîte de dialogue de mise à jour est maintenant traduite.

### Version 0.9.0

#### Ajouté

##### Général
* Un outil CLI, appelé `pb`, pour convertir rapidement n'importe quel format supporté par Paperback en HTML, Markdown ou texte brut.
* Une option pour recharger les documents qui ont été modifiés par d'autres programmes sur le disque.
* Une option Afficher la source pour ouvrir la source d'un document dans un nouvel onglet, utile par exemple pour éditer du Markdown.
* Le texte du document est maintenant paginé, ce qui signifie que vous pouvez charger des livres avec des dizaines de millions de mots en seulement quelques secondes. Veuillez signaler toute bizarrerie trouvée à ce sujet.

##### Prise en charge de plateforme
* Prise en charge ARM64 de Windows !
* Prise en charge macOS native !
* Un basculement plein écran.

##### Boîte de dialogue Tous les documents
* Un bouton localiser pour localiser les livres manquants qui viennent de changer de chemin.
* Un filtre d'état et une barre d'état, afin que vous puissiez filtrer par état du document et voir combien de documents sont affichés et sélectionnés.
* Le raccourci `Ctrl+Shift+A` pour désélectionner tous les documents.

##### Options et lisibilité
* Un onglet lisibilité, avec les options suivantes :
    * Retour à la ligne automatique (déplacé depuis le général) ;
    * Rendre les tableaux en ligne (nouveau dans cette version, voir ci-dessous) ;
    * Police ;
    * Couleur de fond ;
    * Interligne ;
    * Espacement des paragraphes ;
    * Espacement des lettres ;
    * Alignement du texte.
* Un élément de menu de retour à la ligne automatique et sa touche d'accès rapide.
* Un basculement pour déterminer comment vous souhaitez que les tableaux soient affichés, et uniformiser l'affichage des tableaux dans les documents.

##### Navigation
* Prise en charge de la navigation par conteneur.
* Une option pour déplacer automatiquement le curseur au début de la ligne lors de la navigation entre les lignes, similaire au mode parcours dans les lecteurs d'écran.
* Le raccourci clavier égal pour annoncer votre pourcentage actuel dans un document.

##### Signets
* Signets temporaires : vous pouvez en avoir un par document, et ils persistent. Utilisez la barre oblique pour en définir un et la barre oblique inverse pour sauter vers lui.

##### Nombre de mots
* Temps de lecture estimé dans la boîte de dialogue du nombre de mots, ainsi que la possibilité de définir votre vitesse de lecture pour rendre cette métrique réellement utile.
* Si une sélection est active lorsque vous ouvrez la boîte de dialogue du nombre de mots, le nombre de mots sélectionnés s'affichera maintenant.

##### Raccourcis clavier
* La possibilité de personnaliser tous les raccourcis clavier de l'application via une simple boîte de dialogue.
* Un raccourci clavier configurable pour restaurer Paperback à partir du plateau système.

##### Langues
* Néerlandais, finnois et polonais.

##### Export
* Développé l'élément du menu d'export pour permettre l'export en HTML et Markdown, en plus du texte brut.

##### Mise à jour
* Un bouton Annuler à la boîte de dialogue de mise à jour en cours.
* Le programme de mise à jour valide maintenant que le fichier téléchargé n'a pas été modifié.

##### Web View
* La webview est maintenant ouverte à votre position de lecture actuelle.

##### Livres DAISY
* Prise en charge des livres DAISY 2.0.
* Prise en charge de la lecture audio DAISY 2.02.

##### Livres audio
* La possibilité de lire des livres audio, supportant actuellement à la fois l'audio DAISY (y compris l'audio DAISY + texte) et les archives de fichiers audio.
* Les raccourcis clavier et les éléments de menu pour lire/mettre en pause la narration, avancer et reculer, et ajuster le montant de la recherche.
* Options pour synchroniser le curseur de lecture à la lecture audio, définir le montant de la recherche audio et choisir si la recherche au-delà de la fin d'un chapitre continue dans le suivant.

##### Documents CHM
* Prise en charge des listes, des éléments de liste, des figures et des images.

##### PowerPoint
* Les documents PowerPoint prennent maintenant en charge les tableaux.

#### Corrigé

##### Général
* Les documents encodés dans les anciens encodages CJK, tels que GBK, Big5 et Shift_JIS, s'affichent maintenant correctement au lieu de s'afficher comme un tas de mojibake.
* « Rouvrir le dernier fermé » tentant de rouvrir le fichier readme fourni.
* Votre onglet sélectionné ne recevant pas correctement la focalisation après le redémarrage de Paperback.
* La gestion des fichiers de Paperback sur les lecteurs réseau Windows : appuyer sur afficher le fichier dans le dossier focalise maintenant correctement le fichier sur le stockage réseau, et les chemins ne contiennent plus de caractères étranges.
* Les fichiers `.paperback` ne seront plus chargés de force lors de la restauration du document ; vous serez plutôt demandé une confirmation lorsqu'un est trouvé.
* Ouvrir le dossier contenant focalise maintenant le fichier donné dans l'explorateur.
* L'ouverture du fichier readme respectera maintenant votre langue sélectionnée.
* L'interface utilisateur de Paperback se redimensionnera maintenant correctement sur les affichages haute résolution.
* Le menu se met maintenant à jour correctement et la focalisation se déplace vers le contrôle de texte lors de l'ouverture de l'aide dans Paperback.
* Passage à une méthode beaucoup plus sécurisée d'IPC sur Windows.
* Le titre du document actif sera maintenant lu lors du passage d'un onglet à l'autre.
* Réduction de l'utilisation de la mémoire sur les grands documents en réduisant de moitié la taille des tables d'index internes par caractère.

##### Boîte de dialogue Tous les documents
* Escape ne fermant pas les boîtes de dialogue Informations sur le document et Tous les documents.
* La barre de titre ne se mettant pas à jour après la fermeture d'un document à partir de la boîte de dialogue tous les documents.
* Le fichier Readme.html ne sera plus ajouté à votre liste de tous les documents lorsqu'il est ouvert via `Shift+F1`.
* La suppression de documents de la boîte de dialogue des récents fermera maintenant également leurs onglets actifs.
* Votre filtre de recherche est maintenant préservé après la suppression d'un document.

##### Navigation
* La navigation par page annonçant le texte de ligne incorrect dans certaines situations.
* Aller à la ligne, Aller à la page et Aller au pourcentage plaçant votre curseur à la mauvaise position dans les grands documents.
* Rechercher et Rechercher suivant ne respectant pas la fenêtre du document chargé dans les grands documents.

##### Signets
* Les sons des signets/notes doivent maintenant se jouer correctement uniquement lorsque vous naviguez sur un mot en contenant un.

##### Lisibilité
* L'application du retour à la ligne automatique vous envoyant au début de votre document.

##### Web View
* La boîte de dialogue webview n'étant pas redimensionnable et apparaissant à une taille initiale très petite.
* Les images doivent maintenant s'afficher correctement dans la webview intégrée.

##### Mise à jour
* Le programme de mise à jour affiche maintenant correctement le contenu des balises de code markdown dans les notes de version.

##### Livres DAISY
* Les livres DAISY affichant des informations incorrectes dans la barre d'état.
* Chargement des livres DAISY avec des déclarations d'encodage bidon.

##### Documents RTF
* Analyse des documents RTF contenant des caractères non latins.
* Les groupes RTF `\pict` de sorte que les données d'image intégrées ne fuient plus dans le texte du document.

##### Livres Mobi/AZW3
* Les ancres Filepos dans les livres Mobi divisent les balises HTML et mettent des ordures dans le texte du livre.
* Les liens dans les livres Mobi hérités.
* Analyse AZW3 grandement améliorée.

##### Documents Word
* Les documents Word avec des noms de style localisés ne rendant pas correctement leurs titres.

##### Documents HTML/XHTML
* Les éléments `dl`, `dt` et `dd` ne produisant pas de sauts de ligne dans les documents XHTML.

##### Documents PDF
* Paperback revient maintenant à l'extraction de texte brut pour les PDF mal étiquetés.
* Les documents PDF contenant des caractères de contrôle dans leurs titres et/ou signets ne feront plus planter Paperback à l'ouverture.

### Version 0.8.5
* Ajout du support des pages aux livres epub.
* Ajout du support des documents Microsoft Office chiffrés. Actuellement, Word hérité, Word moderne et PowerPoint moderne sont pris en charge, avec PowerPoint hérité prévu pour l'avenir.
* Ajout du support des documents Microsoft Word hérités !
* Ajout du support des présentations PowerPoint hérités !
* Ajout du support des livres mobi et AZW3 !
* Ajout du support des fichiers PDF étiquetés !
* Ajout du raccourci `ctrl+q` pour quitter l'application.
* Ajout du support des livres zippés de Bookshare (DAISY et Word) !
* Le texte alternatif des images intégrées doit maintenant s'afficher correctement.
* La navigation interne des documents CHM fonctionne maintenant correctement.
* Correction de aller à la page étant décalé de 1.
* Correction de la touche d'échappement ne fonctionnant pas pour fermer la boîte de dialogue Ouvrir en tant que.
* Correction du menu contextuel du lecteur ne s'affichant pas au clic droit ou à la touche Applications.
* Correction du mauvais document étant parfois focalisé lors de l'ouverture de documents à partir de la ligne de commande.
* Les PDF uniquement image sont de nouveau détectés et vous avertissent de leur existence.
* Il est maintenant possible de naviguer dans les images et les figures avec `g`/`shift+g` et `f`/`shift+f`, respectivement.
* Paperback respectera maintenant votre paramètre de mode sombre d'application.
* Suppression du support DAISY XML, car il n'est plus nécessaire.
* Retour à la navigation par première lettre Win32 native dans l'arborescence de la table des matières.
* La boîte de dialogue d'erreur de chargement affiche maintenant des messages d'erreur plus détaillés.
* La webview s'ouvrira maintenant beaucoup plus rapidement et en douceur.

### Version 0.8.2
* Ajout du support des pages aux documents RTF !
* Correction d'un bogue où l'ouverture de la webview dans les epub contenant des liens externes les activerait automatiquement.
* Correction d'un bogue où l'analyseur RTF n'ajouterait pas d'espace entre les mots dans de rares cas.
* Correction des paragraphes étant divisés en plusieurs lignes courtes dans certains documents PDF.
* Les documents PDF ont maintenant un support de navigation de lien et de titre de base !
* Les onglets et les sauts de ligne RTF sont maintenant rendus exactement comme ils apparaissent dans le document.
* Retour à la bibliothèque pdfium éprouvée pour l'analyse des PDF, ce qui rend l'analyse PDF beaucoup plus fiable à nouveau.

### Version 0.8.1
* Ajout de `Ctrl+Shift+T` pour rouvrir le dernier document fermé.
* La boîte de dialogue Tous les documents supporte maintenant la sélection de plusieurs documents à ouvrir à la fois.
* Correction de quelques bogues avec l'analyseur RTF.
* Correction des chemins de fichiers contenant des caractères non-ASCII (tels que le serbe š, č, ć, ž) devenant corrompus lors de l'ouverture d'un fichier via une deuxième instance de Paperback.
* Correction du texte PDF étant lu dans le mauvais ordre et de l'espacement incorrect autour des mots capitalisés.
* Correction du chargement lent de documents lors de l'ouverture de gros fichiers.
* Correction de la localisation des boutons Oui/Non dans les boîtes de dialogue de confirmation.

### Version 0.8.0
* Ajout des traductions en japonais, chinois simplifié et vietnamien !
* Ajout d'un programme de mise à jour automatique qui remplacera maintenant votre version actuellement installée de Paperback au lieu de simplement télécharger la nouvelle version !
* Ajout de rétroaction sonore optionnelle pour atteindre un signet ou une note, merci Andre Louis pour les sons !
* Ajout du support des documents RTF !
* Ajout du support des documents DAISY XML.
* Ajout du support des fichiers Flat Open Document Text !
* Ajout du support des présentations Flat Open Document !
* Ajout du support des séparateurs avec `s` et `shift+s`.
* Tout mouvement supérieur à 300 caractères ajoutera maintenant automatiquement à votre historique de navigation.
* Correction de la restauration de la fenêtre de Paperback à partir du plateau système.
* Correction des documents Markdown affichant le texte brut au lieu du HTML rendu dans la Web View.
* Correction des tableaux ne s'affichant pas correctement dans les fichiers Markdown.
* Les PDF uniquement image vous avertissent maintenant de leur existence lorsque vous tentez d'en charger un.
* Intégration correcte des informations de version dans l'exécutable Paperback.
* Répartition de la boîte de dialogue des options en onglets pour faciliter l'utilisation et la navigation.
* Passage à Hayro pour l'analyse des PDF, ce qui améliore la fiabilité, la vitesse et réduit les DLL.
* Réécriture complète de l'application en Rust. La nouvelle base de code est plus sûre, charge les documents plus rapidement et est plus facile à maintenir et à étendre.
* Le menu contextuel du contrôle de texte inclura maintenant des actions spécifiques au lecteur au lieu d'éléments génériques tels que couper et coller.

### Version 0.7.0
* Ajout du support des tableaux pour les documents basés sur HTML et XHTML ! Naviguez entre les tableaux en utilisant `T` et `Shift+T`, et appuyez sur Entrée pour en afficher un dans une webview.
* Ajout d'une fonctionnalité de rendu web basique ! Appuyez sur `Ctrl+Shift+V` pour ouvrir la section actuelle de votre document dans un moteur de rendu basé sur le web, utile pour du contenu comme le formatage complexe ou les exemples de code.
* Ajout d'une traduction russe, merci Ruslan Gulmagomedov !
* Ajout d'un bouton Tout effacer à la boîte de dialogue Tous les documents.
* Le vérificateur de mise à jour affiche maintenant les notes de version lorsqu'une nouvelle version est disponible.
* Correction de la restauration de la fenêtre à partir du plateau système.
* Correction de la traduction des boutons Oui/Non dans les boîtes de dialogue de confirmation.
* Correction du chargement des configs lors de l'exécution en tant qu'administrateur.
* Correction de la gestion des commentaires dans les documents XML et HTML.
* Correction de l'analyse de la table des matières dans les livres Epub 2.
* Correction de la navigation vers l'élément suivant portant la même lettre dans la table des matières.
* Correction de la boîte de dialogue de recherche ne se cachant pas correctement lors de l'utilisation des boutons suivant/précédent.
* Correction des tables des matières d'epub vous jetant occasionnellement sur le mauvais élément.
* Correction de divers problèmes de gestion des espaces dans XML, HTML et les balises pre.
* Correction de l'erreur hors par un dans la navigation de lien.
* Correction de certains livres ayant des espaces blancs de fin sur leurs lignes.
* Correction de divers problèmes d'analyseur.
* Les éléments du menu liés aux signets ainsi que la liste des éléments sont maintenant correctement désactivés lorsqu'aucun document n'est ouvert.
* Amélioration de la gestion des listes dans divers formats de document.
* Amélioration du flux de travail de traduction pour les contributeurs.
* De nombreuses refactorisations internes, déplaçant la majorité de la logique métier de l'application de C++ vers Rust pour améliorer les performances et la maintenabilité.

### Version 0.6.1
* Ajout du support des PDF protégés par mot de passe !
* Ajout d'une fonction très basique d'accès à la position précédente/suivante. Si vous appuyez sur Entrée sur un lien interne et qu'il déplace votre curseur, cette position sera maintenant mémorisée et pourra être navigué avec les flèches `alt+left`/`right`.
* Ajout d'une liste d'éléments ! Actuellement, il affiche seulement une arborescence de tous les titres de votre document ou une liste de liens, mais il y a des plans pour l'étendre à l'avenir.
* Ajout d'une option pour démarrer Paperback en mode maximisé par défaut.
* Correction des liens dans certains documents Epub ne fonctionnant pas correctement.
* Correction de l'analyse des tables des matières d'Epub contenant des chemins relatifs.
* Correction de certains documents epub n'affichant pas de titre ou d'auteur.
* Correction des titres de certains chapitres epub ne s'affichant pas correctement dans la boîte de dialogue de table des matières.
* Correction de l'impossibilité d'utiliser la barre d'espace pour activer les boutons OK/Annuler dans la boîte de dialogue de la table des matières.
* Amélioration de la gestion des titres dans les documents Word.
* Vous obtiendrez maintenant une rétroaction parlée si la liste des documents récents est vide lorsque vous essayez d'afficher la boîte de dialogue.

### Version 0.6.0
* Une nouvelle option pour afficher le menu d'accès dans une forme beaucoup plus compacte a été ajoutée à la boîte de dialogue des options, cochée par défaut.
* Ajout d'une option pour que la navigation par éléments structurels se termine en boucle.
* Ajout d'une option au menu Outils pour ouvrir le dossier contenant le document actuellement focalisé.
* Ajout d'un système de mise à jour assez simple, mais très efficace.
* Ajout d'une fonction de minuterie de sommeil basique, accessible avec `Ctrl+Shift+S`.
* Ajout du support de l'analyse des livres électroniques FB2 !
* Ajout du support de l'analyse des présentations OpenDocument !
* Ajout du support de l'analyse des fichiers OpenDocument Text !
* Les signets peuvent maintenant marquer une ligne entière ou marquer seulement du texte spécifié. Si vous n'avez pas de sélection active lors du placement d'un signet, le comportement est comme avant la version 0.6, et il marquera la ligne entière. Cependant, si vous sélectionnez du texte, seul ce texte sera inclus dans le signet.
* Les signets peuvent maintenant avoir des notes de texte facultatives attachées ! Naviguez entre les signets contenant des notes avec `N` et `Shift+N`, ou ouvrez la boîte de dialogue des signets avec tous les signets, seulement les notes ou seulement les non-notes sélectionnés avec des raccourcis spécifiques.
* Les signets dans la boîte de dialogue des signets n'auront plus de préfixe ennuyeux « signet x ».
* Les livres Epub contenant du contenu HTML prétendant être XML seront maintenant gérés correctement.
* Correction du chargement de gros documents Markdown.
* Correction de l'appui sur l'espace dans l'arborescence de la table des matières activant le bouton OK.
* Correction de la gestion des espaces blancs au début des balises pre dans les documents HTML et XHTML.
* Correction du contrôle de texte ne reprenant pas toujours la focalisation lors du retour à la fenêtre de Paperback.
* Correction du champ de texte dans la boîte de dialogue d'aller au pourcentage ne mettant pas à jour la valeur du curseur.
* Correction du rendu des ID HTML personnalisés dans les documents Markdown.
* Le HTML à l'intérieur des blocs de code Markdown sera maintenant rendu correctement.
* Si le chargement d'un livre avec un paramètre de ligne de commande prend plus de 5 secondes alors qu'une instance de Paperback existante est en cours d'exécution, vous n'obtiendrez plus d'erreur.
* Si vous exécutez Paperback en tant qu'administrateur, la configuration sera maintenant correctement chargée et sauvegardée.
* Il est maintenant possible de supprimer un signet directement depuis la boîte de dialogue des signets.
* Il est maintenant possible d'importer et d'exporter vos signets et position de lecture pour un document particulier. Le fichier généré est nommé d'après le fichier avec une extension `.paperback`. Si un tel fichier est trouvé dans le même répertoire qu'un fichier lors de son chargement, il sera automatiquement chargé. Sinon, vous pouvez les importer manuellement en utilisant un élément du menu Outils.
* Les liens à l'intérieur des documents sont maintenant entièrement pris en charge ! Utilisez `k` et `shift+k` pour vous déplacer avant et après dans les liens, et appuyez sur Entrée pour en ouvrir/activer un.
* De nombreuses refactorisations internes, rendant l'application plus rapide et le binaire plus petit.
* Le contenu Markdown est maintenant prétraité pour être conforme à CommonMark avant le rendu.
* La navigation par listes et leurs éléments est maintenant entièrement prise en charge ! Utilisez `L` et `Shift+L` pour aller par listes elles-mêmes, et `I` et `Shift+I` pour naviguer dans les éléments de liste.
* La suppression de bloc-notes fonctionne maintenant aussi pour supprimer des documents de la barre d'onglets en plus de la suppression normale.
* Paperback peut maintenant optionnellement se minimiser dans votre plateau système ! Cette option est désactivée par défaut, mais l'activer fera que l'option de minimisation dans le menu système mettra Paperback dans votre plateau, pouvant être restauré en cliquant sur l'icône créée.
* Paperback est maintenant entièrement traduisible ! La liste des langues qu'il supporte est actuellement assez petite, mais elle grandit constamment !
* Paperback a maintenant un site Web officiel, sur [paperback.dev](https://paperback.dev) !
* Les documents PPTX affichent maintenant une table des matières de base, contenant toutes les diapositives.
* Le chemin complet vers le document ouvert s'affichera maintenant dans la boîte de dialogue d'informations sur le document.
* Le programme d'installation inclut maintenant une option pour afficher le fichier readme dans votre navigateur après l'installation.
* La liste des documents récents a été considérablement développée ! Au lieu de simplement vous montrer les 10 derniers documents que vous avez ouverts, elle affichera maintenant un nombre personnalisable, avec le reste des documents que vous avez jamais ouverts étant accessible via une petite boîte de dialogue.
* Diverses petites améliorations aux analyseurs sur toute la ligne, y compris l'insertion d'une ligne vierge entre les diapositives dans les présentations PPTX, la correction de la gestion des sauts de ligne dans les paragraphes des documents Word et l'ajout de puces aux éléments de liste.

### Version 0.5.0
* Ajout du support des documents Microsoft Word !
* Ajout du support des présentations PowerPoint !
* Correction de certains éléments de menu n'étant pas désactivés sans documents ouverts.
* Correction de l'orientation du curseur aller au pourcentage.
* Correction de la table des matières dans les livres Epub avec des chemins de fichiers codés en URL et/ou des ID de fragment.
* Correction de l'espace blanc étant supprimé des titres XHTML de manière étrange.
* Correction de la gestion des espaces blancs à l'intérieur des balises pre imbriquées dans les documents HTML.
* Les documents HTML et Markdown supportent maintenant la fonction de table des matières ! Lorsque vous chargez un document HTML/Markdown, Paperback construira sa propre table des matières à partir de la structure des titres de votre document, et elle vous la montrera dans la boîte de dialogue `ctrl+t`.
* Les documents HTML auront maintenant le titre tel que défini dans la balise de titre, le cas échéant. Sinon, ils continueront à utiliser le nom de fichier sans l'extension.
* Passage de UniversalSpeech à l'utilisation d'une région dynamique pour signaler la parole. Cela signifie qu'aucune DLL de lecteur d'écran n'est expédiée avec le programme, et plus de lecteurs d'écran seront maintenant pris en charge, tels que Microsoft Narrator.
* Passage aux bibliothèques zip pour permettre l'ouverture d'un plus large éventail de livres epub.
* La boîte de dialogue vous demandant si vous souhaitez ouvrir votre document en texte brut a été complètement refaite, et elle vous permet maintenant d'ouvrir votre document en texte brut, HTML ou Markdown.
* La boîte de dialogue d'aller au pourcentage inclut maintenant un champ de texte vous permettant d'entrer manuellement un pourcentage pour sauter.
* L'analyseur HTML reconnaîtra maintenant `dd`, `dt` et `dl` comme éléments de liste.
* La table des matières dans les livres Epub sera de nouveau préservée exactement.
* L'espace insécable unicode est maintenant pris en compte lors de la suppression des lignes vierges.
* Vous ne serez plus jamais demandé comment vous souhaitez ouvrir un fichier non reconnu à chaque fois que vous le chargez, seulement la première fois.

### Version 0.4.1
* Ajout d'une icône de menu Démarrer optionnelle au programme d'installation.
* La table des matières devrait maintenant être plus propre dans quelques cas, par exemple si vous avez un élément enfant et parent avec le même texte à la même position, vous ne verrez maintenant que l'élément parent.
* Correction de la table des matières dans certains documents CHM.
* Correction de la table des matières dans les livres Epub 3 avec des chemins absolus.
* Les documents CHM devraient maintenant afficher leur titre tel que défini dans le fichier de métadonnées.

### Version 0.4.0
* Ajout du support des fichiers CHM !
* Ajout du support des signets ! Vous pouvez avoir autant de signets que vous le souhaitez dans tous les documents. Vous pouvez sauter avant et après avec `b` et `shift+b`, en définir un avec `control+shift+b`, et ouvrir une boîte de dialogue pour sauter à un signet spécifique avec `control+b`.
* Ajout d'un programme d'installation aux côtés du fichier zip portable ! Le programme d'installation installera Paperback dans votre répertoire Program Files et configurera automatiquement les associations de fichiers pour vous.
* Les fichiers texte avec les BOM devraient maintenant être décodés correctement, et le BOM ne s'affichera plus au début du texte non plus.
* Ajout de beaucoup plus d'informations à la barre d'état. Elle vous affichera maintenant votre ligne actuelle, caractère et pourcentage de lecture.
* Les commentaires HTML, ainsi que le contenu des balises script et style, ne s'affichent plus dans la sortie texte.
* Si vous transmettez un chemin relatif à Paperback sur la ligne de commande, il sera maintenant correctement résolu.
* Le mouvement en pourcentage est maintenant géré par sa propre boîte de dialogue à curseur, accessible avec `control+shift+g`.
* Les documents sans titres ou auteurs connus auront maintenant un défaut.
* La logique d'économie de position est maintenant beaucoup plus intelligente et devrait seulement écrire sur le disque si c'est absolument nécessaire.
* Le document sur lequel vous aviez focalisé lorsque vous avez fermé Paperback est maintenant mémorisé lors des redémarrages d'application.
* L'entrée dans les boîtes de dialogue aller à la ligne et aller à la page devrait maintenant être assainie plus strictement.
* Correction de la navigation de la table des matières dans les livres epub 3 avec des chemins relatifs dans leurs manifestes.

### Version 0.3.0
* Correction de la table des matières dans les livres epub avec des manifestes codés en URL.
* Correction de la navigation par titre dans les documents HTML contenant des caractères Unicode multi-octets.
* Correction de l'utilisation élevée du processeur dans les documents avec de longs titres en raison d'une régression dans wxWidgets.
* Correction du chargement des fichiers texte UTF-8.
* Correction des éléments imbriqués de la table des matières dans les livres Epub plaçant votre curseur à la mauvaise position.
* Correction d'un plantage à la sortie de l'application dans certains cas.
* Ajout d'une case à cocher dans la boîte de dialogue des options pour activer ou désactiver le retour à la ligne automatique !
* Il est maintenant possible de faire un don au développement de Paperback, soit via le nouvel élément de don du menu Aide, soit via le lien sponsor ce projet en bas de la page principale du référentiel GitHub.
* Les documents Markdown auront maintenant toujours un titre, et Paperback devrait maintenant être capable de charger pratiquement n'importe quel fichier Markdown.
* Les documents PDF auront maintenant toujours un titre, même si les métadonnées manquent.
* Passage à la bibliothèque PDF utilisée dans Chromium, ce qui conduit à une analyse PDF bien plus fiable sur toute la ligne.
* Vous ne pouvez maintenant avoir qu'une seule instance de Paperback en cours d'exécution à la fois. L'exécution de `paperback.exe` avec un nom de fichier alors qu'il est déjà en cours d'exécution ouvrira ce document dans l'instance déjà en cours d'exécution.
* Vous pouvez maintenant appuyer sur Supprimer sur un document dans le contrôle d'onglet pour le fermer.

### Version 0.2.1
* Ajout du nombre total de pages à l'étiquette de page dans la boîte de dialogue aller à la page.
* Autoriser le passage d'onglet du contenu du document à votre liste de documents ouverts.
* Correction de l'ouverture occasionnelle de documents récents par les raccourcis de titre si vous en aviez assez.
* Paperback supprimera maintenant les traits d'union facultatifs inutiles du texte de sortie.
* Correction de la navigation par titre vous plaçant parfois sur le mauvais caractère.

### Version 0.2.0
* Ajout du support des documents Markdown !
* Ajout du support des documents PDF, y compris la possibilité de naviguer entre les pages !
* Ajout de raccourcis clavier pour la navigation par titres dans le contenu HTML, y compris les livres epub et les documents Markdown. Ces raccourcis clavier ont été conçus pour fonctionner de manière similaire à un lecteur d'écran.
* Correction du chargement des epub avec des noms de fichiers codés en URL dans leurs manifestes.
* Correction du chargement des livres epub 3 avec XHTML intégré à l'intérieur.
* Un message est maintenant parlé si le document ne supporte pas une table des matières ou des sections, par opposition à la désactivation des éléments du menu.
* Ajout d'un menu de documents récents ! Il stocke actuellement vos 10 derniers documents ouverts, et appuyer sur Entrée sur l'un d'eux l'ouvrira pour la lecture.
* Réécriture complète de la boîte de dialogue Rechercher, la rendant beaucoup plus simple à utiliser, tout en ajoutant un historique de vos 25 dernières recherches et le support des expressions régulières !
* Les documents précédemment ouverts sont maintenant mémorisés lors des redémarrages d'application. Ceci est configurable via le nouvel élément options du menu Outils.
* Ajout de `shift+f1` pour ouvrir directement le fichier readme dans Paperback.

### Version 0.1.0
* Version initiale.
