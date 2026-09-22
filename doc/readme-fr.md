<!-- machine-translated from doc/readme.md (source-hash: 651d0b411879a6d8; sections: 84030068,db723a70,df2f4c18,14335443,91be3b41,6c87c514,94527a25,ce87a64f,a9eba369,e9860ee8,007c0542); please review and edit as needed -->

# Paperback - version 1.0

## Introduction

Paperback est un lecteur léger, rapide et accessible pour les livres électroniques, les documents et les livres audio, pour tout le monde, des lecteurs occasionnels aux utilisateurs avancés. Il est conçu pour l'accessibilité aux lecteurs d'écran, des vitesses rapides et une expérience sans surcharge.

## Configuration requise

Paperback fonctionne sur Windows 10/11, toutes les versions modernes d'ARM macOS, Linux, iOS 17 et ultérieur, et Android 7 et ultérieur. Les applications iOS et Android sont disponibles sur l'App Store et Google Play.

## Fonctionnalités

* Complètement autonome, ne nécessitant aucun logiciel à installer sur votre ordinateur pour commencer à lire.
* Incroyablement rapide, même sur du matériel ancien.
* Interface à onglets simple, vous permettant d'ouvrir autant de documents que vous le souhaitez côte à côte.
* Enregistre votre position de lecture exacte dans chaque document que vous ouvrez.
* Se souvient optionnellement des documents que vous aviez ouverts à la fermeture du programme, et les restaure au prochain lancement.
* Inclut une fonctionnalité de navigation similaire à celle trouvée dans le mode de navigation Web de nombreux lecteurs d'écran pour naviguer rapidement et facilement dans les documents.
* Inclut une boîte de dialogue de recherche robuste, avec des fonctionnalités telles que l'historique et le support des expressions régulières.
* Peut être exécuté entièrement de manière portable, ou installé avec les associations de fichiers configurées automatiquement.
* Prend en charge un large éventail de formats de fichiers courants.
* Lit les livres audio, avec une vitesse ajustable et des signets qui mémorisent le temps exact.
* Lit les pages PDF numérisées avec l'OCR intégré à Windows et macOS.
* Signets et notes, pour que vous puissiez marquer votre place et y revenir.
* Chaque raccourci clavier peut être modifié.
* Livré avec `pb`, un outil en ligne de commande qui convertit tout document pris en charge en HTML, Markdown ou texte brut.

## Compatibilité des lecteurs d'écran

Paperback fonctionne bien avec tous les lecteurs d'écran majeurs. Il existe cependant un problème connu pour les utilisateurs de JAWS.

### JAWS et afficheurs Braille

Si vous utilisez JAWS avec un afficheur Braille, vous pouvez constater que les longs paragraphes sont tronqués lors du panoramique vers l'avant avec les touches de navigation de votre afficheur. La commande de lecture du paragraphe actuel est également affectée. C'est un bogue dans la gestion par JAWS du contrôle de texte RICHEDIT50W, et non quelque chose dans Paperback lui-même, et celui-ci a pris un certain temps à faire surface avec l'enthousiasme de Vispero pour répondre aux problèmes des logiciels open source.

Le contournement, finalement découvert par le groupe de discussion JAWS après des mois d'attente, consiste à éditer `paperback.jcf` et à définir « Braille Presentation and Panning » sur « Always use DOM if available ». Vous voudrez également activer « Pan Text by Paragraph », sinon votre afficheur restera sur le paragraphe actif au lieu d'avancer. Avec les deux paramètres en place, le panoramique devrait fonctionner correctement.

## Types de fichiers actuellement pris en charge

Paperback prend en charge les formats et extensions suivants :

* Archives de bandes dessinées (`.cbz`)
* Fichiers d'aide CHM (`.chm`)
* Livres DAISY (`.opf`, `.zip`)
* Livres EPUB (`.epub`)
* Livres électroniques FB2 (`.fb2`)
* Documents HTML (`.htm`, `.html`, `.xhtml`)
* Pages de manuel, à la fois `man` et BSD `mdoc` (`.1` à `.9`, `.man`, `.roff`, et les formes compressées de chacun)
* Documents Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Documents Microsoft Word (`.docx`, `.docm`, `.doc`)
* Livres audio M4B (`.m4b`)
* Livres MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
* Livres audio MP3 (`.mp3`)
* Présentations OpenDocument (`.odp`, `.fodp`)
* Fichiers texte OpenDocument (`.odt`, `.fodt`)
* Documents PDF (`.pdf`)
* Présentations PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Documents RTF (`.rtf`)
* Documents Windows Write (`.wri`)
* Fichiers WinHelp (`.hlp`)
* Fichiers texte brut et journaux (`.txt`, `.log`)

## Raccourcis clavier

Paperback est conçu pour une utilisation en priorité au clavier. Voici les raccourcis actuels.

Les raccourcis ci-dessous concernent Windows. Lorsque macOS diffère, l'équivalent est indiqué entre parenthèses — principalement parce que Ctrl+G, Ctrl+W et Alt+Left/Right sont déjà utilisés par d'autres conventions système ou d'application sur cette plateforme.

### Menu Fichier

