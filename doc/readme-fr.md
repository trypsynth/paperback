<!-- machine-translated from doc/readme.md (source-hash: 11f05688d690d71a; sections: f48ce952,d4d8160f,a02f4421,a4ffb7f7,91be3b41,a028b7f6,2fb18876,71df8e94,e9860ee8,a7ac6234); please review and edit as needed -->

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

* Archives de bandes dessinées (`.cbz`)
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

Paperback est conçu pour une utilisation orientée vers le clavier. Voici les raccourcis actuels.

Les raccourcis ci-dessous sont pour Windows. Lorsque macOS diffère, l'équivalent est noté entre parenthèses — principalement parce que `Ctrl+G`, `Ctrl+W` et `Alt+Left/Right` sont déjà utilisés par d'autres conventions système ou d'application sur cette plateforme.

### Menu Fichier

* `Ctrl+O` : Ouvrir un document.
* `Ctrl+F4` (macOS : `Cmd+W`) : Fermer le document actuel.
* `Ctrl+Shift+F4` (macOS : `Cmd+Shift+W`) : Fermer tous les documents ouverts.
* `Ctrl+Shift+T` : Rouvrir le dernier document fermé.
* `Ctrl+R` : Afficher la boîte de dialogue « Tous les documents » (depuis les documents récents).
* `Ctrl+Q` : Quitter (Windows uniquement ; sur macOS, c'est dans le menu de l'application).

### Menu Aller

