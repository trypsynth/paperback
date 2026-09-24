<!-- machine-translated from doc/readme.md (source-hash: 4d3bd6acdc082011; sections: 84030068,db723a70,df2f4c18,14335443,d44bf4c8,3887c286,94527a25,ca4819ea,a9eba369,e9860ee8,fabb029c); please review and edit as needed -->

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

## Compatibilité avec les lecteurs d'écran

Paperback fonctionne bien avec tous les principaux lecteurs d'écran. Il existe cependant un problème connu pour les utilisateurs de JAWS.

### JAWS et afficheurs braille

Si vous utilisez JAWS avec un afficheur braille, vous constaterez peut-être que les longs paragraphes sont tronqués lors du défilement vers l'avant avec les touches de navigation de votre afficheur. La commande de lecture du paragraphe actuel est également affectée. Il s'agit d'un bogue dans la gestion par JAWS du contrôle de texte RICHEDIT50W, et non quelque chose dans Paperback lui-même, et c'est un bogue qui a pris un certain temps avant qu'une correction soit apportée, compte tenu de l'enthousiasme de Vispero pour répondre aux problèmes des logiciels open source.

La solution de contournement, finalement trouvée par le groupe de discussion JAWS après des mois d'attente, consiste à modifier `paperback.jcf` et à définir « Braille Presentation and Panning » sur « Always use DOM if available ». Vous voudrez également activer « Pan Text by Paragraph », sinon votre afficheur restera sur le paragraphe actif au lieu d'avancer. Avec ces deux paramètres en place, le défilement devrait fonctionner correctement.

### JAWS et les messages de Paperback

Paperback dit des choses comme « No pages. » ou « This document has no audio. » comme notifications d'accessibilité, ce qui permet à un lecteur d'écran de les énoncer par-dessus ce qu'il dit. JAWS n'agit sur celles-ci que lorsque « Enable accessible notification events » est activé pour l'application, et sur certaines machines, ce n'est pas le cas.

Si JAWS ne dit rien quand vous appuyez sur une touche qui devrait rapporter quelque chose, ouvrez Settings Center avec Paperback en avant (`Insert+6`), recherchez « notification », et cochez « Enable accessible notification events ». Cela écrit le paramètre dans `paperback.jcf`, de sorte qu'il s'applique à Paperback seul.

## Types de fichiers actuellement pris en charge

Paperback prend en charge les formats et extensions suivants :

* Archives de bandes dessinées (`.cbz`)
* Fichiers d'aide CHM (`.chm`)
* Livres DAISY (`.opf`, `.zip`)
* Livres EPUB (`.epub`)
* Livres électroniques FB2 (`.fb2`)
* Documents HTML (`.htm`, `.html`, `.xhtml`)
* Pages de manuel, à la fois `man` et BSD `mdoc` (`.1` à `.9`, `.man`, `.roff` et les versions compressées de chacun)
* Documents Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Documents Microsoft Word (`.docx`, `.docm`, `.doc`)
* Livres audio M4B (`.m4b`)
* Livres MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
* Livres audio MP3 (`.mp3`)
* Présentations OpenDocument (`.odp`, `.fodp`)
* Fichiers texte OpenDocument (`.odt`, `.fodt`)
* Documents PDF (`.pdf`)
* Présentations PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Documents reStructuredText (`.rst`, `.rest`)
* Documents RTF (`.rtf`)
* Documents Windows Write (`.wri`)
* Fichiers WinHelp (`.hlp`)
* Fichiers texte brut et fichiers journaux (`.txt`, `.log`)

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

Les applications iOS et Android utilisent le même moteur de lecture que le bureau, ils ouvrent donc les mêmes formats et se souviennent de votre place de la même manière. Elles sont conçues pour être utilisées avec VoiceOver sur iOS et TalkBack sur Android.

### Ouverture de documents

* Utilisez le bouton Open Book, ou ouvrez un document depuis l'application Files ou une autre application et choisissez Paperback.
* Sur Android, vous pouvez activer le navigateur de fichiers intégré dans les Paramètres à la place. Il a besoin de la permission d'accès à tous les fichiers, et ouvre les gros fichiers directement au lieu de les copier d'abord.
* Appuyez longuement sur le bouton Open Book pour importer ou exporter les données d'un document (`.paperback`), les mêmes fichiers que l'application de bureau utilise.

### Lecture et écoute

Chaque application a deux façons de lire un document. En mode texte, vous lisez le texte avec votre lecteur d'écran. En mode lecture à haute voix, Paperback lit le texte pour vous avec la voix que vous choisissez dans les Paramètres, et continue en arrière-plan et depuis l'écran de verrouillage. Basculez entre les deux depuis le menu Plus d'options.

Les livres audio, comme DAISY, M4B et les livres MP3, lisent leur propre enregistrement à la place.

### La barre de lecture

La barre en bas de l'écran a, de gauche à droite :

* L'unité de navigation, comme paragraphe, titre, page ou lien. Balayez vers le haut ou vers le bas pour la changer.
* Les boutons Précédent, Lecture et Suivant. Précédent et Suivant se déplacent par l'unité de navigation.
* La vitesse de parole. Balayez vers le haut ou vers le bas pour changer la vitesse de lecture de Paperback.

Vous pouvez également balayer vers le haut ou vers le bas sur le bouton de lecture pour vous déplacer par l'unité de navigation, sans avoir besoin d'atteindre les boutons Précédent et Suivant. Si c'est tout ce que vous utilisez, le paramètre Masquer les boutons Précédent et Suivant les retire du chemin de votre lecteur d'écran. Le paramètre Balayage vers le haut avance choisit la direction dans laquelle va un balayage.

### Plus d'options

Le menu Plus d'options est l'endroit où tout le reste se trouve. Certains éléments fonctionnent un peu différemment sur chaque application.