* `Ctrl+O` : Ouvrir un document.
* `Ctrl+F4` (macOS : `Cmd+W`) : Fermer le document actif.
* `Ctrl+Shift+F4` (macOS : `Cmd+Shift+W`) : Fermer tous les documents ouverts.
* `Ctrl+Shift+T` : Rouvrir le dernier document fermé.
* `Ctrl+R` : Afficher la boîte de dialogue « Tous les documents » (à partir des Documents récents).
* `Ctrl+Q` : Quitter (Windows uniquement ; sur macOS, cette option se trouve dans le menu de l'application).

### Menu Aller

* `Ctrl+F` : Afficher la boîte de dialogue Rechercher.
* `F3` (macOS : `Cmd+G`) : Rechercher le suivant.
* `Shift+F3` (macOS : `Cmd+Shift+G`) : Rechercher le précédent.
* `Ctrl+G` (macOS : `Cmd+L`) : Aller à la ligne.
* `Ctrl+Shift+G` (macOS : `Cmd+Shift+L`) : Aller au pourcentage.
* `Ctrl+P` : Aller à la page (si supporté par le document actif).
* `=` : Annoncer votre pourcentage de lecture actuel et votre page, par exemple « 15%, page 30 ». La page est omise pour les documents sans numéros de page.
* `Alt+Left` (macOS : `Cmd+[`) : Revenir en arrière dans l'historique de navigation.
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
* `Shift+M` : Formule précédente.
* `M` : Formule suivante.
* `Shift+S` : Séparateur précédent.
* `S` : Séparateur suivant.
* `Shift+L` : Liste précédente.
* `L` : Liste suivante.
* `Shift+I` : Élément de liste précédent.
* `I` : Élément de liste suivant.
* `Shift+,` : Aller au début du conteneur actif (liste ou tableau).
* `,` : Dépasser la fin du conteneur actif (liste ou tableau).

### Menu Outils

* `Ctrl+W` (macOS : `RawCtrl+W`, c'est-à-dire la touche Control physique plutôt que Cmd) : Afficher le nombre de mots du document actif.
* `Ctrl+I` : Afficher les informations du document.
* `Ctrl+T` : Afficher la table des matières.
* `F7` : Afficher la liste des éléments.
* `Ctrl+Shift+C` : Ouvrir le dossier contenant.
* `Ctrl+Shift+V` : Ouvrir le contenu actuel dans la Vue Web.
* `Ctrl+U` : Afficher la source du document dans un nouvel onglet.
* `Ctrl+Shift+E` : Exporter les données du document (`.paperback`).
* `Ctrl+Shift+I` : Importer les données du document (`.paperback`).
* `Ctrl+E` : Exporter le document actif en texte brut.
* `Ctrl+Shift+B` : Basculer le signet à la sélection/position du curseur actuelle.
* `Ctrl+Shift+N` : Ajouter ou éditer une note de signet à la sélection/position du curseur actuelle.
* `Ctrl+Alt+W` : Basculer le retour à la ligne automatique.
* `Ctrl+Space` (macOS : `RawCtrl+Space`, c'est-à-dire la touche Control physique, car Cmd+Space ouvre Spotlight) : Lecture/pause de la narration audio.
* `'` : Avancer rapidement dans la narration audio.
* `;` : Reculer rapidement dans la narration audio.
* `Shift+'` : Augmenter le montant de l'avance rapide audio.
* `Shift+;` : Diminuer le montant de l'avance rapide audio.
* `Ctrl+Shift+.` : Accélérer la narration audio.
* `Ctrl+Shift+,` : Ralentir la narration audio.
* `F11` (macOS : `RawCtrl+Ctrl+F`, c'est-à-dire Control+Command+F) : Basculer le mode plein écran.
* `Ctrl+,` : Ouvrir les Paramètres (macOS : dans le menu de l'application).
* `Ctrl+Shift+S` : Basculer la minuterie de sommeil.
* `Ctrl+Shift+O` : Reconnaître une plage de pages PDF numérisées avec OCR.
* `Alt+F9` (macOS : `Cmd+F9`) : Marquer le début d'une sélection, de sorte que tout ce qui va d'ici à votre position actuelle puisse être copié en une seule fois.
* `Alt+F10` (macOS : `Cmd+F10`) : Copier tout du début marqué de la sélection à la position actuelle.
* `Alt+Shift+F9` (macOS : `Cmd+Shift+F9`) : Revenir au début marqué de la sélection, en laissant la marque en place.

### Menu Aide

* `Ctrl+F1` : Afficher la boîte de dialogue À propos.
* `F1` : Afficher l'aide dans votre navigateur par défaut.
* `Shift+F1` : Afficher l'aide dans Paperback.
* `Ctrl+Shift+U` : Vérifier les mises à jour.
* `Ctrl+D` : Ouvrir la page de don dans votre navigateur par défaut.

### Touches supplémentaires de la vue des documents

* `Delete` / `Numpad Delete` sur le contrôle d'onglet : Fermer l'onglet de document sélectionné.
* `Enter` ou `Space` dans le texte du document : Suivre un lien ou ouvrir une vue de tableau ou de formule au curseur.
* `Enter` sur une page PDF numérisée : Reconnaître la page avec OCR.
* `Shift+F10` ou la touche Menu/Application dans le texte du document : Ouvrir le menu contextuel.

## iOS et Android

Les applications iOS et Android utilisent le même moteur de lecture que la version de bureau, elles ouvrent donc les mêmes formats et mémorisent votre position de la même façon. Elles sont conçues pour être utilisées avec VoiceOver sur iOS et TalkBack sur Android.

### Ouverture de documents

* Utilisez le bouton Ouvrir un livre, ou ouvrez un document depuis l'application Fichiers ou une autre application et choisissez Paperback.
* Sur Android, vous pouvez activer le navigateur de fichiers intégré dans les Paramètres. Il nécessite l'autorisation Accès à tous les fichiers, et ouvre les fichiers volumineux directement au lieu de les copier d'abord.
* Maintenez le bouton Ouvrir un livre enfoncé pour importer ou exporter les données d'un document (`.paperback`), les mêmes fichiers que ceux utilisés par l'application de bureau.

### Lecture et écoute

Chaque application a deux façons de lire un document. En mode texte, vous lisez le texte avec votre lecteur d'écran. En mode lecture à haute voix, Paperback lit le texte pour vous avec la voix que vous choisissez dans les Paramètres, et continue en arrière-plan et depuis l'écran verrouillé. Basculez entre les deux à partir du menu Plus d'options.

Les livres audio, comme DAISY, M4B et les livres MP3, jouent leur propre enregistrement à la place.

### La barre de lecture

La barre en bas de l'écran contient, de gauche à droite :

* L'unité de navigation, comme un paragraphe, un titre, une page ou un lien. Balayez vers le haut ou vers le bas pour la modifier.
* Les boutons précédent, lecture et suivant. Précédent et suivant se déplacent selon l'unité de navigation.
* La vitesse de lecture. Balayez vers le haut ou vers le bas pour modifier la vitesse de lecture de Paperback.

Vous pouvez également balayer vers le haut ou vers le bas sur le bouton de lecture pour vous déplacer selon l'unité de navigation, sans avoir à atteindre les boutons précédent et suivant. Si c'est tout ce que vous utilisez, le paramètre Masquer les boutons précédent et suivant les enlève du chemin de votre lecteur d'écran. Le paramètre Le balayage vers le haut avance sélectionne la direction du balayage.

### Plus d'options

Le menu Plus d'options est l'endroit où tout le reste se trouve. Certains éléments fonctionnent un peu différemment sur chaque application.

* **Passer au mode TTS ou Passer au mode Texte :** bascule entre le mode lecture à haute voix et le mode texte, décrits ci-dessus. Android a aussi un élément Lecture à haute voix qui démarre et met en pause la lecture à haute voix.
* **Table des matières :** les chapitres du livre. Choisissez-en un pour y aller directement. Sur Android, les entrées avec des chapitres sous elles peuvent être développées et réduites, en utilisant les actions du lecteur d'écran. Sur iOS, la liste entière s'affiche à la fois.
* **Éléments :** une liste des titres ou des liens du document. Basculez entre les deux avec le sélecteur Type sur iOS, ou les onglets sur Android, puis choisissez-en un pour y aller.
* **Rechercher :** tapez ce que vous cherchez, et choisissez si vous voulez respecter la casse, chercher des mots entiers seulement, ou utiliser une expression régulière. Sur Android, une barre avec Rechercher précédent et Rechercher suivant reste en bas de l'écran jusqu'à ce que vous la fermiez, et les recherches antérieures se trouvent sous Historique de recherche. Sur iOS, les boutons Rechercher précédent et Rechercher suivant se trouvent sur l'écran de recherche, et Rechercher apparaît aussi comme une unité de navigation sur la barre de lecture, pour que vous puissiez parcourir les résultats à partir de là.
* **Aller à :** sauter à une ligne, une page, ou un pourcentage du document. Choisissez lequel avec le sélecteur Mode.
* **Documents récents :** tous les documents que vous avez ouverts, chacun marqué comme actuellement ouvert, fermé, ou fichier manquant. Chacun a deux actions du lecteur d'écran : Supprimer l'enlève de la liste, et Localiser vous permet de trouver un document dont le fichier a été déplacé. Effacer les documents récents vide la liste sans supprimer aucun document.
* **Nombre de mots :** le nombre de mots dans le document.
* **Infos du document :** le titre, l'auteur, le nom du fichier, et sur iOS aussi les nombres de lignes et de caractères.
* **Exporter :** enregistre le document en texte brut, HTML ou Markdown.
* **Minuteur de veille :** arrête la lecture après 5, 10, 15, 30, 45 ou 60 minutes, ou une durée personnalisée. Ouvrez-le à nouveau pendant qu'il fonctionne pour voir combien de temps il reste, ou pour l'annuler.
* **Aide :** ouvre ce readme.
* **Paramètres :**
    * **Synthèse vocale :** la voix, la vitesse de lecture et la hauteur, un bouton Lire un exemple pour les entendre, et la pause entre les paragraphes. Android vous permet aussi de choisir le moteur de synthèse vocale. Sur iOS, c'est aussi l'endroit où se trouve le dictionnaire de synthèse vocale : des règles qui changent la façon dont les mots sont prononcés, pour chaque voix ou seulement certaines.
    * **Lisibilité :** taille du texte, interligne, espacement des paragraphes et alignement. iOS a aussi l'apparence claire et sombre, et le texte à contraste élevé.
    * **Comportement :** si rouvrir vos documents au démarrage de l'application, la direction qu'un balayage sur le bouton de lecture prend, et s'il faut masquer les boutons précédent et suivant. Android a aussi le navigateur de fichiers intégré ici.

### Claviers et casques

Avec un clavier, les raccourcis de bureau pour ouvrir des livres, les documents récents, Rechercher, Aller à, la table des matières, le nombre de mots, les infos du document, l'export et la minuterie de veille fonctionnent tous, en utilisant `Cmd` à la place de `Ctrl` sur iOS. Il en est de même pour les touches avec lettres uniques pour se déplacer par titre, page, lien, et le reste, et `Space` joue et met en pause. Sur iOS, les touches avec lettres uniques ne touchent Paperback que lorsque la Navigation rapide avec lettres uniques de VoiceOver est désactivée.

Sur Android, un bouton de casque joue et met en pause avec une pression, avance avec deux, et revient en arrière avec trois.

## Langues supportées

Paperback est traduit dans de nombreuses langues différentes, avec d'autres qui s'ajoutent continuellement. Une liste complète suit ci-dessous.

Pour apprendre comment contribuer, veuillez lire notre [Guide de traduction](translating.md).

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
* Ukrainien
* Vietnamien

## Crédits
### Développement
* Quin Gillespie : développeur principal et fondateur du projet.
* Aryan Choudhary : contributeur principal.

### Donations
Les personnes suivantes ont fait des dons de diverses montants au développement de Paperback. Si vous faites un don, votre nom ne sera pas automatiquement ajouté ici, je n'ajoute que les personnes qui souhaitent que leur don soit rendu public.

Remarque : je considère un parrain GitHub public comme un motif d'inclusion automatique dans cette liste.

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

### Version 1.0

La version 1.0 est la première version sur les cinq plateformes : Windows, macOS, Linux, iOS et Android, avec les applications iOS et Android dans l'App Store et Google Play.

#### Ajouts

##### Général
* Prise en charge de Linux, sous forme d'AppImage ou de tar.gz, avec intégration bureautique pour que les documents s'ouvrent à partir de votre gestionnaire de fichiers.
* Marquez le début d'une sélection avec `Alt+F9`, copiez tout de là où vous êtes avec `Alt+F10`, et retournez à la marque avec `Alt+Shift+F9`, pour copier une longue étendue de texte sans parcourir avec Maj. Les trois se trouvent sous Outils > Sélectionner et copier.
* Le raccourci `=` annonce maintenant la page ainsi que le pourcentage, par exemple « 15 %, page 30 », et reste tel qu'il était pour les documents sans numéros de page.
* La boîte À propos affiche maintenant la licence de Paperback et chaque traducteur.
* Une traduction ukrainienne.

##### Nouveaux formats
* Archives de bandes dessinées (`.cbz`).
* Audiolivres M4B, divisés en chapitres.
* Pages de manuel, à la fois `man` et BSD `mdoc`, compressées ou non.
* Audiolivres MP3, divisés en chapitres quand le fichier les contient.
* Fichiers Windows Write (`.wri`).
* Fichiers WinHelp (`.hlp`).
* Documents Word 6 et Word 95.

##### OCR
* Les pages PDF numérisées peuvent maintenant être reconnues avec l'OCR intégré à Windows et macOS. Appuyez sur `Entrée` sur une page numérisée pour la reconnaître, ou utilisez Batch OCR (`Ctrl+Shift+O`) pour une plage de pages.

##### Navigation
* Les formules MathML dans EPUB et HTML sont rendues en tant qu'AsciiMath en utilisant MathCAT. Utilisez `M` ou `Shift+M` pour naviguer dans les formules, puis `Entrée` ou `Espace` pour ouvrir le MathML original en Vue des formules.
* Un bouton Rechercher tout dans la boîte de dialogue Rechercher, listant chaque ligne avec une correspondance afin que vous puissiez accéder directement à celle que vous souhaitez.
* Vues Tableaux, Listes et Pages dans la liste des éléments (`F7`).
* Aller à la ligne, Aller à la page et Aller au pourcentage acceptent maintenant `+n` et `-n` pour vous déplacer par rapport à votre position actuelle.
* Les livres EPUB, MOBI et CHM sans titres propres obtiennent maintenant la navigation par titres à partir de leur table des matières.
* Les livres KF8 (AZW3) supportent maintenant la navigation par sections.
* Les pages EPUB qui ne sont qu'une image affichent maintenant une ligne pour celle-ci, afin que vous puissiez y accéder au lieu de la sauter directement.

##### Livres audio
* Contrôles de vitesse de lecture, de demi-vitesse à trois fois plus rapide. Utilisez `Ctrl+Shift+.` et `Ctrl+Shift+,`, ou le menu Outils.
* Les signets et les notes dans les livres audio seuls mémorisent maintenant l'heure exacte à laquelle vous les avez définis.
* Position suivante et précédente (`Alt+Left` et `Alt+Right`) fonctionnent maintenant dans les livres audio.
* La progression dans un livre audio est maintenant mesurée par son enregistrement, afin que Aller au pourcentage et la barre d'état correspondent à votre progression réelle.

##### Documents récents
* Un élément Effacer les documents récents dans le sous-menu Documents récents.

##### Documents PDF
* Un paramètre pour garder chaque ligne d'un PDF séparée, plutôt que de les joindre en paragraphes.
* Les images et figures dans les PDF sont maintenant annoncées.
* Les PDF qui portent une structure de lecture mais ne balisent aucune de leurs images annoncent maintenant ces images, plutôt que de les laisser entièrement en dehors du livre.

##### Vue web
* N'importe quel document peut maintenant être ouvert dans la vue web, pas seulement EPUB, HTML et Markdown.

##### Lisibilité
* Les titres sont maintenant dessinés à une taille qui correspond à leur niveau, et les images et tableaux sont séparés du texte environnant.

##### pb
* `pb --list-formats` énumère tous les formats que pb peut lire.
* pb dit maintenant quel fichier il n'a pas pu lire et pourquoi.

#### Corrigé

##### Général
* Correction d'un plantage lors de la fermeture de Paperback.
* La fermeture de Paperback masque maintenant la fenêtre immédiatement, au lieu de la laisser à l'écran pendant qu'elle enregistre.
* L'ouverture d'un document n'active plus l'option Rouvrir le dernier fermé s'il n'y a rien à rouvrir.
* Paperback n'essaie plus de réouvrir les documents manquants de votre liste récente et limite le nombre de documents récents stockés.
* L'ancien fichier de paramètres INI est maintenant supprimé une fois qu'il a été transféré au nouveau format.
* Les titres des boîtes de dialogue Police et Couleur, ainsi que le menu Exporter sous en vietnamien, sont maintenant traduits.
* La mise à jour amène maintenant la fenêtre relancée au premier plan, au lieu de la laisser derrière toutes les autres fenêtres dans Alt+Tab.
* L'habillage du texte s'applique maintenant immédiatement sur les documents volumineux, au lieu de recharger l'ensemble du document.

##### Navigation
* `Alt+Left` revient maintenant à l'endroit d'où vous avez sauté, plutôt qu'à une position plus ancienne.
* Les sons des signets ne se jouent maintenant que lorsque vous passez sur un signet, pas lorsque vous atterrissez sur la ligne où il se trouve.
* La fermeture de la table des matières, de la liste des éléments et des boîtes de dialogue Aller à vous amène maintenant directement à la ligne sur laquelle vous atterrissez, au lieu de vous faire écouter le lecteur d'écran lire à nouveau la fenêtre.
* Aller à la ligne, Aller à la page et Aller au pourcentage refusent maintenant les nombres en dehors du document au lieu d'aller discrètement ailleurs.
* NVDA ne coupe plus l'annonce lorsqu'un document n'a pas de pages.
* Appuyer sur OK dans la table des matières sans bouger va maintenant à l'entrée qui était déjà sélectionnée.
* La table des matières, la liste des éléments et la liste des signets ne ralentissent plus ni ne gèlent sur les livres avec des milliers d'entrées.
* Les flèches Haut et Bas se souviennent maintenant de leur colonne par document, au lieu de la transporter quand vous changez d'onglet.

##### Livres audio
* La lecture audio utilise maintenant `Control+Space` sur macOS, car `Command+Space` appartient à Spotlight.

##### Documents PDF
* Correction des PDF exportés depuis Apple Pages qui se lisaient en tant que texte brut, sans les titres et listes avec lesquels ils ont été écrits.
* Correction des paragraphes et titres PDF se divisant à chaque ligne, et des mots se séparant à chaque espace.
* Correction des titres PDF numérotés s'exécutant ensemble en un seul titre.
* Correction des PDF dont l'arborescence de structure ne mène à aucun texte s'ouvrant vide.
* Les en-têtes et pieds de page ne sont plus lus sur chaque page des PDF non balisés.
* Les PDF qui balisent leurs en-têtes et pieds de page en tant que texte ordinaire ne répètent plus le titre et le numéro de page entre deux paragraphes sur chaque page.
* Les PDF affichent maintenant leur vrai titre, plutôt que leur nom de fichier.
* Les lignes définies dans une police à espacement fixe, comme du code, ne sont plus jointes dans des paragraphes.

##### Livres MOBI/AZW3
* Les gros livres MOBI ne manquent plus de mémoire et ne sont plus coupés après 20 Mo.
* Les livres MOBI et AZW3 s'ouvrent maintenant beaucoup plus rapidement.
* Correction des livres MOBI perdant leur liste de chapitres.
* Correction du texte brouillé où les livres MOBI passent d'un enregistrement à l'autre.

##### Vue web
* La vue web ne charge plus l'ensemble d'un énorme livre à la fois.
* La vue web affiche maintenant les documents en entier lorsque le lecteur les affiche en entier, plutôt que seulement une tranche de ceux-ci.

##### Autres formats
* Les livres FictionBook (.fb2) écrits en windows-1251, ce qui est le cas pour la plupart d'entre eux, s'ouvrent maintenant au lieu d'échouer complètement à la lecture.
* Les livres FictionBook qui utilisent un espace de noms ou une entité HTML qu'ils n'ont jamais déclarés s'ouvrent maintenant, au lieu d'être refusés comme corrompus.
* Les livres dans les anciens encodages s'ouvrent maintenant beaucoup plus rapidement.
* Correction de certains fichiers texte en chinois qui s'ouvraient en tant que texte brouillé.
* Les fichiers OpenDocument protégés par mot de passe demandent maintenant leur mot de passe, au lieu d'être signalés comme corrompus.
* Les fichiers PowerPoint hérités protégés par mot de passe s'ouvrent maintenant, et les diapositives PowerPoint hérités ne perdent plus leur texte.
* Les fichiers texte brut enregistrés avec une extension `.rtf` s'ouvrent maintenant en tant que texte, au lieu d'échouer avec une erreur.
* Les mots-clés de contrôle RTF n'apparaissent plus en tant que texte.

#### iOS et Android

Les applications iOS et Android ouvrent tous les formats que le bureau supporte, et incluent :

* Lecture à voix haute, avec votre choix de voix, de débit et de hauteur, un contrôle de débit de parole directement dans la barre de lecture, et une pause optionnelle entre les paragraphes.
* Lecture des livres audio DAISY, M4B et MP3, qui continue en arrière-plan et depuis l'écran de verrouillage.
* Navigation par titres, pages, liens, tableaux, listes et bien d'autres à partir de la barre de lecture, ainsi que la table des matières et Rechercher.
* Une minuterie de sommeil, un compte de mots et une exportation de documents, plus un dictionnaire de parole sur iOS.
* Options de taille et d'espacement du texte, ainsi que du texte à contraste élevé sur iOS.
* Les raccourcis clavier correspondant au bureau.

### Version 0.9.2
* Les livres audio ne font plus lire à votre lecteur d'écran une suite d'espaces lorsque vous donnez le focus au champ texte.
* Les livres audio nomment maintenant le fichier au fur et à mesure que vous les parcourez par section.
* Les livres audio signalent maintenant leur durée réelle, au lieu de prétendre que chaque fichier qu'ils contiennent dure 24 heures.
* La fermeture de la Web View avec Échap n'affiche plus d'alerte de débogage après que vous ayez suivi un lien à l'intérieur.
* La copie après Sélectionner tout vous donne maintenant l'intégralité du document, au lieu de seulement la partie actuellement chargée.
* Rechercher va maintenant directement à la ligne trouvée, au lieu de vous faire écouter le lecteur d'écran relire la fenêtre à mesure que le focus revient au livre.
* Correction des EPUB contenant un bloc ZIP64 égaré qui refusaient de s'ouvrir avec « Invalid local file header ».
* Correction des documents longs qui revenaient à leur début tandis qu'un lecteur d'écran les lisait continuellement.
* Les liens dans la WebView vous mènent maintenant à la section vers laquelle ils pointent, au lieu d'échouer avec « File not found ».
* L'annonce automatique « Document reloaded » ne coupe plus votre lecteur d'écran en pleine phrase, mais attend qu'il finisse ce qu'il était en train de dire.
* L'onglet Général de la boîte de dialogue Paramètres parcourt maintenant ses options dans l'ordre où elles apparaissent à l'écran, le canal de mise à jour étant directement après l'option de vérification des mises à jour.
* Windows affichera maintenant toujours « Paperback » dans le menu Ouvrir avec, au lieu de la ligne de signature complète du programme.
* Nombre de mots et Informations sur le document affichent maintenant le nombre de fichiers qu'un livre audio contient, et sa durée totale.

### Version 0.9.1
* Les sons de signet et de note jouent maintenant sur macOS.
* Les livres DAISY jouent maintenant leur audio sur macOS, au lieu de s'ouvrir et de suivre leur chronologie en silence.
* Correction des guillemets courbes, tirets cadratins et caractères similaires qui disparaissaient des documents RTF, en joignant les mots environnants à mesure qu'ils disparaissaient.
* Correction des images RTF qui fuyaient leurs données brutes dans le document sous forme de texte garbled.
* Correction du sous-menu Documents récents qui conservait les entrées obsolètes jusqu'à ce que quelque chose d'autre le reconstruise.
* Les accélérateurs clavier sont de retour dans chaque traduction, de sorte que les menus russes ont à nouveau accès au clavier.
* Les grands documents CHM s'ouvrent maintenant jusqu'à sept fois plus rapidement.
* Les documents ouverts sont maintenant enregistrés auprès de Windows, de sorte qu'ils apparaissent dans la liste de raccourcis de la barre des tâches et la liste récente du menu Démarrer.
* Options a été renommé en Paramètres, ce qui correspond aux applications mobiles et, sur macOS, à la convention de la plateforme.
* Paperback mémorise maintenant la position, la taille et l'état maximisé de sa fenêtre entre les exécutions.
* Les formes plurielles sont maintenant traduites, de sorte que les messages qui comptent les choses se lisent correctement dans les langues qui nécessitent plus d'une forme.
* La sélection du fichier ncc.html d'un livre DAISY ouvre maintenant le livre audio complet au lieu de seulement son texte.
* Les noms d'actions de la boîte de dialogue Personnaliser les raccourcis clavier peuvent maintenant être traduits.
* Le titre du document apparaît maintenant en premier dans la barre de titre, de sorte que les livres ouverts peuvent être distingués dans la barre des tâches et Alt+Tab.
* La boîte de dialogue de mise à jour est maintenant traduite.

### Version 0.9.0

#### Ajouté

##### Général
* Un outil CLI, appelé pb, pour convertir rapidement n'importe quel format pris en charge par Paperback en HTML, Markdown ou texte brut.
* Une option pour recharger les documents qui ont été modifiés par d'autres programmes sur le disque.
* Une option Afficher la source pour ouvrir la source d'un document dans un nouvel onglet, utile par exemple pour éditer du Markdown.
* Le texte des documents est maintenant paginé, ce qui signifie que vous pouvez charger des livres contenant des dizaines de millions de mots en seulement quelques secondes. Veuillez signaler toute anomalie trouvée avec ceci.

##### Support des plates-formes
* Support ARM64 Windows !
* Support natif macOS !
* Un bouton de basculement en plein écran.

##### Boîte de dialogue Tous les documents
* Un bouton localiser pour localiser les livres manquants dont le chemin vient de changer.
* Un filtre de statut et une barre de statut, afin que vous puissiez filtrer par statut de document et voir combien de documents sont affichés et sélectionnés.
* Le raccourci `Ctrl+Shift+A` pour désélectionner tous les documents.

##### Options et lisibilité
* Un onglet lisibilité, avec les options suivantes :
    * Retour à la ligne automatique (déplacé à partir de général) ;
    * Afficher les tableaux en ligne (nouveau dans cette version, voir ci-dessous) ;
    * Police ;
    * Couleur de fond ;
    * Interligne ;
    * Espacement des paragraphes ;
    * Espacement des lettres ;
    * Alignement du texte.
* Un élément de menu retour à la ligne automatique et la touche associée.
* Un bouton pour déterminer comment vous souhaitez que les tableaux s'affichent, et unification de l'affichage des tableaux dans les documents.

##### Navigation
* Support de la navigation par conteneur.
* Une option pour déplacer automatiquement le curseur au début de la ligne lors de la navigation entre les lignes, similaire au mode de navigation dans les lecteurs d'écran.
* Le raccourci clavier égal pour annoncer votre pourcentage actuel dans un document.

##### Signets
* Signets temporaires : vous pouvez en avoir un par document, et ils persistent. Utilisez la barre oblique pour en définir un et la barre oblique inverse pour y accéder.

##### Compteur de mots
* Temps de lecture estimé dans la boîte de dialogue du compteur de mots, ainsi que la possibilité de définir votre vitesse de lecture pour rendre cette métrique vraiment utile.
* Si une sélection est active lorsque vous ouvrez la boîte de dialogue du compteur de mots, le nombre de mots que vous avez sélectionnés s'affichera maintenant.

##### Raccourcis clavier
* La possibilité de personnaliser chaque raccourci clavier de l'application via une boîte de dialogue simple.
* Un raccourci clavier configurable pour restaurer Paperback à partir de la barre d'état système.

##### Langues
* Néerlandais, finnois et polonais.

##### Exporter
* Expansion de l'élément du menu exporter pour permettre l'exportation en HTML et Markdown, en plus du texte brut.

##### Mise à jour
* Un bouton d'annulation à la boîte de dialogue de mise à jour en cours.
* Le programme de mise à jour valide maintenant que le fichier téléchargé n'a pas été modifié.

##### Affichage Web
* L'affichage web s'ouvre maintenant à votre position de lecture actuelle.

##### Livres DAISY
* Support des livres DAISY 2.0.
* Support de la lecture audio DAISY 2.02.

##### Livres audio
* La possibilité de lire des livres audio, actuellement en support à la fois pour l'audio DAISY (y compris l'audio DAISY + texte) et les zips de fichiers audio.
* Des raccourcis clavier et des éléments de menu pour lire/mettre en pause la narration, avancer et reculer, et ajuster la quantité de recherche.
* Options pour synchroniser le curseur de lecture à la lecture audio, définir la quantité de recherche audio et choisir si la recherche au-delà de la fin d'un chapitre continue dans le suivant.

##### Documents CHM
* Support des listes, éléments de liste, figures et images.

##### PowerPoint
* Les documents PowerPoint supportent maintenant les tableaux.

#### Corrigé

##### Général
* Les documents codés dans les encodages CJK hérités, tels que GBK, Big5 et Shift_JIS, s'affichent désormais correctement au lieu de s'afficher en caractères mal encodés.
* « Réouvrir le dernier fermé » tentant de rouvrir le readme fourni.
* Votre onglet sélectionné ne recevant pas correctement la mise au point après le redémarrage de Paperback.
* La gestion par Paperback des fichiers sur les lecteurs réseau Windows : appuyer sur « Afficher le fichier dans le dossier » met désormais correctement en avant le fichier sur le stockage réseau, et les chemins ne contiennent plus de caractères étranges.
* Les fichiers .paperback ne seront plus forcément chargés lors de la restauration du document ; vous serez plutôt invité à confirmer si l'un d'eux est trouvé.
* « Ouvrir le dossier contenant » met désormais l'accent sur le fichier donné dans l'explorateur.
* L'ouverture du readme respecte désormais votre langue sélectionnée.
* L'interface utilisateur de Paperback s'adaptera désormais correctement sur les écrans haute résolution.
* Le menu se met désormais à jour correctement et le focus se déplace vers le contrôle de texte lors de l'ouverture de l'aide dans Paperback.
* Passage à une méthode beaucoup plus sécurisée d'IPC sous Windows.
* Le titre du document actif sera désormais lu lors du basculement entre les onglets.
* Réduction de l'utilisation mémoire sur les grands documents en réduisant de moitié la taille des tables d'index interne par caractère.

##### Dialogue Tous les documents
* Échap ne fermant pas les dialogues Document Info et Tous les documents.
* La barre de titre ne se mettant pas à jour après la fermeture d'un document à partir du dialogue de tous les documents.
* Readme.html ne sera plus ajouté à votre liste de tous les documents lors de l'ouverture via `Shift+F1`.
* La suppression de documents du dialogue récents fermera désormais également leur onglet actif.
* Votre filtre de recherche est désormais conservé après la suppression d'un document.

##### Navigation
* La navigation de page annonçant un texte de ligne incorrect dans certaines situations.
* « Aller à la ligne », « Aller à la page » et « Aller au pourcentage » plaçant votre curseur à la mauvaise position dans les grands documents.
* « Rechercher » et « Rechercher suivant » ne respectant pas la fenêtre du document chargé dans les grands documents.

##### Signets
* Les sons des signets/notes doivent désormais être lus exclusivement lorsque vous naviguez sur un mot contenant l'un d'eux.

##### Lisibilité
* L'application du retour à la ligne vous envoyant au début de votre document.

##### Web View
* Le dialogue webview ne pouvait pas être redimensionné et s'affichait à une taille initiale très petite.
* Les images doivent désormais s'afficher correctement dans le webview intégré.

##### Mises à jour
* La mise à jour affiche désormais correctement le contenu des balises de code markdown dans les notes de version.

##### Livres DAISY
* Les livres DAISY affichant des informations incorrectes dans la barre d'état.
* Chargement de livres DAISY avec des déclarations d'encodage incorrectes.

##### Documents RTF
* Analyse des documents RTF contenant des caractères non-latins.
* Les groupes RTF `\pict` afin que les données d'image intégrées ne s'écoulent plus dans le texte du document.

##### Livres Mobi/AZW3
* Les ancrages filepos dans les livres Mobi divisant les balises HTML et mettant des données parasites dans le texte du livre.
* Les liens dans les livres Mobi hérités.
* Analyse AZW3 considérablement améliorée.

##### Documents Word
* Les documents Word avec des noms de style spécifiques aux paramètres régionaux ne rendant pas correctement leurs en-têtes.

##### Documents HTML/XHTML
* Les éléments dl, dt et dd ne produisant pas de sauts de ligne dans les documents XHTML.

##### Documents PDF
* Paperback revient désormais à l'extraction de texte brut pour les PDF mal étiquetés.
* Les documents PDF contenant des caractères de contrôle dans leurs titres et/ou signets ne bloqueront plus Paperback à l'ouverture.

### Version 0.8.5
* Ajout de la prise en charge des pages pour les livres epub.
* Ajout de la prise en charge des documents Microsoft Office chiffrés. Actuellement, les Word hérités, les Word modernes et les Powerpoint modernes sont pris en charge, avec Powerpoint hérité prévu pour l'avenir.
* Ajout de la prise en charge des documents Microsoft Word hérités !
* Ajout de la prise en charge des présentations Powerpoint hérités !
* Ajout de la prise en charge des livres mobi et AZW3 !
* Ajout de la prise en charge des fichiers PDF étiquetés !
* Ajout du raccourci `ctrl+q` pour quitter l'application.
* Ajout de la prise en charge des livres zippés de Bookshare (DAISY et Word) !
* Le texte alternatif pour les images intégrées doit désormais être correctement affiché.
* Les documents CHM prennent désormais correctement en charge la navigation par lien interne.
* Correction d'une erreur de décalage de 1 pour l'option d'accès à la page.
* Correction du fait que la touche Échap ne fonctionne pas pour fermer le dialogue Ouvrir comme.
* Correction du menu contextuel du lecteur qui ne s'affichait pas au clic droit ou à la touche Applications.
* Correction du mauvais document parfois mises au point lors de l'ouverture de documents à partir de la ligne de commande.
* Les PDF contenant uniquement des images sont à nouveau détectés et vous alertent de leur existence.
* Il est désormais possible de naviguer dans les images et les figures avec `g`/`shift+g` et `f`/`shift+f`, respectivement.
* Paperback respecte désormais votre paramètre de mode sombre de l'application.
* Suppression de la prise en charge DAISY XML, car elle n'est plus nécessaire.
* Passage à la navigation native Win32 en première lettre dans l'arborescence de la table des matières.
* Le dialogue de chargement d'erreur affiche désormais des messages d'erreur plus détaillés.
* Le webview s'ouvrira désormais beaucoup plus rapidement et en douceur.

### Version 0.8.2
* Ajout de la prise en charge des pages pour les documents RTF !
* Correction d'un bogue où l'ouverture du webview dans les épubs contenant des liens externes les activait automatiquement.
* Correction d'un bogue où l'analyseur RTF ne mettait pas d'espace entre les mots dans de rares cas.
* Correction de la division des paragraphes en plusieurs lignes courtes dans certains documents PDF.
* Les documents PDF ont désormais une prise en charge basique de la navigation par lien et en-tête !
* Les onglets RTF et les sauts de ligne sont désormais rendus exactement comme ils apparaissent dans le document.
* Retour à la bibliothèque pdfium éprouvée pour l'analyse des PDF, rendant le rendu PDF beaucoup plus fiable à nouveau.

### Version 0.8.1
* Ajout de `Ctrl+Shift+T` pour rouvrir le dernier document fermé.
* Le dialogue Tous les documents prend désormais en charge la sélection de plusieurs documents à ouvrir à la fois.
* Correction de quelques bugs du parseur RTF.
* Correction des chemins de fichiers contenant des caractères non-ASCII (comme le bosniaque š, č, ć, ž) devenant corrompus lors de l'ouverture d'un fichier via une deuxième instance de Paperback.
* Correction du texte PDF étant lu dans le mauvais ordre et d'un espacement incorrect autour des mots en majuscules.
* Correction du chargement lent des documents lors de l'ouverture de fichiers volumineux.
* Correction de la localisation des boutons Oui/Non dans les dialogues de confirmation.

### Version 0.8.0
* Ajout de traductions en japonais, chinois simplifié et vietnamien !
* Ajout d'un système de mise à jour automatique qui remplace désormais votre version actuellement installée de Paperback au lieu de simplement télécharger la nouvelle version !
* Ajout d'un retour sonore optionnel lorsque vous atteignez un signet ou une note, merci à Andre Louis pour les sons !
* Ajout de la prise en charge des documents RTF !
* Ajout de la prise en charge des documents DAISY XML.
* Ajout de la prise en charge des fichiers Open Document Text plats !
* Ajout de la prise en charge des présentations Open Document plats !
* Ajout de la prise en charge des séparateurs avec s et shift+s.
* Tout déplacement supérieur à 300 caractères sera désormais automatiquement ajouté à votre historique de navigation.
* Correction de la restauration de la fenêtre de Paperback depuis la barre d'état système.
* Correction des documents Markdown affichant du texte brut au lieu du HTML rendu dans la Web View.
* Correction du rendu inadéquat des tableaux dans les fichiers Markdown.
* Les PDF contenant uniquement des images vous avertissent désormais de leur existence lorsque vous tentez d'en charger un.
* Intégration correcte des informations de version dans l'exécutable Paperback.
* Division de la boîte de dialogue des options en onglets pour faciliter l'utilisation et la navigation.
* Passage à Hayro pour l'analyse des PDF, offrant plus de fiabilité, de vitesse et moins de DLL.
* Réécriture de l'application entière en Rust. La nouvelle base de code est plus sûre, charge les documents plus rapidement et est plus facile à maintenir et à étendre.
* Le menu contextuel du contrôle de texte inclura désormais des actions spécifiques au lecteur au lieu d'éléments génériques tels que couper et coller.

### Version 0.7.0
* Ajout de la prise en charge des tableaux pour les documents basés sur HTML et XHTML ! Naviguez entre les tableaux en utilisant T et Shift+T, et appuyez sur Entrée pour en afficher un dans une webview.
* Ajout d'une fonctionnalité de rendu web basique ! Appuyez sur Ctrl+Shift+V pour ouvrir la section actuelle de votre document dans un rendu basé sur le web, utile pour des contenus tels que des formatages complexes ou des exemples de code.
* Ajout d'une traduction russe, merci à Ruslan Gulmagomedov !
* Ajout d'un bouton Effacer tout à la boîte de dialogue Tous les documents.
* Le vérificateur de mise à jour affiche désormais les notes de version lorsqu'une nouvelle version est disponible.
* Correction de la restauration de la fenêtre depuis la barre d'état système.
* Correction des traductions des boutons Oui/Non dans les boîtes de dialogue de confirmation.
* Correction du chargement des configurations lors de l'exécution en tant qu'administrateur.
* Correction de la gestion des commentaires dans les documents XML et HTML.
* Correction de l'analyse du TOC dans les livres Epub 2.
* Correction de la navigation vers l'élément suivant avec la même lettre dans la table des matières.
* Correction de la boîte de dialogue de recherche ne se cachant pas correctement lors de l'utilisation des boutons suivant/précédent.
* Correction du TOC epub vous envoyant occasionnellement au mauvais élément.
* Correction de divers problèmes de gestion des espaces blancs dans les balises XML, HTML et pre.
* Correction d'une erreur de décalage d'une unité dans la navigation des liens.
* Correction de certains livres ayant des espaces inutiles à la fin de leurs lignes.
* Correction de divers problèmes d'analyseur.
* Les éléments de menu liés aux signets ainsi que la liste des éléments sont maintenant correctement désactivés lorsqu'aucun document n'est ouvert.
* Amélioration de la gestion des listes dans divers formats de documents.
* Amélioration du flux de traduction pour les contributeurs.
* De nombreuses refactorisations internes, déplaçant la majorité de la logique métier de l'application de C++ vers Rust pour améliorer les performances et la maintenabilité.

### Version 0.6.1
* Ajout de la prise en charge des PDF protégés par mot de passe !
* Ajout d'une fonctionnalité très basique d'accès à la position précédente/suivante. Si vous appuyez sur Entrée sur un lien interne et qu'il déplace votre curseur, cette position sera désormais mémorisée et peut être navigable avec les flèches alt+left/right.
* Ajout d'une liste d'éléments ! Actuellement, elle n'affiche qu'une arborescence de tous les en-têtes de votre document ou une liste de liens, mais il y a des plans pour l'étendre à l'avenir.
* Ajout d'une option pour démarrer Paperback en mode maximisé par défaut.
* Correction des liens dans certains documents Epub ne fonctionnant pas correctement.
* Correction de l'analyse des TOC Epub contenant des chemins relatifs.
* Correction de certains documents epub n'affichant pas de titre ou d'auteur.
* Correction des titres de certains chapitres epub ne s'affichant pas correctement dans la boîte de dialogue TOC.
* Correction de l'impossibilité d'utiliser la barre d'espace pour activer les boutons OK/Annuler dans la boîte de dialogue TOC.
* Amélioration de la gestion des en-têtes dans les documents Word.
* Vous recevrez désormais un retour parlé si la liste des documents récents est vide lorsque vous tentez d'afficher la boîte de dialogue.

### Version 0.6.0
* Une nouvelle option pour afficher le menu de navigation sous une forme bien plus compacte a été ajoutée à la boîte de dialogue des options, activée par défaut.
* Ajout d'une option pour que la navigation par éléments structurels soit cyclique.
* Ajout d'une option au menu outils pour ouvrir le dossier contenant le document actuellement actif.
* Ajout d'un système de mise à jour assez simple, mais très efficace.
* Ajout d'une fonction basique de minuteur de sommeil, accessible avec Ctrl+Shift+S.
* Ajout du support pour l'analyse des ebooks FB2 !
* Ajout du support pour l'analyse des présentations OpenDocument !
* Ajout du support pour l'analyse des fichiers OpenDocument Text !
* Les signets peuvent désormais marquer une ligne entière, ou seulement du texte spécifié. Si vous n'avez pas de sélection active lorsque vous placez un signet, le comportement est comme pré-0.6, et il marquera la ligne entière. Cependant, si vous sélectionnez du texte, seul ce texte sera inclus dans le signet.
* Les signets peuvent désormais avoir des notes de texte facultatives attachées ! Naviguez entre les signets contenant des notes avec N et Shift+N, ou ouvrez la boîte de dialogue des signets avec tous les signets, seulement les notes, ou seulement les non-notes sélectionnés avec des raccourcis clavier spécifiques.
* Les signets dans la boîte de dialogue des signets n'auront plus le préfixe ennuyeux « signet x ».
* Les livres Epub contenant du contenu HTML prétendant être du XML seront désormais traités correctement.
* Correction du chargement de grands documents Markdown.
* Correction de l'appui sur l'espace dans l'arborescence de la table des matières activant le bouton OK.
* Correction de la gestion des espaces au début des balises pre dans les documents HTML et XHTML.
* Correction du contrôle de texte ne regagnant pas le focus parfois lors du retour à la fenêtre de Paperback.
* Correction du champ de texte dans la boîte de dialogue aller au pourcentage ne mettant pas à jour la valeur du curseur.
* Correction du rendu des identifiants HTML personnalisés dans les documents Markdown.
* Le HTML dans les blocs de code Markdown sera désormais rendu correctement.
* Si vous chargez un livre avec un paramètre de ligne de commande alors qu'une instance Paperback existante est en cours d'exécution, vous n'obtiendrez plus d'erreur si le chargement de votre document prend plus de 5 secondes.
* Si vous exécutez Paperback en tant qu'administrateur, la configuration sera désormais correctement chargée et enregistrée.
* Il est désormais possible de supprimer un signet directement à partir de la boîte de dialogue des signets.
* Il est désormais possible d'importer et d'exporter vos signets et votre position de lecture pour un document particulier. Le fichier généré est nommé d'après le fichier avec une extension `.paperback`. Si un tel fichier est trouvé dans le même répertoire qu'un fichier lors de son chargement, il sera chargé automatiquement. Sinon, vous pouvez les importer manuellement à l'aide d'un élément du menu outils.
* Les liens à l'intérieur des documents sont désormais entièrement pris en charge ! Utilisez k et shift+k pour avancer et reculer à travers eux, et appuyez sur Entrée pour ouvrir/activer un lien.
* De nombreuses refactorisations internes, rendant l'application plus rapide et le binaire plus petit.
* Le contenu Markdown est désormais prétraité pour être conforme à CommonMark avant le rendu.
* La navigation par listes et leurs éléments est désormais entièrement prise en charge ! Utilisez L et Shift+L pour naviguer par les listes elles-mêmes, et I et Shift+I pour naviguer dans les éléments de liste.
* La suppression du pavé numérique fonctionne désormais pour supprimer les documents de la barre d'onglets en plus de la suppression normale.
* Paperback peut désormais optionnellement se réduire dans votre plateau système ! Cette option est désactivée par défaut, mais son activation fera en sorte que l'option de réduction dans le menu système place Paperback dans votre plateau, pouvant être restauré en cliquant sur l'icône créée.
* Paperback est désormais entièrement traduisible ! La liste des langues qu'il supporte est actuellement assez réduite, mais elle s'agrandit constamment !
* Paperback possède désormais un site officiel, à [paperback.dev](https://paperback.dev) !
* Les documents PPTX affichent désormais une table des matières basique, contenant toutes les diapositives.
* Le chemin complet vers le document ouvert sera désormais affiché dans la boîte de dialogue des informations du document.
* L'installateur inclut désormais une option pour afficher le fichier lisezmoi dans votre navigateur après l'installation.
* La liste des documents récents a été considérablement étendue ! Au lieu de simplement vous montrer les 10 derniers documents que vous avez ouverts, elle affichera désormais un nombre personnalisable, les autres documents que vous avez jamais ouverts étant accessibles via une petite boîte de dialogue.
* Diverses petites améliorations apportées aux analyseurs, notamment l'ajout d'une ligne vide entre les diapositives dans les présentations PPTX, la correction de la gestion des sauts de ligne à l'intérieur des paragraphes dans les documents Word, et l'ajout de puces aux éléments de liste.

### Version 0.5.0
* Ajout de la prise en charge des documents Microsoft Word !
* Ajout de la prise en charge des présentations PowerPoint !
* Correction de certains éléments de menu qui n'étaient pas désactivés lorsqu'aucun document n'était ouvert.
* Correction de l'orientation du curseur de pourcentage.
* Correction de la table des matières dans les livres Epub avec des chemins de fichier codés en URL et/ou des identifiants de fragment.
* Correction de l'espacement blanc supprimé des en-têtes XHTML de manière étrange.
* Correction de la gestion des espaces blancs à l'intérieur des balises pre imbriquées dans les documents HTML.
* Les documents HTML et Markdown prennent maintenant en charge la fonction table des matières ! Lorsque vous chargez un document HTML/Markdown, Paperback créera sa propre table des matières à partir de la structure des en-têtes de votre document, et la montrera dans la boîte de dialogue `ctrl+t`.
* Les documents HTML auront maintenant le titre défini dans la balise title, s'il existe. Sinon, ils continueront à utiliser le nom de fichier sans l'extension.
* Passage de UniversalSpeech à l'utilisation d'une région active pour signaler la parole. Cela signifie qu'aucune DLL de lecteur d'écran n'est plus expédiée avec le programme, et davantage de lecteurs d'écran seront pris en charge, comme Microsoft Narrator.
* Passage à des bibliothèques zip pour permettre l'ouverture d'un plus grand nombre de livres epub.
* La boîte de dialogue vous demandant si vous souhaitez ouvrir votre document en texte brut a été complètement refaite, et elle vous permet maintenant d'ouvrir votre document en texte brut, HTML ou Markdown.
* La boîte de dialogue aller au pourcentage inclut désormais un champ de texte vous permettant d'entrer manuellement un pourcentage pour aller à.
* L'analyseur HTML reconnaîtra maintenant `dd`, `dt` et `dl` comme éléments de liste.
* La table des matières des livres Epub sera de nouveau préservée exactement.
* L'espace insécable unicode est maintenant considéré lors de la suppression des lignes vides.
* Vous ne serez plus interrogé sur la façon dont vous souhaitez ouvrir un fichier non reconnu à chaque fois que vous le chargez, seulement la première fois.

### Version 0.4.1
* Ajout d'une icône facultative du menu Démarrer au programme d'installation.
* La table des matières devrait être plus claire dans quelques cas, par exemple si vous avez un élément enfant et parent avec le même texte à la même position, vous ne verrez maintenant que l'élément parent.
* Correction de la table des matières dans certains documents CHM.
* Correction de la table des matières dans les livres Epub 3 avec des chemins absolus.
* Les documents CHM devraient maintenant afficher leur titre tel qu'il est défini dans le fichier de métadonnées.

### Version 0.4.0
* Ajout de la prise en charge des fichiers CHM !
* Ajout de la prise en charge des signets ! Vous pouvez avoir autant de signets que vous le souhaitez dans autant de documents que vous le souhaitez. Vous pouvez sauter en avant et en arrière en utilisant `b` et `shift+b`, en définir un avec `control+shift+b`, et afficher une boîte de dialogue pour accéder à un signet spécifique avec `control+b`.
* Ajout d'un programme d'installation à côté du fichier zip portable ! Le programme d'installation installera Paperback dans votre répertoire Program Files et configurera automatiquement les associations de fichiers pour vous.
* Les fichiers texte avec des nomenclatures doivent maintenant être décodés correctement, et la nomenclature ne s'affichera plus au début du texte.
* Ajout de beaucoup plus d'informations à la barre d'état. Elle affichera désormais votre ligne actuelle, caractère et pourcentage de lecture.
* Les commentaires HTML, ainsi que le contenu des balises script et style, ne s'afficheront plus dans la sortie textuelle.
* Si vous transmettez un chemin relatif à Paperback sur la ligne de commande, il le résoudra correctement.
* Le mouvement en pourcentage est désormais géré par sa propre boîte de dialogue basée sur un curseur, accessible avec `control+shift+g`.
* Les documents sans titres ou auteurs connus auront désormais toujours un défaut.
* La logique de sauvegarde de position est désormais beaucoup plus intelligente et ne devrait écrire sur le disque que si c'est absolument nécessaire.
* Le document sur lequel vous vous trouviez lorsque vous avez fermé Paperback est maintenant mémorisé lors des redémarrages de l'application.
* L'entrée dans les boîtes de dialogue aller à la ligne et aller à la page devrait désormais être assainie plus strictement.
* Correction de la navigation dans la table des matières des livres epub 3 avec des chemins relatifs dans leurs manifestes.

### Version 0.3.0
* Correction de la table des matières dans les livres epub avec des manifestes codés en URL.
* Correction de la navigation par en-tête dans les documents HTML contenant des caractères Unicode multi-octets.
* Correction de la consommation élevée du CPU dans les documents avec de longs titres en raison d'une régression dans wxWidgets.
* Correction du chargement des fichiers texte UTF-8.
* Correction des éléments TOC imbriqués dans les livres Epub plaçant votre curseur à la mauvaise position.
* Correction d'un plantage à la fermeture de l'application dans certains cas.
* Ajout d'une case à cocher dans la boîte de dialogue des options pour activer ou désactiver le retour à la ligne !
* Il est maintenant possible de faire un don au développement de Paperback, soit par le nouvel élément de don dans le menu d'aide, soit par le lien de parrainage du projet en bas de la page principale du référentiel GitHub.
* Les documents Markdown auront maintenant toujours un titre, et Paperback devrait pouvoir charger pratiquement n'importe quel fichier Markdown.
* Les documents PDF auront maintenant toujours un titre, même si les métadonnées sont manquantes.
* Passage à des bibliothèques PDF utilisées dans Chromium, entraînant une analyse PDF beaucoup plus fiable dans l'ensemble.
* Vous ne pouvez maintenant avoir qu'une seule instance de Paperback en cours d'exécution à la fois. L'exécution de `paperback.exe` avec un nom de fichier alors qu'il est déjà en cours d'exécution ouvrira ce document dans l'instance déjà en cours d'exécution.
* Vous pouvez maintenant appuyer sur `delete` sur un document dans le contrôle d'onglet pour le fermer.

### Version 0.2.1
* Ajout du nombre total de pages à l'étiquette de page dans la boîte de dialogue aller à la page.
* Autoriser la tabulation du contenu du document à votre liste de documents ouverts.
* Correction des touches de navigation par en-tête ouvrant parfois des documents récents si vous en aviez assez.
* Paperback supprimera désormais les traits d'union souples inutiles de la sortie textuelle.
* Correction de la navigation par en-tête qui vous mettait parfois sur le mauvais caractère.

### Version 0.2.0
* Ajout du support des documents markdown !
* Ajout du support des documents PDF, y compris la possibilité de naviguer entre les pages !
* Ajout de raccourcis clavier pour naviguer par titres dans le contenu HTML, y compris les livres epub et les documents markdown. Ces raccourcis ont été conçus pour fonctionner de manière similaire à un lecteur d'écran.
* Correction du chargement des epub avec des noms de fichiers encodés en URL dans leurs manifestes.
* Correction du chargement des livres epub 3 avec XHTML intégré à l'intérieur.
* Un message est maintenant énoncé si le document ne supporte pas de table des matières ou de sections, au lieu que les éléments de menu soient désactivés.
* Ajout d'un menu des documents récents ! Il stocke actuellement vos 10 derniers documents ouverts, et en appuyant sur Entrée sur l'un d'eux, il s'ouvrira pour la lecture.
* Réécriture complète de la boîte de dialogue Rechercher, ce qui la rend beaucoup plus simple à utiliser, tout en ajoutant un historique de vos 25 dernières recherches et le support des expressions régulières !
* Les documents précédemment ouverts sont maintenant mémorisés lors des redémarrages de l'application. Ceci est configurable via le nouvel élément options du menu outils.
* Ajout de `Shift+F1` pour ouvrir le fichier readme directement dans Paperback.

### Version 0.1.0
* Version initiale.