* `Ctrl+F` : Afficher la boîte de dialogue Rechercher.
* `F3` (macOS : `Cmd+G`) : Rechercher suivant.
* `Shift+F3` (macOS : `Cmd+Shift+G`) : Rechercher précédent.
* `Ctrl+G` (macOS : `Cmd+L`) : Aller à la ligne.
* `Ctrl+Shift+G` (macOS : `Cmd+Shift+L`) : Aller au pourcentage.
* `Ctrl+P` : Aller à la page (lorsque pris en charge par le document actuel).
* `=` : Annoncer votre pourcentage de lecture actuel et votre page, par exemple « 15%, page 30 ». La page est omise pour les documents qui n'ont pas de numéros de page.
* `Alt+Left` (macOS : `Cmd+[`) : Retourner dans l'historique de navigation.
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
* `\` : Sauter à votre signet temporaire.
* `Shift+N` : Note précédente.
* `N` : Note suivante.
* `Ctrl+B` : Sauter à tous les signets et notes.
* `Ctrl+Alt+B` : Sauter aux signets uniquement.
* `Ctrl+Alt+M` : Sauter aux notes uniquement.
* `Ctrl+Shift+W` (macOS : `RawCtrl+Shift+W`, c'est-à-dire la touche Control physique plutôt que Cmd) : Afficher le texte de la note à la position actuelle.
* `Shift+K` : Lien précédent.
* `K` : Lien suivant.
* `Shift+G` : Image précédente.
* `G` : Image suivante.
* `Shift+F` : Figure précédente.
* `F` : Figure suivante.
* `Shift+T` : Tableau précédent.
* `T` : Tableau suivant.
* `Shift+M` : Formule précédente.
* `M` : Formule suivante.
* `Shift+S` : Séparateur précédent.
* `S` : Séparateur suivant.
* `Shift+L` : Liste précédente.
* `L` : Liste suivante.
* `Shift+I` : Élément de liste précédent.
* `I` : Élément de liste suivant.
* `Shift+,` : Aller au début du conteneur actuel (liste ou tableau).
* `,` : Aller au-delà de la fin du conteneur actuel (liste ou tableau).

### Menu Outils

* `Ctrl+W` (macOS : `RawCtrl+W`, c'est-à-dire la touche Control physique plutôt que Cmd) : Afficher le nombre de mots du document actuel.
* `Ctrl+I` : Afficher les informations du document.
* `Ctrl+T` : Afficher la table des matières.
* `F7` : Afficher la liste des éléments.
* `Ctrl+Shift+C` : Ouvrir le dossier contenant.
* `Ctrl+Shift+V` : Ouvrir le contenu actuel dans la vue web.
* `Ctrl+U` : Afficher la source du document dans un nouvel onglet.
* `Ctrl+Shift+E` : Exporter les données du document (`.paperback`).
* `Ctrl+Shift+I` : Importer les données du document (`.paperback`).
* `Ctrl+E` : Exporter le document actuel en texte brut.
* `Ctrl+Shift+B` : Basculer le signet à la sélection/curseur actuel.
* `Ctrl+Shift+N` : Ajouter ou modifier une note de signet à la sélection/curseur actuel.
* `Ctrl+Alt+W` : Basculer l'habillage du texte.
* `Ctrl+Space` : Lecture/pause de la narration audio.
* `'` : Avancer dans la narration audio.
* `;` : Reculer dans la narration audio.
* `Ctrl+'` : Augmenter la quantité de recherche audio.
* `Ctrl+;` : Diminuer la quantité de recherche audio.
* `F11` (macOS : `RawCtrl+Ctrl+F`, c'est-à-dire Control+Command+F) : Basculer le mode plein écran.
* `Ctrl+,` : Ouvrir les options (macOS : Préférences, dans le menu de l'application).
* `Ctrl+Shift+S` : Basculer la minuterie de sommeil.
* `Alt+F9` (macOS : `Cmd+F9`) : Marquer le début d'une sélection, afin que tout ce qui se trouve entre ce point et l'endroit où vous vous rendez puisse être copié en une seule fois.
* `Alt+F10` (macOS : `Cmd+F10`) : Copier tout ce qui va du début marqué de la sélection à la position actuelle.
* `Alt+Shift+F9` (macOS : `Cmd+Shift+F9`) : Retourner au début marqué de la sélection, en gardant la marque en place.

### Menu Aide

* `Ctrl+F1` : Afficher la boîte de dialogue À propos.
* `F1` : Afficher l'aide dans votre navigateur par défaut.
* `Shift+F1` : Afficher l'aide dans Paperback.
* `Ctrl+Shift+U` : Vérifier les mises à jour.
* `Ctrl+D` : Ouvrir la page de donation dans votre navigateur par défaut.

### Touches supplémentaires de la vue du document

* `Delete` / `Numpad Delete` sur le contrôle d'onglet : Fermer l'onglet de document sélectionné.
* `Enter` ou `Space` dans le texte du document : Suivre un lien ou ouvrir une vue de tableau ou de formule au curseur.
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
* Les livres audio ne font plus lire à votre lecteur d'écran une suite d'espaces lorsque vous focalisez le champ de texte.
* Les livres audio nomment maintenant le fichier au fur et à mesure que vous le parcourez par section.
* Les livres audio rapportent maintenant leur vraie durée, au lieu de prétendre que chaque fichier dure 24 heures.
* La fermeture de la Vue Web avec Échap ne déclenche plus d'alerte de débogage après avoir suivi un lien à l'intérieur.
* Copier après Sélectionner tout donne maintenant tout le document, au lieu de seulement la partie actuellement chargée.
* Rechercher va maintenant directement à la ligne trouvée, au lieu de vous faire subir la lecture du lecteur d'écran de la fenêtre à nouveau à mesure que le focus revient au livre.
* Correction des EPUBs portant un bloc ZIP64 errant refusant de s'ouvrir avec « En-tête de fichier local invalide ».
* Correction des documents longs revenant au début pendant qu'un lecteur d'écran les lisait continuellement.
* Les liens dans la Vue Web vous amènent maintenant à la section vers laquelle ils pointent, au lieu d'échouer avec « Fichier introuvable ».
* Marquez le début d'une sélection avec `Alt+F9`, copiez tout de là jusqu'où vous êtes arrivé avec `Alt+F10`, et retournez à la marque avec `Alt+Shift+F9`, pour copier une longue étendue de texte sans parcourir avec la touche Maj. Tous les trois sont dans Outils > Sélectionner et copier.
* Le raccourci `=` annonce maintenant aussi la page en plus du pourcentage, par exemple « 15 %, page 30 », et reste comme avant pour les documents sans numéros de page.
* L'annonce automatique « Document rechargé » n'interrompt plus votre lecteur d'écran en pleine phrase, attendant plutôt qu'il finisse ce qu'il était en train de dire.
* L'onglet Général du dialogue Paramètres parcourt maintenant ses options dans l'ordre qu'elles apparaissent à l'écran, avec le canal de mise à jour directement après l'option vérifier les mises à jour.
* La mise à jour amène maintenant la fenêtre relancée au premier plan, au lieu de la laisser derrière toutes les autres fenêtres dans Alt+Tab.
* Windows affichera maintenant toujours « Paperback » dans le menu Ouvrir avec, plutôt que le slogan complet du programme.
* Le Nombre de mots et les Infos du document montrent maintenant combien de fichiers contient un livre audio et sa durée totale.

### Version 0.9.1
* Les sons de signets et de notes jouent maintenant sur macOS.
* Les livres DAISY jouent maintenant leur audio sur macOS, au lieu de s'ouvrir et de suivre leur chronologie en silence.
* Correction des guillemets bouclés, tirets em et caractères similaires disparaissant des documents RTF, fusionnant les mots environnants.
* Correction des images RTF laissant fuir leurs données brutes dans le document sous forme de texte brouillé.
* Correction du sous-menu Documents récents conservant des entrées obsolètes jusqu'à ce que quelque chose d'autre le reconstruise.
* Les accélérateurs clavier sont de retour dans chaque traduction, le menu russe a donc à nouveau accès au clavier.
* Les grands documents CHM s'ouvrent maintenant jusqu'à sept fois plus vite.
* Les documents ouverts sont maintenant enregistrés avec Windows, ils apparaissent donc dans la liste de sauts de la barre des tâches et la liste récente du menu Démarrer.
* Options a été renommé en Paramètres, correspondant aux applications mobiles et, sur macOS, à la convention de la plateforme.
* Paperback se souvient maintenant de sa position de fenêtre, de sa taille et de son état maximisé entre les exécutions.
* Les formes plurielles sont maintenant traduites, les messages qui comptent les choses se lisent correctement dans les langues qui ont besoin de plus d'une forme.
* Sélectionner le ncc.html d'un livre DAISY ouvre maintenant le livre audio complet au lieu de juste son texte.
* Les noms d'action du dialogue Personnaliser les raccourcis clavier peuvent maintenant être traduits.
* Le titre du document apparaît maintenant en premier dans la barre de titre, les livres ouverts peuvent être distingués dans la barre des tâches et Alt+Tab.
* Le dialogue de mise à jour est maintenant traduit.

### Version 0.9.0

#### Ajouté

##### Général
* Un outil CLI, appelé pb, pour convertir rapidement l'un des formats pris en charge par Paperback en HTML, Markdown ou texte brut.
* Une option pour recharger les documents modifiés par d'autres programmes sur le disque.
* Une option Afficher la source pour ouvrir la source d'un document dans un nouvel onglet, utile pour éditer Markdown par exemple.
* Le texte du document est maintenant paginé, ce qui signifie que vous pouvez charger des livres de dizaines de millions de mots en seulement quelques secondes. Veuillez signaler toute bizarrerie trouvée avec cela.

##### Support des plateformes
* Support d'ARM64 Windows !
* Support macOS natif !
* Un bouton bascule plein écran.

##### Dialogue Tous les documents
* Un bouton localiser pour localiser les livres manquants qui viennent de changer de chemin.
* Un filtre de statut et une barre de statut, afin que vous puissiez filtrer par statut du document et voir combien de documents sont affichés et sélectionnés.
* Le raccourci `Ctrl+Shift+A` pour désélectionner tous les documents.

##### Options et lisibilité
* Un onglet lisibilité, avec les options suivantes :
    * Retour à la ligne automatique (déplacé de général) ;
    * Rendre les tableaux en ligne (nouveau dans cette version, voir ci-dessous) ;
    * Police ;
    * Couleur d'arrière-plan ;
    * Espacement des lignes ;
    * Espacement des paragraphes ;
    * Espacement des lettres ;
    * Alignement du texte.
* Un élément de menu de retour à la ligne automatique et un raccourci clavier ultérieur.
* Un bouton bascule pour déterminer comment vous voulez afficher les tableaux, et uniformiser l'affichage des tableaux dans les documents.

##### Navigation
* Les formules MathML dans EPUB et HTML sont rendues sous forme d'AsciiMath en utilisant MathCAT. Utilisez `M` ou `Shift+M` pour naviguer dans les formules, puis `Enter` ou `Space` pour ouvrir le MathML original dans la Vue de formule.
* Support pour naviguer par conteneur.
* Une option pour déplacer automatiquement le curseur au début de la ligne lors de la navigation entre les lignes, similaire au mode consultation dans les lecteurs d'écran.
* Le raccourci clavier égal pour annoncer votre pourcentage actuel dans un document.

##### Signets
* Signets temporaires : vous pouvez en avoir un par document, et ils persistent. Utilisez la barre oblique pour en définir un et la barre oblique inverse pour sauter vers lui.

##### Nombre de mots
* Temps de lecture estimé dans le dialogue de nombre de mots, ainsi que la possibilité de définir votre vitesse de lecture pour rendre cette métrique réellement utile.
* Si une sélection est active lorsque vous ouvrez le dialogue de nombre de mots, le nombre de mots sélectionnés sera maintenant affiché.

##### Raccourcis clavier
* La capacité de personnaliser chaque raccourci clavier de l'application via un dialogue simple.
* Un raccourci clavier configurable pour restaurer Paperback depuis le plateau système.

##### Langues
* Néerlandais, finnois et polonais.

##### Exportation
* L'élément du menu d'exportation a été étendu pour permettre l'exportation en HTML et Markdown, en plus du texte brut.

##### Mise à jour
* Un bouton Annuler au dialogue de mise à jour en cours.
* La mise à jour valide maintenant que le fichier téléchargé n'a pas été altéré.

##### Vue Web
* La vue web est maintenant ouverte à votre position de lecture actuelle.

##### Livres DAISY
* Support des livres DAISY 2.0.
* Support de la lecture audio DAISY 2.02.

##### Livres audio
* La capacité de lire des livres audio, supportant actuellement à la fois l'audio DAISY (y compris l'audio DAISY + texte) et les zips de fichiers audio.
* Raccourcis clavier et éléments de menu pour lire/mettre en pause la narration, chercher en avant et en arrière, et ajuster la quantité de recherche.
* Options pour synchroniser le curseur de lecture avec la lecture audio, définir la quantité de recherche audio et choisir si la recherche au-delà de la fin d'un chapitre continue au suivant.

##### Documents CHM
* Support des listes, éléments de liste, figures et images.

##### PowerPoint
* Les documents PowerPoint supportent maintenant les tableaux.

#### Correction

##### Général
* Les documents codés dans les encodages CJK hérités, tels que GBK, Big5 et Shift_JIS, s'affichent maintenant correctement au lieu d'un lot de mojibake.
* « Rouvrir le dernier fermé » tentant de rouvrir le fichier readme inclus.
* Votre onglet sélectionné ne étant pas correctement focalisé après le redémarrage de Paperback.
* La gestion des fichiers sur les lecteurs réseau Windows de Paperback : appuyer sur afficher le fichier dans le dossier focalise maintenant correctement le fichier sur le stockage réseau, et les chemins ne contiennent plus de caractères étranges.
* Les fichiers .paperback ne seront plus chargés de force lors de la restauration du document ; à la place, vous serez invité à confirmer quand on en trouve un.
* Ouvrir le dossier contenant focalise maintenant le fichier donné dans l'explorateur.
* Ouvrir le fichier readme respectera maintenant votre langue sélectionnée.
* L'interface utilisateur de Paperback s'adaptera maintenant correctement sur les écrans haute résolution.
* Le menu se met à jour maintenant correctement, et le focus se déplace vers le contrôle de texte, lors de l'ouverture de l'aide dans Paperback.
* Passage à une méthode beaucoup plus sécurisée de l'IPC sur Windows.
* Le titre du document actif sera maintenant lu lors du basculement entre les onglets.
* Utilisation réduite de la mémoire sur les grands documents en réduisant de moitié la taille des tables d'index internes par caractère.

##### Dialogue Tous les documents
* Échap ne fermant pas les dialogues Infos sur le document et Tous les documents.
* La barre de titre ne se mettant pas à jour après la fermeture d'un document à partir du dialogue tous les documents.
* Readme.html ne sera plus ajouté à votre liste de tous les documents quand ouvert via Maj+F1.
* La suppression de documents du dialogue récents fermera maintenant aussi leurs onglets actifs.
* Votre filtre de recherche est maintenant conservé après la suppression d'un document.

##### Navigation
* La navigation sur les pages annonçant un texte de ligne incorrect dans certaines situations.
* Aller à la ligne, Aller à la page et Aller au pourcentage plaçant votre curseur à la mauvaise position dans les grands documents.
* Rechercher et Rechercher suivant ne respectant pas la fenêtre du document chargé dans les grands documents.

##### Signets
* Les sons de signet/note devraient maintenant jouer correctement de manière exclusive lorsque vous naviguez sur un mot contenant un.

##### Lisibilité
* L'application du retour à la ligne automatique vous projetant au début de votre document.

##### Vue Web
* Le dialogue de la vue web n'étant pas redimensionnable et apparaissant à une taille initiale très petite.
* Les images devraient maintenant s'afficher correctement dans la vue web intégrée.

##### Mise à jour
* La mise à jour affiche maintenant correctement le contenu des balises de code markdown dans les notes de version.

##### Livres DAISY
* Les livres DAISY affichant des infos incorrectes dans la barre de statut.
* Chargement des livres DAISY avec des déclarations d'encodage bidons.

##### Documents RTF
* Analyse des documents RTF contenant des caractères non-latins.
* Les groupes RTF `\pict` afin que les données d'images intégrées ne s'écoulent plus dans le texte du document.

##### Livres Mobi/AZW3
* Les ancres filepos dans les livres Mobi divisant les balises HTML et mettant des ordures dans le texte du livre.
* Les liens dans les livres Mobi hérités.
* Analyse AZW3 considérablement améliorée.

##### Documents Word
* Les documents Word avec des noms de style spécifiques aux paramètres régionaux ne rendaient pas correctement leurs titres.

##### Documents HTML/XHTML
* Les éléments dl, dt et dd ne produisant pas de sauts de ligne dans les documents XHTML.

##### Documents PDF
* Paperback revient maintenant à l'extraction de texte brut pour les PDF mal balisés.
* Les documents PDF contenant des caractères de contrôle dans leurs titres et/ou signets ne planteront plus Paperback à l'ouverture.

### Version 0.8.5
* Ajout du support des pages aux livres epub.
* Ajout du support des documents Microsoft Office chiffrés. Actuellement, Word hérité, Word moderne et Powerpoint moderne sont pris en charge, avec Powerpoint hérité prévu pour l'avenir.
* Ajout du support des documents Microsoft Word hérités !
* Ajout du support des présentations Powerpoint hérités !
* Ajout du support des livres mobi et AZW3 !
* Ajout du support des fichiers PDF balisés !
* Ajout du raccourci ctrl+q pour quitter l'application.
* Ajout du support des livres zippés de Bookshare (DAISY et Word) !
* Le texte alt des images intégrées devrait maintenant être affiché correctement.
* Les documents CHM supportent maintenant correctement la navigation des liens internes.
* Correction de aller à la page étant décalé de 1.
* Correction de la touche Échap ne fonctionnant pas pour fermer le dialogue ouvrir en tant que.
* Correction du menu contextuel du lecteur ne s'affichant pas au clic droit ou sur la touche Applications.
* Correction du mauvais document étant parfois focalisé lors de l'ouverture de documents à partir de la ligne de commande.
* Les PDF contenant uniquement des images sont à nouveau détectés et vous avertissent de leur existence.
* Il est maintenant possible de naviguer dans les images et les figures avec g/maj+g et f/maj+f, respectivement.
* Paperback respectera maintenant votre paramètre de mode sombre d'application.
* Suppression du support XML DAISY, car il n'est plus nécessaire.
* Retour à la première navigation de lettre Win32 native dans l'arborescence de la table des matières.
* Le dialogue de chargement d'erreur affiche maintenant des messages d'erreur plus détaillés.
* La vue web s'ouvrira maintenant beaucoup plus rapidement et en douceur.

### Version 0.8.2
* Ajout du support des pages aux documents RTF !
* Correction d'un bug où l'ouverture de la vue web dans les epubs contenant des liens externes les activerait automatiquement.
* Correction d'un bug où l'analyseur RTF ne mettrait pas d'espace entre les mots dans les cas rares.
* Correction des paragraphes se divisant en plusieurs lignes courtes dans certains documents PDF.
* Les documents PDF ont maintenant un support de navigation de lien et d'en-tête basique !
* Les onglets RTF et les retours à la ligne sont maintenant rendus exactement comme ils apparaissent dans le document.
* Retour à la bibliothèque pdfium éprouvée pour l'analyse des PDF, rendant le rendu PDF beaucoup plus fiable à nouveau.

### Version 0.8.1
* Ajout de Ctrl+Maj+T pour rouvrir le dernier document fermé.
* Le dialogue Tous les documents supportent maintenant la sélection de plusieurs documents à ouvrir à la fois.
* Correction de quelques bugs avec l'analyseur RTF.
* Correction des chemins de fichiers contenant des caractères non-ASCII (comme le bosniaque š, č, ć, ž) devenant corrompus lors de l'ouverture d'un fichier via une deuxième instance de Paperback.
* Correction du texte PDF étant lu dans le mauvais ordre et d'un espacement incorrect autour des mots en majuscules.
* Correction d'un chargement lent du document lors de l'ouverture de fichiers volumineux.
* Correction de la localisation des boutons Oui/Non dans les dialogues de confirmation.

### Version 0.8.0
* Ajout de traductions en japonais, chinois simplifié et vietnamien !
* Ajout d'une mise à jour automatique qui remplacera maintenant votre version actuellement installée de Paperback au lieu de simplement télécharger la nouvelle version !
* Ajout d'une rétroaction sonore facultative pour atteindre un signet ou une note, merci à Andre Louis pour les sons !
* Ajout du support des documents RTF !
* Ajout du support des documents XML DAISY.
* Ajout du support des fichiers de texte OpenDocument aplati !
* Ajout du support des présentations OpenDocument aplaties !
* Ajout du support des séparateurs avec s et maj+s.
* Tout mouvement supérieur à 300 caractères ajoutera automatiquement à votre historique de navigation.
* Correction de la restauration de la fenêtre de Paperback depuis le plateau système.
* Correction des documents Markdown affichant le texte brut au lieu du HTML rendu dans la Vue Web.
* Correction des tableaux ne se rendant pas correctement dans les fichiers Markdown.
* Les PDF contenant uniquement des images vous avertissent maintenant de leur existence lorsque vous tentez d'en charger un.
* Intégration correcte des informations de version dans l'exécutable Paperback.
* Scission du dialogue des options en onglets pour faciliter l'utilisation et la navigation.
* Passage à Hayro pour l'analyse des PDF, conduisant à plus de fiabilité, de rapidité et de moins de DLL.
* Réécriture de l'application entière en Rust. La nouvelle base de code est plus sûre, charge les documents plus rapidement et est plus facile à maintenir et à étendre.
* Le menu contextuel du contrôle de texte incluera maintenant des actions spécifiques au lecteur au lieu d'éléments génériques tels que couper et coller.

### Version 0.7.0
* Ajout du support des tableaux pour les documents basés sur HTML et XHTML ! Naviguez entre les tableaux en utilisant T et Maj+T, et appuyez sur Entrée pour en afficher un dans une vue web.
* Ajout d'une fonctionnalité de rendu web basique ! Appuyez sur Ctrl+Maj+V pour ouvrir la section actuelle de votre document dans un moteur de rendu basé sur le web, utile pour du contenu comme le formatage complexe ou les exemples de code.
* Ajout d'une traduction russe, merci Ruslan Gulmagomedov !
* Ajout d'un bouton Effacer tout au dialogue Tous les documents.
* Le vérificateur de mise à jour affiche maintenant les notes de version quand une nouvelle version est disponible.
* Correction de la restauration de la fenêtre depuis le plateau système.
* Correction de la traduction des boutons Oui/Non dans les dialogues de confirmation.
* Correction du chargement des configurations lors de l'exécution en tant qu'administrateur.
* Correction de la gestion des commentaires dans les documents XML et HTML.
* Correction de l'analyse de la table des matières dans les livres Epub 2.
* Correction de la navigation vers l'élément suivant avec la même lettre dans la table des matières.
* Correction du dialogue de recherche ne se cachant pas correctement lors de l'utilisation des boutons suivant/précédent.
* Correction des tables des matières epub vous jetant occasionnellement au mauvais élément.
* Correction de divers problèmes de gestion des espaces blancs dans les balises XML, HTML et pre.
* Correction d'une erreur de décalage d'un dans la navigation des liens.
* Correction de certains livres ayant des espaces blancs de fin sur leurs lignes.
* Correction de divers problèmes d'analyseur.
* Les éléments du menu relatifs aux signets ainsi que la liste des éléments sont maintenant correctement désactivés quand aucun document n'est ouvert.
* Amélioration de la gestion des listes dans divers formats de document.
* Amélioration du flux de travail de traduction pour les contributeurs.
* Nombreuses refactorisations internes, déplaçant la majorité de la logique métier de l'application de C++ à Rust pour améliorer les performances et la maintenabilité.

### Version 0.6.1
* Ajout du support des PDF protégés par mot de passe !
* Ajout d'une fonctionnalité très basique d'accès à la position précédente/suivante. Si vous appuyez sur Entrée sur un lien interne et qu'il déplace votre curseur, cette position sera maintenant mémorisée, et peut être navigable avec les flèches alt+gauche/droite.
* Ajout d'une liste d'éléments ! Actuellement elle ne montre qu'une arborescence de tous les titres dans votre document ou une liste de liens, mais il y a des plans pour l'étendre dans le futur.
* Ajout d'une option pour démarrer Paperback en mode maximisé par défaut.
* Correction des liens dans certains documents Epub ne fonctionnant pas correctement.
* Correction de l'analyse des tables des matières Epub contenant des chemins relatifs.
* Correction de certains documents epub ne montrant pas de titre ou d'auteur.
* Correction des titres de certains chapitres epub ne s'affichant pas correctement dans le dialogue de la table des matières.
* Correction du fait de ne pas pouvoir utiliser la barre d'espace pour activer les boutons OK/annuler dans le dialogue de la table des matières.
* Amélioration de la gestion des titres dans les documents Word.
* Vous recevrez maintenant une rétroaction parlée si la liste des documents récents est vide quand vous essayez d'afficher le dialogue.

### Version 0.6.0
* Une nouvelle option pour afficher le menu Aller sous une forme beaucoup plus compacte a été ajoutée au dialogue des options, cochée par défaut.
* Ajout d'une option pour que la navigation par éléments structurels s'enroule.
* Ajout d'une option au menu Outils pour ouvrir le dossier contenant du document actuellement focalisé.
* Ajout d'un système de mise à jour assez simple, mais très efficace.
* Ajout d'une fonctionnalité de minuteur de sommeil basique, accessible avec Ctrl+Maj+S.
* Ajout du support de l'analyse des ebooks FB2 !
* Ajout du support de l'analyse des présentations OpenDocument !
* Ajout du support de l'analyse des fichiers de texte OpenDocument !
* Les signets peuvent maintenant être créés pour marquer une ligne entière, ou pour ne marquer que du texte spécifié. Si vous n'avez aucune sélection active lors de la création d'un signet, le comportement est comme pré-0.6, et il marquera la ligne entière. Cependant, si vous sélectionnez du texte, seul ce texte sera inclus dans le signet.
* Les signets peuvent maintenant avoir des notes de texte optionnelles attachées ! Naviguez entre les signets contenant des notes avec N et Maj+N, ou affichez le dialogue des signets avec tous les signets, uniquement les notes ou uniquement les non-notes sélectionnés avec des raccourcis spécifiques.
* Les signets dans le dialogue des signets n'auront plus de préfixe ennuyeux « signet x ».
* Les livres Epub contenant du contenu HTML prétendant être du XML sont maintenant gérés correctement.
* Correction du chargement de grands documents Markdown.
* Correction de l'appui sur l'espace dans l'arborescence de la table des matières activant le bouton OK.
* Correction de la gestion des espaces blancs au début des balises pre dans les documents HTML et XHTML.
* Correction du champ de texte ne regagnant pas le focus parfois lors du retour à la fenêtre de Paperback.
* Correction du champ de texte dans le dialogue aller au pourcentage ne mettant pas à jour la valeur du curseur.
* Correction du rendu des ID HTML personnalisés dans les documents Markdown.
* Le HTML à l'intérieur des blocs de code Markdown sera maintenant rendu correctement.
* Si le chargement d'un livre avec un paramètre de ligne de commande alors qu'une instance Paperback existante est en cours d'exécution, vous n'obtiendrez plus d'erreur si le chargement de votre document prend plus de 5 secondes.
* Si Paperback est exécuté en tant qu'administrateur, la configuration sera maintenant correctement chargée et enregistrée.
* Il est maintenant possible de supprimer un signet directement depuis le dialogue des signets.
* Il est maintenant possible d'importer et d'exporter vos signets et votre position de lecture pour un document particulier. Le fichier généré est nommé d'après le fichier avec une extension .paperback. Si un tel fichier est trouvé dans le même répertoire qu'un fichier lors de son chargement, il sera automatiquement chargé. Sinon, vous pouvez les importer manuellement en utilisant un élément dans le menu Outils.
* Les liens à l'intérieur des documents sont maintenant entièrement pris en charge ! Utilisez k et maj+k pour avancer et reculer dans les liens, et appuyez sur Entrée pour ouvrir/activer un.
* De nombreuses refactorisations internes, rendant l'application plus rapide et le binaire plus petit.
* Le contenu Markdown est maintenant prétraité pour être conforme à CommonMark avant le rendu.
* La navigation par listes et leurs éléments est maintenant entièrement prise en charge ! Utilisez L et Maj+L pour aller par les listes elles-mêmes, et I et Maj+I pour parcourir les éléments de liste.
* Supprimer sur le pavé numérique fonctionne maintenant pour supprimer des documents de la barre d'onglets en plus de la suppression normale.
* Paperback peut maintenant optionnellement être réduit à votre plateau système ! Cette option est désactivée par défaut, mais l'activation rendra l'option de réduction dans le menu système à placer Paperback dans votre plateau, pouvant être restauré en cliquant sur l'icône générée.
* Paperback est maintenant entièrement traduisible ! La liste des langues qu'il supporte est actuellement assez petite, mais elle grandit constamment !
* Paperback a maintenant un site officiel, à [paperback.dev](https://paperback.dev) !
* Les documents PPTX affichent maintenant une table des matières basique, contenant toutes les diapositives.
* Le chemin complet du document ouvert sera maintenant affiché dans le dialogue des infos sur le document.
* Le programme d'installation inclut maintenant une option pour afficher le fichier readme dans votre navigateur après l'installation.
* La liste des documents récents a été considérablement étendue ! Au lieu de simplement vous montrer les 10 derniers documents que vous avez ouverts, elle vous affichera maintenant un nombre personnalisable, les documents restants que vous avez jamais ouverts étant accessibles via un petit dialogue.
* Diverses petites améliorations des analyseurs dans tous les domaines, y compris l'insertion d'une ligne vide entre les diapositives dans les présentations PPTX, la correction de la gestion des sauts de ligne à l'intérieur des paragraphes dans les documents Word, et l'ajout de puces aux éléments de liste.

### Version 0.5.0
* Ajout du support des documents Microsoft Word !
* Ajout du support des présentations PowerPoint !
* Correction de certains éléments de menu n'étant pas désactivés sans documents ouverts.
* Correction de l'orientation du curseur du dialogue aller au pourcentage.
* Correction de la table des matières dans les livres Epub contenant des chemins de fichiers codés en URL et/ou des ID de fragments.
* Correction des espaces blancs étant supprimés des titres XHTML de manière étrange.
* Correction de la gestion des espaces blancs à l'intérieur des balises pre imbriquées dans les documents HTML.
* Les documents HTML et Markdown supportent maintenant la fonctionnalité de table des matières ! Quand vous chargez un document HTML/Markdown, Paperback construira sa propre table des matières à partir de la structure des titres dans votre document, et il vous l'affichera dans le dialogue ctrl+t.
* Les documents HTML auront maintenant le titre tel que défini dans la balise de titre, s'il existe. Sinon, ils continueront à utiliser le nom de fichier sans l'extension.
* Passage de UniversalSpeech à l'utilisation d'une région en direct pour signaler la parole. Cela signifie qu'aucune DLL de lecteur d'écran n'est fournie avec le programme, et plus de lecteurs d'écran seront maintenant pris en charge, comme Microsoft Narrator.
* Changement de bibliothèques zip pour permettre l'ouverture d'un plus grand nombre de livres epub.
* Le dialogue vous demandant si vous voulez ouvrir votre document en texte brut a été complètement refait, et il vous permet maintenant d'ouvrir votre document en texte brut, HTML ou Markdown.
* Le dialogue aller au pourcentage inclut maintenant un champ de texte vous permettant d'entrer manuellement un pourcentage pour sauter.
* L'analyseur HTML reconnaîtra maintenant dd, dt et dl comme des éléments de liste.
* La table des matières dans les livres Epub sera à nouveau préservée exactement.
* L'espace insécable unicode est maintenant pris en compte lors de la suppression des lignes vides.
* Vous ne serez plus demandé comment ouvrir un fichier non reconnu à chaque fois que vous le charger, seulement la première fois.

### Version 0.4.1
* Ajout d'une icône de menu Démarrer optionnelle au programme d'installation.
* La table des matières devrait maintenant être plus propre dans quelques cas, par exemple si vous avez un élément enfant et parent avec le même texte à la même position, vous ne verrez maintenant que l'élément parent.
* Correction de la table des matières dans certains documents CHM.
* Correction de la table des matières dans les livres Epub 3 avec des chemins absolus.
* Les documents CHM devraient maintenant afficher leur titre tel que défini dans le fichier de métadonnées.

### Version 0.4.0
* Ajout du support des fichiers CHM !
* Ajout du support des signets ! Vous pouvez avoir autant de signets que vous le souhaitez dans autant de documents que vous le souhaitez. Vous pouvez sauter avant et arrière à travers eux avec b et maj+b, en définir un avec contrôle+maj+b, et afficher un dialogue pour sauter à un signet spécifique avec contrôle+b.
* Ajout d'un programme d'installation à côté du fichier zip portable ! Le programme d'installation installera Paperback dans votre répertoire Program Files et configurera automatiquement les associations de fichiers pour vous.
* Les fichiers texte avec BOM devraient maintenant être décodés correctement, et le BOM ne sera plus affiché au début du texte non plus.
* Ajout de beaucoup plus d'informations à la barre de statut. Il affichera maintenant votre ligne actuelle, caractère et pourcentage de lecture.
* Les commentaires HTML, ainsi que le contenu des balises script et style, ne seront plus affichés dans la sortie de texte.
* Si vous transmettez un chemin relatif à Paperback sur la ligne de commande, il le résoudra maintenant correctement.
* Le mouvement en pourcentage est maintenant géré par son propre dialogue basé sur le curseur, accessible avec contrôle+maj+g.
* Les documents sans titres ou auteurs connus auront maintenant un par défaut.
* La logique de sauvegarde de position est maintenant beaucoup plus intelligente et devrait écrire sur le disque seulement en cas d'absolue nécessité.
* Le document que vous aviez focalisé quand vous avez fermé Paperback est maintenant mémorisé entre les redémarrages d'application.
* L'entrée dans les dialogues aller à la ligne et aller à la page devrait maintenant être désinfectée plus strictement.
* Correction de la table des matières dans les livres epub 3 avec des chemins relatifs dans leurs manifestes.

### Version 0.3.0
* Correction de la table des matières dans les livres epub contenant des manifestes codés en URL.
* Correction de la navigation des titres dans les documents HTML contenant des caractères Unicode multi-octets.
* Correction d'une utilisation élevée du CPU dans les documents avec de longs titres en raison d'une régression dans wxWidgets.
* Correction du chargement des fichiers texte UTF-8.
* Correction des éléments de la table des matières imbriquée dans les livres Epub plaçant votre curseur à la mauvaise position.
* Correction d'un plantage à la sortie de l'application dans certains cas.
* Ajout d'une case à cocher dans le dialogue des options pour activer ou désactiver le retour à la ligne automatique !
* Il est maintenant possible de faire un don au développement de Paperback, soit via le nouvel élément de don dans le menu Aide, soit via le lien de parrainage de ce projet en bas de la page principale du référentiel GitHub.
* Les documents Markdown auront maintenant toujours un titre, et Paperback devrait maintenant pouvoir charger pratiquement n'importe quel fichier Markdown.
* Les documents PDF auront maintenant toujours un titre, même si les métadonnées manquent.
* Passage aux bibliothèques PDF utilisées dans Chromium, conduisant à une analyse PDF beaucoup plus fiable dans tous les domaines.
* Vous ne pouvez maintenant avoir qu'une seule instance de Paperback en cours d'exécution à la fois. L'exécution de paperback.exe avec un nom de fichier alors qu'il est déjà en cours d'exécution ouvrira ce document dans l'instance déjà en cours d'exécution.
* Vous pouvez maintenant appuyer sur la suppression sur un document dans le contrôle d'onglet pour le fermer.

### Version 0.2.1
* Ajout du nombre total de pages à l'étiquette de page dans le dialogue aller à la page.
* Permettre la tabulation du contenu du document à votre liste de documents ouverts.
* Correction de quelques frappes de titre ouvrant parfois les documents récents si vous en aviez assez.
* Paperback supprimera maintenant les traits d'union souples inutiles du texte brut.
* Correction de la navigation des titres vous plaçant parfois sur le mauvais caractère.

### Version 0.2.0
* Ajout du support des documents Markdown !
* Ajout du support des documents PDF, y compris la capacité de naviguer entre les pages !
* Ajout de frappes clavier pour naviguer par les titres dans le contenu HTML, y compris les livres epub et les documents Markdown. Ces frappes ont été conçues pour fonctionner de manière similaire à un lecteur d'écran.
* Correction du chargement des epubs contenant des noms de fichier codés en URL dans leurs manifestes.
* Correction du chargement des livres epub 3 contenant du XHTML intégré à l'intérieur.
* Un message est maintenant parlé si le document ne supporte pas une table des matières ou des sections, au lieu que les éléments du menu soient désactivés.
* Ajout d'un menu des documents récents ! Il stocke actuellement vos 10 derniers documents ouverts, et appuyer sur Entrée sur un l'ouvrira pour la lecture.
* Réécriture complète du dialogue Rechercher, le rendant beaucoup plus simple à utiliser, tout en ajoutant un historique de vos 25 dernières recherches et le support des expressions régulières !
* Les documents précédemment ouverts sont maintenant mémorisés entre les redémarrages d'application. C'est configurable via le nouvel élément options dans le menu Outils.
* Ajout de maj+f1 pour ouvrir le fichier readme directement dans Paperback lui-même.

### Version 0.1.0
* Version initiale.