* **Passer au mode TTS ou Passer au mode Texte :** bascule entre le mode lecture à haute voix et le mode texte, décrit ci-dessus. En mode texte, un élément Lire à haute voix lance et met en pause la lecture à haute voix sans quitter le mode texte.
* **Table des matières :** les chapitres du livre, ouverts à celui que vous lisez. Choisissez-en un pour y aller directement. Les entrées ayant des chapitres sous elles peuvent être développées et réduites avec les actions du lecteur d'écran.
* **Éléments :** une liste des titres ou des liens du document. Basculez entre les deux avec le sélecteur Type sur iOS, ou les onglets sur Android, puis choisissez-en un pour y aller.
* **Rechercher :** tapez ce que vous cherchez, ou choisissez une recherche antérieure dans l'Historique de recherche, et choisissez s'il faut respecter la casse, faire correspondre des mots entiers seulement, ou utiliser une expression régulière. Rechercher le précédent et Rechercher le suivant sautent à une correspondance et indiquent où c'est tombé, et Rechercher reste ouvert pour que vous puissiez continuer. En mode lecture à haute voix, Rechercher s'affiche également comme une unité de navigation sur la barre de lecture, ce qui vous permet de parcourir les correspondances à partir de là aussi.
* **Aller à :** sautez à une ligne, une page, ou un pourcentage du document. Choisissez lequel avec le sélecteur Mode.
* **Documents récents :** tous les documents que vous avez ouverts, chacun marqué comme actuellement ouvert, fermé ou fichier manquant. Chacun a deux actions du lecteur d'écran : Supprimer l'enlève de la liste, et Localiser vous permet de trouver un document dont le fichier a été déplacé. Effacer les documents récents vide la liste sans supprimer de documents.
* **Nombre de mots :** le nombre de mots du document.
* **Informations sur le document :** le titre, l'auteur, le nom du fichier, et sur iOS aussi le nombre de lignes et de caractères.
* **Exporter :** enregistre le document en texte brut, HTML ou Markdown.
* **Minuteur de sommeil :** arrête la lecture après 5, 10, 15, 30, 45 ou 60 minutes, ou une durée de votre choix. Ouvrez-le à nouveau pendant qu'il fonctionne pour voir combien de temps il reste, ou pour l'annuler.
* **Aide :** ouvre ce fichier readme.
* **Paramètres :**
    * **Synthèse vocale :** la voix, la vitesse de parole et la hauteur, un bouton Lire un exemple pour les entendre, et la pause entre les paragraphes. Android vous permet également de choisir le moteur de synthèse vocale. Sur iOS, c'est aussi là que se trouve le dictionnaire de parole : des règles qui modifient la façon dont les mots sont prononcés, pour toutes les voix ou juste certaines.
    * **Lisibilité :** taille du texte, interligne, espacement des paragraphes, alignement et texte à contraste élevé. iOS a aussi une apparence claire et sombre.
    * **Comportement :** s'il faut rouvrir vos documents au démarrage de l'application, la direction dans laquelle un balayage sur le bouton de lecture se déplace, et s'il faut masquer les boutons Précédent et Suivant. Android a aussi le navigateur de fichiers intégré ici.

### Claviers et casques

Avec un clavier, les raccourcis de bureau pour ouvrir des livres, les documents récents, Rechercher, Aller à, la table des matières, le nombre de mots, les informations sur le document, l'exportation et la minuterie de sommeil fonctionnent tous, en utilisant `Cmd` à la place de `Ctrl` sur iOS. Tout comme les touches à une seule lettre pour se déplacer par titre, page, lien et le reste, et `Space` pour jouer et mettre en pause. Sur iOS, les touches à une seule lettre ne touchent Paperback que lorsque la navigation rapide à une lettre VoiceOver est désactivée.

Sur Android, un bouton de casque joue et met en pause avec une pression, avance avec deux, et revient avec trois.

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

## Historique des versions

### Version 1.0

1.0 est la première version sur les cinq plates-formes : Windows, macOS, Linux, iOS et Android, avec les applications iOS et Android disponibles sur l'App Store et Google Play.

#### Ajouts

##### Général
* Support de Linux, en tant qu'AppImage ou tar.gz, avec intégration au bureau pour que les documents s'ouvrent depuis votre gestionnaire de fichiers.
* Marquez le début d'une sélection avec `Alt+F9`, copiez tout de là jusqu'où vous êtes parvenu avec `Alt+F10`, et revenez à la marque avec `Alt+Shift+F9`, pour copier un long passage de texte sans faire défiler avec maj enfoncée. Les trois sont sous Outils > Sélectionner et copier.
* Le raccourci `=` annonce maintenant la page ainsi que le pourcentage, par exemple « 15 %, page 30 », et reste tel qu'il était pour les documents sans numéros de page.
* La boîte À propos affiche maintenant la licence de Paperback et chaque traducteur.
* Une traduction ukrainienne.

##### Nouveaux formats
* Archives de bandes dessinées (`.cbz`).
* Audiolivres M4B, divisés en leurs chapitres.
* Pages de manuel, à la fois `man` et BSD `mdoc`, compressés ou non.
* Audiolivres MP3, divisés en chapitres lorsque le fichier en contient.
* Documents reStructuredText.
* Fichiers Windows Write (`.wri`).
* Fichiers WinHelp (`.hlp`).
* Documents Word 6 et Word 95.

##### OCR
* Les pages PDF numérisées peuvent maintenant être reconnues avec l'OCR intégré à Windows et macOS. Appuyez sur `Enter` sur une page numérisée pour la reconnaître, ou utilisez l'OCR par lot (`Ctrl+Shift+O`) pour une plage de pages.

##### Navigation
* Les formules MathML dans EPUB et HTML sont rendues en AsciiMath en utilisant MathCAT. Utilisez `M` ou `Shift+M` pour naviguer dans les formules, puis `Enter` ou `Space` pour ouvrir le MathML original dans la vue Formule.
* Un bouton Trouver tout dans la boîte de dialogue Rechercher, listant chaque ligne avec une correspondance pour que vous puissiez accéder directement à celle que vous voulez.
* Les vues Tableaux, Listes et Pages dans la liste des éléments (`F7`).
* Aller à la ligne, Aller à la page et Aller au pourcentage acceptent maintenant `+n` et `-n` pour vous déplacer relativement à votre position actuelle.
* Les livres EPUB, MOBI et CHM sans titres propres obtiennent maintenant la navigation par titres de leur table des matières.
* Les livres KF8 (AZW3) supportent maintenant la navigation par section.
* Les pages EPUB qui ne contiennent que une image affichent maintenant une ligne pour celle-ci, vous pouvez donc y accéder au lieu de les sauter.

##### Audiolivres
* Contrôles de la vitesse de lecture, de la demi-vitesse à trois fois plus vite. Utilisez `Ctrl+Shift+.` et `Ctrl+Shift+,`, ou le menu Outils.
* Les signets et notes dans les livres audio uniquement mémorisent maintenant le moment exact où vous les avez définis.
* Position suivante et précédente (`Alt+Left` et `Alt+Right`) fonctionnent maintenant dans les audiolivres.
* La progression dans un audiolivre est maintenant mesurée par son enregistrement, donc Aller au pourcentage et la barre d'état correspondent à votre progression réelle.

##### Documents récents
* Un élément Effacer les documents récents dans le sous-menu Documents récents.

##### Documents PDF
* Un paramètre pour conserver chaque ligne d'un PDF séparée, plutôt que de les joindre en paragraphes.
* Les images et figures dans les PDF sont maintenant annoncées.
* Les PDF qui portent une structure de lecture mais ne balisent aucune de leurs images annoncent maintenant ces images, plutôt que de les laisser de côté du livre.

##### Vue Web
* N'importe quel document peut maintenant être ouvert dans la vue web, pas seulement EPUB, HTML et Markdown.

##### Lisibilité
* Les titres sont maintenant dessinés à une taille qui correspond à leur niveau, et les images et tableaux sont séparés du texte environnant.

##### pb
* `pb --list-formats` liste tous les formats que pb peut lire.
* pb indique maintenant quel fichier il n'a pas pu lire, et pourquoi.

#### Correctifs

##### Général
* Correction d'un plantage lors de la fermeture de Paperback.
* La fermeture de Paperback masque désormais immédiatement la fenêtre, au lieu de la laisser à l'écran pendant qu'il enregistre.
* L'ouverture d'un document n'active plus l'option Rouvrir le dernier fermé s'il n'y a rien à rouvrir.
* Paperback n'essaie plus de charger les documents disparus de votre liste récente, et limite le nombre de documents récents qu'il conserve.
* L'ancien fichier de paramètres INI est désormais supprimé après sa migration au nouveau format.
* Les titres des dialogues de police et de couleur, et le menu Exporter sous en vietnamien, sont maintenant traduits.
* La mise à jour apporte désormais la fenêtre relancée au premier plan, au lieu de la laisser derrière tous les autres dans Alt+Tab.
* Le retour à la ligne s'applique désormais immédiatement sur les gros documents, au lieu de recharger tout le contenu.

##### Navigation
* `Alt+Left` revient désormais à l'endroit d'où vous avez sauté, plutôt qu'à une position antérieure.
* Les sons des signets ne se jouent désormais que lorsque vous vous déplacez sur un signet, pas lorsque vous vous posez sur la ligne où il se trouve.
* La fermeture de la table des matières, de la liste des éléments et des dialogues Aller à vous amène désormais directement à la ligne où vous vous posez, au lieu de vous faire écouter le lecteur d'écran relire la fenêtre.
* Aller à la ligne, Aller à la page et Aller au pourcentage refusent désormais les nombres en dehors du document au lieu d'aller silencieusement ailleurs.
* NVDA ne coupe plus l'annonce lorsqu'un document n'a pas de pages.
* Appuyer sur OK dans la table des matières sans se déplacer va désormais à l'entrée déjà sélectionnée.
* La table des matières, la liste des éléments et la liste des signets ne se figent plus ou ne ralentissent plus sur les livres avec des milliers d'entrées.
* Les flèches Haut et Bas se souviennent désormais de leur colonne par document, au lieu de la conserver lorsque vous changez d'onglet.

##### Audiobooks
* La lecture audio utilise désormais `Control+Space` sur macOS, car `Command+Space` appartient à Spotlight.

##### Documents PDF
* Correction des PDF exportés depuis Apple Pages se lisant comme du texte brut, sans aucune des en-têtes et listes avec lesquels ils ont été écrits.
* Correction des paragraphes et en-têtes PDF se divisant à chaque ligne, et des mots se divisant aux espaces.
* Correction des en-têtes PDF numérotés se fusionnant en un seul en-tête.
* Correction des PDF dont l'arborescence de structure ne mène à aucun texte et s'ouvrent vides.
* Les en-têtes et pieds de page ne sont plus lus sur chaque page des PDF non marqués.
* Les PDF qui marquent leurs en-têtes et pieds de page comme du texte ordinaire ne répètent plus le titre et le numéro de page entre deux paragraphes sur chaque page.
* Les PDF affichent désormais leur vrai titre, plutôt que leur nom de fichier.
* Les lignes définies dans une police à largeur fixe, comme le code, ne sont plus jointes dans les paragraphes.

##### Livres MOBI/AZW3
* Les gros livres MOBI ne manquent plus de mémoire et ne sont plus coupés après 20 MB.
* Les livres MOBI et AZW3 s'ouvrent désormais beaucoup plus rapidement.
* Correction des livres MOBI perdant leur liste de chapitres.
* Correction du texte garbled où les livres MOBI passent d'un enregistrement à l'autre.

##### Web View
* Le web view ne charge plus la totalité d'un énorme livre à la fois.
* Le web view affiche désormais les documents en entier lorsque le lecteur les affiche en entier, plutôt que seulement une tranche d'entre eux.

##### Autres formats
* Les livres FictionBook (.fb2) écrits en windows-1251, qui en sont la plupart, s'ouvrent désormais au lieu d'échouer complètement à la lecture.
* Les livres FictionBook qui utilisent un espace de noms ou une entité HTML qu'ils n'ont jamais déclaré s'ouvrent désormais, au lieu d'être refusés comme défectueux.
* Les livres en encodages hérités s'ouvrent désormais beaucoup plus rapidement.
* Correction de certains fichiers texte chinois s'ouvrant en texte garbled.
* Les fichiers OpenDocument protégés par mot de passe demandent désormais leur mot de passe, au lieu d'être signalés comme défectueux.
* Les fichiers PowerPoint hérités protégés par mot de passe s'ouvrent désormais, et les diapositives PowerPoint hérités ne perdent plus leur texte.
* Les fichiers texte brut enregistrés avec une extension `.rtf` s'ouvrent désormais en tant que texte, au lieu d'échouer avec une erreur.
* Les mots de contrôle RTF n'apparaissent plus sous forme de texte.

#### iOS et Android

Les applications iOS et Android ouvrent tous les formats que le bureau offre, et incluent:

* Lecture à haute voix, avec le choix de la voix, du débit et de la hauteur, un contrôle du débit de parole directement sur la barre de lecture, et une pause optionnelle entre les paragraphes.
* Lecture d'audiobooks DAISY, M4B et MP3, qui continue en arrière-plan et depuis l'écran de verrouillage.
* Navigation par en-têtes, pages, liens, tableaux, listes et bien d'autres depuis la barre de lecture, plus la table des matières et Rechercher.
* Un minuteur de sommeil, un compte de mots et une exportation de document, plus un dictionnaire de parole sur iOS.
* Options de taille de texte, d'espacement et de texte à contraste élevé.
* Raccourcis clavier qui correspondent au bureau.

### Version 0.9.2
* Les audiobooks ne font plus lire à votre lecteur d'écran une série d'espaces lorsque vous mettez le focus sur le champ de texte.
* Les audiobooks nomment désormais le fichier au fur et à mesure que vous les traversez par section.
* Les audiobooks signalent désormais leur durée réelle, plutôt que de prétendre que chaque fichier dure 24 heures.
* La fermeture de la Web View avec Échap ne lève plus une alerte de débogage après que vous ayez suivi un lien à l'intérieur.
* La copie après Sélectionner tout vous donne désormais tout le document, au lieu de seulement la partie actuellement chargée.
* Rechercher va désormais directement à la ligne trouvée, au lieu de vous faire écouter le lecteur d'écran relire la fenêtre au fur et à mesure que le focus revient au livre.
* Correction des EPUB portant un bloc ZIP64 égare refusant de s'ouvrir avec "Invalid local file header".
* Correction des longs documents revenant à leur début tandis qu'un lecteur d'écran les lisait continuellement.
* Les liens dans la WebView vous amènent désormais à la section vers laquelle ils pointent, au lieu d'échouer avec "File not found".
* L'annonce automatique "Document rechargé" ne coupe plus votre lecteur d'écran en pleine phrase, attendant plutôt qu'il finisse ce qu'il disait.
* L'onglet Général du dialogue Paramètres parcourt désormais ses options dans l'ordre où elles apparaissent à l'écran, avec le canal de mise à jour directement après l'option de vérification des mises à jour.
* Windows affichera désormais toujours "Paperback" dans le menu Ouvrir avec, plutôt que le slogan complet du programme.
* Nombre de mots et Infos document affichent désormais combien de fichiers un audiobook contient et combien de temps il dure au total.

### Version 0.9.1
* Les sons des signets et des notes sont maintenant lus sur macOS.
* Les livres DAISY jouent maintenant leur audio sur macOS, au lieu de s'ouvrir et de suivre leur chronologie en silence.
* Correction des guillemets courbes, tirets em et caractères similaires disparaissant des documents RTF, fusionnant les mots environnants.
* Correction des images RTF fuyant leurs données brutes dans le document sous forme de texte brouillé.
* Correction du sous-menu Documents récents conservant les entrées obsolètes jusqu'à ce que quelque chose d'autre ne le reconstruise.
* Les accélérateurs clavier sont de retour dans chaque traduction, donc les menus russes ont à nouveau accès au clavier.
* Les grands documents CHM s'ouvrent maintenant jusqu'à sept fois plus rapidement.
* Les documents ouverts sont maintenant enregistrés avec Windows, ils apparaissent dans la liste de sauts de la barre des tâches et dans la liste récente du menu Démarrer.
* Options a été renommé en Paramètres, correspondant aux applications mobiles et, sur macOS, à la convention de la plateforme.
* Paperback mémorise maintenant sa position de fenêtre, sa taille et son état maximisé entre les exécutions.
* Les formes plurielles sont maintenant traduites, les messages qui comptent les choses sont lus correctement dans les langues qui ont besoin de plus d'une forme.
* La sélection du ncc.html d'un livre DAISY ouvre maintenant le livre audio complet au lieu de seulement son texte.
* Les noms d'actions de la boîte de dialogue Personnaliser les raccourcis clavier peuvent maintenant être traduits.
* Le titre du document figure maintenant en premier dans la barre de titre, de sorte que les livres ouverts peuvent être distingués dans la barre des tâches et Alt+Tab.
* La boîte de dialogue de mise à jour est maintenant traduite.

### Version 0.9.0

#### Ajouts

##### Général
* Un outil CLI, appelé pb, pour convertir rapidement n'importe quel format supporté par Paperback en HTML, Markdown ou texte brut.
* Une option pour recharger les documents modifiés par d'autres programmes sur le disque.
* Une option Afficher la source pour ouvrir la source d'un document dans un nouvel onglet, utile par exemple pour éditer du Markdown.
* Le texte du document est maintenant paginé, ce qui signifie que vous pouvez charger des livres avec des dizaines de millions de mots en seulement quelques secondes. Veuillez signaler toute anomalie trouvée avec ceci.

##### Support des plateformes
* Support de Windows ARM64 !
* Support natif de macOS !
* Un bouton de basculement du mode plein écran.

##### Boîte de dialogue Tous les documents
* Un bouton localiser pour localiser les livres manquants qui viennent de changer de chemin.
* Un filtre d'état et une barre d'état, vous pouvez donc filtrer par état du document et voir combien de documents sont affichés et sélectionnés.
* Le raccourci `Ctrl+Shift+A` pour désélectionner tous les documents.

##### Options et lisibilité
* Un onglet lisibilité, avec les options suivantes :
    * Retour à la ligne automatique (déplacé depuis général);
    * Afficher les tableaux en ligne (nouveau dans cette version, voir ci-dessous);
    * Police;
    * Couleur de fond;
    * Interligne;
    * Espacement des paragraphes;
    * Espacement des lettres;
    * Alignement du texte.
* Un élément de menu retour à la ligne automatique et une touche de raccourci correspondante.
* Un bouton pour déterminer comment vous souhaitez afficher les tableaux, et harmonisation de l'affichage des tableaux dans les documents.

##### Navigation
* Support de la navigation par conteneur.
* Une option pour déplacer automatiquement le curseur au début de la ligne lors de la navigation entre les lignes, similaire au mode de navigation dans les lecteurs d'écran.
* Le raccourci clavier égal pour annoncer votre pourcentage actuel dans un document.

##### Signets
* Signets temporaires : vous pouvez en avoir un par document, et ils persistent. Utilisez la barre oblique pour en définir un et la barre oblique inverse pour sauter à celui-ci.

##### Compte des mots
* Temps de lecture estimé dans la boîte de dialogue compte des mots, ainsi que la possibilité de définir votre vitesse de lecture pour rendre cette métrique réellement utile.
* Si une sélection est active lorsque vous ouvrez la boîte de dialogue compte des mots, le nombre de mots sélectionnés sera maintenant affiché.

##### Raccourcis clavier
* La capacité à personnaliser chaque raccourci clavier dans l'application via une boîte de dialogue simple.
* Un raccourci clavier configurable pour restaurer Paperback à partir du bac système.

##### Langues
* Néerlandais, finnois et polonais.

##### Export
* Expansion de l'élément de menu export pour permettre l'exportation en HTML et Markdown, en plus du texte brut.

##### Mise à jour
* Un bouton annuler dans la boîte de dialogue mise à jour en cours.
* Le programme de mise à jour valide maintenant que le fichier téléchargé n'a pas été modifié.

##### Vue web
* La vue web s'ouvre maintenant à votre position de lecture actuelle.

##### Livres DAISY
* Support des livres DAISY 2.0.
* Support de la lecture audio DAISY 2.02.

##### Livres audio
* La capacité à lire des livres audio, supportant actuellement à la fois DAISY audio (y compris DAISY audio + texte) et les zips de fichiers audio.
* Les raccourcis clavier et les éléments de menu pour lire/mettre en pause la narration, chercher en avant et en arrière, et ajuster la quantité de recherche.
* Les options de synchroniser le curseur de lecture à la lecture audio, définir la quantité de recherche audio et choisir si la recherche au-delà de la fin d'un chapitre continue dans le suivant.

##### Documents CHM
* Support des listes, éléments de liste, figures et images.

##### PowerPoint
* Les documents PowerPoint supportent maintenant les tableaux.

#### Corrigé

##### Général
* Les documents encodés dans des encodages CJK hérités, tels que GBK, Big5 et Shift_JIS, s'affichent maintenant correctement au lieu de s'afficher sous forme de mojibake.
* « Rouvrir le dernier fermé » tentait de rouvrir le readme groupé.
* Votre onglet sélectionné ne recevant pas correctement le focus après le redémarrage de Paperback.
* La gestion des fichiers de Paperback sur les lecteurs réseau Windows : appuyer sur « Afficher le fichier dans le dossier » met maintenant correctement en évidence le fichier sur le stockage réseau, et les chemins ne contiennent plus de caractères étranges.
* Les fichiers .paperback ne seront plus forcément chargés lors de la restauration du document ; à la place, vous serez invité à confirmer quand un est trouvé.
* L'ouverture du dossier contenant met maintenant en évidence le fichier donné dans l'explorateur.
* L'ouverture du readme respectera maintenant votre langue sélectionnée.
* L'interface utilisateur de Paperback s'adaptera maintenant correctement sur les écrans haute résolution.
* Le menu se met à jour correctement maintenant, et le focus se déplace vers la commande texte, lors de l'ouverture de l'aide dans Paperback.
* Passage à une méthode IPC beaucoup plus sécurisée sur Windows.
* Le titre du document actif sera maintenant lu lors du basculement entre les onglets.
* Réduction de l'utilisation de la mémoire sur les grands documents en réduisant de moitié la taille des tableaux d'index internes par caractère.

##### Dialogue Tous les documents
* Échap ne fermant pas les dialogues Informations sur le document et Tous les documents.
* La barre de titre ne se met pas à jour après la fermeture d'un document à partir de la boîte de dialogue tous les documents.
* Readme.html ne sera plus ajouté à votre liste de tous les documents lors de l'ouverture via `Shift+F1`.
* La suppression de documents du dialogue des récents fermera maintenant également leur onglet actif.
* Votre filtre de recherche est maintenant préservé après la suppression d'un document.

##### Navigation
* La navigation des pages annonçant un texte de ligne incorrect dans certaines situations.
* Aller à la ligne, Aller à la page et Aller au pourcentage plaçant votre curseur à la mauvaise position dans les grands documents.
* Trouver et Trouver suivant ne respectant pas la fenêtre du document chargé dans les grands documents.

##### Signets
* Les sons de signet/note devraient maintenant se jouer correctement exclusivement quand vous naviguez sur un mot en contenant un.

##### Lisibilité
* L'application du retour à la ligne vous envoyant au début de votre document.

##### Affichage Web
* La boîte de dialogue webview n'étant pas redimensionnable et s'affichant à une taille initiale très petite.
* Les images devraient maintenant s'afficher correctement dans la webview intégrée.

##### Mise à jour
* Le programme de mise à jour affiche maintenant correctement le contenu des balises de code markdown dans les notes de version.

##### Livres DAISY
* Les livres DAISY affichant des informations incorrectes dans la barre d'état.
* Chargement de livres DAISY avec des déclarations d'encodage fausses.

##### Documents RTF
* Analyse des documents RTF avec des caractères non latins dedans.
* Les groupes RTF `\pict` pour que les données d'image intégrées ne s'échappent plus dans le texte du document.

##### Livres Mobi/AZW3
* Les ancres filepos dans les livres Mobi divisant les balises HTML et mettant des ordures dans le texte du livre.
* Les liens dans les livres Mobi hérités.
* Analyse AZW3 considérablement améliorée.

##### Documents Word
* Les documents Word avec des noms de style localisés ne rendant pas correctement leurs en-têtes.

##### Documents HTML/XHTML
* Les éléments dl, dt et dd ne produisant pas de sauts de ligne dans les documents XHTML.

##### Documents PDF
* Paperback revient maintenant à l'extraction de texte brut pour les PDF incorrectement balisés.
* Les documents PDF contenant des caractères de contrôle dans leurs titres et/ou signets ne bloqueront plus Paperback à l'ouverture.

### Version 0.8.5
* Ajout de la prise en charge des pages aux livres epub.
* Ajout de la prise en charge des documents Microsoft Office chiffrés. Actuellement, le Word hérité, le Word moderne et le Powerpoint moderne sont pris en charge, avec Powerpoint hérité prévu pour l'avenir.
* Ajout de la prise en charge des documents Microsoft Word hérités !
* Ajout de la prise en charge des présentations Powerpoint héritées !
* Ajout de la prise en charge des livres mobi et AZW3 !
* Ajout de la prise en charge des fichiers PDF balisés !
* Ajout du raccourci `ctrl+q` pour quitter l'application.
* Ajout de la prise en charge des livres compressés de Bookshare (à la fois DAISY et Word) !
* Le texte alternatif des images intégrées devrait maintenant s'afficher correctement.
* Les documents CHM prennent maintenant correctement en charge la navigation des liens internes.
* Correction de la page de destination étant décalée de 1.
* Correction de la touche Échap ne fonctionnant pas pour fermer la boîte de dialogue ouvrir en tant que.
* Correction du menu contextuel du lecteur ne s'affichant pas au clic droit ou à la touche Applications.
* Correction du mauvais document parfois mis en focus lors de l'ouverture de documents à partir de la ligne de commande.
* Les PDF contenant uniquement des images sont à nouveau détectés et vous alertent de leur existence.
* Il est maintenant possible de naviguer dans les images et les figures avec `g`/`shift+g` et `f`/`shift+f`, respectivement.
* Paperback respectera maintenant votre paramètre de mode sombre de l'application.
* Suppression de la prise en charge de DAISY XML, car elle n'est plus nécessaire.
* Retour à la navigation Win32 native de la première lettre dans l'arborescence de la table des matières.
* La boîte de dialogue de chargement d'erreur affiche maintenant des messages d'erreur plus détaillés.
* La webview s'ouvrira maintenant beaucoup plus rapidement et en douceur.

### Version 0.8.2
* Ajout de la prise en charge des pages aux documents RTF !
* Correction d'un bogue où l'ouverture de la webview dans les epub contenant des liens externes les activerait automatiquement.
* Correction d'un bogue où l'analyseur RTF ne mettrait pas d'espace entre les mots dans de rares cas.
* Correction des paragraphes étant divisés en plusieurs lignes courtes dans certains documents PDF.
* Les documents PDF ont maintenant une prise en charge basique de la navigation des liens et des en-têtes !
* Les onglets et les sauts de ligne RTF sont maintenant rendus exactement comme ils apparaissent dans le document.
* Retour à la bibliothèque pdfium fiable et éprouvée pour l'analyse des PDF, rendant le rendu PDF beaucoup plus fiable à nouveau.

### Version 0.8.1
* Ajout de `Ctrl+Shift+T` pour rouvrir le dernier document fermé.
* La boîte de dialogue Tous les documents prend maintenant en charge la sélection de plusieurs documents à ouvrir à la fois.
* Correction de quelques bogues avec l'analyseur RTF.
* Correction des chemins de fichiers contenant des caractères non-ASCII (tels que le bosniaque š, č, ć, ž) devenant corrompus lors de l'ouverture d'un fichier via une deuxième instance de Paperback.
* Correction du texte PDF lu dans le mauvais ordre et des espaces incorrects autour des mots en majuscules.
* Correction du chargement lent de documents lors de l'ouverture de fichiers volumineux.
* Correction de la localisation des boutons Oui/Non dans les boîtes de dialogue de confirmation.

### Version 0.8.0
* Added Japanese, simplified Chinese, and Vietnamese translations!
* Added an automatic updater that will now replace your currently installed version of Paperback instead of just downloading the new version!
* Added optional sound feedback for reaching a bookmark or a note, thanks Andre Louis for the sounds!
* Added RTF document support!
* Added support for DAISY XML documents.
* Added support for Flat Open Document Text files!
* Added support for Flat Open Document presentations!
* Added support for separators with s and shift+s.
* Any movement of greater than 300 characters will now automatically add to your navigation history.
* Fixed restoring Paperback's window from the system tray.
* Fixed Markdown documents showing raw text instead of rendered HTML in the Web View.
* Fixed tables not rendering properly in Markdown files.
* Image only PDFs will now warn you of their existence when you attempt to load one.
* Properly embed version information in the Paperback executable.
* Split the options dialog into tabs for ease of use and navigation.
* Switched to Hayro for parsing PDFs, leading to more reliability, speed, and fewer DLLs.
* Rewrote the entire app in Rust. The new codebase is safer, loads documents faster, and is easier to maintain and extend.
* The text control's context menu will now include reader-specific actions instead of generic items such as cut and paste.

### Version 0.7.0
* Added table support for HTML and XHTML-based documents! Navigate between tables using T and Shift+T, and press Enter to view one in a webview.
* Added a basic web rendering feature! Press Ctrl+Shift+V to open the current section of your document in a web-based renderer, useful for content like complex formatting or code samples.
* Added a Russian translation, thanks Ruslan Gulmagomedov!
* Added a Clear All button to the All Documents dialog.
* The update checker now displays release notes when a new version is available.
* Fixed restoring the window from the system tray.
* Fixed Yes/No button translations in confirmation dialogs.
* Fixed loading configs when running as administrator.
* Fixed comment handling in XML and HTML documents.
* Fixed TOC parsing in Epub 2 books.
* Fixed navigating to the next item with the same letter in the table of contents.
* Fixed the find dialog not hiding properly when using the next/previous buttons.
* Fixed epub TOC's occasionally throwing you to the wrong item.
* Fixed various whitespace handling issues in XML, HTML, and pre tags.
* Fixed off-by-one error in link navigation.
* Fixed some books having trailing whitespace on their lines.
* Fixed various parser issues.
* Bookmark-related menu items as well as the elements list are now properly disabled when no document is open.
* Improved list handling in various document formats.
* Improved the translation workflow for contributors.
* Many internal refactors, moving the majority of the application's business logic from C++ to Rust for improved performance and maintainability.

### Version 0.6.1
* Added password-protected PDF support!
* Added a very basic go to previous/next position feature. If you press enter on an internal link and it moves your cursor, that position will now be remembered, and can be navigated to with alt+left/right arrows.
* Added an elements list! Currently it only shows a tree of all the headings in your document or a list of links, but there are plans to expand it in the future.
* Added an option to start Paperback in maximized mode by default.
* Fixed links in some Epub documents not working properly.
* Fixed parsing Epub TOCs containing relative paths.
* Fixed some epub documents not showing a title or author.
* Fixed the titles of some epub chapters not showing up properly in the TOC dialog.
* Fixed you not being able to use the space bar to activate the OK/cancel buttons in the TOC dialog.
* Improved the handling of headings in Word documents.
* You will now get spoken feedback if the recent documents list is empty when you try to bring up the dialog.

### Version 0.6.0
* Une nouvelle option pour afficher le menu de navigation sous une forme beaucoup plus compacte a été ajoutée à la boîte de dialogue des options, activée par défaut.
* Ajout d'une option pour que la navigation par éléments structurels s'enroule.
* Ajout d'une option au menu outils pour ouvrir le dossier contenant le document actuellement actif.
* Ajout d'un système de mise à jour simple mais très efficace.
* Ajout d'une fonction de minuteur de veille basique, accessible avec `Ctrl+Shift+S`.
* Ajout du support de l'analyse des ebooks FB2 !
* Ajout du support de l'analyse des présentations OpenDocument !
* Ajout du support de l'analyse des fichiers OpenDocument Text !
* Les signets peuvent maintenant marquer une ligne entière, ou seulement du texte spécifié. Si vous n'avez pas de sélection active lors du placement d'un signet, le comportement est comme avant la version 0.6, et il marquera la ligne entière. Cependant, si vous sélectionnez du texte, seul ce texte sera inclus dans le signet.
* Les signets peuvent maintenant avoir des notes de texte optionnelles attachées ! Naviguez entre les signets contenant des notes avec N et `Shift+N`, ou ouvrez la boîte de dialogue des signets avec tous les signets, uniquement les notes, ou uniquement les non-notes sélectionnés avec des touches de raccourci spécifiques.
* Les signets dans la boîte de dialogue des signets n'auront plus de préfixe ennuyeux « signet x ».
* Les livres Epub contenant du contenu HTML prétendant être du XML seront maintenant traités correctement.
* Correction du chargement de gros documents Markdown.
* Correction de l'appui sur la barre d'espace dans l'arborescence de la table des matières activant le bouton OK.
* Correction de la gestion des espaces au début des balises pre dans les documents HTML et XHTML.
* Correction du contrôle de texte ne reprenant pas le focus parfois lors du retour à la fenêtre de Paperback.
* Correction du champ de texte dans la boîte de dialogue « aller au pourcentage » ne mettant pas à jour la valeur du curseur.
* Correction du rendu des ID HTML personnalisés dans les documents Markdown.
* Le HTML à l'intérieur des blocs de code Markdown sera maintenant rendu correctement.
* Si vous chargez un livre avec un paramètre de ligne de commande alors qu'une instance Paperback existante est en cours d'exécution, vous n'obtiendrez plus d'erreur si le chargement de votre document prend plus de 5 secondes.
* Si Paperback s'exécute en tant qu'administrateur, la configuration sera maintenant correctement chargée et sauvegardée.
* Il est maintenant possible de supprimer un signet directement à partir de la boîte de dialogue des signets.
* Il est maintenant possible d'importer et d'exporter vos signets et votre position de lecture pour un document particulier. Le fichier généré est nommé d'après le fichier avec une extension `.paperback`. Si un tel fichier est trouvé dans le même répertoire qu'un fichier lors de son chargement, il sera automatiquement chargé. Sinon, vous pouvez les importer manuellement en utilisant un élément du menu outils.
* Les liens à l'intérieur des documents sont maintenant entièrement pris en charge ! Utilisez k et `shift+k` pour vous déplacer d'avant en arrière à travers eux, et appuyez sur entrée pour en ouvrir/activer un.
* De nombreuses refactorisations internes, rendant l'application plus rapide et le binaire plus petit.
* Le contenu Markdown est maintenant prétraité pour être conforme à CommonMark avant le rendu.
* La navigation par listes et leurs éléments est maintenant entièrement prise en charge ! Utilisez L et `Shift+L` pour naviguer par les listes elles-mêmes, et I et `Shift+I` pour parcourir les éléments de liste.
* Suppr du pavé numérique fonctionne maintenant pour supprimer des documents de la barre d'onglets en plus de la touche supprimer normale.
* Paperback peut maintenant s'minimiser optionnellement dans votre barre système ! Cette option est désactivée par défaut, mais l'activer fera que l'option de minimisation dans le menu système placera Paperback dans votre barre système, pouvant être restauré en cliquant sur l'icône générée.
* Paperback est maintenant entièrement traductible ! La liste des langues qu'il supporte est actuellement assez restreinte, mais elle s'agrandit constamment !
* Paperback dispose maintenant d'un site Web officiel, sur [paperback.dev](https://paperback.dev) !
* Les documents PPTX affichent maintenant une table des matières basique, contenant toutes les diapositives.
* Le chemin complet du document ouvert s'affiche maintenant dans la boîte de dialogue des informations sur le document.
* Le programme d'installation inclut maintenant une option pour afficher le fichier lisezmoi dans votre navigateur après l'installation.
* La liste des documents récents a été considérablement améliorée ! Au lieu d'afficher simplement les 10 derniers documents que vous avez ouverts, elle affichera maintenant un nombre personnalisable, les autres documents que vous avez jamais ouverts étant accessibles via une petite boîte de dialogue.
* Diverses petites améliorations des analyseurs dans l'ensemble, y compris l'insertion d'une ligne vierge entre les diapositives dans les présentations PPTX, la correction de la gestion des sauts de ligne à l'intérieur des paragraphes dans les documents Word, et l'ajout de puces aux éléments de liste.

### Version 0.5.0
* Ajout de la prise en charge des documents Microsoft Word !
* Ajout de la prise en charge des présentations PowerPoint !
* Correction de certains éléments de menu qui n'étaient pas désactivés sans documents ouverts.
* Correction de l'orientation du curseur du pourcentage.
* Correction de la table des matières dans les livres Epub avec des chemins de fichiers codés en URL et/ou des identifiants de fragment.
* Correction de l'espacement blanc supprimé des en-têtes XHTML de façons bizarres.
* Correction de la gestion de l'espacement blanc à l'intérieur des balises pre imbriquées dans les documents HTML.
* Les documents HTML et Markdown supportent maintenant la fonction de table des matières ! Lorsque vous chargez un document HTML/Markdown, Paperback construira sa propre table des matières à partir de la structure des en-têtes de votre document, et l'affichera dans la boîte de dialogue `ctrl+t`.
* Les documents HTML afficheront désormais le titre défini dans la balise title, s'il existe. Sinon, ils continueront à utiliser le nom de fichier sans l'extension.
* Passage de UniversalSpeech à l'utilisation d'une région active pour signaler la parole. Cela signifie qu'aucune DLL de lecteur d'écran n'est plus fournie avec le programme, et plus de lecteurs d'écran seront désormais pris en charge, comme Microsoft Narrator.
* Changement des bibliothèques zip pour permettre l'ouverture d'un plus large éventail de livres epub.
* La boîte de dialogue vous demandant si vous souhaitez ouvrir votre document en texte brut a été complètement refaite, et permet maintenant d'ouvrir votre document en texte brut, HTML ou Markdown.
* La boîte de dialogue du pourcentage inclut maintenant un champ de texte permettant d'entrer manuellement un pourcentage auquel sauter.
* L'analyseur HTML reconnaît maintenant les éléments `dd`, `dt` et `dl` comme des éléments de liste.
* La table des matières dans les livres Epub sera à nouveau préservée exactement.
* L'espace insécable Unicode est maintenant pris en compte lors de la suppression des lignes vides.
* Vous ne serez plus demandé comment vous souhaitez ouvrir un fichier non reconnu chaque fois que vous le chargez, seulement la première fois.

### Version 0.4.1
* Ajout d'une icône de menu Démarrer optionnelle au programme d'installation.
* La table des matières devrait maintenant être plus propre dans quelques cas, par exemple si vous avez un élément enfant et parent avec le même texte à la même position, vous ne verrez que l'élément parent.
* Correction de la table des matières dans certains documents CHM.
* Correction de la table des matières dans les livres Epub 3 avec des chemins absolus.
* Les documents CHM devraient maintenant afficher leur titre tel que défini dans le fichier de métadonnées.

### Version 0.4.0
* Ajout de la prise en charge des fichiers CHM !
* Ajout de la prise en charge des signets ! Vous pouvez avoir autant de signets que vous le souhaitez dans autant de documents que vous le souhaitez. Vous pouvez naviguer vers l'avant et vers l'arrière avec `b` et `shift+b`, en définir un avec `control+shift+b`, et afficher une boîte de dialogue pour sauter à un signet spécifique avec `control+b`.
* Ajout d'un programme d'installation à côté du fichier zip portable ! Le programme d'installation installera Paperback dans votre répertoire Program Files et configurera automatiquement les associations de fichiers pour vous.
* Les fichiers texte avec BOM devraient maintenant être décodés correctement, et le BOM ne s'affichera plus au début du texte non plus.
* Ajout de beaucoup plus d'informations à la barre d'état. Elle affichera maintenant votre ligne actuelle, votre caractère et votre pourcentage de lecture.
* Les commentaires HTML, ainsi que le contenu des balises script et style, ne s'afficheront plus dans la sortie texte.
* Si vous transmettez un chemin relatif à Paperback en ligne de commande, il le résoudra maintenant correctement.
* Le mouvement de pourcentage est maintenant géré par sa propre boîte de dialogue basée sur un curseur, accessible avec `control+shift+g`.
* Les documents sans titres ou auteurs connus auront maintenant toujours une valeur par défaut.
* La logique de sauvegarde de position est maintenant beaucoup plus intelligente et ne devrait écrire sur le disque que si absolument nécessaire.
* Le document sur lequel vous aviez le focus lorsque vous avez fermé Paperback est maintenant mémorisé lors des redémarrages de l'application.
* L'entrée dans les boîtes de dialogue aller à la ligne et aller à la page devrait maintenant être désinfectée plus strictement.
* Correction de la navigation de la table des matières dans les livres epub 3 avec des chemins relatifs dans leurs manifestes.

### Version 0.3.0
* Correction de la table des matières dans les livres epub avec des manifestes codés en URL.
* Correction de la navigation par en-têtes dans les documents HTML contenant des caractères Unicode multi-octets.
* Correction d'une utilisation élevée du CPU dans les documents avec de longs titres due à une régression dans wxWidgets.
* Correction du chargement des fichiers texte UTF-8.
* Correction des éléments TOC imbriqués dans les livres Epub mettant votre curseur à la mauvaise position.
* Correction d'un crash à la fermeture de l'application dans certains cas.
* Ajout d'une case à cocher dans la boîte de dialogue des options pour activer ou désactiver le retour à la ligne !
* Il est maintenant possible de faire un don au développement de Paperback, soit par le nouvel élément Donner dans le menu Aide, soit par le lien Parrainez ce projet en bas de la page principale du dépôt GitHub.
* Les documents Markdown auront maintenant toujours un titre, et Paperback devrait maintenant pouvoir charger pratiquement n'importe quel fichier Markdown.
* Les documents PDF auront maintenant toujours un titre, même si les métadonnées manquent.
* Changement des bibliothèques PDF pour celle utilisée dans Chromium, ce qui permet une analyse PDF beaucoup plus fiable dans l'ensemble.
* Vous ne pouvez maintenant avoir qu'une seule instance de Paperback en cours d'exécution à la fois. L'exécution de `paperback.exe` avec un nom de fichier alors qu'il est déjà en cours d'exécution ouvrira ce document dans l'instance déjà en cours d'exécution.
* Vous pouvez maintenant appuyer sur Delete sur un document dans le contrôle d'onglet pour le fermer.

### Version 0.2.1
* Ajout du nombre total de pages à l'étiquette de page dans la boîte de dialogue aller à la page.
* Autorisation de la tabulation du contenu du document à votre liste de documents ouverts.
* Correction des raccourcis clavier d'en-têtes ouvrant parfois des documents récents si vous en aviez suffisamment.
* Paperback supprimera maintenant les traits d'union légers inutiles de la sortie texte.
* Correction de la navigation par en-têtes mettant parfois le curseur au mauvais caractère.

### Version 0.2.0
* Ajout du support des documents markdown !
* Ajout du support des documents PDF, y compris la possibilité de naviguer entre les pages !
* Ajout de touches de clavier pour naviguer par rubriques dans le contenu HTML, y compris les livres epub et les documents markdown. Ces touches de clavier ont été conçues pour fonctionner de manière similaire à un lecteur d'écran.
* Correction du chargement des epub avec des noms de fichiers codés en URL dans leurs manifestes.
* Correction du chargement des livres epub 3 avec XHTML intégré à l'intérieur.
* Un message est maintenant énoncé si le document ne prend pas en charge une table des matières ou des sections, plutôt que d'avoir les éléments de menu désactivés.
* Ajout d'un menu des documents récents ! Il stocke actuellement vos 10 derniers documents ouverts, et appuyer sur Entrée sur l'un d'eux l'ouvrira pour la lecture.
* Réécriture complète de la boîte de dialogue Rechercher, la rendant beaucoup plus simple à utiliser, tout en ajoutant un historique de vos 25 dernières recherches et le support des expressions régulières !
* Les documents précédemment ouverts sont maintenant mémorisés lors des redémarrages de l'application. Ceci est configurable via le nouvel élément options du menu outils.
* Ajout de `Shift+F1` pour ouvrir le readme directement dans Paperback.

### Version 0.1.0
* Première version.
